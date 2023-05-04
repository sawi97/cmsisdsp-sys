//! System bindings for CMSIS DSP
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![cfg_attr(feature = "intrinsics", feature(core_intrinsics))]

#[cfg(feature = "ffi")]
pub mod ffi;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arm_abs() {
        let mut vin = [-1.0, 0.0, 1.0];
        unsafe { arm_abs_f32(vin.as_ptr(), vin.as_mut_ptr(), vin.len() as u32) }
        assert_eq!(vin, [1.0, 0.0, 1.0]);

        let mut vin = [-1.0, 0.0, 1.0];
        unsafe { arm_abs_f64(vin.as_ptr(), vin.as_mut_ptr(), vin.len() as u32) }
        assert_eq!(vin, [1.0, 0.0, 1.0]);
    }

}
