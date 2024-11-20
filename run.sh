#!/bin/bash
DEFMT_LOG=debug cargo build || exit 1;
~/.pico-sdk/openocd/0.12.0+dev/openocd -s ~/.pico-sdk/openocd/0.12.0+dev/scripts -f interface/cmsis-dap.cfg -f target/rp2350.cfg -c 'adapter speed 5000' -c 'program ./target/thumbv8m.main-none-eabihf/debug/embassy_rp_2_template verify reset exit'
# sudo socat /dev/ttyACM0,rawer,b115200 STDOUT | defmt-print -e target/thumbv8m.main-none-eabihf/debug/embassy_rp_2_template