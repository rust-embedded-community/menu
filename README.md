# Menu

## Introduction

A simple command-line menu system in Rust. Works on embedded systems, but also
on your command-line.

**NOTE:** This crates works only in `&str` - there's no heap allocation, but
there's also no automatic conversion to integers, boolean, etc.

```console
user@host: ~/menu $ cargo run --example simple
   Compiling menu v0.5.0 (file:///home/user/menu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84 secs
     Running `target/debug/examples/simple`
In enter_root()
> help
AVAILABLE ITEMS:
  foo <a> [ <b> ] [ OPTIONS... ]
  bar
  sub
  help [ <command> ]


> help foo
SUMMARY:
  foo <a> [ <b> ] [ --verbose ] [ --level=INT ]

PARAMETERS:
  <a>
       This is the help text for 'a'
  <b>
       No help text found
  --verbose
       No help text found
  --level=INT
       Set the level of the dangle


DESCRIPTION:
Makes a foo appear.

This is some extensive help text.

It contains multiple paragraphs and should be preceeded by the parameter list.

> foo --level=3 --verbose 1 2
In select_foo. Args = ["--level=3", "--verbose", "1", "2"]
a = Ok(Some("1"))
b = Ok(Some("2"))
verbose = Ok(Some(""))
level = Ok(Some("3"))
no_such_arg = Err(())


> foo
Error: Insufficient arguments given!

> foo 1 2 3 3
Error: Too many arguments given

> sub

/sub> help
AVAILABLE ITEMS:
  baz
  quux
  exit
  help [ <command> ]

> exit

> help
AVAILABLE ITEMS:
  foo <a> [ <b> ] [ OPTIONS... ]
  bar
  sub
  help [ <command> ]


> ^C
user@host: ~/menu $
```

## Using

See `examples/simple.rs` for a working example that runs on Linux or Windows. Here's the menu definition from that example:

```rust
const ROOT_MENU: Menu<Output> = Menu {
    label: "root",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: select_foo,
                parameters: &[
                    Parameter::Mandatory {
                        parameter_name: "a",
                        help: Some("This is the help text for 'a'"),
                    },
                    Parameter::Optional {
                        parameter_name: "b",
                        help: None,
                    },
                    Parameter::Named {
                        parameter_name: "verbose",
                        help: None,
                    },
                    Parameter::NamedValue {
                        parameter_name: "level",
                        argument_name: "INT",
                        help: Some("Set the level of the dangle"),
                    },
                ],
            },
            command: "foo",
            help: Some(
                "Makes a foo appear.

This is some extensive help text.

It contains multiple paragraphs and should be preceeded by the parameter list.
",
            ),
        },
        &Item {
            item_type: ItemType::Callback {
                function: select_bar,
                parameters: &[],
            },
            command: "bar",
            help: Some("fandoggles a bar"),
        },
        &Item {
            item_type: ItemType::Menu(&Menu {
                label: "sub",
                items: &[
                    &Item {
                        item_type: ItemType::Callback {
                            function: select_baz,
                            parameters: &[],
                        },
                        command: "baz",
                        help: Some("thingamobob a baz"),
                    },
                    &Item {
                        item_type: ItemType::Callback {
                            function: select_quux,
                            parameters: &[],
                        },
                        command: "quux",
                        help: Some("maximum quux"),
                    },
                ],
                entry: Some(enter_sub),
                exit: Some(exit_sub),
            }),
            command: "sub",
            help: Some("enter sub-menu"),
        },
    ],
    entry: Some(enter_root),
    exit: Some(exit_root),
};

```

## Changelog

See [`CHANGELOG.md`](./CHANGELOG.md).

## License

The contents of this repository are dual-licensed under the _MIT OR Apache 2.0_
License. That means you can choose either the MIT license or the Apache 2.0
license when you re-use this code. See [`LICENSE-MIT`](./LICENSE-MIT) or
[`LICENSE-APACHE`](./LICENSE-APACHE) for more information on each specific
license. Our Apache 2.0 notices can be found in [`NOTICE`](./NOTICE).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
