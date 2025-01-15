# CHLANG
A Language and bytecode interpreter for chess evaluation functions.

## End goal
The goal is to create a byte-code language where any byte-array above a certain size can be interpreted and run as an evaluation function in a chess engine. This way random chess engine's (may be an overstatement to call them engines) can be created and also genetic algorithms can be used to train engines. A cool thing is that if a random chess engine that is actually good/interesting can be shared by just sharing the bytecode string of the eval-function. (like a "engine id")

I also wan't to create a way to interacte with the engines (web site or terminal app wich can be ssh'd into) and a high-level language that can be translated into the bytecode so that it can easialy be used.

I also wan't to be able to decompile the bytecode into the higher level language again. This will restrict the layout of byte code but it could make for some interesting insights into how engines evaluate positions.

## Tasks
- [ ] Chess game
  - [x] board representation
  - [x] move generation
  - [x] finished headless chess game
  - [ ] ui
    - [x] terminal
    - [ ] gui
- [ ] Chess Engine
  - [ ] tree search
  - [ ] basic (hardcoded) eval
  - [ ] pruning
- [ ] Chlang-language
  - [ ] bytecode interpreter for piece-values, weight's for attacks, positioning of pieces, checks, pins, skewers etc.
  - [ ] possible byte-code feature extensions 
  - [ ] high-level language compiler
  - [ ] decompiler
- [ ] Interface
  - [ ] over ssh
  - [ ] website

## Problem Notebook
### 15/1 
When finishing the headless chess game I noticed it was REALLY slow. I tried out 
using [flamegraph](https://github.com/flamegraph-rs/flamegraph) for cpu profiling, 
something I've seen people on reddit do to analyse performance issues. After running 
it I got this flamegraph

[!bench.svg] 
