pub mod api;

mod array;
mod context;
mod exception;
mod value;

pub use array::MRubyArray;
pub use context::MRubyContext;
pub use exception::{Exception, MRubyException};
pub use value::MRubyValue;

use api::*;
use context::Context;
use std::ffi::CString;

pub struct MRuby {
    mrb: *mut mrb_state,
}

impl MRuby {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mrb = unsafe { mrb_open() } as *mut mrb_state;
        Self { mrb }
    }

    pub fn close(&mut self) {
        unsafe { mrb_close(self.mrb) }
    }

    pub fn exec_from_string<T: AsRef<str>>(
        &self,
        s: T,
        args: &[String],
        name: Option<&str>,
    ) -> Result<(), Exception> {
        let ctx = self.context();

        let filename = name.unwrap_or("-");
        ctx.set_filename(filename);
        ctx.set_capture_errors(true);

        let ary = MRubyArray::new(self.mrb);
        for a in args {
            ary.push(MRubyValue::from_str(self.mrb, a));
        }

        let argv = CString::new("ARGV").unwrap();
        unsafe { mrb_define_global_const(self.mrb, argv.as_ptr(), ary.as_raw()) };

        let c_str = CString::new(s.as_ref()).unwrap();
        unsafe {
            mrb_load_string_cxt(self.mrb, c_str.as_ptr(), ctx.as_ptr());
        };

        match self.exception() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

impl Drop for MRuby {
    fn drop(&mut self) {
        self.close()
    }
}
