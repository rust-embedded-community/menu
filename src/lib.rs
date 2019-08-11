#![no_std]

type MenuCallbackFn<T> = fn(menu: &Menu<T>);
type ItemCallbackFn<T> = fn(menu: &Menu<T>, item: &Item<T>, args: &str, context: &mut T);

/// Describes a parameter to the command
pub enum Parameter<'a> {
    Mandatory(&'a str),
    Optional(&'a str),
    Named {
        parameter_name: &'a str,
        argument_name: &'a str,
    },
}

/// Do we enter a sub-menu when this command is entered, or call a specific
/// function?
pub enum ItemType<'a, T>
where
    T: 'a,
{
    Callback {
        function: ItemCallbackFn<T>,
        parameters: &'a [Parameter<'a>],
    },
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
            cb_fn(menu);
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
            writeln!(self.context).unwrap();
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
            writeln!(self.context).unwrap();
            self.process_command()
        } else if (input == 0x08) || (input == 0x7F) {
            // Handling backspace or delete
            if self.used > 0 {
                write!(self.context, "\u{0008} \u{0008}").unwrap();
                self.used -= 1;
            }
            Outcome::NeedMore
        } else if self.used < self.buffer.len() {
            self.buffer[self.used] = input;
            self.used += 1;

            // We have to do this song and dance because `self.prompt()` needs
            // a mutable reference to self, and we can't have that while
            // holding a reference to the buffer at the same time.
            // This line grabs the buffer, checks it's OK, then releases it again
            let valid = core::str::from_utf8(&self.buffer[0..self.used]).is_ok();
            // Now we've released the buffer, we can draw the prompt
            if valid {
                write!(self.context, "\r").unwrap();
                self.prompt(false);
            }
            // Grab the buffer again to render it to the screen
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

    fn process_command(&mut self) -> Outcome {
        if let Ok(command_line) = core::str::from_utf8(&self.buffer[0..self.used]) {
            if command_line == "help" {
                let menu = self.menus[self.depth].unwrap();
                for item in menu.items {
                    self.print_help(&item);
                }
                if self.depth != 0 {
                    writeln!(self.context, "* exit - leave this menu.").unwrap();
                }
                writeln!(self.context, "* help - print this help text").unwrap();
                Outcome::CommandProcessed
            } else if command_line == "exit" && self.depth != 0 {
                if self.depth == self.menus.len() {
                    writeln!(self.context, "Can't enter menu - structure too deep.").unwrap();
                } else {
                    self.menus[self.depth] = None;
                    self.depth -= 1;
                }
                Outcome::CommandProcessed
            } else {
                let mut parts = command_line.split(' ');
                if let Some(cmd) = parts.next() {
                    let mut found = false;
                    let menu = self.menus[self.depth].unwrap();
                    for item in menu.items {
                        if cmd == item.command {
                            match item.item_type {
                                ItemType::Callback {
                                    function,
                                    parameters,
                                } => self.call_function(
                                    function,
                                    parameters,
                                    menu,
                                    item,
                                    command_line,
                                ),
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
                        writeln!(self.context, "Command {:?} not found. Try 'help'.", cmd).unwrap();
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
    }

    fn print_help(&mut self, item: &Item<T>) {
        match item.item_type {
            ItemType::Callback { parameters, .. } => {
                if !parameters.is_empty() {
                    write!(self.context, "* {}", item.command).unwrap();
                    for param in parameters.iter() {
                        match param {
                            Parameter::Mandatory(name) => {
                                write!(self.context, " <{}>", name).unwrap();
                            }
                            Parameter::Optional(name) => {
                                write!(self.context, " [ <{}> ]", name).unwrap();
                            }
                            Parameter::Named {
                                parameter_name,
                                argument_name,
                            } => {
                                write!(self.context, " [ --{}={} ]", parameter_name, argument_name)
                                    .unwrap();
                            }
                        }
                    }
                } else {
                    write!(self.context, "* {}", item.command).unwrap();
                }
            }
            ItemType::Menu(_menu) => {
                write!(self.context, "* {}", item.command).unwrap();
            }
        }
        if let Some(help) = item.help {
            write!(self.context, " - {}", help).unwrap();
        }
        writeln!(self.context).unwrap();
    }

    fn call_function(
        &self,
        _function: ItemCallbackFn<T>,
        _parameters: &[Parameter],
        _parent_menu: &Menu<T>,
        _item: &Item<T>,
        _command: &str,
    ) {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
