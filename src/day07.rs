use crate::utils;
use regex::Regex;


struct Id {
    index: usize,
}


struct File {
    size: u32,
    name: String,
    parent: Option<Id>,
}


struct Directory {
    name: String,
    parent: Option<Id>,
    files: Vec<Id>,
    directories: Vec<Id>,
}


struct FileSystem {
    directories: Vec<Directory>,
    files: Vec<File>,
}


impl FileSystem {
    fn new_file(&mut self, name: String, size: u32) -> Id {

        let index =self.files.len();

        self.files.push(File{
            size,
            name,
            parent: None,
        });
        Id{index}
    }

    fn new_directory(&mut self, name: String) -> Id {

        let index =self.directories.len();

        self.directories.push(Directory{
            name,
            files: vec![],
            directories: vec![],
            parent: None,
        });
        Id{index}
    }

    fn add_file_to_dir(&mut self, name: String, size: u32, dir_index: &Id) -> Id {
        let file_id = self.new_file(name, size);
        self.files[file_id.index].parent = Some(Id{index: dir_index.index});
        self.directories[dir_index.index].files.push(
            Id{index: file_id.index}
        );
        file_id
    }

    fn add_dir_to_dir(&mut self, name: String, parent_index: &Id) -> Id {
        let new_dir_id = self.new_directory(name);
        self.directories[new_dir_id.index].parent = Some(Id{index: parent_index.index});
        self.directories[parent_index.index].directories.push(
            Id{index: new_dir_id.index}
        );
        new_dir_id
    }

    fn move_dir(&mut self, cwd_id: Id, dir: String) -> Id {
        let cwd = &self.directories[cwd_id.index];

        if dir == ".." {
            return Id{
                index: cwd.parent.as_ref().expect("Tried to move to null directory").index
            };
        } else {
            return
                match cwd.directories.iter().find(|d| self.directories[d.index].name == dir) {
                    Some(d) => Id{index: d.index},
                    None => self.add_dir_to_dir(dir, &cwd_id)
                }
        }
    }

    fn dir_size(&self, id: &Id) -> u32 {
        let dir = &self.directories[id.index];
        let file_size: u32 = dir.files.iter().map(
            |file_id| {self.files[file_id.index].size}
        ).sum();

        let dir_size: u32 = dir.directories.iter().map(
            |dir_id| {self.dir_size(dir_id)}
        ).sum();

        file_size + dir_size
    }
}


fn create_filesystem() -> FileSystem {
    let mut filesystem = FileSystem{
        directories: vec![],
        files: vec![],
    };
    let mut cwd_id = filesystem.new_directory(String::from("/"));

    for line in utils::read_input("inputs/07").lines() {
        let l = line.trim();
        if l.starts_with("$ cd") {
            if !(l == "$ cd /") {
                let (_, dir) = l.split_at(5);
                cwd_id = filesystem.move_dir(cwd_id, dir.to_string());
            }
        } else if l.starts_with("$ ls") {
            continue;
        } else if l.starts_with("dir") {
            let (_, name) = l.split_at(4);
            filesystem.add_dir_to_dir(
                name.to_string(),
                &cwd_id,
            );
        } else {
            let re = Regex::new(r"^(\d+) ([a-z.]+)$").unwrap();
            let caps = re.captures(l).unwrap();

            filesystem.add_file_to_dir(
                caps[2].to_string(),
                caps[1].parse::<u32>().unwrap(),
                &cwd_id,
            );
        }
    }
    filesystem
}


pub fn star_1() -> u32 {
    let filesystem = create_filesystem();

    let mut total_size = 0u32;
    for (index, _) in filesystem.directories.iter().enumerate() {
        let dir_size = filesystem.dir_size(&Id{index});
        if dir_size <= 100000 {
            total_size += dir_size;
        };
    }
    total_size
}


pub fn star_2() -> u32 {
    let filesystem = create_filesystem();

    assert_eq!(filesystem.directories[0].name, "/");
    let required_space = filesystem.dir_size(&Id{index: 0}) - 40000000;

    let mut smallest_dir = 70000000;
    for (index, _) in filesystem.directories.iter().enumerate() {
        let dir_size = filesystem.dir_size(&Id{index});
        if dir_size >= required_space && dir_size < smallest_dir {
            smallest_dir = dir_size;
        };
    }
    smallest_dir
}
