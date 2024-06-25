# MarkCode
BrainF*ck interpretter using whitespace to encode hidden code in ordinary text.


MarkCode instruction is set of 3 continous blank characters (space, tabulator) in row not interrupted by a new like character.

### This is how MarkCode translates to regular BrainF*ck
(for simplicity in the following table space is represented as 0 and tabulator as 1)

000 - `>`

001 - `<`

010 - `+`

011 - `-`

100 - `.`

101 - `,`

110 - `[`

111 - `]`


To decode file you use cargo new `cargo new FILEPATH`. To generate code use included python script (`main.py`).