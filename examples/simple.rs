extern crate menu;

use menu::*;
use pancurses::{endwin, initscr, noecho, Input};

const FOO_ITEM: Item<Output> = Item {
    item_type: ItemType::Callback {
        function: select_foo,
        parameters: &[
            Parameter::Mandatory("a"),
            Parameter::Optional("b"),
            Parameter::Named {
                parameter_name: "verbose",
                argument_name: "VALUE",
            },
        ],
    },
    command: "foo",
    help: Some("makes a foo appear"),
};

const BAR_ITEM: Item<Output> = Item {
    item_type: ItemType::Callback {
        function: select_bar,
        parameters: &[],
    },
    command: "bar",
    help: Some("fandoggles a bar"),
};

const ENTER_ITEM: Item<Output> = Item {
    item_type: ItemType::Menu(&SUB_MENU),
    command: "sub",
    help: Some("enter sub-menu"),
};

const ROOT_MENU: Menu<Output> = Menu {
    label: "root",
    items: &[&FOO_ITEM, &BAR_ITEM, &ENTER_ITEM],
    entry: Some(enter_root),
    exit: Some(exit_root),
};

const BAZ_ITEM: Item<Output> = Item {
    item_type: ItemType::Callback {
        function: select_baz,
        parameters: &[],
    },
    command: "baz",
    help: Some("thingamobob a baz"),
};

const QUUX_ITEM: Item<Output> = Item {
    item_type: ItemType::Callback {
        function: select_quux,
        parameters: &[],
    },
    command: "quux",
    help: Some("maximum quux"),
};

const SUB_MENU: Menu<Output> = Menu {
    label: "sub",
    items: &[&BAZ_ITEM, &QUUX_ITEM],
    entry: Some(enter_sub),
    exit: Some(exit_sub),
};

struct Output(pancurses::Window);

impl std::fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.0.printw(s);
        Ok(())
    }
}

fn main() {
    let window = initscr();
    noecho();
    let mut buffer = [0u8; 64];
    let mut o = Output(window);
    let mut r = Runner::new(&ROOT_MENU, &mut buffer, &mut o);
    loop {
        match r.context.0.getch() {
            Some(Input::Character('\n')) => {
                r.input_byte(b'\r');
            }
            Some(Input::Character(c)) => {
                let mut buf = [0; 4];
                for b in c.encode_utf8(&mut buf).bytes() {
                    r.input_byte(b);
                }
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
                r.context.0.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }
    endwin();
}

fn enter_root(_menu: &Menu<Output>) {
    println!("In enter_root");
}

fn exit_root(_menu: &Menu<Output>) {
    println!("In exit_root");
}

fn select_foo<'a>(_menu: &Menu<Output>, _item: &Item<Output>, input: &str, _context: &mut Output) {
    println!("In select_foo: {}", input);
}

fn select_bar<'a>(_menu: &Menu<Output>, _item: &Item<Output>, input: &str, _context: &mut Output) {
    println!("In select_bar: {}", input);
}

fn enter_sub(_menu: &Menu<Output>) {
    println!("In enter_sub");
}

fn exit_sub(_menu: &Menu<Output>) {
    println!("In exit_sub");
}

fn select_baz<'a>(_menu: &Menu<Output>, _item: &Item<Output>, input: &str, _context: &mut Output) {
    println!("In select_baz: {}", input);
}

fn select_quux<'a>(_menu: &Menu<Output>, _item: &Item<Output>, input: &str, _context: &mut Output) {
    println!("In select_quux: {}", input);
}
