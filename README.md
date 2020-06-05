# Precompiles benchmarking vectors

Code in this repo generates precompile benchmarking data.

## Available vectors

All vectors are subfolder of `vectors`. Each subfolder in turn contains two subfolders:
- `current` - test vectors that have encoded precompile cost as of moment before EIP 2666
- `proposed` - test vectors that have encoded precompile cost as of moment after EIP 2666

Filenames are encoded as `input_param_scalar_{param}_gas_{gas_value}.csv` where `param` meaning varies between the precompiles and , and `gas` is an expected gas spent for evaluation of this test vector by the the corresponding precompile. Each `csv` file contains two columns and no header. First column is hex-encoded input bytestring, second column is hex encoded output bytestring. 

### SHA256

`param` is an input length

### RIPEMD160

`param` is an input length

### BNADD

`param` is always equal to 0 (no variety)

### BNMUL

`param` is always equal to 0 (no variety)

### BNPAIR

`param` is number of pairs

### BLAKE2f

`param` is number of rounds

### Notes 

Important: during benchmarking it's suggested NOT to compare output of the precompile to the expected output in a function being benchmarked.

Client developers are free to use any benchmarking harness to get a precompile running time (and expected gas spend). If measurements are performed in a simple loop then at least `1_000` repeats should be performed for each vector before averaging.