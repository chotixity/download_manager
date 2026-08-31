use std::path::Path;

use tokio::sync::mpsc;

use crate::engine::download::{download, ProgressEvent};
use crate::frb_generated::StreamSink;

const EVENT_BUFFER: usize = 128;

pub async fn download_file(
    url: String,
    dest: String,
    num_segments: u32,
    on_event: StreamSink<ProgressEvent>,
) {
    let (tx, mut rx) = mpsc::channel(EVENT_BUFFER);

    let pump = async {
        while let Some(event) = rx.recv().await {
            if on_event.add(event).is_err() {
                break;
            }
        }
    };

    let (result, ()) =
        tokio::join!(download(&url, Path::new(&dest), num_segments as u64, tx), pump);

    if let Err(err) = result {
        let _ = on_event.add_error(err.to_string());
    }
}
