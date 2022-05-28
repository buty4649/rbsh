use super::{api::*, MRubyValue};
use std::ffi::CString;

pub struct MRubyArray(MRubyValue);
impl MRubyArray {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(mrb: *mut mrb_state) -> Self {
        let ary = unsafe { mrb_ary_new(mrb) };
        let v = MRubyValue { mrb, v: ary };
        Self(v)
    }

    pub fn as_raw(&self) -> mrb_value {
        self.0.as_raw()
    }

    pub fn push(&self, v: MRubyValue) {
        unsafe { mrb_ary_push(self.0.mrb, self.0.v, v.v) };
    }

    pub fn shift(&self) -> Option<MRubyValue> {
        if self.0.is_nil() {
            return None;
        }

        let v = unsafe { mrb_ary_shift(self.0.mrb, self.0.v) };
        match mrb_nil_p(v) {
            true => None,
            false => Some(MRubyValue::from_raw(self.0.mrb, v)),
        }
    }

    pub fn len(&self) -> usize {
        if self.0.is_nil() {
            return 0;
        }

        let v = unsafe {
            let c_str = CString::new("length").unwrap();
            mrb_funcall(self.0.mrb, self.0.v, c_str.as_ptr(), 0)
        };
        mrb_integer(v) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Iterator for MRubyArray {
    type Item = MRubyValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.shift()
    }
}

impl From<MRubyValue> for MRubyArray {
    fn from(v: MRubyValue) -> MRubyArray {
        MRubyArray(v)
    }
}
