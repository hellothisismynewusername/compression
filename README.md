It's a lossless file compressor that kinda sucks and is slow and a memory hog. At best I made something 3% smaller (that wasn't just a test file full of the same stuff), but it really depends on the file because it just uses run length encoding.

`cargo build` to build and then either `compression.exe` or `./compression` to run.
Example: 
`./compression file_to_be_compressed` for compression.
`./compression file_to_be_decompressed -d` for decompression.