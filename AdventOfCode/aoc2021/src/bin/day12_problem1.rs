use std::collections::HashMap;
use std::fs;
use std::time::Instant;

struct Cave {
    is_small_cave: bool,
    adjacent_cave_indices: Vec<usize>,
}

impl Cave {
    fn add_adjacent_cave_index(&mut self, index: usize) {
        self.adjacent_cave_indices.push(index);
    }

    fn enter_and_count_paths_to_end(
        &self,
        cave_index: usize,
        caves: &Vec<Cave>,
        caves_open: &mut Vec<bool>,
    ) -> usize {
        if self.is_small_cave {
            if !caves_open[cave_index] {
                return 0;
            }
            caves_open[cave_index] = false;
        };
        let sub_path_count: usize = self
            .adjacent_cave_indices
            .iter()
            .map(|&next_cave_index| {
                // Check if we are at the end
                if next_cave_index == 1 {
                    1
                } else {
                    let next_cave = &caves[next_cave_index];
                    next_cave.enter_and_count_paths_to_end(next_cave_index, caves, caves_open)
                }
            })
            .sum();

        // Before back-tracking, ensure this cave is open again
        caves_open[cave_index] = true;
        sub_path_count
    }
}

type CaveIndexLookup<'a> = HashMap<&'a str, usize>;

struct CaveSystem<'a> {
    caves: Vec<Cave>,
    cave_index_lkp: CaveIndexLookup<'a>,
}

impl<'a> CaveSystem<'a> {
    fn new() -> Self {
        let mut cave_system = CaveSystem {
            caves: Vec::<Cave>::new(),
            cave_index_lkp: CaveIndexLookup::<'a>::new(),
        };
        cave_system.add_cave("start");
        cave_system.add_cave("end");
        cave_system
    }

    fn add_cave(&mut self, name: &'a str) {
        if self.cave_index_lkp.contains_key(name) {
            return;
        }
        let cave_index = self.caves.len();
        let is_small_cave = name.chars().next().unwrap().is_lowercase();
        self.caves.push(Cave {
            is_small_cave,
            adjacent_cave_indices: Vec::new(),
        });
        self.cave_index_lkp.insert(name, cave_index);
    }

    fn add_connection_between_caves<'b: 'a, 'c: 'a>(
        &mut self,
        cave1_name: &'b str,
        cave2_name: &'c str,
    ) {
        self.add_cave(cave1_name);
        self.add_cave(cave2_name);
        let cave1_index = self.cave_index_lkp[cave1_name];
        let cave2_index = self.cave_index_lkp[cave2_name];
        let cave1 = &mut self.caves[cave1_index];
        cave1.add_adjacent_cave_index(cave2_index);
        let cave2 = &mut self.caves[cave2_index];
        cave2.add_adjacent_cave_index(cave1_index);
    }

    fn count_paths_from_start_to_end(&mut self) -> usize {
        let mut caves_open = vec![true; self.caves.len()];
        self.caves[0].enter_and_count_paths_to_end(0, &self.caves, &mut caves_open)
    }
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day12_input.txt").unwrap();
    let mut cave_system = CaveSystem::new();
    for line in contents.lines() {
        let (node_name1, node_name2) = line.split_once('-').unwrap();
        cave_system.add_connection_between_caves(node_name1, node_name2);
    }
    let path_count = cave_system.count_paths_from_start_to_end();
    let duration = start_time.elapsed();
    println!("# of paths from start to end: {}", path_count);
    println!("Duration (incl reading from file): {:?}", duration); // 277.461µs
}
