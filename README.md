# Menu

A simple command-line menu system in Rust. Works on embedded systems, but also
on your command-line.

```
$ cargo run --example simple
   Compiling menu v0.1.0 (file:///home/jonathan/Documents/programming/menu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84 secs
     Running `target/debug/examples/simple`
In enter_root()
> help
foo - makes a foo appear
bar - fandoggles a bar
sub - enter sub-menu
help - print this help text.

> foo
In select_foo(): foo

> sub

sub> help
baz - thingamobob a baz
quux - maximum quux
exit - leave this menu.
help - print this help text.

> exit

> help
foo - makes a foo appear
bar - fandoggles a bar
sub - enter sub-menu
help - print this help text.

> ^C
$
```
