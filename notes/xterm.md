#Xterm for Rust

A library for using Xterm from Rust.

##Philosophy

Terminal abstractions are all very well, but sometimes you simply want xterm-native behavior. Ultimately we may expect a Rustacean curses type library; the xterm crate is intended, as the name suggests, as a wrapper around xterm behaviors.


##Types

We need to treat escape sequences as conceptually different from other strings. The reason being graphemes: escape sequences aren't Unicode and would be counted as graphemes. One of the reasons I'm writing this is that I like data presented in neat boxes, which requires knowing how long strings are. Simply concatenating escape sequences into a string will make this needlessly complicated.

What we need is a growable vector which can hold either a String or an XString, with a Show trait that concatenates it into a single string and prints it. This way we can decorate the string with colors, replace newlines with jumps, and stream the result to the terminal. 

Using traits will also allow, for example, a `"({:b}",XVec)` trait that strips all colors and replaces jumps with newlines, producing an acceptable ansi-free stream. Terminals should degrade gracefully when possible.

Corrolary: We want more string types than just XString and String. At minimum we need a Ret string type that represents a newline replaced with a jump, so that we can turn it back for e.g. writing to a log. 

##More Sophistication

Eventually we need a full i/o API, so we can capture things like mouse movements. I also want to implement some kind of Palette structure, so that colors can be dereferenced and late-bound to various actual values. Everything I write wants to grow into an editing environment...