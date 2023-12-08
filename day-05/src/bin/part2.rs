use utils;

#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end - 1)
    }
}
impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end - 1)
    }
}

fn main() {
    let output = part2("day-05/src/input.txt");
    dbg!(output);
}

fn part2(file_path: &str) -> u64 {
    let mut phase = 0;
    let mut transformed_in_current_phase: Vec<Range> = vec![];
    let mut ranges_to_proccess_in_current_phase: Vec<Range> = vec![];

    utils::process_file(file_path, |line: &str| {
        if line == "" {
        } else if line.starts_with("seeds:") {
            let mut reading_range_start = true;
            let mut range_start: u64 = 0;

            for part in line.split(' ') {
                if !part.as_bytes()[0].is_ascii_digit() {
                    continue;
                }
                let number = part.parse::<u64>().unwrap();

                if reading_range_start {
                    range_start = number;
                } else {
                    ranges_to_proccess_in_current_phase.push(Range {
                        start: range_start,
                        end: range_start + number,
                    })
                }
                reading_range_start = !reading_range_start;
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            let mut line_split = line.split(' ');
            let dest_start = line_split.next().unwrap().parse::<u64>().unwrap();
            let src_start = line_split.next().unwrap().parse::<u64>().unwrap();
            let range = line_split.next().unwrap().parse::<u64>().unwrap();

            let src_range = Range {
                start: src_start,
                end: src_start + range,
            };
            let dest_range = Range {
                start: dest_start,
                end: dest_start + range,
            };

            println!("{}: Transform: {} -> {}", phase, src_range, dest_range);

            let mut new_transformed_in_current_phase: Vec<Range> =
                (*transformed_in_current_phase).to_vec();
            let mut new_ranges_to_proccess_in_current_phase: Vec<Range> = vec![];

            println!(
                "    Untouched in phase: {:?}",
                &ranges_to_proccess_in_current_phase
            );
            for seeds_range in &ranges_to_proccess_in_current_phase {
                if seeds_range.start >= src_range.start && seeds_range.end <= src_range.end {
                    // seeds_range is inside of src_range
                    // src   v--------v  |  v--------v  |  v--------v       v--------v  |  v--------v  |  v--------v
                    // seed     ^--^     |  ^--^        |        ^--^   ->     |--|     |  |--|        |        |--|
                    let diff = seeds_range.start - src_range.start;
                    let seeds_range_length = seeds_range.end - seeds_range.start;

                    new_transformed_in_current_phase.push(Range {
                        start: dest_range.start + diff,
                        end: dest_range.start + diff + seeds_range_length,
                    });
                } else if seeds_range.start < src_range.start && seeds_range.end >= src_range.end {
                    // src_range is inside of seeds_range
                    // src       v--v                      v--v
                    // seed  ^----------^       ->     ^--^|--|^--^
                    let diff = src_range.start - seeds_range.start;
                    let seeds_range_length = src_range.end - src_range.start;

                    new_ranges_to_proccess_in_current_phase.push(Range {
                        start: seeds_range.start,
                        end: src_range.start,
                    });
                    new_transformed_in_current_phase.push(Range {
                        start: dest_range.start + diff,
                        end: dest_range.start + diff + seeds_range_length,
                    });
                    new_ranges_to_proccess_in_current_phase.push(Range {
                        start: src_range.end,
                        end: seeds_range.end,
                    });
                } else if seeds_range.end <= src_range.start || seeds_range.start > src_range.end {
                    // do not overlap
                    // src   v----v           |           v----v
                    // seed          ^----^   |   ^----^
                    new_ranges_to_proccess_in_current_phase.push(Range {
                        start: seeds_range.start,
                        end: seeds_range.end,
                    });
                } else if seeds_range.start < src_range.start && seeds_range.end < src_range.end {
                    // pertially overlap 1
                    // src       v--------v               v--------v
                    // seed   ^------^         ->     ^--^|--|
                    let diff = seeds_range.end - src_range.start;

                    new_ranges_to_proccess_in_current_phase.push(Range {
                        start: seeds_range.start,
                        end: src_range.start,
                    });
                    new_transformed_in_current_phase.push(Range {
                        start: dest_range.start,
                        end: dest_range.start + diff,
                    });
                } else if seeds_range.start >= src_range.start && seeds_range.end >= src_range.end {
                    // pertially overlap 2
                    // src    v--------v              v--------v
                    // seed       ^-------^    ->           |--|^--^
                    let diff = seeds_range.start - src_range.start;
                    let seeds_range_length = src_range.end - seeds_range.start;

                    new_transformed_in_current_phase.push(Range {
                        start: dest_range.start + diff,
                        end: dest_range.start + diff + seeds_range_length,
                    });
                    new_ranges_to_proccess_in_current_phase.push(Range {
                        start: src_range.end,
                        end: seeds_range.end,
                    });
                }
            }

            transformed_in_current_phase = new_transformed_in_current_phase;
            ranges_to_proccess_in_current_phase = new_ranges_to_proccess_in_current_phase;

            println!(
                "    New untouched in phase: {:?}",
                &ranges_to_proccess_in_current_phase
            );
            println!(
                "    New transformed in phase: {:?}",
                &transformed_in_current_phase
            );
        } else {
            if phase != 0 {
                let mut new_seeds_ranged: Vec<Range> = vec![];
                for range in &ranges_to_proccess_in_current_phase {
                    new_seeds_ranged.push(Range {
                        start: range.start,
                        end: range.end,
                    });
                }
                for range in &transformed_in_current_phase {
                    new_seeds_ranged.push(Range {
                        start: range.start,
                        end: range.end,
                    });
                }
                ranges_to_proccess_in_current_phase = (*new_seeds_ranged).to_vec();
                transformed_in_current_phase = vec![];
            }

            phase += 1;
        }
    });

    let mut min_location = std::u64::MAX;
    for range in &ranges_to_proccess_in_current_phase {
        if range.start < min_location {
            min_location = range.start;
        }
    }
    for range in &transformed_in_current_phase {
        if range.start < min_location {
            min_location = range.start;
        }
    }

    return min_location;
}
