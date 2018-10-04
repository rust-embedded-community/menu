#![no_std]

type MenuCallbackFn<T> = fn(menu: &Menu<T>, context: &mut T);
type ItemCallbackFn<T> = fn(menu: &Menu<T>, item: &Item<T>, args: &str, context: &mut T);

pub enum ItemType<'a, T>
where
    T: 'a,
{
    Callback(ItemCallbackFn<T>),
    Menu(&'a Menu<'a, T>),
}

/// Menu Item
pub struct Item<'a, T>
where
    T: 'a,
{
    pub command: &'a str,
    pub help: Option<&'a str>,
    pub item_type: ItemType<'a, T>,
}

/// A Menu is made of Items
pub struct Menu<'a, T>
where
    T: 'a,
{
    pub label: &'a str,
    pub items: &'a [&'a Item<'a, T>],
    pub entry: Option<MenuCallbackFn<T>>,
    pub exit: Option<MenuCallbackFn<T>>,
}

pub struct Runner<'a, T>
where
    T: core::fmt::Write,
    T: 'a,
{
    buffer: &'a mut [u8],
    used: usize,
    /// Maximum four levels deep
    menus: [Option<&'a Menu<'a, T>>; 4],
    depth: usize,
    pub context: &'a mut T,
}

enum Outcome {
    CommandProcessed,
    NeedMore,
}

impl<'a, T> Runner<'a, T>
where
    T: core::fmt::Write,
{
    pub fn new(menu: &'a Menu<'a, T>, buffer: &'a mut [u8], context: &'a mut T) -> Runner<'a, T> {
        if let Some(cb_fn) = menu.entry {
            cb_fn(menu, context);
        }
        let mut r = Runner {
            menus: [Some(menu), None, None, None],
            depth: 0,
            buffer,
            used: 0,
            context,
        };
        r.prompt(true);
        r
    }

    pub fn prompt(&mut self, newline: bool) {
        if newline {
            write!(self.context, "\n").unwrap();
        }
        if self.depth != 0 {
            let mut depth = 1;
            while depth <= self.depth {
                if depth > 1 {
                    write!(self.context, "/").unwrap();
                }
                write!(self.context, "/{}", self.menus[depth].unwrap().label).unwrap();
                depth += 1;
            }
        }
        write!(self.context, "> ").unwrap();
    }

    pub fn input_byte(&mut self, input: u8) {
        // Strip carriage returns
        if input == 0x0A {
            return;
        }
        let outcome = if input == 0x0D {
            write!(self.context, "\n").unwrap();
            if let Ok(s) = core::str::from_utf8(&self.buffer[0..self.used]) {
                if s == "help" {
                    let menu = self.menus[self.depth].unwrap();
                    let mut max_len = 1;
                    for item in menu.items {
                        max_len = max_len.max(item.command.len());
                    }
                    for item in menu.items {
                        if let Some(help) = item.help {
                            writeln!(self.context, "* {:width$}  - {}", item.command, help, width=max_len).unwrap();
                        } else {
                            writeln!(self.context, "* {}", item.command).unwrap();
                        }
                    }
                    if self.depth != 0 {
                        writeln!(self.context, "* {:width$}  - leave this menu.", "exit", width=max_len).unwrap();
                    }
                    writeln!(self.context, "* {:width$}  - print this help text.", "help", width=max_len).unwrap();
                    Outcome::CommandProcessed
                } else if s == "exit" && self.depth > 0 {
                    // Can't be None if we've managed to get to this level
                    let menu = self.menus[self.depth].unwrap();
                    self.menus[self.depth] = None;
                    if let Some(cb_fn) = menu.exit {
                        cb_fn(menu, &mut self.context);
                    }
                    self.depth -= 1;
                    Outcome::CommandProcessed
                } else {
                    let mut parts = s.split(' ');
                    if let Some(cmd) = parts.next() {
                        let mut found = false;
                        let menu = self.menus[self.depth].unwrap();
                        for item in menu.items {
                            if cmd == item.command {
                                match item.item_type {
                                    ItemType::Callback(f) => f(menu, item, s, &mut self.context),
                                    ItemType::Menu(m) => {
                                        if (self.depth + 1) >= self.menus.len() {
                                            writeln!(self.context, "Can't enter menu - structure too deep.").unwrap();
                                        } else {
                                            // self.menus[0] is the root menu, so go up one level first.
                                            self.depth += 1;
                                            self.menus[self.depth] = Some(m);
                                            if let Some(cb_fn) = m.entry {
                                                cb_fn(m, &mut self.context);
                                            }
                                        }
                                    }
                                }
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            writeln!(self.context, "Command {:?} not found. Try 'help'.", cmd)
                                .unwrap();
                        }
                        Outcome::CommandProcessed
                    } else {
                        writeln!(self.context, "Input empty").unwrap();
                        Outcome::CommandProcessed
                    }
                }
            } else {
                writeln!(self.context, "Input not valid UTF8").unwrap();
                Outcome::CommandProcessed
            }
        } else if input == 0x08 {
            // Handling backspace
            if self.used > 0 {
                write!(self.context, "\u{0008} \u{0008}").unwrap();
                self.used -= 1;
            }
            Outcome::NeedMore
        } else if self.used < self.buffer.len() {
            self.buffer[self.used] = input;
            self.used += 1;

            let valid = if let Ok(_) = core::str::from_utf8(&self.buffer[0..self.used]) {
                true
            } else {
                false
            };
            if valid {
                write!(self.context, "\r").unwrap();
                self.prompt(false);
            }
            if let Ok(s) = core::str::from_utf8(&self.buffer[0..self.used]) {
                write!(self.context, "{}", s).unwrap();
            }
            Outcome::NeedMore
        } else {
            writeln!(self.context, "Buffer overflow!").unwrap();
            Outcome::NeedMore
        };
        match outcome {
            Outcome::CommandProcessed => {
                self.used = 0;
                self.prompt(true);
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
