# Learning
Here I will be taking notes about everything I've learned when making this project


## Bytes
- A `bit` is a 0 or a 1
- A `byte` is 8 `bits`
- In decimal notation, a byte is a number from 0 to 255
- in Rust, bytes are represented by the type `u8`
- Bytes are "fun" because their meaning is usually open to interpretation
    - Individual bytes can be interpreted as 8 indivual bites or as a number from 0 to 255
    - They can also be interpreted in groups
        - a 32-bit integer is 4 bytes
            - To do this in rust we use `u32::from_be_bytes([1,2,3,4])`
        - a rust `String` is just a `Vec<u8>` whose bytes have been validated as `UTF-8`


## Chunk Layout
- Each chunk consists of 4 parts
    1. Length
        - A 4-byte unsigned integer giving the number of bytes in the chunk's data field
        - Counts only the data field, not itself, the chunk type code, or the CRC
        - Zero is a valid length
        - Value must not exceed 2^31 bytes
    2. Chunk Type
        - a 4-byte chunk type code
        - Restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal)
    3. Chunk Data
        - the data bytes appropriate to teh chunk type
        - can be of length 0
    4. CRC
        - a 4-byte CRC (Cyclic Redundancy Check) calculated on the preceding bytes in the chunk
        - includes the chunk type code and chunk data fields, but not including the length field
        - CRC is always present, even for chunks containing no data


## Chunk Types
- PNG files are basically a list of `chunks`
    - Each `chunk` has a type that can be represented as a 4 char string
- There are standard chunk types for things like images, but there's no rule 
  that would prevent you from inserting your own chunks with whatever data you want
- We can tell PNG decoders to ignore our chunks depending on how we capitalize our chunk types
- Semantics of property bits
    - Ancillary bit: bit 5 of first byte
        - 0 (uppercase) = critical, 1 (lowercase) = ancillary
        - These are neccessary to display the file
            - In our case, our secret chunks are not so we will be using lowercase probably
    - Private bit: bit 5 of second byte
        - 0 (uppercase) = public, 1 (lowercase) = private
        - These are custom-defined, not part of the official PNG specification.
            - Again, we will be using these ones in our secret messages
    - Reserved bit: bit 5 of third byte
        - Must be 0 (uppercase)
    - Safe-to-copy bit: bit 5 of fourth byte
        - 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy
        - This means that if we make edits to the file or do anything with an editor, it will be copied even if the editor doesn't recognize it
            - We will be doing this so that our secret messages persist when the file is changed

```
For example, the hypothetical chunk type name bLOb has the property bits:  

   bLOb  <-- 32 bit chunk type code represented in text form  
   ||||  
   |||+- Safe-to-copy bit is 1 (lowercase letter; bit 5 is 1)  
   ||+-- Reserved bit is 0     (uppercase letter; bit 5 is 0)  
   |+--- Private bit is 0      (uppercase letter; bit 5 is 0)  
   +---- Ancillary bit is 1    (lowercase letter; bit 5 is 1)  
```


## PNG File Structure
- A PNG consists of a PNG signature followed by a series of chunks
- The first 8 bytes of a PNG file are ALWAYS the same
    [137, 80, 78, 71, 13, 10, 26, 10]
