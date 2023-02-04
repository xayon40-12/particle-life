# Particle life

This is my toy implementation of Particle life, a simulation with basic laws that show complex emergent behaviours.


## Configuration

Currently there is no configurations possible. The only way to change the behaviour is to modify the values in the code and run again.  
The bloc:
```rust
    // ----------------------- PARAMETERS -----------------------
    
    // ----------------------------------------------------------
  
```
in the file `src/main.rs` can be modified to alter the behaviour.  
Some interface might be added in the future to control these parameters directly from the running application.

## Run

To run the project you will need the rust project manager called `cargo` that is most easily obtained by installing the `rustup` rust installer.  
Once you have `cargo`, simply run anywhere in this repository:

```sh
cargo run --release
```

and the simulation should start.