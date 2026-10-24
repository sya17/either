// SPDX-License-Identifier: GPL-3.0-or-later
//! Newline-delimited framing, kept free of I/O so it can be tested on its own.

use std::fmt;

use serde::Serialize;

/// Longest accepted line in bytes, not counting the terminating `\n`.
pub const MAX_LINE_BYTES: usize = 1024 * 1024;

#[derive(Debug)]
pub enum FrameError {
    /// A line is longer than [`MAX_LINE_BYTES`].
    LineTooLong,
    /// The message could not be serialized.
    Json(serde_json::Error),
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LineTooLong => write!(f, "line is longer than {MAX_LINE_BYTES} bytes"),
            Self::Json(err) => write!(f, "cannot serialize message: {err}"),
        }
    }
}

impl std::error::Error for FrameError {}

impl From<serde_json::Error> for FrameError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

/// Serializes one message as a line, `\n` included. serde_json escapes newlines inside strings,
/// so the output is always exactly one line.
pub fn encode_line<T: Serialize>(message: &T) -> Result<Vec<u8>, FrameError> {
    let mut line = serde_json::to_vec(message)?;
    if line.len() > MAX_LINE_BYTES {
        return Err(FrameError::LineTooLong);
    }
    line.push(b'\n');
    Ok(line)
}

/// Splits a byte stream into lines. Feed it whatever the socket returns with [`push`](Self::push),
/// then take complete lines with [`next_line`](Self::next_line).
#[derive(Debug, Default)]
pub struct LineDecoder {
    buf: Vec<u8>,
    /// Bytes at the start of `buf` already searched for `\n`.
    scanned: usize,
    /// Inside a line that was already reported as too long.
    discarding: bool,
}

impl LineDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    /// The next complete line without its `\n`, an error for a line over the limit, or `None`
    /// when more bytes are needed. A line over the limit is reported once, as soon as it crosses
    /// the limit, and the rest of it is skipped, so memory stays bounded.
    pub fn next_line(&mut self) -> Option<Result<Vec<u8>, FrameError>> {
        loop {
            let Some(offset) = self.buf[self.scanned..].iter().position(|&b| b == b'\n') else {
                if self.discarding {
                    self.buf.clear();
                } else if self.buf.len() > MAX_LINE_BYTES {
                    self.buf.clear();
                    self.discarding = true;
                    self.scanned = 0;
                    return Some(Err(FrameError::LineTooLong));
                }
                self.scanned = self.buf.len();
                return None;
            };
            let mut line: Vec<u8> = self.buf.drain(..=self.scanned + offset).collect();
            line.pop();
            self.scanned = 0;
            if std::mem::take(&mut self.discarding) {
                continue;
            }
            if line.len() > MAX_LINE_BYTES {
                return Some(Err(FrameError::LineTooLong));
            }
            return Some(Ok(line));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(decoder: &mut LineDecoder) -> Vec<Result<Vec<u8>, String>> {
        std::iter::from_fn(|| decoder.next_line())
            .map(|r| r.map_err(|e| e.to_string()))
            .collect()
    }

    #[test]
    fn splits_several_lines_from_one_read() {
        let mut d = LineDecoder::new();
        d.push(b"{\"a\":1}\n{\"b\":2}\n{\"c\"");
        assert_eq!(
            lines(&mut d),
            [Ok(b"{\"a\":1}".to_vec()), Ok(b"{\"b\":2}".to_vec())]
        );
        d.push(b":3}\n");
        assert_eq!(lines(&mut d), [Ok(b"{\"c\":3}".to_vec())]);
    }

    #[test]
    fn joins_a_line_split_across_reads() {
        let mut d = LineDecoder::new();
        for chunk in [&b"{\"jsonrpc\""[..], b":\"2.0\"", b"}", b"\n"] {
            d.push(chunk);
        }
        assert_eq!(lines(&mut d), [Ok(b"{\"jsonrpc\":\"2.0\"}".to_vec())]);
    }

    #[test]
    fn accepts_a_line_of_exactly_the_limit() {
        let mut d = LineDecoder::new();
        d.push(&vec![b'x'; MAX_LINE_BYTES]);
        assert!(d.next_line().is_none());
        d.push(b"\n");
        let line = d.next_line().and_then(Result::ok).map(|l| l.len());
        assert_eq!(line, Some(MAX_LINE_BYTES));
    }

    #[test]
    fn rejects_a_complete_line_over_the_limit_and_keeps_reading() {
        let mut d = LineDecoder::new();
        let mut input = vec![b'x'; MAX_LINE_BYTES + 1];
        input.extend_from_slice(b"\nnext\n");
        d.push(&input);
        let got = lines(&mut d);
        assert!(matches!(got.as_slice(), [Err(_), Ok(l)] if l == b"next"));
    }

    #[test]
    fn reports_a_growing_line_once_and_skips_the_rest_of_it() {
        let mut d = LineDecoder::new();
        d.push(&vec![b'x'; MAX_LINE_BYTES + 1]);
        assert!(matches!(d.next_line(), Some(Err(FrameError::LineTooLong))));
        assert!(d.next_line().is_none());
        d.push(&vec![b'x'; MAX_LINE_BYTES]);
        assert!(d.next_line().is_none());
        assert!(d.buf.is_empty(), "skipped bytes are not kept");
        d.push(b"xx\nnext\n");
        assert_eq!(lines(&mut d), [Ok(b"next".to_vec())]);
    }

    #[test]
    fn encoded_message_is_one_line_even_with_newlines_in_strings() {
        let line = encode_line(&serde_json::json!({ "s": "a\nb" })).unwrap();
        assert_eq!(line.iter().filter(|&&b| b == b'\n').count(), 1);
        assert_eq!(line.last(), Some(&b'\n'));
    }

    #[test]
    fn refuses_to_encode_a_message_over_the_limit() {
        let big = "x".repeat(MAX_LINE_BYTES);
        assert!(matches!(encode_line(&big), Err(FrameError::LineTooLong)));
    }
}
