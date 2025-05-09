# CHLANG
A Language and bytecode interpreter for chess evaluation functions.

## End goal
The goal is to create a byte-code language where any byte-array above a certain size can be interpreted and run as an evaluation function in a chess engine. This way random chess engine's (may be an overstatement to call them engines) can be created and also genetic algorithms can be used to train engines. A cool thing is that if a random chess engine that is actually good/interesting can be shared by just sharing the bytecode string of the eval-function. (like a "engine id")

I also want to create a way to interact with the engines (web site or terminal app which you can ssh into) and a high-level language that can be translated into the bytecode so that it can easily be used.

I also want to be able to decompile the bytecode into the higher level language again. This will restrict the layout of byte code but it could make for some interesting insights into how engines evaluate positions.

## Tasks
- [X] Chess game
  - [X] board representation
  - [X] move generation
  - [X] finished headless chess game
  - [X] ui
    - [X] terminal
    - [X] gui
- [X] Chess Engine
  - [X] tree search
  - [X] basic (hardcoded) eval
    - [X] Evaluators
      - [X] Material Only 
      - [X] Positional
  - [X] pruning
- [ ] Chlang-language
  - [X] bytecode interpreter for piece-values, weight's for attacks, positioning of pieces, checks, pins, skewers etc.
  - [ ] ~possible byte-code feature extensions~ 
  - [X] high-level language compiler
  - [ ] de-compiler
- [X] Interface
  - [ ] ~over ssh~
  - [X] website
- [X] Tools
  - [X] Bot compare
  - [X] Bot training (done but slow)


## Finished Evaluator specification
The finished evaluator should be able to evaluate:
- [X] Piece material
- [X] Positioning of pieces
- [X] Attacks/defenders of pieces (maybe except en passant, since it's kinda ambiguous how to implement)
- [X] Possibility of castle

I also want it to be able to evaluate (though these might be harder to implement / could affect performance more):
- [ ] ~Pins~
- [ ] ~Skewers~
- [ ] ~Forks~
- [ ] Piece number of moves (you would for example want to be able to move your king)

These should be representable in bytes as follows:
bytes(<values for pieces>, <value_for_pieces_for_square>, <value_for_piece_attacks>, <value_for_castle_long_short>)
this should be 6+6*64+6+2 bytes.

High level it could look like:
```

Extra:
  LongCastle:
    5
  ShortCastle:
    5
Pawn:
  Base:
    10
  Position:
    6 6 6 6 6 6 6 6
    5 5 5 5 5 5 5 5
    4 4 4 4 4 4 4 4
    3 3 3 3 3 3 3 3
    2 2 2 2 2 2 2 2
    1 1 1 1 1 1 1 1
    0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0
  Attack:
    10
  Moves:
    3

Knight:
  Base:
    30
...
...
...
and so on
```


## Notebook

### Unknown date
I got a stack overflow since I accidentally ran a infinite recursive call while generating king moves.
I used [this crate](https://crates.io/crates/backtrace-on-stack-overflow) to debug it and it was really helpful.

### 15/1 
When finishing the headless chess game I noticed it was REALLY slow. I tried out 
using [flamegraph](https://github.com/flamegraph-rs/flamegraph) for cpu profiling, 
something I've seen people on reddit do to analyze performance issues. After running 
it I got this flamegraph

![Flamegraph](flamegraph1.svg) 

Looking at this we can see that most of our time is spent in the make_move method, 
and furthermore most of that time is spent cloning boards. This is probably on [this line](https://github.com/TageDan/Chlang/blob/6b280c7d83fb85c042fa5aa506071c701b65f278/src/board.rs#L122) 
where we save the current boards state so that we can undo moves and iterate 
through old positions for determining threefold repetition. Making some minor changes as 
to how old boards are stored we can get rid of that clone. And sure enough, by using [hyperfine](https://github.com/sharkdp/hyperfine) 
(a terminal benchmarking tool) on `test_game3` we see that our time goes from an 
average of 456.5 ms ± 14.1 ms to _1.1ms ± 0.2ms_! That's a 450x speedup (if it scales linearly). 
Before, running `test_game` would cause wsl to crash for me and now it too runs in 1.1ms on average.

### 23/1
Today I re-added a faster hashmap. I had done this before but the last few days I did some git mistakes and managed to undo that change. Anyways, it's re-added now. The
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
before implementing a (hopefully) better pruning. Note that these games are deterministic which makes the
benchmarks more consistent but on the other hand might not transfer well to other games. 

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

I did another benchmark with a slight cache improvement but saw no effect. Honestly I don't even know if the cache makes any difference at the moment. It feels like a
cache hit is too rare for the added overhead of searching the cache for every node. I should probably develop/find some method to test this further.

### 9/2
Today I decided to limit the range for coefficients to 0-128 (not including 128) since that is what normal ascii supports and I can't be bothered to do something about the rest.

### 5/3
Today I looked at the string for a bot and realized that I should have done that earlier. As it turns out, most ascii values are non visible. becuase of this I will further restrict values to the visible non space characters. (33-126 (94 different values)), it's a little bit sad the the bots can't have that much fine grained detail anymore but I think it's worth it for the sake of having nice looking string representation.

### 26/3
Today I decided to scratch features with a lower priority. Mainly prioritizing a high level language and a web interface.

### 9/4
Over the last few days I have created a little web interface for the bots. It has a input field for the string representation of a bot and a "randomize" button. You can also choose to play over the board without a bot. Next thing to add to the interface is the ability to play two bots against each other, I think this would be a cool way to compare bots visually easily. I choose [Leptos](https://leptos.dev/) as the web framework for building the application even though I've had a pretty good time using web-components lately for my blog, there is 3 reasons for this decision.

* I wanted to learn a rust web framework.
* By using a rust web framework I could reuse a lot of the code I've already written, including using the structs, enums and interfaces that I've made (really nice compared to javascript where enums isn't really a thing).
* I listened to a talk made by the creator by Leptos and I quite liked his view on [why to use wasm for web development](https://youtu.be/V1cqQRmVAK0?t=276).

I also implemented a compiler yesterday, It's really unstable, but it works, creating a de-compiler next which i want showing the code on the website. (so that you can edit the values in the high-level code and it will edit the values in the low-level representation)

### 10/4
Today I added the option to set a value for each pieces available moves. I also fixed a bug in the pruning algorithm.
