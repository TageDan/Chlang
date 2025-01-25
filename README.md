# CHLANG
A Language and bytecode interpreter for chess evaluation functions.

## End goal
The goal is to create a byte-code language where any byte-array above a certain size can be interpreted and run as an evaluation function in a chess engine. This way random chess engine's (may be an overstatement to call them engines) can be created and also genetic algorithms can be used to train engines. A cool thing is that if a random chess engine that is actually good/interesting can be shared by just sharing the bytecode string of the eval-function. (like a "engine id")

I also wan't to create a way to interacte with the engines (web site or terminal app wich can be ssh'd into) and a high-level language that can be translated into the bytecode so that it can easialy be used.

I also wan't to be able to decompile the bytecode into the higher level language again. This will restrict the layout of byte code but it could make for some interesting insights into how engines evaluate positions.

## Tasks
- [X] Chess game
  - [X] board representation
  - [X] move generation
  - [X] finished headless chess game
  - [X] ui
    - [X] terminal
    - [X] gui
- [ ] Chess Engine
  - [X] tree search
  - [X] basic (hardcoded) eval
    - [ ] Evaluators
      - [X] Material Only 
  - [ ] pruning
- [ ] Chlang-language
  - [ ] bytecode interpreter for piece-values, weight's for attacks, positioning of pieces, checks, pins, skewers etc.
  - [ ] possible byte-code feature extensions 
  - [ ] high-level language compiler
  - [ ] decompiler
- [ ] Interface
  - [ ] over ssh
  - [ ] website

## Notebook

### Unkown date
I got a stack overflow since I accidentaly ran a infinite recursive call while generating king moves.
I used [this crate](https://crates.io/crates/backtrace-on-stack-overflow) to debug it and it was really helpful.

### 15/1 
When finishing the headless chess game I noticed it was REALLY slow. I tried out 
using [flamegraph](https://github.com/flamegraph-rs/flamegraph) for cpu profiling, 
something I've seen people on reddit do to analyse performance issues. After running 
it I got this flamegraph

![Flamegraph](flamegraph1.svg) 

Looking at this we can see that most of our time is spent in the make_move method, 
and furthermore most of that time is spent cloning boards. This is probably on [this line](https://github.com/TageDan/Chlang/blob/6b280c7d83fb85c042fa5aa506071c701b65f278/src/board.rs#L122) 
where we save the current boardstate so that we can undo moves and iterate 
through old positions for determining threefold repetition. Making some minor changes as 
to how old boards are stored we can get rid of that clone. And sure enough, by using [hyperfine](https://github.com/sharkdp/hyperfine) 
(a terminal benchmarking tool) on `test_game3` we see that our time goes from an 
average of 456.5 ms ± 14.1 ms to _1.1ms ± 0.2ms_! That's a 450x speedup (if it scales linearly). 
Before, running `test_game` would cuase wsl to crash for me and now it too runs in 1.1ms on average.

### 23/1
Today I readded a faster hashmap. I had done this before but the last few days I did some git mistakes and managed to undo that change. Anyways it's readded now. The
hashmap is supposed to be faster. It's used in the rustc compiler and can be found [here](https://crates.io/crates/rustc-hash)

I also ran some benchmarks, again using hyperfine:
```
Benchmark 1: target/release/chlang POSITIONAL 3 POSITIONAL 3
  Time (mean ± σ):      1.351 s ±  0.025 s    [User: 1.331 s, System: 0.019 s]
  Range (min … max):    1.310 s …  1.383 s    10 runs

Benchmark 1: target/release/chlang POSITIONAL 4 POSITIONAL 4
  Time (mean ± σ):      9.091 s ±  0.169 s    [User: 9.019 s, System: 0.060 s]
  Range (min … max):    8.857 s …  9.448 s    10 runs
```
before implementing a (hopefully) better pruning. Note that these games are determenistic which makes the
benchmark more consistent but on the other hand might not transfer well to other games. 

After the fixing the pruning the benchmarks looks like this:
```
Benchmark 1: target/release/chlang POSITIONAL 3 POSITIONAL 3
  Time (mean ± σ):     747.9 ms ±  12.2 ms    [User: 735.8 ms, System: 11.1 ms]
  Range (min … max):   728.7 ms … 759.5 ms    10 runs

Benchmark 1: target/release/chlang POSITIONAL 4 POSITIONAL 4
  Time (mean ± σ):      3.967 s ±  0.082 s    [User: 3.922 s, System: 0.041 s]
  Range (min … max):    3.855 s …  4.097 s    10 runs  
```

Yaaaay! Double the speed (for these benchmarks)

I did another benchmark with a slight cache improvement but saw no effect. Honestly I don't even know if the cache makes any differens at the moment. It feels like a
cache hit is too rare for the added overhead of searching the cache for every node. I should probably develop/find some method to test this further.



