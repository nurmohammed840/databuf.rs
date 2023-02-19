#### Variable-Length Integer Encoding

This encoding ensures that smaller integer values need fewer bytes to encode. Support types are `BEU15`, `BEU22`, `BEU29`, Encoded in little endian.
By default, `BEU29` is used to encode length.
 
Encoding algorithm is very straightforward,
The most significant bits of the first byte determine the byte length to encode the number in little endian.

#### BEU15

|  MSB  | Length | Usable Bits | Range    |
| :---: | :----: | :---------: | :------- |
|   0   |   1    |      7      | 0..128   |
|   1   |   2    |     15      | 0..32768 |

#### BEU22

|  MSB  | Length | Usable Bits | Range      |
| :---: | :----: | :---------: | :--------- |
|   0   |   1    |      7      | 0..128     |
|  10   |   2    |     14      | 0..16384   |
|  11   |   3    |     22      | 0..4194304 |

#### BEU29

|  MSB   | Length | Usable Bits | Range        |
| :---:  | :----: | :---------: | :----------- |
|  0     |   1    |      7      | 0..128       |
|  10    |   2    |     14      | 0..16384     |
|  110   |   3    |     21      | 0..2097152   |
|  111   |   4    |     29      | 0..536870912 |

#### BEU30

|  MSB  | Length | Usable Bits | Range         |
| :---: | :----: | :---------: | :-----------  |
|  00   |   1    |      6      | 0..64         |
|  01   |   2    |     14      | 0..16384      |
|  10   |   3    |     22      | 0..4194304    |
|  11   |   4    |     30      | 0..1073741824 |

 
For example, Binary representation of `0x_C0DE` is `0x11000000_11011110`
 
`BEU22(0x_C0DE)` is encoded in 3 bytes:
 
```yml
1st byte:        11  # MSB is 11, so read next 2 bytes
2nd byte:  11000000
3rd byte:  11011110
```

Another example, `BEU22(107)` is encoded in just 1 byte:

```yml
1st byte: 0b01101011      # MSB is 0, So we don't have to read extra bytes.
```