# Growtopia.rs
First ever Private server for Growtopia made with Rust!

## Directories
`/src` -> Contains the rust files  
`/js` -> A folder that contains the js files to host the http server, will soon move to rust.

## Running
### As a script
At the root of the dir, simply do `cargo run`

### As an Executable
At the root of the dir, simply do `cargo build`  
Then open up the `/target/debug` folder then run `ps.exe`  
Take note: Even though the folder name is "debug", the optimization is the same as "release".  
If you'd like it to be in the "release" folder, run `cargo build --release` instead.

## Credits
Credits to [Mempler](https://github.com/Mempler) for the VariantList crate.