use std::io::{self, Cursor, Read};

use js_proxy_gen_macro::pi_js_export;
use pi_share::Share;

#[pi_js_export]
#[derive(Debug, Clone)]
pub struct Blob {
    pub data: Share<Vec<u8>>,
    pub start: usize,
    pub end: usize,
}

unsafe impl Sync for Blob {}
unsafe impl Send for Blob {}

#[allow(non_snake_case)]
impl Blob {
    #[pi_js_export]
    pub fn new(buf: &[u8]) -> Self {
        let len = buf.len();
        Self {
            data: Share::new(buf.to_vec()),
            start: 0,
            end: len,
        }
    }

    pub fn init(buf: Vec<u8>) -> Self {
        let len = buf.len();
        Self {
            data: Share::new(buf),
            start: 0,
            end: len,
        }
    }

    #[pi_js_export]
    pub fn size(&self) -> u32 {
        (self.end - self.start) as u32
    }

    #[pi_js_export]
    pub async fn arrayBuffer(&self) -> Vec<u8> {
        self.data[self.start..self.end].to_vec()
    }

    #[pi_js_export]
    pub fn slice(
        &self,
        start: Option<u32>,
        end: Option<u32>,
        _content_type: Option<String>,
    ) -> Result<Self, String> {
        let mut s = 0;
        let mut e = self.size() as usize;
        if let Some(start) = start {
            s = start as usize + self.start;
            if s >= self.end {
                return Err(format!(
                    "error: rang({}, {}) start: {}!!!",
                    self.start, self.end, s
                ));
            }

            if let Some(end) = end {
                if start >= end {
                    return Err(format!("error: rang({}, {})!!!", start, end));
                }
                let len = (end - start) as usize;
                e = s + len;
                if e > self.end {
                    return Err(format!(
                        "error: rang({}, {}) start: {}, end: {}!!!",
                        self.start, self.start, start, end
                    ));
                }
            }
        }

        Ok(Self {
            data: self.data.clone(),
            start: self.start + s,
            end: e,
        })
    }

    #[pi_js_export]
    pub async fn text(&self) -> Option<String> {
        if let Ok(str) = String::from_utf8(self.data[self.start..self.end].to_vec()) {
            return Some(str);
        }
        None
    }

    #[pi_js_export]
    pub fn stream(&self) -> ReadStream {
        ReadStream(self.clone())
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data[self.start..self.end]
    }
}

pub struct ReadStream(pub Blob);

impl ReadStream {
    pub fn read(&self, size: Option<u32>) -> Vec<u8> {
        let mut s = self.0.end - self.0.start;
        if let Some(size) = size {
            if (size as usize) < s {
                s = size as usize;
            }
        }
        self.0.data[self.0.start..self.0.start + s].to_vec()
    }
}

impl Read for Blob {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        todo!()
    }

    fn read_to_string(&mut self, _: &mut String) -> io::Result<usize> {
        todo!()
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut r = Cursor::new(&self.data[self.start..self.end]);
        r.read_to_end(buf)
    }
}
