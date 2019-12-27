use std::io;
use std::io::prelude::*;

pub(crate) struct ReaderStub {
    contents: String,
}

impl ReaderStub {
    pub(crate) fn new(contents: String) -> Self {
        Self { contents }
    }
}

impl Read for ReaderStub {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(1)
    }

    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.contents.as_bytes().read_to_string(buf)?;
        Ok(buf.len())
    }
}

pub(crate) struct WriterMock {
    written_content: String,
    expected_content: String,
}

impl WriterMock {
    pub(crate) fn new<I: Into<String>>(expected_content: I) -> Self {
        Self {
            written_content: "".to_string(),
            expected_content: expected_content.into(),
        }
    }

    pub(crate) fn all_config_written(&self) -> bool {
        self.written_content == self.expected_content
    }
}

impl Write for WriterMock {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Ok(1)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.written_content = String::from_utf8(buf.to_vec()).unwrap();
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
