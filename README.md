# Friday, 28 Agustus 2026;

# Compile, Test and Run:



### Without Cargo

1. Compile simpel file:

`rustc simple-file.rs`

2. Run simple file 

`./simple-file.rs`

3. Compile and run inside different folders, assume we have 2 folders its src and bin.
   make sure you are in the root of your projects:

`rustc src/simple-file.rs -o bin/simple-file; ./bin/simple-file.rs`



### With Cargo

1. Compile and run:

`cargo run`

2. Compile (make distribution file):

`cargo build --release`

3. Compile (debug) unoptimezed)

`cargo build`

4. Run all the test function:

`cargo test`

5. Run specific test function:

`cargo test simple_function_target -- --exact`

6. Run test with no capture (it will show the println):

`cargo test -- --nocapture`

