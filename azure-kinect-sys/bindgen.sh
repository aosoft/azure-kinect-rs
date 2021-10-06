#!/bin/sh

bindgen \
  --output ./src/k4a.rs \
  --allowlist-function k4a_.*\
  --dynamic-loading k4a \
  --dynamic-link-require-all \
  --with-derive-default \
  --no-derive-debug \
  wrapper.h -- -I ./include -fms-extensions
sed -i -n "/impl k4a/q;p" ./src/k4a.rs
sed -i -e "/libloading/d" ./src/k4a.rs
