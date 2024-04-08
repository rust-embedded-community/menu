extern crate menu;

use embedded_io::Write;

use menu::*;
use pancurses::{endwin, initscr, noecho, Input};

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

struct Output {
    window: pancurses::Window,
    input: Vec<u8>,
}

impl embedded_io::ErrorType for Output {
    type Error = core::convert::Infallible;
}

impl embedded_io::ReadReady for Output {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.input.is_empty())
    }
}

impl embedded_io::Read for Output {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let len = (&self.input[..]).read(buf).unwrap();
        self.input.drain(..len);
        Ok(len)
    }
}

impl embedded_io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let string = String::from_utf8(buf.to_vec()).unwrap();
        self.window.printw(string);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn main() {
    let window = initscr();
    window.scrollok(true);
    noecho();
    let mut buffer = [0u8; 64];
    let mut context = Output {
        window,
        input: Vec::new(),
    };
    let mut r = Runner::new(ROOT_MENU, &mut buffer, &mut context);
    loop {
        match context.window.getch() {
            Some(Input::Character('\n')) => {
                context.input.push(b'\r');
            }
            Some(Input::Character(c)) => {
                let mut buf = [0; 4];
                context
                    .input
                    .extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                r.process(&mut context);
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
                context
                    .input
                    .extend_from_slice(format!("{:?}", input).as_bytes());
            }
            None => (),
        }
        r.process(&mut context);
    }
    endwin();
}

fn enter_root(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In enter_root").unwrap();
}

fn exit_root(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In exit_root").unwrap();
}

fn select_foo(_menu: &Menu<Output>, item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_foo. Args = {:?}", args).unwrap();
    writeln!(
        context,
        "a = {:?}",
        ::menu::argument_finder(item, args, "a")
    )
    .unwrap();
    writeln!(
        context,
        "b = {:?}",
        ::menu::argument_finder(item, args, "b")
    )
    .unwrap();
    writeln!(
        context,
        "verbose = {:?}",
        ::menu::argument_finder(item, args, "verbose")
    )
    .unwrap();
    writeln!(
        context,
        "level = {:?}",
        ::menu::argument_finder(item, args, "level")
    )
    .unwrap();
    writeln!(
        context,
        "no_such_arg = {:?}",
        ::menu::argument_finder(item, args, "no_such_arg")
    )
    .unwrap();
}

fn select_bar(_menu: &Menu<Output>, _item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_bar. Args = {:?}", args).unwrap();
}

fn enter_sub(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In enter_sub").unwrap();
}

fn exit_sub(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In exit_sub").unwrap();
}

fn select_baz(_menu: &Menu<Output>, _item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_baz: Args = {:?}", args).unwrap();
}

fn select_quux(_menu: &Menu<Output>, _item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_quux: Args = {:?}", args).unwrap();
}
