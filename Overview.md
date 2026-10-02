# A Rust JIT (Just‑In‑Time Compiler)

## Introduction

I have always wanted to learn how code works at the assembly level, and this project is where I finally get to explore
that. I’m going to try to make a compiler in Rust. My experience with Rust is limited, so memory management will be the
hardest part of the learning process. Still, I love a challenge.

The reason I chose Rust is because I want to learn it properly, and because so much modern software is either written in
Rust or being rewritten in it. Building a compiler will let me experience Rust’s strengths—especially its memory‑safety
guarantees—first‑hand.

## Language Design Goals

I’ve been thinking about the syntax and behaviour of my language for around a month (as of 2/10/26). My work experience
with Microsoft Power Apps, their type system, and their philosophy of making code accessible to everyone has influenced
how I want my language to feel and function.

I want the language to be as modular as possible. By that, I mean it should act as an intermediary language capable of
interacting with many different data types and APIs. This includes controlling data inside the program and communicating
with external systems.

One example is using JSON as the format for key–value pairs in the language. Another is supporting multiple kinds of
decimal numbers. As you may know, floats come in different sizes (32‑bit and 64‑bit), but there is also a *decimal*
type, which consists of three integers: the whole number, the decimal portion, and the number of digits after the
decimal point. I discovered this while looking at Dataverse types (Microsoft’s database for the Power Platform).

Floating‑point numbers are made from:

- a **sign bit** (positive/negative),
- an **exponent** (representing \(2^n\)),
- and the **mantissa**.

A simple way to understand this is that smaller numbers have the same amount of space for the mantissa, so the “jumps”
it represents are smaller, allowing for greater precision. Larger numbers have bigger jumps and therefore less
precision.

## Execution Model

Enough waffling about design—let’s talk about how I want it to run.

Since I want it to be a JIT compiler, the idea is to transpile my language (still haven’t named it yet) into a language
where I can compile it efficiently. My philosophy is that the language should be usable everywhere, so I want to borrow
Java’s idea of bytecode. I also chose this because I want to learn how bytecode works.

With bytecode, I can create instructions that are lower‑level but not as low‑level as raw assembly. From there, I can
compile the bytecode into assembly depending on the system, and use WebAssembly (WASM) for web execution.

To begin with, I want to target x86‑64 assembly because I currently use Windows, and I want to learn a Linux distro
next. For my first proper distro, I want something good for customisation and “ricing” a system.

## Closing Thoughts

This is just the planning stage of my language so far. But oh well—I’m 17, I’m aspirational, and I’m going to give it a
shot.
