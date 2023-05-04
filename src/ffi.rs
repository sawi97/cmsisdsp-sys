//! FFI for some basic math functions required by the CMSIS DSP library.
//!
//! These can be enabled with the [fft] feature.
//!
//! Some optimizations can achieved by enabling the `intrinsics` feature for specific cpus.
#[no_mangle]
pub unsafe extern "C" fn exp(v: f64) -> f64 {
    libm::exp(v)
}

#[no_mangle]
pub unsafe extern "C" fn expf(v: f32) -> f32 {
    libm::expf(v)
}

#[no_mangle]
pub unsafe extern "C" fn log(v: f64) -> f64 {
    libm::log(v)
}

#[no_mangle]
pub unsafe extern "C" fn logf(v: f32) -> f32 {
    libm::logf(v)
}

#[no_mangle]
pub unsafe extern "C" fn pow(b: f64, e: f64) -> f64 {
    libm::pow(b, e)
}

#[no_mangle]
pub unsafe extern "C" fn powf(b: f32, e: f32) -> f32 {
    libm::powf(b, e)
}

#[cfg(all(feature = "intrinsics", feature = "cortex-m33-dsp"))]
#[no_mangle]
pub unsafe extern "C" fn sqrtf(v: f32) -> f32 {
    core::intrinsics::sqrtf32(v)
}

#[cfg(not(feature = "intrinsics"))]
#[no_mangle]
pub unsafe extern "C" fn sqrtf(v: f32) -> f32 {
    libm::sqrtf(v)
}

#[no_mangle]
pub unsafe extern "C" fn sqrt(v: f64) -> f64 {
    libm::sqrt(v)
}
