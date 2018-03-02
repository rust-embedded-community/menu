#![no_std]

type MenuCallbackFn = fn(menu: &Menu);
type ItemCallbackFn = fn(menu: &Menu, item: &Item, args: &str);

pub enum ItemType<'a> {
    Callback(ItemCallbackFn),
    Menu(&'a Menu<'a>),
}

/// Menu Item
pub struct Item<'a> {
    pub command: &'a str,
    pub help: Option<&'a str>,
    pub item_type: ItemType<'a>,
}

/// A Menu is made of Items
pub struct Menu<'a> {
    pub label: &'a str,
    pub items: &'a [&'a Item<'a>],
    pub entry: Option<MenuCallbackFn>,
    pub exit: Option<MenuCallbackFn>,
}

pub struct Runner<'a, T>
where
    T: core::fmt::Write,
    T: 'a,
{
    buffer: &'a mut [u8],
    used: usize,
    /// Maximum four levels deep
    menus: [Option<&'a Menu<'a>>; 4],
    depth: usize,
    pub output: &'a mut T,
}

enum Outcome {
    CommandProcessed,
    NeedMore,
}

impl<'a, T> Runner<'a, T>
where
    T: core::fmt::Write,
{
    pub fn new(menu: &'a Menu<'a>, buffer: &'a mut [u8], output: &'a mut T) -> Runner<'a, T> {
        if let Some(cb_fn) = menu.entry {
            cb_fn(menu);
        }
        let mut r = Runner {
            menus: [Some(menu), None, None, None],
            depth: 0,
            buffer,
            used: 0,
            output,
        };
        r.prompt();
        r
    }

    pub fn prompt(&mut self) {
        write!(self.output, "\n").unwrap();
        if self.depth != 0 {
            let mut depth = 1;
            while depth <= self.depth {
                if depth > 1 {
                    write!(self.output, "/").unwrap();
                }
                write!(self.output, "/{}", self.menus[depth].unwrap().label).unwrap();
                depth += 1;
            }
        }
        write!(self.output, "> ").unwrap();
    }

    pub fn input_byte(&mut self, input: u8) {
        // Strip carriage returns
        if input == 0x0A {
            return;
        }
        let outcome = if input == 0x0D {
            write!(self.output, "\n").unwrap();
            if let Ok(s) = core::str::from_utf8(&self.buffer[0..self.used]) {
                if s == "help" {
                    let menu = self.menus[self.depth].unwrap();
                    for item in menu.items {
                        if let Some(help) = item.help {
                            writeln!(self.output, "{} - {}", item.command, help).unwrap();
                        } else {
                            writeln!(self.output, "{}", item.command).unwrap();
                        }
                    }
                    if self.depth != 0 {
                        writeln!(self.output, "exit - leave this menu.").unwrap();
                    }
                    writeln!(self.output, "help - print this help text.").unwrap();
                    Outcome::CommandProcessed
                } else if s == "exit" && self.depth != 0 {
                    if self.depth == self.menus.len() {
                        writeln!(self.output, "Can't enter menu - structure too deep.").unwrap();
                    } else {
                        self.menus[self.depth] = None;
                        self.depth -= 1;
                    }
                    Outcome::CommandProcessed
                } else {
                    let mut parts = s.split(' ');
                    if let Some(cmd) = parts.next() {
                        let mut found = false;
                        let menu = self.menus[self.depth].unwrap();
                        for item in menu.items {
                            if cmd == item.command {
                                match item.item_type {
                                    ItemType::Callback(f) => f(menu, item, s),
                                    ItemType::Menu(m) => {
                                        self.depth += 1;
                                        self.menus[self.depth] = Some(m);
                                    }
                                }
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            writeln!(self.output, "Command {:?} not found. Try 'help'.", cmd)
                                .unwrap();
                        }
                        Outcome::CommandProcessed
                    } else {
                        writeln!(self.output, "Input empty").unwrap();
                        Outcome::CommandProcessed
                    }
                }
            } else {
                writeln!(self.output, "Input not valid UTF8").unwrap();
                Outcome::CommandProcessed
            }
        } else if self.used < self.buffer.len() {
            self.buffer[self.used] = input;
            self.used += 1;
            Outcome::NeedMore
        } else {
            writeln!(self.output, "Buffer overflow!").unwrap();
            Outcome::NeedMore
        };
        match outcome {
            Outcome::CommandProcessed => {
                self.used = 0;
                self.prompt();
            }
            Outcome::NeedMore => {}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
