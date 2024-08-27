extern crate menu;

use embedded_io::{ErrorType, Read as EmbRead, Write as EmbWrite};
use menu::*;
use noline::builder::EditorBuilder;
use std::io::{self, Read as _, Stdin, Stdout, Write as _};
use termion::raw::IntoRawMode;

pub struct IOWrapper {
    stdin: Stdin,
    stdout: Stdout,
}

impl IOWrapper {
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }
}

impl Default for IOWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorType for IOWrapper {
    type Error = embedded_io::ErrorKind;
}

impl EmbRead for IOWrapper {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(self.stdin.read(buf).map_err(|e| e.kind())?)
    }
}

impl EmbWrite for IOWrapper {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut written = 0;
        let parts = buf.split(|b| *b == b'\n').collect::<Vec<_>>();

        for (i, part) in parts.iter().enumerate() {
            written += self.stdout.write(part).map_err(|e| e.kind())?;

            if i != parts.len() - 1 {
                let _ = self.stdout.write(b"\r\n").map_err(|e| e.kind())?;
                written += 1;
            }
        }

        Ok(written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(self.stdout.flush().map_err(|e| e.kind())?)
    }
}

#[derive(Default)]
struct Context {
    _inner: u32,
}

const ROOT_MENU: Menu<IOWrapper, Context> = Menu {
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

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();

    let mut io = IOWrapper::new();
    let mut editor = EditorBuilder::new_unbounded()
        .with_unbounded_history()
        .build_sync(&mut io)
        .unwrap();

    let mut context = Context::default();
    let mut r = Runner::new(ROOT_MENU, &mut editor, io, &mut context);

    while let Ok(_) = r.input_line(&mut context) {}
}

fn enter_root(_menu: &Menu<IOWrapper, Context>, interface: &mut IOWrapper, _context: &mut Context) {
    writeln!(interface, "In enter_root").unwrap();
}

fn exit_root(_menu: &Menu<IOWrapper, Context>, interface: &mut IOWrapper, _context: &mut Context) {
    writeln!(interface, "In exit_root").unwrap();
}

fn select_foo(
    _menu: &Menu<IOWrapper, Context>,
    item: &Item<IOWrapper, Context>,
    args: &[&str],
    interface: &mut IOWrapper,
    _context: &mut Context,
) {
    writeln!(interface, "In select_foo. Args = {:?}", args).unwrap();
    writeln!(
        interface,
        "a = {:?}",
        ::menu::argument_finder(item, args, "a")
    )
    .unwrap();
    writeln!(
        interface,
        "b = {:?}",
        ::menu::argument_finder(item, args, "b")
    )
    .unwrap();
    writeln!(
        interface,
        "verbose = {:?}",
        ::menu::argument_finder(item, args, "verbose")
    )
    .unwrap();
    writeln!(
        interface,
        "level = {:?}",
        ::menu::argument_finder(item, args, "level")
    )
    .unwrap();
    writeln!(
        interface,
        "no_such_arg = {:?}",
        ::menu::argument_finder(item, args, "no_such_arg")
    )
    .unwrap();
}

fn select_bar(
    _menu: &Menu<IOWrapper, Context>,
    _item: &Item<IOWrapper, Context>,
    args: &[&str],
    interface: &mut IOWrapper,
    _context: &mut Context,
) {
    writeln!(interface, "In select_bar. Args = {:?}", args).unwrap();
}

fn enter_sub(_menu: &Menu<IOWrapper, Context>, interface: &mut IOWrapper, _context: &mut Context) {
    writeln!(interface, "In enter_sub").unwrap();
}

fn exit_sub(_menu: &Menu<IOWrapper, Context>, interface: &mut IOWrapper, _context: &mut Context) {
    writeln!(interface, "In exit_sub").unwrap();
}

fn select_baz(
    _menu: &Menu<IOWrapper, Context>,
    _item: &Item<IOWrapper, Context>,
    args: &[&str],
    interface: &mut IOWrapper,
    _context: &mut Context,
) {
    writeln!(interface, "In select_baz: Args = {:?}", args).unwrap();
}

fn select_quux(
    _menu: &Menu<IOWrapper, Context>,
    _item: &Item<IOWrapper, Context>,
    args: &[&str],
    interface: &mut IOWrapper,
    _context: &mut Context,
) {
    writeln!(interface, "In select_quux: Args = {:?}", args).unwrap();
}
