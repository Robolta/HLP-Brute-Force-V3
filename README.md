# HLP Brute Force V3

A program designed to solve the Hex Layer Problem.

## Current Progress

- [x] [Layer Struct](https://github.com/Robolta/HLP-Brute-Force-V3/blob/main/src/search/layers.rs#L36)
- [x] Layer Method to generate the [Unique Layers](https://github.com/Robolta/HLP-Brute-Force-V3/blob/a2e1daa8f66d5061bcd67b76f8f92b7f5a894bee/src/search/layers.rs#L87)
- [x] Layer Method to populate the [Layer Children](https://github.com/Robolta/HLP-Brute-Force-V3/blob/a2e1daa8f66d5061bcd67b76f8f92b7f5a894bee/src/search/layers.rs#L119C5-L119C5)
- [ ] Resursive Search Function for [Short Skipping](#short-skipping)

## Why are there 3 Versions?

Version 1

Pros:
- Does its job

Cons:
- Horribly designed, hard to modify

Version 2

Pros:
- Well designed, easy to modify

Cons:
- Doesn't work properly

I could go back and fix these issues in the previous versions, but I've attempted that and don't care to continue.

## The Hex Layer Problem
*What is the Hex Layer Problem anyway?*

### Short Explanation

The short explanation is that the problem asks the following:

Given a target sequence, find the optimal (shortest) function of layers which reaches it.  
"Reaching" a target sequence just means treating the sequence as a function (index = input) would have the same funcitonality as the layer-composed function.

### Long Explanation

First, what is a layer?

A layer is a redstone circuit within the game Minecraft.  
It was first designed by Aminotreal and Powsi.
It has 1024 possible static states which affect the main input-output relationship.

![image](https://media.discordapp.net/attachments/721120731974598726/1017443702332993616/unknown.png)

Each of the barrels (brown) can produce a signal from 0 to 15.  
Each of the right two comparators (white) can be in either a "compare" or "subtract" state.

16 * 16 * 2 * 2 = 1024 states

We can reach more than 1024 possible functions by simply chaining together the inputs and outputs of multiple layers.

![image](https://user-images.githubusercontent.com/58904726/209972849-b7c6413a-4b86-40d7-a08e-642f22cd952b.png)

It has been proven that a subset of all layers can reach any arbitrary function (with 1 input and 1 output, both 0-15).  
This means that using the entire set of layers can certainly do the same.

So how does a layer work?  
Let's start with how a comparator works.  
Within my program, comparators are represented by the following function.

```rust
// Implementation of a Minecraft Redstone Comparator
fn comparator(back: u64, side: u64, mode: bool) -> u64 {
    if back < side {
        return 0;
    } else if mode {
        return back - side;
    }
    back
}
```

Compare mode means the back outputs unless the side is bigger, in which case the output is 0.  
Subtract mode means the side is subtracted from the back.  
The output can never be less than 0, so subtract mode is at least 0. (Negative subtractions would give 0)

After the two comparators produce their outputs, the larger of the two reaches the final output.

So, again, the problem boils down to finding a configuration of layers which uses as few as possible to reach a target function.  
[Sorting by Prefix Reversals is NP-Hard](https://arxiv.org/abs/1111.0434v1) and they are the only layers we can use to sort while maintaining the full 16 output values.  
This means that our problem only proves to be doable optimally with brute force.

## Optimizations

WIP

### Short Skipping