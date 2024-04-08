//! The Menu Manager looks after the menu and where we currently are within it.
#![deny(missing_docs)]

use super::{ItemType, Menu};

/// Holds a nested tree of Menus and remembers which menu within the tree we're
/// currently looking at.
pub struct MenuManager<'a, T> {
    menu: Menu<'a, T>,
    /// Maximum four levels deep
    menu_index: [Option<usize>; 4],
}

impl<'a, T> MenuManager<'a, T> {
    /// Create a new MenuManager.
    ///
    /// You will be at the top-level.
    pub fn new(menu: Menu<'a, T>) -> Self {
        Self {
            menu,
            menu_index: [None, None, None, None],
        }
    }

    /// How deep into the tree are we?
    pub fn depth(&self) -> usize {
        self.menu_index.iter().take_while(|x| x.is_some()).count()
    }

    /// Go back up to a higher-level menu
    pub fn pop_menu(&mut self) {
        if let Some(pos) = self.menu_index.iter_mut().rev().find(|x| x.is_some()) {
            pos.take();
        }
    }

    /// Drop into a sub-menu.
    ///
    /// The index must be the index of a valid sub-menu, not any other kind of
    /// item. Do not push too many items.
    pub fn push_menu(&mut self, index: usize) {
        let menu = self.get_menu(None);
        let item = menu.items[index];
        if !matches!(item.item_type, ItemType::Menu(_)) {
            panic!("Specified index is not a menu");
        }

        let pos = self.menu_index.iter_mut().find(|x| x.is_none()).unwrap();
        pos.replace(index);
    }

    /// Get a menu.
    ///
    /// Menus are nested. If `depth` is `None`, get the current menu. Otherwise
    /// if it is `Some(i)` get the menu at depth `i`.
    pub fn get_menu(&self, depth: Option<usize>) -> &Menu<'a, T> {
        let mut menu = &self.menu;

        let depth = depth.unwrap_or_else(|| self.depth());

        for position in self
            .menu_index
            .iter()
            .take_while(|x| x.is_some())
            .map(|x| x.unwrap())
            .take(depth)
        {
            if let ItemType::Menu(m) = menu.items[position].item_type {
                menu = m
            } else {
                panic!("Selected item is not a menu");
            }
        }

        menu
    }
}
