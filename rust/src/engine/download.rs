use std::path::{Path, PathBuf};
use std::time::Duration;

use futures_util::StreamExt;
use reqwest::StatusCode;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

use crate::engine::segments::{plan_segments, ByteRange};

const MAX_SEGMENTS: u64 = 16;
const PROGRESS_BYTE_STEP: u64 = 64 * 1024; // report at most every 64 KiB per segment

#[derive(Debug, Clone)]
pub enum ProgressEvent {
    Started { total_bytes: Option<u64>, num_segments: u64 },
    SegmentProgress { index: usize, bytes_downloaded: u64 },
    SegmentDone { index: usize },
    Merged { total_bytes: u64 },
}

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("server returned status {0}")]
    BadStatus(u16),
    #[error("destination path has no file name: {0}")]
    BadDestPath(PathBuf),
    #[error("segment {index} incomplete: expected {expected} bytes, got {got}")]
    Incomplete { index: usize, expected: u64, got: u64 },
    #[error("segment task failed: {0}")]
    TaskFailed(String),
}

/// HEAD the URL: how big is the file, and will the server honor Range requests?
/// Returns `total_len = None` when the server doesn't report Content-Length
/// (e.g. chunked responses) - callers must fall back to a single unranged GET.
pub async fn probe(client: &reqwest::Client, url: &str) -> Result<(Option<u64>, bool), EngineError> {
    let resp = client.head(url).send().await?;
    if !resp.status().is_success() {
        return Err(EngineError::BadStatus(resp.status().as_u16()));
    }
    let len = resp
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());
    let accepts_ranges = resp
        .headers()
        .get(reqwest::header::ACCEPT_RANGES)
        // RFC 9110: the range-unit token is case-insensitive.
        .map(|v| v.as_bytes().eq_ignore_ascii_case(b"bytes"))
        .unwrap_or(false);
    Ok((len, accepts_ranges))
}

/// Downloads one byte range (or, if `range` is `None`, the whole resource) to its
/// own temp file. Each segment gets its own `File` handle so segments never
/// contend on a shared file position - the tradeoff is a merge step at the end.
///
/// `expected_len` is how many bytes this segment must produce: the range length
/// for a ranged segment, or the resource's `Content-Length` for the unranged
/// fallback. `None` only when the server never reported a length at all.
async fn download_segment(
    client: reqwest::Client,
    url: String,
    range: Option<ByteRange>,
    expected_len: Option<u64>,
    index: usize,
    segment_path: PathBuf,
    progress: mpsc::Sender<ProgressEvent>,
) -> Result<(), EngineError> {
    let mut req = client.get(&url);
    if let Some(r) = range {
        req = req.header(reqwest::header::RANGE, r.header_value());
    }
    let resp = req.send().await?;

    // If we asked for a range, the server must honor it with 206 - a plain 200
    // means it ignored the header and is about to hand us the entire file,
    // which would silently corrupt the merge if we accepted it.
    let expected_status = if range.is_some() { StatusCode::PARTIAL_CONTENT } else { StatusCode::OK };
    if resp.status() != expected_status {
        return Err(EngineError::BadStatus(resp.status().as_u16()));
    }

    let mut file = File::create(&segment_path).await?;
    let mut stream = resp.bytes_stream();
    let mut downloaded: u64 = 0;
    let mut last_reported: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;

        if downloaded - last_reported >= PROGRESS_BYTE_STEP {
            last_reported = downloaded;
            let _ = progress.try_send(ProgressEvent::SegmentProgress { index, bytes_downloaded: downloaded });
        }
    }

    // Durable on disk before we report done, and before merge_segments reads it back.
    file.flush().await?;
    file.sync_all().await?;

    // Applies to the unranged fallback as much as to a ranged segment: a connection
    // that drops mid-transfer ends the stream cleanly from the client's point of
    // view, so without this check a truncated download reports success and is
    // merged into the destination as if it were whole.
    if let Some(expected) = expected_len {
        if downloaded != expected {
            return Err(EngineError::Incomplete { index, expected, got: downloaded });
        }
    }

    let _ = progress.try_send(ProgressEvent::SegmentDone { index });
    Ok(())
}

/// Downloads `url` to `dest` using up to `num_segments` parallel connections.
/// Falls back to a single unranged GET if the server doesn't support ranges or
/// didn't report a length.
pub async fn download(
    url: &str,
    dest: &Path,
    num_segments: u64,
    progress: mpsc::Sender<ProgressEvent>,
) -> Result<(), EngineError> {
    let file_name = dest
        .file_name()
        .ok_or_else(|| EngineError::BadDestPath(dest.to_path_buf()))?
        .to_string_lossy()
        .into_owned();
    let tmp_dir = dest.parent().unwrap_or_else(|| Path::new("."));

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(300))
        .build()?;

    let (total_len, accepts_ranges) = probe(&client, url).await?;

    // Some(range) per segment, or a single None meaning "one segment, no Range header".
    let ranges: Vec<Option<ByteRange>> = match total_len {
        Some(len) if accepts_ranges && len > 0 => {
            let capped = num_segments.clamp(1, MAX_SEGMENTS);
            plan_segments(len, capped).into_iter().map(Some).collect()
        }
        _ => vec![None],
    };

    let _ = progress
        .send(ProgressEvent::Started {
            total_bytes: total_len,
            num_segments: ranges.len() as u64,
        })
        .await;

    // Paths are built up front and owned here, not recovered from the tasks, so
    // that cleanup on failure can reach every planned segment - including the one
    // whose task failed, which is the likeliest to have left a partial file behind.
    let mut segment_paths = Vec::with_capacity(ranges.len());
    let mut tasks = Vec::with_capacity(ranges.len());
    for (index, range) in ranges.into_iter().enumerate() {
        let segment_path = tmp_dir.join(format!("{file_name}.part{index}"));
        segment_paths.push(segment_path.clone());
        // A ranged segment owes exactly its range; the unranged fallback owes the
        // whole resource, whenever the server told us how big that is.
        let expected_len = range.map(|r| r.len()).or(total_len);
        tasks.push(tokio::spawn(download_segment(
            client.clone(), // Client is internally Arc'd already - no extra Arc needed
            url.to_string(),
            range,
            expected_len,
            index,
            segment_path,
            progress.clone(),
        )));
    }

    let mut first_error: Option<EngineError> = None;
    for task in tasks {
        match task.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => { first_error.get_or_insert(e); }
            Err(join_err) => {
                first_error.get_or_insert(EngineError::TaskFailed(join_err.to_string()));
            }
        }
    }

    if let Some(err) = first_error {
        // Best-effort cleanup: don't let a failed unlink mask the real error.
        for path in &segment_paths {
            match tokio::fs::remove_file(path).await {
                Ok(()) => {}
                // Expected, not a problem: a segment that failed before creating
                // its file - or never got scheduled - leaves nothing to remove.
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => eprintln!("warning: failed to remove temp segment {path:?}: {e}"),
            }
        }
        return Err(err);
    }

    let total_bytes = merge_segments(&segment_paths, dest).await?;
    let _ = progress.try_send(ProgressEvent::Merged { total_bytes });
    Ok(())
}

/// Concatenates segment files, in order, into a temp file next to `dest`, then
/// atomically renames it into place. A failed or interrupted merge never
/// truncates a pre-existing file at `dest` - the old file (if any) stays untouched
/// until the new one is fully written and flushed.
async fn merge_segments(segment_paths: &[PathBuf], dest: &Path) -> Result<u64, EngineError> {
    let tmp_dest = dest.with_extension("part-merge");

    let mut total = 0u64;
    {
        let mut out = File::create(&tmp_dest).await?;
        for path in segment_paths {
            let mut part = File::open(path).await?;
            total += tokio::io::copy(&mut part, &mut out).await?;
        }
        out.flush().await?;
        out.sync_all().await?;
    } // `out` closes here, before the rename

    tokio::fs::rename(&tmp_dest, dest).await?;

    for path in segment_paths {
        if let Err(e) = tokio::fs::remove_file(path).await {
            eprintln!("warning: failed to remove temp segment {path:?}: {e}");
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn merge_concatenates_segments_in_order_and_renames_atomically() {
        let dir = std::env::temp_dir().join(format!("frb_merge_test_{}", std::process::id()));
        tokio::fs::create_dir_all(&dir).await.unwrap();

        let seg0 = dir.join("seg0");
        let seg1 = dir.join("seg1");
        tokio::fs::write(&seg0, b"hello, ").await.unwrap();
        tokio::fs::write(&seg1, b"world!").await.unwrap();

        let dest = dir.join("merged.txt");
        tokio::fs::write(&dest, b"OLD CONTENT").await.unwrap(); // pre-existing file

        let total = merge_segments(&[seg0.clone(), seg1.clone()], &dest).await.unwrap();
        assert_eq!(total, 13);
        assert_eq!(tokio::fs::read(&dest).await.unwrap(), b"hello, world!");
        assert!(tokio::fs::metadata(&seg0).await.is_err()); // temp files cleaned up
        assert!(tokio::fs::metadata(&seg1).await.is_err());

        tokio::fs::remove_dir_all(&dir).await.ok();
    }

    /// End-to-end download against a real server. Ignored by default so the normal
    /// `cargo test` run stays offline and deterministic. Run it explicitly:
    ///
    /// ```text
    /// DLM_TEST_URL='https://.../video.mp4' \
    ///   cargo test downloads_a_real_file -- --ignored --nocapture
    /// ```
    ///
    /// The URL must point at the bytes themselves. A YouTube watch page is HTML,
    /// not a video - resolve it to a stream URL first (see DLM_TEST_URL below).
    /// Optional: DLM_TEST_SEGMENTS (default 8), DLM_TEST_DIR (default a temp dir).
    #[tokio::test]
    #[ignore = "hits the real network; needs DLM_TEST_URL"]
    async fn downloads_a_real_file() {
        let url = std::env::var("DLM_TEST_URL")
            .expect("set DLM_TEST_URL to a direct file URL");
        let segments: u64 = std::env::var("DLM_TEST_SEGMENTS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8);

        // Name the output after the URL path, ignoring any query string: signed CDN
        // URLs (googlevideo, S3 presigned) carry huge query strings that aren't a name.
        let file_name = url
            .split('?')
            .next()
            .unwrap_or("")
            .rsplit('/')
            .find(|s| !s.is_empty())
            .unwrap_or("download.bin")
            .to_string();

        let dir = std::env::var("DLM_TEST_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir().join("dlm_real_download"));
        tokio::fs::create_dir_all(&dir).await.unwrap();
        let dest = dir.join(&file_name);

        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build client");
        let (total_len, accepts_ranges) = probe(&client, &url).await.expect("probe failed");
        println!("probe: content-length={total_len:?} accept-ranges={accepts_ranges}");
        println!("plan: {} segment(s) -> {}", if total_len.is_some() && accepts_ranges { segments.min(MAX_SEGMENTS) } else { 1 }, dest.display());

        let (tx, mut rx) = mpsc::channel(256);
        let printer = tokio::spawn(async move {
            use std::collections::HashMap;
            let mut per_segment: HashMap<usize, u64> = HashMap::new();
            while let Some(event) = rx.recv().await {
                match event {
                    ProgressEvent::Started { total_bytes, num_segments } => {
                        println!("  started: total_bytes={total_bytes:?} segments={num_segments}");
                    }
                    ProgressEvent::SegmentProgress { index, bytes_downloaded } => {
                        per_segment.insert(index, bytes_downloaded);
                        let sum: u64 = per_segment.values().sum();
                        let pct = total_len
                            .map(|t| format!(" ({:.1}%)", sum as f64 / t as f64 * 100.0))
                            .unwrap_or_default();
                        print!("\r  {} KiB{pct}   ", sum / 1024);
                        let _ = std::io::Write::flush(&mut std::io::stdout());
                    }
                    ProgressEvent::SegmentDone { index } => println!("\r  segment {index} done          "),
                    ProgressEvent::Merged { total_bytes } => println!("  merged {total_bytes} bytes"),
                }
            }
        });

        let started = std::time::Instant::now();
        // `download` owns the only senders, so `printer` ends when this returns.
        download(&url, &dest, segments, tx).await.expect("download failed");
        printer.await.unwrap();

        let size = tokio::fs::metadata(&dest).await.unwrap().len();
        let secs = started.elapsed().as_secs_f64();
        println!(
            "wrote {} - {size} bytes in {secs:.1}s ({:.2} MiB/s)",
            dest.display(),
            size as f64 / (1024.0 * 1024.0) / secs
        );

        assert!(size > 0, "downloaded file is empty");
        if let Some(expected) = total_len {
            assert_eq!(size, expected, "size on disk disagrees with Content-Length");
        }
        assert!(
            tokio::fs::metadata(dir.join(format!("{file_name}.part0"))).await.is_err(),
            "temp segment left behind after a successful merge"
        );
        assert!(
            tokio::fs::metadata(dest.with_extension("part-merge")).await.is_err(),
            "merge temp file left behind after a successful merge"
        );
    }
}