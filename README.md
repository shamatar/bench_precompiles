# Precompiles benchmarking vectors

Code in this repo generates precompile benchmarking data.

## Available vectors

### SHA256

Files are located in `vectors/sha256` folder. There are two variants in the corresponding subfolders:
- `current` - test vectors that have encoded precompile cost as of moment before EIP 2666
- `proposed` - test vectors that have encoded precompile cost as of moment after EIP 2666

Filenames are encoded as `input_param_scalar_{length}_gas_{gas_value}.csv` where `length` is an input length (encoded in a filename for convenience purposes), and `gas` is an expected gas spent for evaluation of this test vector by the the `SHA256` precompile. Each `csv` file contains two columns and no header. First column is hex-encoded input bytestring, second column is hex encoded output bytestring. 

Important: during benchmarking it's suggested NOT to compare output of the precompile to the expected output in a function being benchmarked.

Client developers are free to use any benchmarking harness to get a precompile running time (and expected gas spend). If measurements are performed in a simple loop then at least `10_000` repeats should be performed for each vector before averaging.