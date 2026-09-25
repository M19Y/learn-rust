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

1. [hello-world](./rust basic/src/hello_world.rs)
2. [variabels](./rust basic/src/variabels.rs)
3. [data-types](./rust basic/src/data_types.rs)
4. [comparison-operator](./rust basic/src/comparison_operator.rs)
5. [tuple](./rust basic/src/tuple.rs)
6. [array](./rust basic/src/array.rs)
7. [constant](./rust basic/src/constant.rs)
8. [string](./rust basic/src/string.rs)
9. [ownership](./rust basic/src/ownership.rs)
10. [if-else](./rust basic/src/if_else.rs)
11. [loop](./rust basic/src/loop.rs)
12. [while-loop](./rust basic/src/while_loop.rs)
13. [for-loop](./rust basic/src/for_loop.rs)
14. [function](./rust basic/src/function.rs)
15. [ownership-function](./rust basic/src/function_ownership.rs)
16. [reference-and-borrowing](./rust basic/src/reference_and_borrowing.rs)
17. [slice](./rust basic/src/slice.rs)
18. [string-slice](./rust basic/src/string_slice.rs)
19. [struct](./rust basic/src/struct.rs)
20. [method](./rust basic/src/method.rs)
21. [enum](./rust basic/src/enum.rs)
22. [pattern-matching](./rust basic/src/pattern_matching.rs)
23. [type-alias](./rust basic/src/type_alias.rs)
24.1 [module](./rust basic/src/learn_module/mdl.rs)
24.2 [use-keyword](./rust basic/src/learn_module/use_key.rs)
24.3 [separate-module](./rust basic/src/learn_module/separate_module.rs)
25. [crate](./rust basic/src/learn_crate/crt.rs)
26.1 [trait](./rust basic/src/learn_trait/trt.rs)
26.2 [trait-as-parameter](./rust basic/src/learn_trait/trt_as_param.rs)
26.3 [trait-as-return](./rust basic/src/learn_trait/trt_as_return.rs)
27.1 [generic-struct](./rust basic/src/learn_generic/generic_struct.rs)
27.2 [generic-trait](./rust basic/src/learn_generic/generic_trait.rs)
27.3 [generic-enum](./rust basic/src/learn_generic/generic_enum.rs)
27.4 [generic-type-bound](./rust basic/src/learn_generic/generic_type_bound_and_where_clause.rs)
27.5 [generic-default_type](./rust basic/src/learn_generic/generic_default_type.rs)
27.6 [generic-function](./rust basic/src/learn_generic/generic_function.rs)
27.7 [generic-method](./rust basic/src/learn_generic/generic_method.rs)
28. [overloadable-operators](./rust basic/src/overloadable_operators.rs)
29. [optional-value](./rust basic/src/optional_value.rs)
30. [comparing](./rust basic/src/comparing.rs)
31. [string-manipulation](./rust basic/src/string_manipulation.rs)
32. [formatting-debug-display](./rust basic/src/formating.rs)
33. [closure](./rust basic/src/closure.rs)
34. [sequences](./rust basic/src/sequences.rs)
35. [maps](./rust basic/src/map.rs)
36. [sets](./rust basic/src/sets.rs)
