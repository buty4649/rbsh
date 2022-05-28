use super::mrb_aspec;

#[inline]
pub fn MRB_ARGS_REQ(n: mrb_aspec) -> mrb_aspec {
    (n & 0x1f) << 18
}

#[inline]
pub fn MRB_ARGS_OPT(n: mrb_aspec) -> mrb_aspec {
    (n & 0x1f) << 13
}

#[inline]
pub fn MRB_ARGS_REST() -> mrb_aspec {
    (1 << 12) as mrb_aspec
}

#[inline]
pub fn MRB_ARGS_POST(n: mrb_aspec) -> mrb_aspec {
    (n & 0x1f) << 7
}

#[inline]
pub fn MRB_ARGS_KEY(n: mrb_aspec, f: bool) -> mrb_aspec {
    (n & 0x1f) << 2 | if f { 1 << 1 } else { 0 }
}

#[inline]
pub fn MRB_ARGS_BLOCK() -> mrb_aspec {
    1
}

#[inline]
pub fn MRB_ARGS_NONE() -> mrb_aspec {
    0
}
