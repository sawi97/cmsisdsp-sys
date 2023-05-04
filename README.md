# CMSI DSP Rust bindings

This crate provides Rust bindings for the CMSIS DSP library. The bindings are generated using bindgen.

## Configuration

The CMSIS DSP library can be configured using the `CMSISDSP_CFG` environment variable. 
This needs to be the same format as the Cmake command line options from cmsisdspconfig.py.

Eg, to disable FFT tables (which requires alot of space):

    CMSISDSP_CFG = "-DCONFIGTABLE=ON -DALLFAST=ON"

Configurations can be obtained by cloning the CMSIS-DSP repo and running:

    streamlit run cmsisdspconfig.py
