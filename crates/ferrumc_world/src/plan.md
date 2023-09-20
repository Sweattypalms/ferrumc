### Plan
Since the file system will probably be a bottleneck we will need to be smart about how we store things. First thing is to store commonly accessed world data (seed for chunk gen, time, etc.) in 1 file, then all the other data in another. 
Since the client will need all the data we will have to save all of it, but we can access the minimal version on the server for things like generating chunks and changing things serverside.

As for storing the world, that's gunna be a tough one. We could store it in chunks, but that would be a lot of files. We could store it in one file, but that would be a lot of data to load at once. 
We will probably use the protobuf protocol to compress and serialize the data tho.
One idea is to store chunks in separate files containing 8x8x8 blocks. If the block is all the same, it can be indicated as such by having the first byte indicate everything is the same and the bytes after indicate what block it is.
This would mean each chunk would use 128 files tho, so maybe not a great idea. Maybe in 4x256x4 sized chunks? Would make it harder to store all the same block type tho.
Another idea would be to store the entire chunk, then have it split into y levels. For each level define a "default" block which would be whatever is the most common, then just specify any changes.
This would mean that if a chunk is all the same block, it would only take up 1 block specification, and then only scale with the number of differences. At high and low y levels, this would save a lot of space. Could also specify changes in sections to further reduce the size