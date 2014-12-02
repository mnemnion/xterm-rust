#Towards a Modern Terminal

The terminal is the most important application to the professional computer user.

As is typical of our micro-era, the terminal we use is not the terminal we deserve. Nor can we simply get there from here, by extending the terminals and protocols we have. 

##What makes a terminal

A terminal is a program for professional computer users. Developers, system administrators, and the like. The current terminal is a time machine, allowing us to reach the lower substrate of the uniformly pre-modern operating systems we use.

To my taste, the defining features of a terminal, in rough order: It is textually oriented, monospaced, dumb, and pretends to be the sole interface between user and computer. 

###Textual orientation

A terminal must be oriented around text. Here we encounter our first mismatch between modernity and our tools. UTF-8 has emerged as standard, and is bewilderingly complex, with aspects that thoroughly break the terminal paradigm. Our modern terminal is not an ASCII shoehorn, it is and must be Unicode native. The beauty of UTF-8 is that it is a superset of 7-bit ASCII, so this poses no problem for interfacing with primitive or tiny systems. 

There is a deeper meaning to textual orientation: data flowing through the terminal should be serialized as readable, even if that's not the form in which interaction is taking place. 

###Monospaced

A terminal must have a definite number of rows and columns. Everything of significance must be aligned on this grid. 

This is solvable for Unicode, though much scut work must be done. We simply declare by fiat when graphemes take a single space and when they take two. Those being ones only options. Some languages end up pretty ugly. Those users are used it it, I assure you. 

Can we render proportional text? Images? Video? I don't see why not. Anything that isn't bog text is a) structured by a linear textual sequence, b) rendered by a subsidiary program, c) aligned absolutely on the grid. 

The important part is that the terminal be dumb. But not as dumb as it is right now.

###Dumb

I strongly believe that the terminal renderer should be dumb. Specifically, not Turing complete. Smart enough to have some stacks, not smart enough to have a tree, even an acyclic one. No scripting language whatsoever. I will not budge on this: write a shell. 

The way xterm works is basically correct. It receives, and sometimes sends, streams of ordinary ASCII text. Some of these are escape sequences; they begin with the Esc byte and are parsed to perform such functions as jumping the cursor, paging, deleting text, changing color values, and so on. In general, one can degrade gracefully to pure text by converting jumps to newlines and stripping the rest. These are also used to report mouse values, terminal characteristics, and so on. 

Our magic terminal will have a somewhat richer vocabulary of escape sequences, and provide more awareness of its own state. There is one feature in particular that has no exact analogy: an escape sequence which, when issued, will cause the terminal to reply with a sequence that is expected to be handled by the program. This will cause some of the render stream to be diverted to another program, which is given responsibility over a rectangular area of definite row and column dimensions. This could, for example, render an image. 

The terminal manages this connection once established, so that stdin is pointed at the right terminal socket for the handler depending on cursor and mouse position. It is capable of reporting both the cell clicked and the pixel within the cell, in the case of a mouse event. The handler program sees a terminal of the borrowed row/column dimensions, is aware of the number of pixels involved, and can communicate accordingly. The terminal has appropriate hardware acceleration and bitmap-awareness, it isn't necessary to ASCII serialize pixel data.  

These handlers are ordinary programs that adhere to a standard, which is why they're launched by the user program, through a shell. It would be normal to hand a view of dimension zero to a handler which can process audio: they may be addressed by distinctive sequence, in a way that works the way you'd expect sockets to work. 

The terminal almost has two layers, in that it's just smart enough that if it's told to print something into a handled region by the master stream, it will fail to render, advancing the cursor appropriately, but without surrendering the cursor to the handler. This provides the correct behavior in almost all cases. There is no way to overlap handlers. This will occasionally prove annoying, but will prevent much pain. The same handler can be responsible for two or more regions through distinct sockets, so now you know how to fake it. 

###Sole Interface

A terminal should pretend to take up a full screen. It's smart enough to know when it has changed shape, and to say something when told to. xterm provides the ability to tell an appropriately sized terminal to change shape or minimize. I feel we can route those things around the terminal entirely. The terminal itself shouldn't need to call out to the GUI.

Related: a terminal should capture absolutely every signal within its boundaries except the OS level keystroke for changing programs (⌘-Tab on Macs). The Mac Terminal program is particularly bad for this, striking a balance with the operating system at the expense of a function that shouldn't be surrendered. 

Here we hit a depressing fact: Our keyboards aren't as orthogonal as they look. One would think hitting alt would send an alt, hitting ctrl will send a control: no. It would simply depress me to go into the details, but they aren't that terrible, if we have a strong guarantee that we will receive everything that flows from the fingers of our typist, correctly. A terminal must have dibs. This is particularly not true on Macs, for various reasons. 

It's not a dealbreaker. Contemporary GUIs let you hook the keys, though I'm told one must be somewhat aggressive about it. It's a priority: with terminals, we type. 

##Rationale

The terminal is a programmer convenience. Our need for a line-based, ASCII-focused, interactive command line will never disappear. Even when working on a full-screen basis, the ability to think and work on a grid is a lifesaver. 

This is why part of such a standard must be an absolute agreement on what each and every Unicode point means, in terms of cursor movement. A row and column must be partially interchangeable, but not entirely: due to vertical text, a four-hanzi Chinese phrase will take up either eight columns and one row, or four rows and two columns, and this cannot be avoided. The critical thing is that a terminal is expected to render every Unicode point sensibly, and use the same amount of cell space in every instance. Either that, or it isn't rendering text in those cells. These are the only options. 

The important parts of the xterm protocol should work, out of the box. It's no terminal if it can't run bash, tmux etc. The only xterm mode it needs to support is xterm proper, the vt100 part; xterm, as befits its era, is capable of pretending to be all sorts of weird things that our terminal needn't emulate. They're easy enough to shim on an OS level if you find yourself VAXing or something. 

The intention with handlers is to provide a reasonably OS-free way of twiddling the pixels on the screen. It's well to pretend that everything is cellular, but good to provide a byte-level stream that can make comfortable assumptions about color and pixel availability. We shouldn't have to resort to OS resolution for a simple handler to stream bitmap data to the terminal. It should be possible to hardware accelerate a complex 3-d system, if you want to play Minecraft in your terminal I'm not here to crush your dreams. But there's definitely an intermediate level where a handler can get at the pixels and stream them in unserialized form: we want this to be reasonably fine-grained. This is a place where careful aesthetics must rule: this could turn into a minefield of unspecified behavior without good taste. 

I'm thinking a region can have three modes: mixed, bitmap, and opaque. An opaque region might not even have a pixel to its name, handling something like audio, or it may be displaying video or any old thing. Clearly the programmer is responsible for cross-OS or indeed cross-purpose compatibility, how could it be otherwise? An opaque region is able to receive and send terminal commands, but with no particular expectation as to what it will do. 

A bitmapped region is addressed on a per-pixel basis and the terminal simply renders it. A mixed region combines rendering instructions to the terminal with cell-filling bitmaps that are stremed on a per-pixel basis: any given cell must either be an instruction (draw an `a`) or a bitmap, with the usual 'failed to render' symbol filling a region that is incorrectly specified. I don't want to decide what to do with three pixels in between two characters, and neither do you. 

Could you render a full browser in the terminal? Obviously, yes. Better yet, it's feasible to write a usable browser that is terminal oriented and still runs Javascript. It's always going to look kind of weird, and degrade to a canvas-like state when people do tricky things to render text. Can't be helped. 

The browser will always fit somewhat awkwardly within the terminal. They're just not the same idea, there will always be a tradeoff between fidelity of render and providing anything but an opaque region view, which is basically just handing control over to Chrome or whomever. 

More to the point, it should be possible to write a little program that takes some comma separated numbers and graphs them to your terminal. This should be comfortable to do from a shell, though most likely not from bash. Tools like full-screen editors, spreadsheets and the like, should be easier to write, look better, and work more consistently. 

You still need a handler. A stream of pixels won't be meaningful, as provided straight to a terminal, because that's brittle across environments. We envision some kind of simple, consistent handler built into the shell, such that rendering an image into the given space and pixel density is resolved by the GUI in a cross-compatible way. This problem has been solved repeatedly, eloquent idioms are available. 

## Towards a Modern Shell

There's no sense in building a modern terminal without writing a shell that is aware of its capabilities. 

The goal in providing capabilities beyond the textual is mainly to provide tools for interacting with data that is either textual or may be serialized as such. The simplest use of an image handler is to blit a bitmap onto the terminal, but this is by no means the most interesting. 

The wishlist for a shell is necessarily greater, and my instincts on this subject are still developing. Unix heavily constrains the nature of a shell, as does programmer convention. As a result, what I'm calling a shell will make heavy use of an existing Unix shell. It might be more accurate to call it an editor. It's meant to blur those lines, as emacs does, but from the other direction.

The shell boots into a familiar line-based environment, although this is actually more like a buffer in that it has a name and persists by default. One immediate difference is that everything in the directory is executable. `cd foo` and `foo` are exactly equivalent, because a directory calls a handler, which by default changes the `pwd`. typing the name of an image, or clicking it, will print the image to the terminal, returning the prompt, which is quite minimal in many cases, as the shell offers a status line. 

The shell offers two languages: a command-line syntax, which isn't bash-compatible and has a different extension, and a proper language in which it's written. I favor Lua: it's small, LuaJIT is wicked fast, and (meta)tables can do everything. The challenge is that an interactive editing environment should have a complex condition system, with hooks and the like; I have a lot of respect for elisp, at least until I try to read it. Something based on Clojurian syntax would be ideal for me. I'm going to call the language Clua, so we don't get too many ideas about it. 
 
Regardless, the shell exposes its core functions as a C library (no doubt written in Rust), including the virtual machine for Clua. Scripts can be written in any language, .sh merely calls bash, .py calls python, etc. Most of these languages allow sugaring at the command-line so that the shell environment can be loaded implicitly; in some cases we'll need to add a shell comment at the top, but this can easily be native to the language since the shell has a decent parsing engine built in. 

The shell is not a small program, to the point where we'll need to make sure it loads fast and brings libraries in later. In particular it provides a complete text-editing environment, as well as a set of simple primitives for various bitmaps, vectors, audio streaming, at the like. `ed` is remapped to a handler that launches absolutely anything for editing, in an appropriately complex way. Sometimes `ed file` will have the same effect as `file`, sometimes it won't. The original `ed` we'll rename `tte` for The True Editor. Or something. YAGNI.

The editor has an emacs flavor and the ability to write a vim mode. How could it be otherwise? Indeed this may be the only way to break the duopoly: it's not an editor, it's an interactive shell running on a spiffy new terminal. Users start by using the `ed` function out of convenience, add functionality to scratch personal itches, and the system grows from there.

Note that `ed` will open anything and everything for editing. If it's something like a JPEG, the most primitive version of `ed` will load it into a hex editor. A more sophisticted version would parse the header and provide that information sensibly, pushing the rest through a hex function. Eventually you can load a quasi-GUI around the picture and apply filters, doodle on top, etc, and this can become as sophisticated as necessary. 

Load some JSON, and you get a strict mode editor that makes it impossible to type non-JSON information, using the paredit style to reorganize the forms of the data. This can, and should, be built using the shell's native parsing engine. 

The shell needs to come with a proper module and package system, and be properly namespaced. This alone will be a great boon; `curl http://.../blah.sh | sudo sh` will be replaced with `cluapackage run blah` and we can get a signature to go with our script.

This is not a lightweight shell, and doesn't belong on small remote machines. Ourshell has a special relationship with (ba)sh, and can provide some of its typical conveniences to a bash session running remotely via SSH or the like. If you want to examine some JSON, ourshell will tell bash to `cat` the file and will take care of providing that data to a local handler. 

Emacs derives great power from the ability to tune and customize every extension. The idea with Ourshell is to relocate many of the tools that reside within separate programs into the shellspace, where they're transparent and accessible. This is justified by the capabilites of the terminal: we'll naturally want versions of tools that break backwards compatibility. We like calling out to bash and do it often. 

Conversely, the various handlers and libraries of ourshell can be made available to the rest of the Unixverse. It should be reasonable to run bash, call a python script, which calls a handler, which does fancy things to the terminal, but from within bash, not ourshell. Ourshell is providing the handler as a service: this is why handler registration etc. is a terminal level function. Bash has no idea what's going on, stdin gets passed around at a lower level. 

The thing that makes this shell fancy isn't the terminal. Ourshell provides two new categories of shell service: handlers, and a sophisticated parser. These are intimately related: ourshell itself is a handler, specifically, the handler that handles `/` and everything underneath it which is not otherwise registered. Both the command syntax and the programming syntax are parsed by the ourshell parser, as is anything else. Ourshell can either parse a filename with a simple glob like `*.py` and launch Python, or it can in principle parse the Python file, given an appropriate set of combinators, and refuse to run an invalid script prior to launching Python itself. 

I've talked about the parser elsewhere. The notion is that it is binary focused, and capable of being tuned, such that feeding the Python parser the entire standard library will boil down to a set of weights on the combinators. I say "combinator", because that's what's offered from the programming perspective. If the parse file is a regular expression, you'll get a fast regex engine, not a slow parser-combinator regex engine. 

