use std::env;
use std::path::PathBuf;

#[cfg(feature = "build")]
use cmd_lib::*;

#[cfg(not(feature = "build"))]
fn build_mruby(_: &str) {}

#[cfg(feature = "build")]
fn build_mruby(mruby_build_config: &str) {
    const DEFAULT_MRUBY_VERSION: &str = include_str!("mruby_version");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mruby_path = out_path.join("mruby");
    let mruby_version =
        env::var("MRUBY_VERSION").unwrap_or_else(|_| DEFAULT_MRUBY_VERSION.trim_end().to_string());
    let mruby_url = format!(
        "https://github.com/mruby/mruby/archive/refs/tags/{}.tar.gz",
        mruby_version
    );

    if !mruby_path.exists() {
        let out_path = out_path.to_str().unwrap();
        run_cmd!(
            cd "$out_path";
            echo "Download: ${mruby_url}" ;
            wget -O- "${mruby_url}" | tar zxf -;
            mv mruby-$mruby_version mruby;
        )
        .unwrap();
    }

    run_cmd!(
        cd "$mruby_path";
        rake clean;
        rake MRUBY_CONFIG=$mruby_build_config 2>&1;
    )
    .unwrap();

    println!("cargo:rerun-if-changed={}", mruby_version);
    println!("cargo:rustc-link-lib=mruby");
    println!(
        "cargo:rustc-link-search={}",
        mruby_path.join("build/host/lib/").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=mruby");
}

fn main() {
    let build_config = env::var("MRUBY_BUILD_CONFIG").unwrap_or_else(|_| {
        let current_dir = env::var("PWD")
            .map(PathBuf::from)
            .unwrap_or_else(|_| env::current_dir().unwrap());
        current_dir
            .join("build_config.rb")
            .to_str()
            .unwrap()
            .to_string()
    });

    let build_enabled = env::var_os("CARGO_FEATURE_BUILD").is_some();
    if build_enabled {
        build_mruby(&*build_config);
    }

    println!("cargo:rerun-if-changed={}", build_config);
    println!("cargo:rerun-if-env-changed=MRUBY_VERSION");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_BUILD");
}
