
# Rust 3x+1

Simple tool to brute force 3x+1 for a second loop. Only takes positive numbers.

It detects a new loop by checking if the smallest number it has encountered appears again. It is my theory that if there is another loop it will probably be the smallest number it has encoutnered.

This doesn't detect if there is a run away event where a number shoots into infinity (Or the max of a [U128 int](https://learning-rust.github.io/docs/a8.primitive_data_types.html#u8-u16-u32-u64-u128)) but if you ever find an area where it loops into infinity or crashes then you can run that smaller section of numbers if `-d` which will output every operation, use this to find what number it is and what path is taken. 

## Usage

```bash
USAGE:
    rust-3xp1.exe [OPTIONS]

OPTIONS:
    -c, --count <COUNT>     Number of times to try [default: 50000]
    -d, --double-verbose    Weather to output current number every step of 3x+1
    -h, --help              Print help information
    -s, --start <START>     Number to start at [default: 195147905179352825856]
    -v, --verbose           Weather to output current number each time
    -V, --version           Print version information
```


## FAQ

#### What is 3x+1?

check out [this](https://youtu.be/094y1Z2wpJg) video by Veritasium

#### Why?

This hasn't left my brain since I saw the video, I wanted to mess around with a tool to brute force for a new loop.

#### Will a new loop happen on the smallest number?

The base of this entire code is that the smallest number is where a new loop will be found. Based on the [4 known loops](https://en.wikipedia.org/wiki/Collatz_conjecture#Iterating_on_all_integers) where the number that it loops around to is also the closest number to 0 in that cycle I do belive so.

There is no real way of knowing without checking or finding a number that leaves this code running forever at which point you will have to investigate fruther with `-d`.
