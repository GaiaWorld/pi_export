use std::{io::{self, Cursor, Read}, sync::Arc};

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
use pi_async_rt::rt::AsyncRuntime;
pub use pi_winit::window::Window;

#[cfg(not(target_arch="wasm32"))]
#[pi_js_export]
pub fn features(window: &Arc<Window>) -> String {
    let (sender, receiver) = crossbeam_channel::bounded(1);
    let w = window.clone();
    // let runtime = pi_async_rt::rt::local_async_runtime::<()>().unwrap();
    let _ = pi_hal::runtime::MULTI_MEDIA_RUNTIME.spawn(async move {
        let adapter = {
            let instance = wgpu::Instance::default();
            let surface = instance.create_surface(&w).unwrap();
            instance
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await
                .unwrap()
        };

        let features = adapter.features();
        let mut res = "".to_string();

        println!("=============== features: {:?}", features.is_empty());
        if features.contains(wgpu::Features::DEPTH_CLIP_CONTROL) {
            res.push_str(&"DEPTH-CLIP-CONTROL".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::DEPTH32FLOAT_STENCIL8) {
            res.push_str(&"DEPTH32FLOAT-STENCIL8".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC) {
            res.push_str(&"TEXTURE-COMPRESSION-BC".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2) {
            res.push_str(&"TEXTURE-COMPRESSION-ETC2".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC) {
            res.push_str(&"TEXTURE-COMPRESSION-ASTC-LDR".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::INDIRECT_FIRST_INSTANCE) {
            res.push_str(&"INDIRECT-FIRST-INSTANCE".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
            res.push_str(&"TIMESTAMP-QUERY".to_ascii_lowercase());
            res.push_str(" ");
        }
        // if FEATURES.contains(wgpu::Features::SHADER_FLOAT16) {
        //     res.push_str(&"SHADER-FLOAT16".to_ascii_lowercase());
        //     res.push_str(" ")
        // }
        if features.contains(wgpu::Features::TEXTURE_BINDING_ARRAY) {
            res.push_str(&"TEXTURE-BINDING-ARRAY".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::BUFFER_BINDING_ARRAY) {
            res.push_str(&"BUFFER-BINDING-ARRAY".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY) {
            res.push_str(&"STORAGE-RESOURCE-BINDING-ARRAY".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features
            .contains(wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING)
        {
            res.push_str(
                &"SAMPLED-TEXTURE-AND-STORAGE-BUFFER-ARRAY-NON-UNIFORM-INDEXING"
                    .to_ascii_lowercase(),
            );
        }
        if features
            .contains(wgpu::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING)
        {
            res.push_str(
                &"UNIFORM-BUFFER-AND-STORAGE-TEXTURE-ARRAY-NON-UNIFORM-INDEXING"
                    .to_ascii_lowercase(),
            );
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY) {
            res.push_str(&"PARTIALLY-BOUND-BINDING-ARRAY".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::MULTI_DRAW_INDIRECT) {
            res.push_str(&"MULTI-DRAW-INDIRECT".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT) {
            res.push_str(&"MULTI-DRAW-INDIRECT-COUNT".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::PUSH_CONSTANTS) {
            res.push_str(&"PUSH-CONSTANTS".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER) {
            res.push_str(&"ADDRESS-MODE-CLAMP-TO-BORDER".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::POLYGON_MODE_LINE) {
            res.push_str(&"POLYGON-MODE-LINE".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::POLYGON_MODE_POINT) {
            res.push_str(&"POLYGON-MODE-POINT".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES) {
            res.push_str(&"TEXTURE-ADAPTER-SPECIFIC-FORMAT-FEATURES".to_ascii_lowercase());
            res.push_str(" ");
        }
        // if FEATURES.contains(wgpu::Features::SHADER_FLOAT64) {
        //     res.push(&"SHADER-FLOAT64".to_ascii_lowercase());
        //     res.push(" ");
        // }
        if features.contains(wgpu::Features::VERTEX_ATTRIBUTE_64BIT) {
            res.push_str(&"VERTEX-ATTRIBUTE-64BIT".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::CONSERVATIVE_RASTERIZATION) {
            res.push_str(&"CONSERVATIVE-RASTERIZATION".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::VERTEX_WRITABLE_STORAGE) {
            res.push_str(&"VERTEX-WRITABLE-STORAGE".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::CLEAR_TEXTURE) {
            res.push_str(&"CLEAR-TEXTURE".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::SPIRV_SHADER_PASSTHROUGH) {
            res.push_str(&"SPIRV-SHADER-PASSTHROUGH".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::SHADER_PRIMITIVE_INDEX) {
            res.push_str(&"SHADER-PRIMITIVE-INDEX".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::MULTIVIEW) {
            res.push_str(&"MULTIVIEW".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::TEXTURE_FORMAT_16BIT_NORM) {
            res.push_str(&"TEXTURE-FORMAT-16BIT-NORM".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::ADDRESS_MODE_CLAMP_TO_ZERO) {
            res.push_str(&"ADDRESS-MODE-CLAMP-TO-ZERO".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC_HDR) {
            res.push_str(&"TEXTURE-COMPRESSION-ASTC_HDR".to_ascii_lowercase());
            res.push_str(" ");
        }
        // if FEATURES.contains(wgpu::Features::WRITE_TIMESTAMP_INSIDE_PASSES) {
        //     res.push(&"WRITE-TIMESTAMP-INSIDE-PASSES".to_ascii_lowercase());
        //     res.push(" ");
        // }
        if features.contains(wgpu::Features::PIPELINE_STATISTICS_QUERY) {
            res.push_str(&"PIPELINE-STATISTICS-QUERY".to_ascii_lowercase());
            res.push_str(" ");
        }
        if features.contains(wgpu::Features::MAPPABLE_PRIMARY_BUFFERS) {
            res.push_str(&"MAPPABLE-PRIMARY-BUFFERS".to_ascii_lowercase());
            res.push_str(" ");
        }
        adapter.inner.context.unmake_current();
        // res
        sender.send(res).unwrap();
    });

    receiver.recv().unwrap()
}
