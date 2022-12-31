# MD5 hash function

MD5 (message-digest algorithm) is a widely used hash function producing a 128-bit has value. MD5 can be used as a
checksum to verify data integrity. 

## MD5 algorithm

The MD5 algorithm is divided in multiple steps:

- **Binary format**: The first step is to convert the input data into binary format. This is done according to  *ASCII*.
    <p align="center">
    <img src="/static/md5_step1.png" width="500" height="125">
    </p>

- **Padding**: The goal is to obtain 512-bit blocks, with padding added to fill up the rest of the space in the block.
    Assume we have a 176 bits message. After laying out the initial 176 bits, the rest of the block is padded with a
    single one (in the end), then enough zeros to bring it up to a length of 448 bits. In this case, the padding will
    include a one and 271 zeros. The final 64 bits (512 - 448) are reserved to display the initial message's length in
    binary. In this case, 176 in binary is 10110000. The bigger the message length, the fewer zeros. If the binary
    representation of the length has more than 64 bits, only the least significant 64 bits are used (the first 64 bits
    starting from the left). If the initial message is larger than 512 bits, it is divided into 512 blocks and the same
    procedure is applied to the final block. In the case where the remaining input data is exactly 448 bits long, an
    entire extra block would need to be added for the padding.
    
    <p align="center">
    <img src="/static/md5_step2.png" width="1000" height="175">
    </p>

## References

1. [Original paper](/https://datatracker.ietf.org/doc/html/rfc1321)
2. [Noice article](https://www.comparitech.com/blog/information-security/md5-algorithm-with-examples/)
