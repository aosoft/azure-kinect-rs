K4ARecorder (port to Rust)
=====

Ported from the k4arecorder in the [Azure Kinect Sensor SDK](https://github.com/microsoft/Azure-Kinect-Sensor-SDK/tree/develop/tools/k4arecorder).

Copy dll files to the same directory as the executable file ('target' directory).

* depthengine_2_0.dll
* k4a.dll
* k4arecord.dll

This program depends on:

* [clap](https://crates.io/crates/clap) [(LICENSE)](https://github.com/clap-rs/clap/blob/master/LICENSE-MIT)
* [ctrlc](https://crates.io/crates/ctrlc) [(LICENSE)](https://github.com/Detegr/rust-ctrlc/blob/master/LICENSE-MIT)

