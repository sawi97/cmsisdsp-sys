//! VFP FFI Instructions
//!
//! TODO: More optimized methods

// TODO TODO TODO
#[no_mangle]
pub unsafe extern "C" fn expf(v: f32) -> f32 {
    libm::expf(v)
}

#[no_mangle]
pub unsafe extern "C" fn exp(v: f64) -> f64 {
    libm::exp(v)
}

#[no_mangle]
pub unsafe extern "C" fn logf(v: f32) -> f32 {
    libm::logf(v)
}

#[no_mangle]
pub unsafe extern "C" fn log(v: f64) -> f64 {
    libm::log(v)
}

#[no_mangle]
pub unsafe extern "C" fn powf(b: f32, e: f32) -> f32 {
    libm::powf(b, e)
}

#[no_mangle]
pub unsafe extern "C" fn sqrtf(v: f32) -> f32 {
    libm::sqrtf(v)  // TODO: VSQRT
}

#[no_mangle]
pub unsafe extern "C" fn sqrt(v: f64) -> f64 {
    libm::sqrt(v)  // TODO: VSQRT
}
