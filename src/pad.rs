use std::io::{Result as IoResult, Write};

// adopted from libcore's `fmt/builders.rs`

/// Indents all lines written to this by four spaces
pub struct PadAdapter<'a> {
    buf: &'a mut (dyn Write + 'a),
    on_newline: bool,
}

impl<'a> PadAdapter<'a> {
    pub(crate) fn wrap<'fmt: 'a>(buf: &'fmt mut (dyn Write + 'a)) -> Self {
        PadAdapter {
            buf,
            on_newline: true,
        }
    }
}

impl Write for PadAdapter<'_> {
    fn write(&mut self, mut s: &[u8]) -> IoResult<usize> {
        let len = s.len();
        while !s.is_empty() {
            if self.on_newline {
                self.buf.write_all(b"    ")?;
            }

            let split = match s.iter().enumerate().find(|(_, byte)| **byte == b'\n') {
                Some((pos, _)) => {
                    self.on_newline = true;
                    pos.checked_add(1).unwrap()
                }
                None => {
                    self.on_newline = false;
                    s.len()
                }
            };
            self.buf.write_all(&s[..split])?;
            s = &s[split..];
        }

        Ok(len)
    }

    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}
