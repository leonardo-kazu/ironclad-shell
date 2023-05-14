# Ironclad-shell

A "shell" ~~command interpreter~~ made in rust.

## First things first

This is my first time posting to reddit and my first time using rust (also worth noting that english is not my main language), so please any constructive criticism is well seen.

## A little background

I'm a graduate student in computer science and for my OS (Operating Systems) classes my teacher told us to create a "shell" in C/C++, but as I was fascinated by Rust as asked him if i could do it in Rust and for my surprise he said yes. Now, one would think that doing an assignment in a programming language that i have no experience would be a bad call.

> It's!

But I decided to go for it!

It doesn't implement all the required operations asked by my teacher but, I thought it would be nice to share it and receive some tips since no teacher or person I know knows Rust.

## But what does it do?

It's really simple it has and can do the following things:

* It has it's own PS1 variable you can set;
* It has it's own PATH called MYPATH, it copies the original system path and you can later change it;
* It can pipe (`|`) commands ~~this was incredibly hard to do~~;
* It can redirect stdout (`>`) and stderr(`2>`);
* It can read an stdin file (`<`);
* It can run path commands and same directory programs;
* It sort of can run programs in the background (`&`) but offers no control;
* It captures Ctrl+C and it's associated signal, so you can't exit the program with it;
* It can only be exited by the `exit` command and by the use of Ctrl+D;
* It has the `set` command to see all env variables;
* It has the `export`command to set new env variables;
* It stores the history of the commands in the current dir in a `.history` file, until it reaches 50 lines. ~~I just realized that if you changed dir it starts writing the .history file again~~
* It unfortunately does not have the capability do pipe shell commands with path commands;

I think that it's all, at least that I can remember.

## Important points

Now there are two things that I think that are important for me to say before anyone sees the code.

1. I tried to write the code using OOP, since the codes that my teacher showed us were all in C++ and we're written using OOP.
2. I believe that I commited alot of mistakes and/or "bad usage of" when writing the code in regards of types and if statements.

## "So what, do you have any plans for it now?"

Yes! Firstly I want to fix some of the points that do not follow my teacher points, like correct implementation of background processes, usage of the `fg`, `bg` and `kill` (not the /bin/kill one) commands that weren't implemented.

Implement more POSIX commands, and buffer control.

Add some colors to the PS1 and change it from the one ordered by my teacher.

Add the possibility to pipe shell commands and processes.

Fix any issues pointed by the people that see the code.

## Final regards

This my first time having any experience with Rust, and one thing that I learned is, I really liked this language, I work with Typescript daily and have experience with Flutter, Java and C/C++. I always wanted to learn Rust and now I want to learn it even more, I don't know if I should work on other projects, start with stuff that is easier instead of starting with a "shell" implementation. I hope that in the future I end up working with this language.

_This is the same .md posted to reddit as i'm doing this at 11:30 PM in my local time, hope you don't mind😉_
