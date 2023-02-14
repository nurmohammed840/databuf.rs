#### Variable-Length Integer Encoding

This encoding ensures that smaller integer values need fewer bytes to encode. Support types are `LEU15`, `LEU22`, `LEU29`, Encoded in little endian.
By default, `LEU29` is used to encode length.
 
Encoding algorithm is very straightforward,
The most significant bits of the first byte determine the byte length to encode the number in little endian.

#### LEU15

|  MSB  | Length | Usable Bits | Range    |
| :---: | :----: | :---------: | :------- |
|   0   |   1    |      7      | 0..128   |
|   1   |   2    |     15      | 0..32768 |

#### LEU22

|  MSB  | Length | Usable Bits | Range      |
| :---: | :----: | :---------: | :--------- |
|   0   |   1    |      7      | 0..128     |
|  10   |   2    |     14      | 0..16384   |
|  11   |   3    |     22      | 0..4194304 |

#### LEU29

|  MSB   | Length | Usable Bits | Range        |
| :---:  | :----: | :---------: | :----------- |
|  0     |   1    |      7      | 0..128       |
|  10    |   2    |     14      | 0..16384     |
|  110   |   3    |     21      | 0..2097152   |
|  111   |   4    |     29      | 0..536870912 |

 
For example, Binary representation of `0x_C0DE` is `0x_11_00000011_011110`
 
`LEU22(0x_C0DE)` is encoded in 3 bytes:
 
```yml
1st byte: 11_011110      # MSB is 11, so read next 2 bytes
2nd byte:        11
3rd byte:        11
```

Another example, `LEU22(107)` is encoded in just 1 byte:

```yml
1st byte: 0_1101011      # MSB is 0, So we don't have to read extra bytes.
```