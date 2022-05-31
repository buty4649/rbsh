#!/bin/bash

MRUBY_VERSION="$(cd $(dirname $0);cat mruby_version)"

if [ ! -d mruby ]; then
    wget -O- "https://github.com/mruby/mruby/archive/refs/tags/${MRUBY_VERSION}.tar.gz" | tar zxf -
    mv mruby-${MRUBY_VERSION} mruby
fi

type bindgen > /dev/null 2>&1 || cargo install bindgen

bindgen --generate-inline-functions \
        --no-doc-comments \
        --allowlist-function 'mrbc?_.*' \
        --default-enum-style rust \
        --size_t-is-usize \
        --anon-fields-prefix __anon_ \
        --raw-line "#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
mod macros;
mod value;
pub use macros::*;
pub use value::*;" \
        wrapper.h \
        -- \
        -Imruby/include \
        -Imruby/include/mruby \
| sed -e 's/^#\[test\]/#[test]\n#[allow(deref_nullptr)]/g' > src/api.rs
