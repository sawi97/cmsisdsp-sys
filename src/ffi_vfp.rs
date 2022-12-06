//! VFP FFI Instructions
//!
//! TODO: More optimized methods
use micromath::F32Ext;

// TODO TODO TODO
#[no_mangle]
pub unsafe extern "C" fn expf(v: f32) -> f32 {
    v.exp()
}

#[no_mangle]
pub unsafe extern "C" fn exp(v: f64) -> f64 {
    (v as f32).exp() as f64
}

#[no_mangle]
pub unsafe extern "C" fn logf(v: f32) -> f32 {
    v.ln()
}

#[no_mangle]
pub unsafe extern "C" fn log(v: f64) -> f64 {
    (v as f32).ln() as f64
}

#[no_mangle]
pub unsafe extern "C" fn powf(b: f32, e: f32) -> f32 {
    b.powf(e)
}

#[no_mangle]
pub unsafe extern "C" fn sqrtf(v: f32) -> f32 {
    v.sqrt()  // TODO: VSQRT
}

#[no_mangle]
pub unsafe extern "C" fn sqrt(v: f64) -> f64 {
    (v as f32).sqrt() as f64  // TODO: VSQRT
}
