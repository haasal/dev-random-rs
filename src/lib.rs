use std::{
    fs::File,
    io::{BufReader, Read},
};

fn get_file() -> File {
    File::open("/dev/random").expect("File '/dev/random' couldn't be opened")
}

/// High level utility function
pub fn get_bytes(n: usize) -> Vec<u8> {
    let mut buf = vec![0u8; n];
    let mut random = get_file();
    random.read(&mut buf).expect("Couldn't read /dev/random");

    buf
}

/// A buffered Rng struct that allows for efficient /dev/random reads
pub struct Rng {
    reader: BufReader<File>,
    buf: Vec<u8>,
}

impl Rng {
    /// New Rng object with a default buffer size of 256 bytes
    pub fn new() -> Self {
        let file = get_file();
        Self {
            reader: BufReader::new(file),
            buf: vec![0; 256],
        }
    }

    /// New Rng with a buffer size of n bytes
    pub fn with_cap(n: usize) -> Self {
        let file = get_file();
        Self {
            reader: BufReader::new(file),
            buf: vec![0; n],
        }
    }

    /// Fills internal buffer with bytes from /dev/random and returns a reference to the buffer
    pub fn next_bytes(&mut self) -> &[u8] {
        self.reader
            .read(&mut self.buf)
            .expect("Couln't read /dev/random");
        &self.buf
    }

    /// Size of the internal buffer
    pub fn buf_size(&self) -> usize {
        self.buf.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_bytes, Rng};

    #[test]
    fn test_init_rng() {
        let _rng = Rng::new();
    }

    #[test]
    fn test_buf_size() {
        let rng = Rng::new();
        assert_eq!(rng.buf_size(), 256);
    }

    #[test]
    fn test_with_cap() {
        let rng = Rng::with_cap(10);
        assert_eq!(rng.buf_size(), 10);
    }

    #[test]
    fn test_next_bytes() {
        let mut rng = Rng::with_cap(10);
        let buf = rng.next_bytes();
        assert_eq!(buf.len(), 10);
        assert_ne!(buf, &vec![0; 10]);
    }

    #[test]
    fn test_get_bytes() {
        let buf = get_bytes(10);
        assert_eq!(buf.len(), 10);
        assert_ne!(buf, vec![0; 10])
    }
}
