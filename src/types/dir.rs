use std::{
    collections::BTreeMap,
    vec::IntoIter,
};

use super::{File, Path};

#[derive(Debug, Clone)]
pub struct Dir {
    pub name: String,
    pub files: Vec<File>,
    children_dirs: BTreeMap<String, Dir>,
}

impl Dir {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            files: Vec::new(),
            children_dirs: BTreeMap::new(),
        }
    }

    pub fn root() -> Self {
        Self {
            name: "/".to_string(),
            files: Vec::new(),
            children_dirs: BTreeMap::new(),
        }
    }

    pub fn add_new_path(&mut self, mut components: IntoIter<String>) -> &mut Self {
        let mut rel_dir = self;
        while let Some(name) = components.next() {
            rel_dir = rel_dir
                .children_dirs
                .entry(name.to_string())
                .or_insert_with(|| Dir::new(&name));
        }
        rel_dir
    }

    fn find_mut_child(&mut self, name: &str) -> Option<&mut Self> {
        self.children_dirs.get_mut(name)
    }

    pub fn find_directory_by_relative_path(&mut self, path: &Path) -> Option<&Self> {
        let mut components = path.components().into_iter();

        let mut relative_dir = self;
        while let Some(name) = components.next() {
            if let Some(next_dir) = relative_dir.find_mut_child(&name) {
                relative_dir = next_dir;
                continue;
            }
            return None;
        }
        Some(relative_dir)
    }

    pub fn get_children_dirs(&self) -> &BTreeMap<String, Self> {
        &self.children_dirs
    }

    pub fn get_files_messages_ids(&self) -> Vec<i32> {
        self.files.iter().map(|file| file.message_id).collect()
    }
}
