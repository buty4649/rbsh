use super::api::*;
use std::ffi::{c_void, CStr, CString};

pub struct MRubyValue {
    pub(crate) mrb: *mut mrb_state,
    pub(crate) v: mrb_value,
}

impl MRubyValue {
    pub fn from_raw(mrb: *mut mrb_state, v: mrb_value) -> Self {
        Self { mrb, v }
    }

    pub fn from_ptr<T: ?Sized>(mrb: *mut mrb_state, p: *mut T) -> Self {
        let v = mrb_obj_value(p as *mut c_void);
        Self { mrb, v }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn from_str<T: AsRef<str>>(mrb: *mut mrb_state, s: T) -> Self {
        let c_str = CString::new(s.as_ref()).unwrap();
        let v = unsafe { mrb_str_new_cstr(mrb, c_str.as_ptr()) };
        Self { mrb, v }
    }

    pub fn as_raw(&self) -> mrb_value {
        self.v
    }

    pub fn call<T: AsRef<str>>(&self, name: T, argv: Option<&[Self]>) -> Self {
        let name = CString::new(name.as_ref()).unwrap();
        let v = unsafe {
            match argv {
                None => mrb_funcall(self.mrb, self.v, name.as_ptr(), 0),
                Some(a) => mrb_funcall(self.mrb, self.v, name.as_ptr(), a.len() as mrb_int, a),
            }
        };
        Self::from_raw(self.mrb, v)
    }

    pub fn to_s(&self) -> String {
        let s = self.call("to_s", None);
        let c_str = unsafe {
            let c_str = mrb_str_to_cstr(self.mrb, s.v);
            CStr::from_ptr(c_str)
        };
        c_str.to_str().unwrap().to_string()
    }

    pub fn is_nil(&self) -> bool {
        mrb_nil_p(self.v)
    }
}
