# CMSIS DSP Rust bindings [![CI](https://github.com/sawi97/cmsisdsp-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/sawi97/cmsisdsp-sys/actions/workflows/ci.yml)

This crate provides Rust bindings for the CMSIS DSP library. The bindings are generated using bindgen.

## Documentation

The documentation for the CMSIS DSP library can be found [here](https://arm-software.github.io/CMSIS-DSP/v1.15.0/index.html).

## Configuration

The CMSIS DSP library can be configured using the `CMSISDSP_CFG` environment variable. 
This needs to be the same format as the Cmake command line options.

Eg, to disable loop unrolling and adding matrix checks:

    CMSISDSP_CFG = "-DLOOPUNROLL=OFF -DMATRIXCHECK=ON"

Configurations can be obtained by cloning the CMSIS-DSP repo and running:

    cmake -LH
