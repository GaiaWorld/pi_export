use std::io::{self, Cursor, Read};

use js_proxy_gen_macro::pi_js_export;
use pi_share::Share;

#[pi_js_export]
#[derive(Debug, Clone)]
pub struct Blob(pub Share<Vec<u8>>);

#[allow(non_snake_case)]
impl Blob {
    #[pi_js_export]
    pub fn new(buf: Vec<u8>) -> Self {
        Self(Share::new(buf.to_vec()))
    }

    #[pi_js_export]
    pub fn size(&self) -> u32 {
        self.0.len() as u32
    }

    #[pi_js_export]
    pub async fn arrayBuffer(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    #[pi_js_export]
    pub fn slice(
        &self,
        start: Option<u32>,
        end: Option<u32>,
        _content_type: Option<String>,
    ) -> Self {
        let mut s = 0;
        let mut e = self.size() as usize;
        if let Some(start) = start {
            s = start as usize;
            if let Some(end) = end {
                e = end as usize;
            }
        }
        Self(Share::new(self.0[s..e].to_vec()))
    }

    #[pi_js_export]
    pub async fn text(&self) -> Option<String> {
        if let Ok(str) = String::from_utf8(self.0.to_vec()) {
            return Some(str);
        }
        None
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
        let mut r = Cursor::new(self.0.as_ref());
        r.read_to_end(buf)
    }
}
