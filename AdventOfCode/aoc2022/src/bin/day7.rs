use std::time::Instant;

struct IndexRange {
    first_index: usize,
    last_index: usize,
}

enum Entry<'a> {
    File {
        name: &'a str,
        size: usize,
    },
    Dir {
        name: &'a str,
        child_indices: Option<IndexRange>,
    },
}

impl Entry<'_> {
    fn get_name(&self) -> &str {
        match self {
            Entry::File {
                name: file_name,
                size: _
            } => file_name,
            Entry::Dir {
                name: dir_name,
                child_indices: _
            } => dir_name,
        }
    }
}

struct FileSystem<'a> {
    entries: Vec<Entry<'a>>,
    pwd_index: usize,
}

impl<'a> FileSystem<'a> {
    fn new() -> FileSystem<'a> {
        FileSystem {
            entries: vec![Entry::Dir {
                name: "/",
                child_indices: None,
            }],
            pwd_index: 0,
        }
    }

    #[inline]
    fn get_entry_by_index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }

    #[inline]
    fn get_root(&self) -> &Entry {
        self.get_entry_by_index(0)
    }

    fn cd_to_root(&mut self) {
        self.pwd_index = 0;
        #[cfg(debug_assertions)]
        println!("cd /  # {}", self.get_path_to_pwd());
    }

    fn get_parent_index_of_entry_with_index(&self, entry_index: usize) -> Option<usize> {
        self.entries
            .iter()
            .enumerate()
            .find(|&(_, entry)| match entry {
                Entry::Dir {
                    name: _,
                    child_indices: Some(
                        IndexRange {
                            first_index: start,
                            last_index: end,
                        }
                    ),
                } => (*start <= entry_index) && (*end >= entry_index),
                _ => false,
            })
            .map(|(index, _)| index)
    }

    fn cd_up(&mut self) -> Result<(), String> {
        if self.pwd_index == 0 {
            Err("Cannot cd to parent when at root directory".to_string())
        } else {
            if let Some(parent_index) = self.get_parent_index_of_entry_with_index(self.pwd_index) {
                self.pwd_index = parent_index;
                #[cfg(debug_assertions)]
                println!("cd ..  # {}", self.get_path_to_pwd());
                Ok(())
            } else {
                Err("Cannot find parent to navigate up to".to_string())
            }
        }
    }

    fn get_pwd(&self) -> &Entry {
        &self.entries[self.pwd_index]
    }

    fn get_index_of_child_entry(&self, child_name: &str) -> Option<usize> {
        if let Entry::Dir {
            name: _,
            child_indices:
                Some(IndexRange {
                    first_index: start,
                    last_index: end,
                }),
        } = self.get_pwd()
        {
            let child_found = self.entries[*start..=*end]
                .iter()
                .enumerate()
                .find(|&(_, entry)| entry.get_name() == child_name);

            Some(*start + child_found?.0)
        } else {
            None
        }
    }

    fn cd(&mut self, sub_dir_name: &str) -> Result<(), String> {
        if let Entry::File { .. } = *self.get_pwd() {
            Err("The current working directory is a file, not a directory".to_string())
        } else if let Some(sub_dir_index) = self.get_index_of_child_entry(sub_dir_name) {
            if let Entry::Dir { .. } = self.entries[sub_dir_index] {
                self.pwd_index = sub_dir_index;
                #[cfg(debug_assertions)]
                println!("cd {}  # {}", sub_dir_name, self.get_path_to_pwd());
                Ok(())
            } else {
                Err(format!(
                    "Can only cd to a sub-directory, not to a file ({})",
                    sub_dir_name
                ))
            }
        } else {
            Err(format!(
                "A sub-directory named {} could not be found",
                sub_dir_name
            ))
        }
    }

    fn add_child_dir(&mut self, sub_dir_name: &'a str) -> Result<(), String> {
        if let Some(child_index) = self.get_index_of_child_entry(sub_dir_name) {
            if let Entry::Dir { .. } = self.entries[child_index] {
                // It already exists, so don't add it again
                Ok(())
            } else {
                Err(format!(
                    "Cannot create a sub-directory named \"{}\" when a file with that name already exists",
                    sub_dir_name))
            }
        } else {
            let next_index = self.entries.len();
            match self.entries[self.pwd_index] {
                Entry::File { .. } => {
                    Err("The present working directory is not a directory".to_string())
                }
                Entry::Dir {
                    name: _,
                    child_indices: ref mut indices,
                } => {
                    if let Some(IndexRange {
                        first_index: _,
                        last_index: ref mut end,
                    }) = indices
                    {
                        *end = next_index;
                    } else {
                        *indices = Some(IndexRange {
                            first_index: next_index,
                            last_index: next_index,
                        })
                    }
                    self.entries.push(Entry::Dir {
                        name: sub_dir_name,
                        child_indices: None,
                    });
                    Ok(())
                }
            }
        }
    }

    fn add_file_to_pwd(&mut self, file_name: &'a str, file_size: usize) -> Result<(), String> {
        if let Some(_) = self.get_index_of_child_entry(file_name) {
            Err(format!(
                "Cannot create a file named \"{}\" when an entry with that name already exists",
                file_name
            ))
        } else {
            let next_index = self.entries.len();

            match self.entries[self.pwd_index] {
                Entry::File { .. } => {
                    Err("The present working directory is not a directory".to_string())
                }
                Entry::Dir {
                    name: _,
                    child_indices: ref mut indices,
                } => {
                    if let Some(IndexRange {
                        first_index: _,
                        last_index: ref mut end,
                    }) = indices
                    {
                        *end = next_index;
                    } else {
                        *indices = Some(IndexRange {
                            first_index: next_index,
                            last_index: next_index,
                        })
                    }
                    self.entries.push(Entry::File {
                        name: file_name,
                        size: file_size,
                    });
                    Ok(())
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn get_path_to_entry_with_index(&self, index: usize) -> String {
        let mut path: String = self.get_entry_by_index(index).get_name().to_string();
        let mut next_index = index;
        while let Some(parent_index) = self.get_parent_index_of_entry_with_index(next_index) {
            next_index = parent_index;
            let parent_entry = self.get_entry_by_index(next_index);
            if next_index == 0 {
                path = "/".to_string() + path.as_str();
            } else {
                path = parent_entry.get_name().to_string() + "/" + path.as_str();
            }
        };
        path
    }

    #[cfg(debug_assertions)]
    fn get_path_to_pwd(&self) -> String {
        self.get_path_to_entry_with_index(self.pwd_index)
    }
}

fn parse(contents: &str) -> Result<FileSystem, String> {
    let mut file_sys = FileSystem::new();

    for line in contents.lines() {
        let (first, second) = line.split_once(' ').unwrap();
        match (first, second) {
            ("$", "ls") => {} // Ignore these. Identify lines not starting with '$' instead.
            ("$", "cd /") => file_sys.cd_to_root(),
            ("$", "cd ..") => file_sys.cd_up()?,
            ("$", cd_sub_dir_name) => {
                let (cd_str, sub_dir_name) = cd_sub_dir_name.split_once(' ').unwrap();
                if cd_str == "cd" {
                    file_sys.cd(sub_dir_name)?
                } else {
                    return Err(format!("Unrecognized command {} in line {}", cd_str, line));
                }
            }
            ("dir", sub_dir_name) => file_sys.add_child_dir(sub_dir_name)?,
            (size_str, file_name) => {
                let file_size = size_str.parse::<usize>().unwrap();
                file_sys.add_file_to_pwd(file_name, file_size)?
            }
        }
    }
    Ok(file_sys)
}

const MAX_DIR_SIZE: usize = 100_000;

fn sum_of_dir_sizes_with_total_size_up_to_max_dir_size(file_sys: &FileSystem) -> usize {
    let mut sum_of_sizes: usize = 0;
    recursively_sum_entry_sizes(file_sys, file_sys.get_root(), &mut sum_of_sizes);
    sum_of_sizes
}

fn recursively_sum_entry_sizes(
    file_sys: &FileSystem,
    entry: &Entry,
    sum_of_sizes: &mut usize
) -> usize {
   match entry {
       Entry::File {
           name: _,
           size: file_size,
       } => *file_size,
       Entry::Dir {
           name: _,
           child_indices: Some(ref indices),
       } => {
           let mut dir_size: usize = 0;
           for i in indices.first_index..=indices.last_index {
               let child_entry = &file_sys.entries[i];
               dir_size += recursively_sum_entry_sizes(file_sys, child_entry, sum_of_sizes)
           }
           if dir_size <= MAX_DIR_SIZE {
               *sum_of_sizes += dir_size;
           }
           dir_size
       },
       _ => 0
   }
}

fn main() {
    let start_total = Instant::now();

    let mut start_step = start_total;
    let contents = std::fs::read_to_string("data/day7_input.txt").unwrap();
    println!("Time to read inputs from file: {:?}", start_step.elapsed());

    start_step = Instant::now();
    let file_sys = parse(&contents).unwrap();
    println!("Time to parse instructions and construct the file sytem: {:?}", start_step.elapsed());

    start_step = Instant::now();
    let part1_answer = sum_of_dir_sizes_with_total_size_up_to_max_dir_size(&file_sys);
    println!("Time to calculate answer: {:?}", start_step.elapsed());

    println!("2022 day 7 part 1 answer: {}", part1_answer);
    println!("Total time to calculate part 1 answer: {:?}", start_total.elapsed());
}
