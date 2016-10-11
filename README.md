Currently this is a program that produces a wav file of white noise that lasts
for one second.

Some important notes:

1. `#[allow(unused_must_use)]` is intentional, I plan to learn about result types
   and error handling before removing it. This is also why none of the stdout
   writes are wrapped in `try!`

2. This will write garbage to your terminal and corrupt your session if you
   cargo run it. This is also intentional although not the eventually desired
   behavior. You can cargo build it and then redirect the binary's output to a
   wav file `./target/debug/rav > thingy.wav` which is then playable in a
   player if you're into white noise.  You can also change the duration in the
   main function.

So! some questions I have about idiomatic rust!  How would I, instead of
passing a handle to a `StdoutLock` into `write_header`, pass what I would think of
in C as a file pointer? Can the write header function accept either one? do
they share a common trait they can be typed against?  How can I idiomatically
parse [`'argv'`](http://rustbyexample.com/std_misc/arg.html) to make duration
variable based on the first argument? It comes in as a string, right? how can I
default it to 1 if there is no input?  are the type casts as u16 really
necessary? that feels icky to me but I don't see another way to do it in the
header writer.  I have found "solutions" of sorts to all these problems, but
I'm to new to rust to really have a good compass on what is well structured
rust code. So any wise eyes are appreciated!
