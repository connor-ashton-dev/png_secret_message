# Process
Here I will list everything I need to do for each part of the project

## 1. Chunk Types
- [x] Write a `ChunkType` struct 
- [x] Implement a `TryFrom<[u8; 4]>` for the `ChunkType`
- [x] Implement a `FromStr` for the `ChunkType`
- [x] Implement `Display` for the `ChunkType`
- [x] Implement or derive `PartialEq` and `Eq` for the `ChunkType`
##### Required methods
- [x] `fn bytes(&self) -> [u8; 4]`
- [x] `fn is_valid(&self) -> bool`
- [x] `fn is_critical(&self) -> bool`
- [x] `fn is_public(&self) -> bool`
- [x] `fn is_reserved_bit_valid(&self) -> bool`
- [x] `fn is_safe_to_copy(&self) -> bool`


## 2. Chunks
- [x] Write a `Chunk` struct 
- [x] Implement `TryFrom<&[u8]>` for the Chunk.
- [x] Implement `Display` for the Chunk.
##### Required methods:
- [x] `fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chun`k
- [x] `fn length(&self) -> u32`
- [x] `fn chunk_type(&self) -> &ChunkType`
- [x] `fn data(&self) -> &[u8]`
- [x] `fn crc(&self) -> u32`
- [x] `fn data_as_string(&self) -> Result<String>`
- [x] `fn as_bytes(&self) -> Vec<u8>`

## 3. PNG
- [x] Write a Png struct 
- [x] In the impl block, add a public constant called `STANDARD_HEADER` that has the 8 standard header bytes.
- [x] Implement `TryFrom<&[u8]>` for the Png.
- [x] Implement `Display` for the Png.
##### Required methods:
- [x] `fn from_chunks(chunks: Vec<Chunk>) -> Png`
- [x] `fn append_chunk(&mut self, chunk: Chunk)`
- [x] `fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk>`
- [x] `fn header(&self) -> &[u8; 8]`
- [x] `fn chunks(&self) -> &[Chunk]`
- [x] `fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk>`
- [x] `fn as_bytes(&self) -> Vec<u8>`
