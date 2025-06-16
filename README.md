# Working File System with UI for STM32F411CE

## Introduction
This project aims to implement a **working file system on an SD card** using an **STM32F411CE microcontroller**, with the ability to **browse directories and files** displayed on a **Waveshare 1.69-inch LCD module**. It serves as a **proof of concept** for integrating these hardware components with a custom file system in an embedded Rust environment, leveraging the `stm32f4xx-hal`.

## Hardware Modules
* **STM32F411CE Black Pill Board** (or similar STM32F411CE based board)
* **Waveshare 1.69-inch LCD Module**
* **Micro SD Card Reader Module** (SPI interface)

## Project Goals / To-Do List
* **Develop a display driver** for the Waveshare 1.69-inch LCD Module, building upon the `embedded-graphics` to enable graphics and text rendering.
* **Implement an SPI driver** for reliable communication with the Micro SD Card, utilizing the `stm32f4xx-hal` SPI peripheral.
* **Integrate and utilize the `LittleFS2` file system** to manage data on the SD card, including formatting and basic file operations (read, write, list).
* **Design and implement a user interface (UI)** on the LCD to allow for intuitive Browse of directories and display of file names.
