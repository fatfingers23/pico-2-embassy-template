bin := "target/thumbv8m.main-none-eabihf/debug/embassy_rp_2_template"

flash:
    ${PICO_SDK_PATH}/openocd/0.12.0+dev/openocd -s ${PICO_SDK_PATH}/openocd/0.12.0+dev/scripts -f interface/cmsis-dap.cfg -f target/rp2350.cfg -c 'adapter speed 5000' -c 'program ./{{bin}} verify reset exit'

log:
    socat /dev/ttyACM0,rawer,b115200 STDOUT