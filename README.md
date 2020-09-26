# stm32f411re-embedded-rust-sd-card-embedded_sdmmc

Quick Pinout Guide
------

| Nucleo-F411RE Pin | SGP30 Pin   |
|-------------------|-------------|
| 5V                | Vin/5V      |
| PA5               | SCK         |
| PA6               | MISO        |
| PA7               | MOSI        |
| PB6               | CS          |


What is this project?
------

This is based on the [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart) project.

The code is for the stm32-f411re processor, and is meant for the nucleo-f411re board. This is using embedded_sdmmc to write a file to an SD card. Given an SD card this will create a file named "example.txt" in the root directory.

I've included some resources that have helped me along the way.

![Nucleo F411RE Alternate Function Mappings](/alternate-function-mappings-p1.png)

![Arduino Connectors Part 1](/arduino-connectors-p1.png)

![Arduino Connectors Part 2](/arduino-connectors-p2.png)

![Nucleo F411RE Mappings](/nucleo-f411re-mappings.png)
