#!/bin/sh

bindgen \
  --output ./src/k4a.rs \
  --with-derive-default \
  --no-derive-debug \
  wrapper.hpp -- -I ./include -fms-extensions

