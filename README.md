## Raspberry Pico 2 Embassy Template

This is a simple template to get you started with the rp235xa line of chips with Rust and Embassy. Mileage will vary heavily on this. I am going write what I did to get going and what my work flow is. If you do have any questions or problems (never sure if anyone actually checks this stuff out). Feel free to make an issue or can reach out to me via [BlueSky](https://bsky.app/profile/fatfingers.bsky.social)

I am working on Pop!_OS so what these instructions are geared towards

# Features
* One command flashing, running, and logging. All without unpluging or holding down the BOOTSEL button.
* Logging via serial usb
* Easy template to get started without pulling all of embassy

# Requirements
* Too much time and too excited to wait for probe-rs which will be 100x better
* A [debug probe](https://www.raspberrypi.com/products/debug-probe/) or a pico 1 or 2 running the debugprobe firmware. Can find more info and wiring [On page 18 of this pdf](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)
* [just](https://just.systems/man/en/)
* socat or minicom installed, can view [justfile](./justfile) for examples of both
* Setup the udev rules [98-embassy-usb-serial-logger.rules](./98-embassy-usb-serial-logger.rules). This allows you to use socat or minicom without sudo and part of the key of "single command"
* Pico SDK installed and the env `PICO_SDK_PATH` set. It's going sound crazy, but I found the easiest way to get it installed is to install the [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=raspberry-pi.raspberry-pi-pico) and run an example project. This should install it to `~/.pico-sdk`

# Running
Ideally once all that is done you should just be able to do `cargo run` and it handles the reset.

**Note: I have found that at least on my laptop if I plug in the probe first before the Pico i'm flashing that nothing is sent over serial. So may see the same thing. It's a bit like they're both fighting over /dev/ttyACM0. If I switch to using UART for logging i'll update this repo


1. Cargo run builds the firmware
2. The runner passes the name and runs `just flash` via [config.toml](./.cargo/config.toml)
3. Just takes and uses pico-sdk's openocd to flash via the SWD ports then resets the pico to by pass a breakpoint that hits every time you flash
4. Waits a second, and is recommended to have a 1-2second wait in your code like this example for the pico to turn back on and be registered as a serial device.
5. Uses socat to view the serial

Remember this is a template, feel free to change the runner or just file as you see fit. This is to get you hacking on the Pico 2. Good luck and have fun!

# Why is there cyw43-firmware in here?
Hoping to update examples for the RM2 on a Pico 2 as well as the Pico 2 Plus W. 