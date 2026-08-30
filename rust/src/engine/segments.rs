#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    pub start: u64,
    pub end: u64,
}

impl ByteRange {
    pub fn len(&self) -> u64 {
        debug_assert!(self.end >= self.start, "inverted byte range");
        self.end - self.start + 1
    }

    pub fn header_value(&self) -> String {
        format!("bytes={}-{}", self.start, self.end)
    }
}

pub fn plan_segments(total_bytes: u64, requested_segments: u64) -> Vec<ByteRange> {
    if total_bytes == 0 {
        return vec![];
    }
    let  segments =  requested_segments.max(1).min(total_bytes);
    let base = total_bytes / segments;
    let remainder = total_bytes % segments;

    let mut ranges = Vec::with_capacity(segments as usize);
    let mut start = 0u64;
    for i in 0..segments {
        let this_len = base + if i < remainder { 1 } else { 0 };
        let end = start + this_len - 1;
        ranges.push(ByteRange { start, end });
        start = end + 1;
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn splits_evenly(){
        let ranges = plan_segments(100, 4);
        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0], ByteRange { start: 0, end: 24 });
        assert_eq!(ranges[3], ByteRange { start: 75, end: 99 });
    }

    #[test]
    fn distributes_remainder_without_losing_bytes(){
        let ranges = plan_segments(10,3);
        let total: u64 = ranges.iter().map(|r| r.len()).sum();
        assert_eq!(total, 10);
        for pair in ranges.windows(2) {
            assert_eq!(pair[0].end + 1, pair[1].start); // no gaps, no overlaps
        }
    }

    #[test]
    fn clamps_segments_to_total_bytes() {
        assert_eq!(plan_segments(3, 10).len(), 3); // can't have more segments than bytes
    }

    #[test]
    fn empty_file_has_no_segments() {
        assert_eq!(plan_segments(0, 4), vec![]);
    }
}