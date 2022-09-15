# Introduction

Few weeks ago I participated in a discussion about best algorithms' book. This discussion appeared in one of programming chats in the Telegram.

We agreed that Knuth's book is very respectful, but it uses very weird programming language for this time. The MIX, fake assembler language.

First of all, assembler language has too low level of the code. You need to concentrate on registres and memory cells instead of details of algorithms. Then, you can't just check your implementation. MIX machine is imaginary machine, so you need the MIX emulator. Of course they exist, but you need to find them and install them.

Python, C#, and JavaScript are simpler in implementation and checking. But maybe they are very simple. You shouldn't think about efficiency because your languages aren't about efficiency.

Rust is the relatively new program language that exists for real machines, and has enough high level to implement algorithms without knowing registers. At the same time, it has enough low level to make efficient programs. So you need to think about the memory control.

So, may be Rust is the best language to demonstate algorithms this time. I've decided to make some algorithms in Rust.