# **DISCLAIMER**: Super duper deprecated

# Pork <img src="assets/Porkchop.webp" height="30">

Pants OpenCV Reset Kit
A project to efficently reset Minecraft seeds with opencv for speedrunning purposes.

This project is meant to be used along side [Specnr's Wall project](https://github.com/Specnr/MultiResetWall).\
As of now, there is no proper integration between the two and Pork handles resets through control of the mouse.

Pork was built with a standard 1920x1080 resolution and 3x3 instance setup in mind.\
Any other configurations cannot be guaranteed to work.

For more information on how Pork works, check out the [info page](INFO.md).

## Technologies Used

- [Rust](https://github.com/rust-lang/rust)
- [Opencv](https://github.com/opencv/opencv)

## Installation

Pork is very much still in development and might not work on your machine.

Opencv is a requirement for this project. Find instructions for installing it [here](https://docs.opencv.org/4.x/df/d65/tutorial_table_of_content_introduction.html).

### Clone the repo and build

```shell
git clone https://github.com/pants721/pork pork
cd pork
cargo build --release
```

### Run Pork

```shell
cargo run
```

## Authors / Contributors

- pants721 (me!)
