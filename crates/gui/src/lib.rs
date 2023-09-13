#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(type_name_of_val)]
#![feature(box_into_inner)]
#![feature(if_let_guard)]
#![feature(core_panic)]
#![feature(fmt_internals)]
#![feature(fmt_helpers_for_derive)]
#![feature(print_internals)]
#![feature(can_vector)]


use bevy_ecs::prelude::Resource;
use std::{mem::replace, io};
use pi_share::Share;
use std::cell::RefCell;


pub struct ChromeWrite {
	pub buf: Vec<u8>,
}

#[derive(Resource, Clone)]
pub struct ShareChromeWrite {
	value: Share<RefCell<ChromeWrite>>,
}

unsafe impl Send for ShareChromeWrite {}
unsafe impl Sync for ShareChromeWrite {}

impl ShareChromeWrite {
	pub fn new() -> Self {
		Self {
			value: Share::new(RefCell::new(ChromeWrite {
				buf: Vec::new(),
			})),
		}
	}

	pub fn take(&mut self) -> Vec<u8> {
		let mut lock = self.value.borrow_mut();
		replace(&mut lock.buf, Vec::new())
	}
}

impl io::Write for ShareChromeWrite {
	#[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let mut lock = self.value.borrow_mut();
        lock.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        let len = bufs.iter().map(|b| b.len()).sum();
		let mut lock = self.value.borrow_mut();
        lock.buf.reserve(len);
        for buf in bufs {
            lock.buf.extend_from_slice(buf);
        }
        Ok(len)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        true
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		let mut lock = self.value.borrow_mut();
        lock.buf.extend_from_slice(buf);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}


#[cfg(target_arch = "wasm32")]
use pi_async_rt::prelude::{LocalTaskRunner, LocalTaskRuntime};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(target_arch = "wasm32"))]
pub mod native_index;
#[cfg(not(target_arch = "wasm32"))]
pub mod native_debug;
pub mod rr;
#[cfg(target_arch = "wasm32")]
pub mod wasm_debug;
#[cfg(target_arch = "wasm32")]
pub mod wasm_index;

pub mod style;
pub mod index;
