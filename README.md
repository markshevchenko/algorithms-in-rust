# Algorithms in the Rust programming language

Few weeks ago I participated in a discussion about best algorithms' book. This discussion appeared in one of programming chats in the Telegram.

We decided that Knuth's book is very respectful, but it uses very strange programming language for this times. The MIX, fake assembler language.

One of problems of assembler is low level of the code. You need to concentrate on registres and memory cells instead of details of algorithms. The second problem is impossibility of simple check of your implementation. MIX machine is imagine machine, so you need the MIX machine emulator. Of course they exist, but you need to find it and install it.

Python, C#, and JavaScript are simplier to implementation and checking. But may be they are very simple. You shouldn't think about efficienty because your languages aren't about efficienty.

Rust is the relatively new program language that exists for real machines, and has enough high level to implement algorithms withoud knowing of registers. At the same time, it has enough low level to make efficient programs. You need to think about the memory the ownership.

So may be Rust is the best language to demonstate algorithms on this time. And I decided to implement basic algorithms in the Rust.