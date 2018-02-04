extern crate menu;

use std::io::{self, Read, Write};
use menu::*;

const FOO_ITEM: Item = Item {
    item_type: ItemType::Callback(select_foo),
    command: "foo",
    help: Some("makes a foo appear"),
};

const BAR_ITEM: Item = Item {
    item_type: ItemType::Callback(select_bar),
    command: "bar",
    help: Some("fandoggles a bar"),
};

const ENTER_ITEM: Item = Item {
    item_type: ItemType::Menu(&SUB_MENU),
    command: "sub",
    help: Some("enter sub-menu"),
};

const ROOT_MENU: Menu = Menu {
    items: &[&FOO_ITEM, &BAR_ITEM, &ENTER_ITEM],
    entry: Some(enter_root),
    exit: Some(exit_root),
};

const BAZ_ITEM: Item = Item {
    item_type: ItemType::Callback(select_baz),
    command: "baz",
    help: Some("thingamobob a baz"),
};

const QUUX_ITEM: Item = Item {
    item_type: ItemType::Callback(select_quux),
    command: "quux",
    help: Some("maximum quux"),
};

const SUB_MENU: Menu = Menu {
    items: &[&BAZ_ITEM, &QUUX_ITEM],
    entry: Some(enter_sub),
    exit: Some(exit_sub),
};

struct Output;

impl std::fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        let mut stdout = io::stdout();
        write!(stdout, "{}", s).unwrap();
        stdout.flush().unwrap();
        Ok(())
    }
}

fn main() {
    let mut buffer = [0u8; 64];
    let mut o = Output;
    let mut r = Runner::new(&ROOT_MENU, &mut buffer, &mut o);
    loop {
        let mut ch = [0x00u8; 1];
        // Wait for char
        if let Ok(_) = io::stdin().read(&mut ch) {
            // Feed char to runner
            r.input_byte(ch[0]);
        }
    }
}

fn enter_root(_menu: &Menu) {
    println!("In enter_root");
}

fn exit_root(_menu: &Menu) {
    println!("In exit_root");
}

fn select_foo<'a>(_menu: &Menu, _item: &Item, input: &str) {
    println!("In select_foo: {}", input);
}

fn select_bar<'a>(_menu: &Menu, _item: &Item, input: &str) {
    println!("In select_bar: {}", input);
}

fn enter_sub(_menu: &Menu) {
    println!("In enter_sub");
}

fn exit_sub(_menu: &Menu) {
    println!("In exit_sub");
}

fn select_baz<'a>(_menu: &Menu, _item: &Item, input: &str) {
    println!("In select_baz: {}", input);
}

fn select_quux<'a>(_menu: &Menu, _item: &Item, input: &str) {
    println!("In select_quux: {}", input);
}
