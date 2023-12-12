use utils;

fn main() {
    let output = part1("day-05/src/input.txt");
    dbg!(output);
}

fn part1(file_path: &str) -> u64 {
    let mut seeds: Vec<u64> = vec![];
    let mut seed_moved_in_current_phase: Vec<bool> = vec![];
    let mut phase = 0;

    utils::process_file(file_path, |line: &str, line_index: usize| {
        if line == "" {
        } else if line.starts_with("seeds:") {
            for part in line.split(' ') {
                if !part.as_bytes()[0].is_ascii_digit() {
                    continue;
                }
                let number = part.parse::<u64>().unwrap();
                seeds.push(number);
            }
            seed_moved_in_current_phase = vec![false; seeds.len()];
        } else if !line.as_bytes()[0].is_ascii_digit() {
            phase += 1;
            seed_moved_in_current_phase = vec![false; seeds.len()];
        } else {
            let mut line_split = line.split(' ');
            let dest_start = line_split.next().unwrap().parse::<u64>().unwrap();
            let mut src_start = line_split.next().unwrap().parse::<u64>().unwrap();
            let range = line_split.next().unwrap().parse::<u64>().unwrap();

            for (seed_index, seed) in seeds.iter_mut().enumerate() {
                let is_seed_in_range = seed >= &mut src_start && seed < &mut (src_start + range);
                let was_seed_moved_in_current_phase = seed_moved_in_current_phase[seed_index];

                if is_seed_in_range && !was_seed_moved_in_current_phase {
                    let diff = *seed - src_start;

                    *seed = dest_start + diff;
                    seed_moved_in_current_phase[seed_index] = true;
                }
            }
        }
    });

    let mut min_location = std::u64::MAX;
    for seed_location in &seeds {
        if seed_location < &min_location {
            min_location = *seed_location;
        }
    }

    return min_location;
}
