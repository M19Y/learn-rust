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

4. Compile and test with no capture 

`rustc --test src/hello-world.rs -o build/hello-world; ./build/hello-world --nocapture`
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


## Road map you should learn:

1. [hello-world](./src/hello-world.rs)
2. [variabels](./src/variabels.rs)
3. [data-types](./src/data-types.rs)
4. [comparison-operator](./src/comparison-operator.rs)
5. [tuple](./src/tuple.rs)
6. [array](./src/array.rs)
7. [constant](./src/constant.rs)
8. [string](./src/string.rs)
