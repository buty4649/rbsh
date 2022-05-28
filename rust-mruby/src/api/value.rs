use super::*;
use std::{ffi::c_void, mem, ptr};

const BOXWORD_FIXNUM_BIT_POS: usize = 1;
const BOXWORD_SYMBOL_BIT_POS: usize = 2;

const BOXWORD_FIXNUM_SHIFT: usize = BOXWORD_FIXNUM_BIT_POS;
#[allow(dead_code)]
const BOXWORD_SYMBOL_SHIFT: usize = 0;

const BOXWORD_FIXNUM_FLAG: usize = 1 << (BOXWORD_FIXNUM_BIT_POS - 1);
const BOXWORD_SYMBOL_FLAG: usize = 1 << (BOXWORD_SYMBOL_BIT_POS - 1);

const BOXWORD_IMMEDIATE_MASK: usize = 0x07;

// enum mrb_special_consts
const MRB_Qnil: usize = 0;
const MRB_Qfalse: usize = 4;
const MRB_Qtrue: usize = 12;
const MRB_Qundef: usize = 20;

macro_rules! mrb_value_new {
    () => {
        unsafe { mem::zeroed::<mrb_value>() }
    };
}

fn mrb_val_union(v: mrb_value) -> mrb_value_ {
    let mut x: mrb_value_ = unsafe { mem::zeroed() };
    x.value = v;
    x
}

fn mrb_immediate_p(o: mrb_value) -> bool {
    (o.w as usize & BOXWORD_IMMEDIATE_MASK) > 0 || o.w as usize == MRB_Qnil
}

pub fn mrb_integer(o: mrb_value) -> mrb_int {
    if mrb_immediate_p(o) {
        (o.w as usize >> BOXWORD_FIXNUM_SHIFT) as mrb_int
    } else {
        let v = mrb_val_union(o);
        let i = unsafe { ptr::NonNull::new(v.ip as *mut RInteger).unwrap().as_ref() };
        i.i
    }
}

pub fn mrb_nil_p(o: mrb_value) -> bool {
    o.w == MRB_Qnil
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn mrb_float_value(mrb: *mut mrb_state, f: mrb_float) -> mrb_value {
    unsafe { mrb_word_boxing_float_value(mrb, f) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn mrb_cptr_value(mrb: *mut mrb_state, p: *mut c_void) -> mrb_value {
    unsafe { mrb_word_boxing_cptr_value(mrb, p) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn mrb_int_value(mrb: *mut mrb_state, i: mrb_int) -> mrb_value {
    unsafe { mrb_word_boxing_int_value(mrb, i) }
}

pub fn mrb_fixnum_value(i: mrb_int) -> mrb_value {
    let mut value = mrb_value_new!();
    value.w = (i << BOXWORD_FIXNUM_SHIFT) as usize | BOXWORD_FIXNUM_FLAG;
    value
}

pub fn mrb_symbol_value(i: mrb_sym) -> mrb_value {
    let mut value: mrb_value_ = unsafe { mem::zeroed() };
    value.__anon_1.sym = i;
    value.__anon_1.sym_flag = BOXWORD_SYMBOL_FLAG as u32;
    unsafe { value.value }
}

pub fn mrb_obj_value<T: ?Sized>(p: *mut T) -> mrb_value {
    let mut value = mrb_value_new!();
    value.w = (p as *mut c_void) as usize;
    value
}

pub fn mrb_nil_value() -> mrb_value {
    let mut value = mrb_value_new!();
    value.w = MRB_Qnil as usize;
    value
}

pub fn mrb_false_value() -> mrb_value {
    let mut value = mrb_value_new!();
    value.w = MRB_Qfalse as usize;
    value
}

pub fn mrb_true_value() -> mrb_value {
    let mut value = mrb_value_new!();
    value.w = MRB_Qtrue as usize;
    value
}

pub fn mrb_bool_value(b: bool) -> mrb_value {
    match b {
        true => mrb_true_value(),
        false => mrb_false_value(),
    }
}

pub fn mrb_undef_value() -> mrb_value {
    let mut value = mrb_value_new!();
    value.w = MRB_Qundef as usize;
    value
}
