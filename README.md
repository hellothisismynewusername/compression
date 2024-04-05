It's a lossless file compressor that kinda sucks and is slow and a memory hog. It just uses run length encoding in a really inefficient way.

`cargo build` to build and then either `compression.exe` or `./compression` to run.
Example:
`./compression file_to_be_compressed` for compression.
`./compression file_to_be_decompressed -d` for decompression.
