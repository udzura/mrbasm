use std::io;

pub struct Binary {}

impl Binary {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, mut out: impl io::Write) -> Result<(), io::Error> {
        out.write("test".as_bytes())?;
        Ok(())
    }
}
