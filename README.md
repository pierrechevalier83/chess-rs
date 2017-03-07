chess-rs (WIP) - a cli frontend for chess written in rust
---------------------------------------------------------
- We use [matrix_display](https://github.com/pierrechevalier83/matrix_display) to pretty print the board in all its colours and unicode glory.
- We use [termion](https://github.com/ticki/termion) for handling user input.

![alt tag](https://github.com/pierrechevalier83/matrix_display/blob/master/screenshots/chess.png)

Install (the easy way)
----------------------
`cargo install chess-rs`

Download
--------
`git clone git@github.com:pierrechevalier83/chess-rs.git`

Build
-----
`cargo build --release`

Run
---
`cargo run --release`

TODO
----
- Consider [this chess library from jordanbray](https://github.com/jordanbray/chess) for the data representation and move generation
- Implement the xboard protocol and interact with engines

