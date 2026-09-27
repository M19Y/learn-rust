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

1. [hello-world](./rust%20basic/src/hello_world.rs)
2. [variables](./rust%20basic/src/variabels.rs)
3. [data-types](./rust%20basic/src/data_types.rs)
4. [comparison-operator](./rust%20basic/src/comparison_operator.rs)
5. [tuple](./rust%20basic/src/tuple.rs)
6. [array](./rust%20basic/src/array.rs)
7. [constant](./rust%20basic/src/constant.rs)
8. [string](./rust%20basic/src/string.rs)
9. [ownership](./rust%20basic/src/ownership.rs)
10. [if-else](./rust%20basic/src/if_else.rs)
11. [loop](./rust%20basic/src/loop.rs)
12. [while-loop](./rust%20basic/src/while_loop.rs)
13. [for-loop](./rust%20basic/src/for_loop.rs)
14. [function](./rust%20basic/src/function.rs)
15. [ownership-function](./rust%20basic/src/function_ownership.rs)
16. [reference-and-borrowing](./rust%20basic/src/reference_and_borrowing.rs)
17. [slice](./rust%20basic/src/slice.rs)
18. [string-slice](./rust%20basic/src/string_slice.rs)
19. [struct](./rust%20basic/src/struct.rs)
20. [method](./rust%20basic/src/method.rs)
21. [enum](./rust%20basic/src/enum.rs)
22. [pattern-matching](./rust%20basic/src/pattern_matching.rs)
23. [type-alias](./rust%20basic/src/type_alias.rs)

24. Module
    1. [module](./rust%20basic/src/learn_module/mdl.rs)
    2. [use-keyword](./rust%20basic/src/learn_module/use_key.rs)
    3. [separate-module](./rust%20basic/src/learn_module/separate_module.rs)

25. [crate](./rust%20basic/src/learn_crate/crt.rs)

26. Trait
    1. [trait](./rust%20basic/src/learn_trait/trt.rs)
    2. [trait-as-parameter](./rust%20basic/src/learn_trait/trt_as_param.rs)
    3. [trait-as-return](./rust%20basic/src/learn_trait/trt_as_return.rs)

27. Generic
    1. [generic-struct](./rust%20basic/src/learn_generic/generic_struct.rs)
    2. [generic-trait](./rust%20basic/src/learn_generic/generic_trait.rs)
    3. [generic-enum](./rust%20basic/src/learn_generic/generic_enum.rs)
    4. [generic-type-bound](./rust%20basic/src/learn_generic/generic_type_bound_and_where_clause.rs)
    5. [generic-default-type](./rust%20basic/src/learn_generic/generic_default_type.rs)
    6. [generic-function](./rust%20basic/src/learn_generic/generic_function.rs)
    7. [generic-method](./rust%20basic/src/learn_generic/generic_method.rs)

28. [overloadable-operators](./rust%20basic/src/overloadable_operators.rs)
29. [optional-value](./rust%20basic/src/optional_value.rs)
30. [comparing](./rust%20basic/src/comparing.rs)
31. [string-manipulation](./rust%20basic/src/string_manipulation.rs)
32. [formatting-debug-display](./rust%20basic/src/formating.rs)
33. [closure](./rust%20basic/src/closure.rs)
34. [sequences](./rust%20basic/src/sequences.rs)
35. [maps](./rust%20basic/src/map.rs)
36. [sets](./rust%20basic/src/sets.rs)
