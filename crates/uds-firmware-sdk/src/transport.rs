use uds_core::error::Error;

pub trait FirmwareTransport {
    fn send(&mut self, data: &[u8]) -> Result<(), Error>;
    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error> { Ok(()) }
}

pub struct TransportAdapter<T: FirmwareTransport> {
    inner: T,
}

impl<T: FirmwareTransport> TransportAdapter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        self.inner.send(data)
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.inner.recv(buf)
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.inner.flush()
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}
