use super::{Menu, ItemType};

pub struct MenuManager<'a, T> {
    menu: Menu<'a, T>,
    /// Maximum four levels deep
    menu_index: [Option<usize>; 4],
}

impl <'a, T> MenuManager<'a, T> {
    pub fn new(menu: Menu<'a, T>) -> Self {
        Self {
            menu,
            menu_index: [None, None, None, None],
        }
    }

    pub fn depth(&self) -> usize {
        self.menu_index.iter().take_while(|x| x.is_some()).count()
    }

    pub fn pop_menu(&mut self) {
        if let Some(pos) = self.menu_index.iter_mut().rev().skip_while(|x| x.is_none()).next() {
            pos.take();
        }
    }

    pub fn push_menu(&mut self, index: usize) {
        let menu = self.get_menu(None);
        let item = menu.items[index];
        if !matches!(item.item_type, ItemType::Menu(_)) {
            panic!("Specified index is not a menu");
        }

        let pos = self.menu_index.iter_mut().skip_while(|x| x.is_some()).next().unwrap();
        pos.replace(index);
    }

    pub fn get_menu(&self, depth: Option<usize>) -> &Menu<'a, T> {
        let mut menu = &self.menu;

        let depth = depth.unwrap_or_else(|| self.depth());

        for position in self.menu_index.iter().take_while(|x| x.is_some()).map(|x| x.unwrap()).take(depth) {
            if let ItemType::Menu(m) = menu.items[position].item_type {
                menu = m
            } else {
                panic!("Selected item is not a menu");
            }
        }

        menu
    }

}
