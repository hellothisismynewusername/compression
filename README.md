It's a compression software that uses a form of run-length encoding. Multiple files can be compressed into one file, although varying directories are not supported. There's a size limit of 4,294,967,295 bytes (~4.2 gb) for compression and decompression. To maximize the effectiveness of the compression, ensure there are many instances of files containing the same byte 8 or more times in a row.

`cargo build` to build and then either `compression.exe` or `./compression` to run.

Flags:
`-d` - Decompress the inputted file. Provide only 1 filename.  
`-v` - Print the stage at which the compression/decompression is at.

Flags and filenames can be provided in any order.

Example:
`./compression file1.bin -v file2.bin file3.jpg` - Outputs a compressed file named `out.crispyfries` containing `file1.bin`, `file2.bin`, and `file3.jpg`.  
`./compression out.crispyfries -d` - Decompresses and outputs the contained files.