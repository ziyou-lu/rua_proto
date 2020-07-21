use std::{cmp, fmt};
use std::io::{Read, Write, Result};
use std::ptr;

pub struct Buffer {
    data: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl Buffer {
    pub fn new() -> Buffer{
        Buffer {
            data: Vec::new(),
            read_pos: 0,
            write_pos: 0,
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn set_read_pos(&mut self, r_pos: usize) {
        self.read_pos = r_pos;
    }

    pub fn get_read_pos(&self) -> usize {
        self.read_pos
    }

    pub fn set_write_pos(&mut self, w_pos: usize) {
        self.write_pos = w_pos;
    }

    pub fn get_write_pos(&self) -> usize {
        self.write_pos
    }

    pub fn drain(&mut self, pos: usize) {
        self.read_pos = self.read_pos - cmp::min(self.read_pos, pos);
        self.write_pos = self.write_pos - cmp::min(self.write_pos, pos);
        let pos = cmp::min(self.data.len(), pos);
        self.data.drain(..pos);
    }

    pub fn drain_collect(&mut self, pos: usize) -> Vec<u8> {
        self.read_pos = self.read_pos - cmp::min(self.read_pos, pos);
        self.write_pos = self.write_pos - cmp::min(self.write_pos, pos);
        let pos = cmp::min(self.data.len(), pos);
        self.data.drain(..pos).collect()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.read_pos = 0;
        self.write_pos = 0;
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "data is {:?}", self.data)
    }
}

impl Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let left = self.data.len() - self.read_pos;
        if left == 0 || buf.len() == 0 {
            return Ok(0)
        }

        let read_len = if left > buf.len() {
            buf.len()
        } else {
            left
        };

        unsafe {
            ptr::copy(&self.data[self.read_pos], &mut buf[0], read_len)
        }

        self.read_pos += read_len;
        Ok(read_len)
    }
}

impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.data.len() < self.write_pos + buf.len() {
            self.data.resize(self.write_pos + buf.len(), 0)
        }

        if buf.len() == 0 {
            return Ok(0);
        }

        unsafe {
            ptr::copy(&buf[0], &mut self.data[self.write_pos], buf.len());
        }

        self.write_pos += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}