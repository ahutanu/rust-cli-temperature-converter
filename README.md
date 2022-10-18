 
# Fahrenheit <> Celsius CLI Temperature Converter in Rust
Done as part of practicing the [3.5 Control Flow section of the official Rust documentation](https://doc.rust-lang.org/book/ch03-05-control-flow.html) which has users to try to build certain programs to practice what they learned.

This program can be used via CLI. It accepts entering a temperature in Celsius or Fahrenheit and will convert it automatically to the other unit. Please see the Demo section below for more details.

## Run Locally  

Clone the project  

~~~bash  
  git clone https://link-to-project
~~~

Go to the project directory  

~~~bash  
  cd rust-cli-temperature-converter
~~~

Build & Run 

~~~bash  
cargo run
~~~

## Demo  

~~~bash  
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/temperature-converter`
This tool converts temperatures between Fahrenheit and Celsius.
Please enter your temperature value, ending in F or C respectively (e.g., 32C, 86.6F).
32C
You have entered: 32C. It will now be converted to F.
The temperature has been converted to: 89.6F
~~~

## Playground  

This project is meant as a playground to practice the Rust programming language while following the official documentation.
The author has no previous experience with Rust and this has been developed with knowledge acquired in Rust by following up until the [3.5 Control Flow section of the official Rust documentation](https://doc.rust-lang.org/book/ch03-05-control-flow.html). 
If further concepts that may improve this program at later chapters are learned, this program will not be refactored to use the new concepts, but rather new programs will be built.
Please consider this when reviewing the code.