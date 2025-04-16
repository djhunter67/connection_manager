# Connection Manager

## Synopsys
- Scan for mDns devices, store all ip address and print hostname's of discovered devices
- Filter for a specific kind of device based on the discovered device's name
- Make a `GET` connection to all discovered devices simultaneously.
- Print the results of the expected `JSON`

## How to use
- The goal is to integrate this project and it dependency, `mdns_scanner` into a web application that will allow users to pick parameters.
- If you have the compiled binary you simply execute: `./connection_manger`
  - I recognize that this method is not user friendly but this is the present way to run this binary.
- if you have cloned this repository then: 
  - ```bash
  cd connection_manager
  ```
  - ```bash
  cargo run
  ```
  
  - The library for this project is available and ready to be integrated into other `Rust` projects.
  - Cross compiling for `C/C++` is available but untested.
  
  ### TODO
  - Use this library inside of a web application.
