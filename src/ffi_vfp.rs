//! VFP FFI Instructions
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
