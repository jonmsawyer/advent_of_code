use std::collections::HashMap;
use std::fmt;

use itertools::Itertools;

use super::{CommandKind, FileType};

#[derive(Debug)]
pub struct FileSystem {
    pub commands: Vec<String>,
    pub root: HashMap<String, Vec<(String, usize)>>,
    pub dir_size: HashMap<String, usize>,
    pub path: Vec<String>,
    pub command_kind: CommandKind,
    pub file_type: FileType,
    pub file_name: String,
    pub file_size: usize,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            ..Default::default()
        }
    }

    #[allow(clippy::suspicious_else_formatting)]
    pub fn parse(&mut self, line: String) {
        self.commands.push(line.clone());

        for el in line.split(' ') {
            // The order of these if {...} else if {...} statements are important.
            if el.eq("$") {
                // Is command prompt?
                print!("$ ");
                continue;
            } else if el.eq("cd") {
                // Is change directory command?
                self.command_kind = CommandKind::CD;
                self.file_type = FileType::Dir;
                continue;
            } else if el.eq("ls") {
                // Is list command?
                println!("ls");
                self.command_kind = CommandKind::LS;
                continue;
            } else if el.eq("dir") {
                // Is directory listing?
                self.file_type = FileType::Dir;
                continue;
            } else if el.eq("..") {
                // Is parent directory?
                self.pop_path();
                println!("cd {:<20}New path  (pop): {}", "..", self.path.join(""));
                self.apply_command();
                continue;
            } else if let Ok(fs) = el.parse::<usize>() {
                // Is file?
                self.file_type = FileType::File;
                self.file_size = fs;
                continue;
            } else if self.command_kind == CommandKind::CD {
                // Is new `cd` directory path?
                self.push_path(el.to_string());
                println!("cd {:<20}New path (push): {}", el, self.path.join(""));
                self.apply_command();
                continue;
            } else if self.file_type == FileType::Dir {
                // Continue directory listing.
                self.file_name = el.to_string();
                println!(
                    "dir {dir:<21}New dir        : {{dir_name: \"{dir}\"}}",
                    dir = el
                );
                self.apply_command();
                continue;
            } else if self.file_type == FileType::File {
                // Continue file listing.
                self.file_name = el.to_string();

                let print = format!("{} {}", self.file_size, el);
                println!(
                    "{:<25}New file       : {{file_name: \"{}\", file_size: {}}}",
                    print, self.file_name, self.file_size
                );

                self.apply_command();
                continue;
            }
        }

        self.reset();
    }

    pub fn apply_command(&mut self) {
        if self.file_type == FileType::File {
            let file = (self.file_name.clone(), self.file_size);

            // Insert the file into the path's vec ([self.root]).
            if let Some(entry) = self.root.get_mut(&self.path.join("")) {
                println!(">>> Apply command: File={:?}", file);
                entry.push(file);
            } else {
                let new_vec = vec![file.clone()];
                println!(">>> Apply command: File={:?}; New Vec={:?}", file, new_vec);
                self.root.entry(self.path.join("")).or_insert(new_vec);
            };

            // Update the directory size as per all file sizes.
            if let Some(entry) = self.dir_size.get_mut(&self.path.join("")) {
                *entry += self.file_size;
            } else {
                self.dir_size
                    .entry(self.path.join(""))
                    .or_insert(self.file_size);
            }

            // Update the parent directory's size (recursive until root (/)).
            let mut new_path = self.path.clone();
            while !new_path.is_empty() {
                new_path.pop();
                if new_path.is_empty() {
                    break;
                }
                new_path.pop();

                if let Some(entry) = self.dir_size.get_mut(&new_path.join("")) {
                    *entry += self.file_size;
                }
            }
        } else if self.file_type == FileType::Dir && !self.file_name.eq("") {
            println!(">>> Apply command: Dir={}", self.file_name);
            let mut new_path = self.path.clone();
            new_path.push(self.file_name.clone());
            new_path.push("/".to_string());
            self.dir_size.entry(new_path.join("")).or_insert(0_usize);
        }
    }

    /// Reset the state of the filesystem parsing.
    pub fn reset(&mut self) {
        self.command_kind = CommandKind::None;
        self.file_type = FileType::None;
        self.file_name.clear();
        self.file_size = 0_usize;
    }

    /// This is called when a `$ cd ..` command is parsed. Go back one directory.
    pub fn pop_path(&mut self) {
        if !self.path.is_empty() {
            self.path.pop();
            if self.path.is_empty() {
                self.path.push("/".to_string()); // We always have a root
            } else {
                self.path.pop();
            }
        }
    }

    /// This is called when a `$ cd <dir>` command is parsed. Go forward one directory.
    pub fn push_path(&mut self, name: String) {
        if name.eq("/") {
            return;
        }

        self.path.push(name);
        self.path.push("/".to_string());
    }

    //
    // Part 1
    //

    pub fn do_part_1(&mut self) {
        let limit = 100_000_usize;
        let mut total = 0_usize;

        println!("Valid directories with size <= {}:", limit);
        for (dir_name, dir_size) in self.dir_size.iter() {
            if *dir_size <= limit {
                total += *dir_size;
                println!("{:>9} {}", dir_size, dir_name);
            }
        }

        println!(
            "Part One: The total of all directores of size 100,000 or less is: {}",
            total
        );
    }

    //
    // Part 2
    //

    pub fn do_part_2(&self) {
        let disk_size = 70_000_000_usize;
        let space_needed = 30_000_000_usize;
        let largest_dir = self.dir_size.get("/").unwrap();
        let space_free = disk_size - largest_dir;
        let need_space = space_needed - space_free;
        let mut dir_to_delete = "/".to_string();
        let mut dir_size_to_delete = space_needed;

        for (dir_name, dir_size) in self.dir_size.iter() {
            if *dir_size >= need_space && *dir_size < dir_size_to_delete {
                dir_to_delete = dir_name.clone();
                dir_size_to_delete = *dir_size;
            }
        }

        println!(
            "Part Two: The smallest directory size we can delete to free up enough space is {} ({})",
            dir_size_to_delete,
            dir_to_delete
        );
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        FileSystem {
            commands: Vec::<String>::new(),
            root: HashMap::new(),
            dir_size: HashMap::new(),
            path: vec!["/".to_string()],
            command_kind: CommandKind::None,
            file_type: FileType::None,
            file_name: String::new(),
            file_size: 0_usize,
        }
    }
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "FileSystem:").expect("Should be able to write diagnostic.");

        for path in self.root.keys().sorted() {
            writeln!(
                f,
                "{:<59} Dir Size={}",
                path,
                self.dir_size.get(path).unwrap()
            )
            .expect("Should be able to write path.");
            if let Some(vec) = self.root.get(path) {
                for (file_name, file_size) in vec.iter() {
                    let output = format!("    | {}", file_name);
                    writeln!(f, "{:<25} {:>6}", output, file_size)
                        .expect("Should be able to write file name and file size.");
                }
            }
        }

        writeln!(f, "Done!")
    }
}
