extern crate core;

use std::env;
use std::path::{Path, PathBuf};

static CONFIG_KEY: &str = "CMSISDSP_CFG";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest);

    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dsp_dir = manifest_dir.join("cmsis/CMSIS-DSP");
    let cmsis5_dir = manifest_dir.join("cmsis/CMSIS_5");

    let manifest_dir = Path::new(&manifest);

    let bb = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_default(false)
        .ctypes_prefix("core::ffi")
        .generate_comments(true)
        .rustfmt_bindings(true)
        .clang_arg(format!("-I{}", manifest_dir.join("include").display()))
        .clang_arg(format!("-I{}", dsp_dir.join("Include").display()))
        .clang_arg(format!("-I{}", dsp_dir.join("PrivateInclude").display()))
        .clang_arg(format!("-I{}", cmsis5_dir.join("CMSIS/Core/Include").display()))
        .clang_arg("-nostdinc")
        .clang_arg("-target")
        .clang_arg("arm")
        .clang_arg("-mcpu=cortex-m33")
        .clang_arg("-mfloat-abi=hard")
        .use_core();

    let cmd = bb.command_line_flags().join(" ");
    eprintln!("{:?}", cmd);

    let bindings = bb.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let mut rust_source = bindings.to_string();

    // Munge Doxygen comments into something Rustdoc can handle
    rust_source = rust_source.replace("#[doc = \" @{\"]", "");

    // Format @param as list element
    let re = regex::Regex::new(r"\s*@[pP]aram\s*(\[(?P<typ>[\w,\s]+)\s*\])?\s*(\\t)?(?P<var>[\w\.]+)\s+").unwrap();
    rust_source = re.replace_all(&rust_source, " * `$var` $typ - ").into();

    // Format @p/@a/@c arguments as inline code
    let re = regex::Regex::new(r"@[pac]\s+(?P<var>[\*A-Za-z0-9_\(\)]+)").unwrap();
    rust_source = re.replace_all(&rust_source, " `$var` ").into();

    // Format NRF_* as ref
    let re = regex::Regex::new(r"(?P<pre>@(returns?|retval|note)\s+.*)(?P<var>NRF_\w+)").unwrap();
    rust_source = re.replace_all(&rust_source, "$pre[$var]").into();

    // #nrf_*
    let re = regex::Regex::new("(?P<pre>#\\[doc.*\\s+)#(?P<var>(nrf|NRF)_\\w+)(?P<post>\\s+.*\"\\])").unwrap();
    rust_source = re
        .replace_all(&rust_source, "$pre[$var]$post")
        .into();

    // Remove @addtogroup stuff
    let re = regex::RegexBuilder::new(r"^#\[doc.*@addtogroup(.|\n)*?^$")
        .multi_line(true)
        .build()
        .unwrap();
    rust_source = re.replace_all(&rust_source, "").into();

    // Format @ref as markdown ref
    let re = regex::Regex::new(r"\s*@(ref|refitem)\s+(?P<var>\w+)(\(\))?").unwrap();
    rust_source = re.replace_all(&rust_source, " [$var]").into();

    // Format deprecation notice (@deprecated) as deprecated
    let re = regex::Regex::new("#\\[doc.*@deprecated\\s*(?P<note>.*)\\.*\".*]").unwrap();
    rust_source = re
        .replace_all(&rust_source, "#[deprecated(note=\"$note\")]")
        .into();

    // Format inline @brief
    let re = regex::Regex::new("#\\[doc = \"\\s*@brief\\s*(?P<msg>.*)\"]").unwrap();
    rust_source = re
        .replace_all(&rust_source, "#[doc = \"$msg\"]")
        .into();

    // Format inline @note as bold
    let re = regex::Regex::new(r"\s*@note:?\s*").unwrap();
    rust_source = re.replace_all(&rust_source, "**Note:** ").into();

    // Format @details as a section
    let re = regex::Regex::new(r"\s*@details?\s*(?P<var>.*)").unwrap();
    rust_source = re.replace_all(&rust_source, "# Details \n$var").into();

    // // Format some sections as headers
    rust_source = rust_source.replace("@return ", "# Returns\n");
    rust_source = rust_source.replace("@returns ", "# Returns\n");
    rust_source = rust_source.replace("@retval ", "# Returns\n");

    // Write bindings to bindings.rs
    std::fs::write(outdir.join("bindings.rs"), rust_source).expect("Couldn't write bindgen output");

    // Compile the library
    let mut cmake_cfg = cmake::Config::new("cmsis/CMSIS-DSP");

    // Set defaults
    cmake_cfg.build_target("CMSISDSP")
        .define("CMSISCORE", manifest_dir.join("cmsis/CMSIS_5/CMSIS/Core"))
        .define("CMAKE_TRY_COMPILE_TARGET_TYPE", "STATIC_LIBRARY")
        .cflag("-Ofast")
        .cflag("-ffast-math");

    // Set build target and disable target flags  (for specific CPU's)
    #[cfg(feature = "cortex-m33")]
    {
        env::set_var("CRATE_CC_NO_DEFAULTS", "1");
        cmake_cfg
            .cflag("-mcpu=cortex-m33")
            .cflag("-mthumb")
            .cflag("-mfloat-abi=hard")
            .cflag("-mfpu=fpv5-sp-d16");
    }

    // Read environment from config
    if let Ok(cfg) = env::var(CONFIG_KEY) {
        println!("{} variable set, reading config", CONFIG_KEY);
        parse_cfg(&cfg, &mut cmake_cfg);
    }

    // Compile the crate
    let dst = cmake_cfg.build();

    println!("cargo:rustc-link-search=native={}", dst.join("build/Source").display());
    println!("cargo:rustc-link-lib=static=CMSISDSP");
}

/// Parse config from the environment variable
fn parse_cfg(cfg_str: &str, cfg: &mut cmake::Config) {
    let re = regex::Regex::new(r"\-D(\w+)=(\w+)").unwrap();
    for m in re.captures_iter(cfg_str) {
        println!("adding cmake flag {}={}", &m[1], &m[2]);
        cfg.define(&m[1], &m[2]);
    }
}
