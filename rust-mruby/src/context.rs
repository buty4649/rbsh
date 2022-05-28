use super::MRuby;
use crate::api::*;
use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

pub struct MRubyContext {
    mrb: *mut mrb_state,
    ctx: *mut mrbc_context,
}

impl MRubyContext {
    pub fn from_ptr(mrb: *mut mrb_state, ctx: *mut mrbc_context) -> Self {
        Self { mrb, ctx }
    }

    pub fn get_filename(&self) -> String {
        let c = unsafe { NonNull::new(self.ctx).unwrap().as_mut() };
        let c_str = unsafe { CStr::from_ptr(c.filename) };
        c_str.to_str().unwrap().to_string()
    }

    pub fn set_filename<T: AsRef<str>>(&self, name: T) {
        let name = CString::new(name.as_ref()).unwrap();
        unsafe { mrbc_filename(self.mrb, self.ctx, name.as_ptr()) };
    }

    pub fn lineno(&self) -> u16 {
        let c = unsafe { NonNull::new(self.ctx).unwrap().as_mut() };
        c.lineno
    }

    pub fn set_capture_errors(&self, f: bool) {
        let f = match f {
            true => 1,
            false => 0,
        };
        let c = unsafe { NonNull::new(self.ctx).unwrap().as_mut() };
        c.set_capture_errors(f)
    }

    pub fn as_ptr(&self) -> *mut mrbc_context {
        self.ctx
    }
}

impl Drop for MRubyContext {
    fn drop(&mut self) {
        unsafe { mrbc_context_free(self.mrb, self.ctx) }
    }
}

pub trait Context {
    fn context(&self) -> MRubyContext;
}

impl Context for MRuby {
    fn context(&self) -> MRubyContext {
        let mrb = self.mrb;
        let ctx = unsafe { mrbc_context_new(mrb) };
        MRubyContext { mrb, ctx }
    }
}
