# rust-mruby
mruby bindings for Rust.

## Usage

Add build_config.rb to the top of the project directory.

e.g.:
```
❯ cat build_config.rb
MRuby::Build.new do |conf|
  conf.toolchain
  conf.gembox 'default'
  conf.enable_bintest
  conf.enable_test
end
```

Next, add rust-mruby to the dependencies in Cargo.toml.

e.g.:
```
❯ cat Cargo.toml
-- snip --
[dependencies]
mruby =  { package = "rust-mruby", git = "https://github.com/buty4649/rust-mruby" }
```

Use mruby in your code.

e.g.:
```rust
use std::ffi::CString;
use std::process;
use mruby;

fn main() {
    let filename = CString::new("Rust").unwrap();
    let s = CString::new("puts 'Hello World'").unwrap();
    unsafe {
        let mrb = mruby::mrb_open();
        let cxt = mruby::mrbc_context_new(mrb);
        mruby::mrbc_filename(mrb, cxt, filename.as_ptr());
        mruby::mrb_load_string_cxt(mrb, s.as_ptr() , cxt);
        if ! (*mrb).exc.is_null() {
            mruby::mrb_print_error(mrb);
            process::exit(1);
        }
        mruby::mrb_close(mrb);
    }
}
```
