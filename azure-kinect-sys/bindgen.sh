#!/bin/sh

bindgen \
  --output ./src/bindgen_k4a.rs \
  --allowlist-function \(?i\)k4a_.* \
  --allowlist-var \(?i\)k4a_.* \
  --dynamic-loading Funcs \
  --dynamic-link-require-all \
  --size_t-is-usize \
  --with-derive-default \
  ./include/k4a/k4a.h -- -I ./include -fms-extensions
sed -i -n "/impl Funcs/q;p" ./src/bindgen_k4a.rs
sed -i -e "/libloading/d" ./src/bindgen_k4a.rs

bindgen \
  --output ./src/bindgen_k4arecord.rs \
  --allowlist-function k4a_\(record\|playback\)_.* \
  --allowlist-var \(?i\)k4a_track_.* \
  --dynamic-loading Funcs \
  --dynamic-link-require-all \
  --size_t-is-usize \
  --with-derive-default \
  wrapper.h -- -I ./include -fms-extensions
sed -i -n "/impl Funcs/q;p" ./src/bindgen_k4arecord.rs
sed -i -e "/libloading/d" ./src/bindgen_k4arecord.rs
