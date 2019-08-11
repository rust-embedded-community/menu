#![no_std]

type MenuCallbackFn<T> = fn(menu: &Menu<T>, context: &mut T);
type ItemCallbackFn<T> = fn(menu: &Menu<T>, item: &Item<T>, args: &[&str], context: &mut T);

#[derive(Debug)]
/// Describes a parameter to the command
pub enum Parameter<'a> {
    /// A mandatory positional parameter
    Mandatory(&'a str),
    /// An optional positional parameter. Must come after the mandatory positional arguments.
    Optional(&'a str),
    /// A named parameter with no argument (e.g. `--verbose` or `--dry-run`)
    Named(&'a str),
    /// A named parameter with argument (e.g. `--mode=foo` or `--level=3`)
    NamedValue {
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
    _Dummy,
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

/// Looks for the named parameter in the parameter list of the item, then
/// finds the correct argument.
///
/// * Returns `Ok(None)` if `parameter_name` gives an optional or named
///   parameter and that argument was not given.
/// * Returns `Ok(arg)` if the argument corresponding to `parameter_name` was
///   found. `arg` is the empty string if the parameter was `Parameter::Named`
///   (and hence doesn't take a value).
/// * Returns `Err(())` if `parameter_name` was not in `item.parameter_list`
///   or `item` wasn't an Item::Callback
pub fn argument_finder<'a, T>(
    item: &'a Item<'a, T>,
    argument_list: &'a [&'a str],
    name_to_find: &'a str,
) -> Result<Option<&'a str>, ()> {
    if let ItemType::Callback { parameters, .. } = item.item_type {
        // Step 1 - Find `name_to_find` in the parameter list.
        let mut found_param = None;
        let mut mandatory_count = 0;
        let mut optional_count = 0;
        for param in parameters.iter() {
            match param {
                Parameter::Mandatory(name) => {
                    mandatory_count += 1;
                    if *name == name_to_find {
                        found_param = Some((param, mandatory_count));
                    }
                }
                Parameter::Optional(name) => {
                    optional_count += 1;
                    if *name == name_to_find {
                        found_param = Some((param, optional_count));
                    }
                }
                Parameter::Named(name) => {
                    if *name == name_to_find {
                        found_param = Some((param, 0));
                    }
                }
                _ => {
                    unimplemented!();
                }
            }
        }
        // Step 2 - What sort of parameter is it?
        match found_param {
            // Step 2a - Mandatory Positional
            Some((Parameter::Mandatory(_name), mandatory_idx)) => {
                // We want positional parameter number `mandatory_idx` of `mandatory_count`.
                let mut positional_args_seen = 0;
                for arg in argument_list {
                    if !arg.starts_with("--") {
                        // Positional
                        positional_args_seen += 1;
                        if positional_args_seen == mandatory_idx {
                            return Ok(Some(arg));
                        }
                    }
                }
                // Valid thing to ask for but we don't have it
                Ok(None)
            }
            // Step 2b - Optional Positional
            Some((Parameter::Optional(_name), optional_idx)) => {
                // We want positional parameter number `mandatory_idx` of `mandatory_count`.
                let mut positional_args_seen = 0;
                for arg in argument_list {
                    if !arg.starts_with("--") {
                        // Positional
                        positional_args_seen += 1;
                        if positional_args_seen == (mandatory_count + optional_idx) {
                            return Ok(Some(arg));
                        }
                    }
                }
                // Valid thing to ask for but we don't have it
                Ok(None)
            }
            // Step 2c - Named
            Some((Parameter::Named(name), _)) => {
                for arg in argument_list {
                    if arg.starts_with("--") && (&arg[2..] == *name) {
                        return Ok(Some(""));
                    }
                }
                // Valid thing to ask for but we don't have it
                Ok(None)
            }
            // Step 2d - NamedValue
            // Step 2e - not found
            _ => Err(()),
        }
    } else {
        // Not an item with arguments
        Err(())
    }
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
                    let item = Item {
                        command: "exit",
                        help: Some("leave this menu"),
                        item_type: ItemType::_Dummy,
                    };
                    self.print_help(&item);
                }
                let item = Item {
                    command: "help",
                    help: Some("show this help"),
                    item_type: ItemType::_Dummy,
                };
                self.print_help(&item);
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
                                } => Self::call_function(
                                    self.context,
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
                                ItemType::_Dummy => {
                                    unreachable!();
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
                    write!(self.context, "{}", item.command).unwrap();
                    for param in parameters.iter() {
                        match param {
                            Parameter::Mandatory(name) => {
                                write!(self.context, " <{}>", name).unwrap();
                            }
                            Parameter::Optional(name) => {
                                write!(self.context, " [ <{}> ]", name).unwrap();
                            }
                            Parameter::Named(name) => {
                                write!(self.context, " [ --{} ]", name).unwrap();
                            }
                            Parameter::NamedValue {
                                parameter_name,
                                argument_name,
                            } => {
                                write!(self.context, " [ --{}={} ]", parameter_name, argument_name)
                                    .unwrap();
                            }
                        }
                    }
                } else {
                    write!(self.context, "{}", item.command).unwrap();
                }
            }
            ItemType::Menu(_menu) => {
                write!(self.context, "{}", item.command).unwrap();
            }
            ItemType::_Dummy => {
                write!(self.context, "{}", item.command).unwrap();
            }
        }
        if let Some(help) = item.help {
            write!(self.context, " - {}", help).unwrap();
        }
        writeln!(self.context).unwrap();
    }

    fn call_function(
        context: &mut T,
        callback_function: ItemCallbackFn<T>,
        parameters: &[Parameter],
        parent_menu: &Menu<T>,
        item: &Item<T>,
        command: &str,
    ) {
        let mandatory_parameter_count = parameters
            .iter()
            .filter(|p| match p {
                Parameter::Mandatory(_) => true,
                _ => false,
            })
            .count();
        let positional_parameter_count = parameters
            .iter()
            .filter(|p| match p {
                Parameter::Mandatory(_) => true,
                Parameter::Optional(_) => true,
                _ => false,
            })
            .count();
        if command.len() >= item.command.len() {
            // Maybe arguments
            let mut argument_buffer: [&str; 16] = [""; 16];
            let mut argument_count = 0;
            let mut positional_arguments = 0;
            for (slot, arg) in argument_buffer
                .iter_mut()
                .zip(command[item.command.len()..].split_whitespace())
            {
                *slot = arg;
                argument_count += 1;
                if arg.starts_with("--") {
                    // Validate named argument
                    let mut found = false;
                    for param in parameters.iter() {
                        match param {
                            Parameter::Named(name) => {
                                if &arg[2..] == *name {
                                    found = true;
                                    break;
                                }
                            }
                            Parameter::NamedValue { parameter_name, .. } => {
                                if let Some(name) = arg[2..].split('=').next() {
                                    if name == *parameter_name {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                            _ => {
                                // Ignore
                            }
                        }
                    }
                    if !found {
                        writeln!(context, "Error: Did not understand {:?}", arg).unwrap();
                        return;
                    }
                } else {
                    positional_arguments += 1;
                }
            }
            if positional_arguments < mandatory_parameter_count {
                writeln!(context, "Error: Insufficient arguments given").unwrap();
            } else if positional_arguments > positional_parameter_count {
                writeln!(context, "Error: Too many arguments given").unwrap();
            } else {
                callback_function(
                    parent_menu,
                    item,
                    &argument_buffer[0..argument_count],
                    context,
                );
            }
        } else {
            // Definitely no arguments
            if mandatory_parameter_count == 0 {
                callback_function(parent_menu, item, &[], context);
            } else {
                writeln!(context, "Error: Insufficient arguments given").unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy(_menu: &Menu<u32>, _item: &Item<u32>, _args: &[&str], _context: &mut u32) {}

    #[test]
    fn find_arg_mandatory() {
        let item = Item {
            command: "dummy",
            help: None,
            item_type: ItemType::Callback {
                function: dummy,
                parameters: &[
                    Parameter::Mandatory("foo"),
                    Parameter::Mandatory("bar"),
                    Parameter::Mandatory("baz"),
                ],
            },
        };
        assert_eq!(
            argument_finder(&item, &["a", "b", "c"], "foo"),
            Ok(Some("a"))
        );
        assert_eq!(
            argument_finder(&item, &["a", "b", "c"], "bar"),
            Ok(Some("b"))
        );
        assert_eq!(
            argument_finder(&item, &["a", "b", "c"], "baz"),
            Ok(Some("c"))
        );
        // Not an argument
        assert_eq!(argument_finder(&item, &["a", "b", "c"], "quux"), Err(()));
    }

    #[test]
    fn find_arg_optional() {
        let item = Item {
            command: "dummy",
            help: None,
            item_type: ItemType::Callback {
                function: dummy,
                parameters: &[
                    Parameter::Mandatory("foo"),
                    Parameter::Mandatory("bar"),
                    Parameter::Optional("baz"),
                ],
            },
        };
        assert_eq!(
            argument_finder(&item, &["a", "b", "c"], "foo"),
            Ok(Some("a"))
        );
        assert_eq!(
            argument_finder(&item, &["a", "b", "c"], "bar"),
            Ok(Some("b"))
        );
        assert_eq!(
            argument_finder(&item, &["a", "b", "c"], "baz"),
            Ok(Some("c"))
        );
        // Not an argument
        assert_eq!(argument_finder(&item, &["a", "b", "c"], "quux"), Err(()));
        // Missing optional
        assert_eq!(argument_finder(&item, &["a", "b"], "baz"), Ok(None));
    }

    #[test]
    fn find_arg_named() {
        let item = Item {
            command: "dummy",
            help: None,
            item_type: ItemType::Callback {
                function: dummy,
                parameters: &[
                    Parameter::Mandatory("foo"),
                    Parameter::Named("bar"),
                    Parameter::Named("baz"),
                ],
            },
        };
        assert_eq!(
            argument_finder(&item, &["a", "--bar", "--baz"], "foo"),
            Ok(Some("a"))
        );
        assert_eq!(
            argument_finder(&item, &["a", "--bar", "--baz"], "bar"),
            Ok(Some(""))
        );
        assert_eq!(
            argument_finder(&item, &["a", "--bar", "--baz"], "baz"),
            Ok(Some(""))
        );
        // Not an argument
        assert_eq!(
            argument_finder(&item, &["a", "--bar", "--baz"], "quux"),
            Err(())
        );
        // Missing named
        assert_eq!(argument_finder(&item, &["a"], "baz"), Ok(None));
    }
}
