extern crate core;

use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=Makefile");
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
        .clang_arg("-DDISABLEFLOAT16")
        .clang_arg("-DARM_DSP_CONFIG_TABLES")
        .clang_arg("-DARM_FAST_ALLOW_TABLES")
        .clang_arg("-DARM_FFT_ALLOW_TABLES")
        .clang_arg("-DARM_FFT_ALLOW_TABLES")
        .clang_arg("-D__ICCARM__")
        .clang_arg("-D__ARMVFP__")
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

    // Compile CMSIS-DSP
    let output = std::process::Command::new("make").spawn().expect("failed to execute 'make'").wait().unwrap();

    // Link
    println!("cargo:rustc-link-search={}", manifest_dir.join("builddir").display());
    println!("cargo:rustc-link-lib=static=CMSISDSP");
}
