use std::convert::TryFrom;

/// The program input.
const INPUT: &'static str = r#"
#.#.###.#.#....#..##.#....
.....#..#..#..#.#..#.....#
.##.##.##.##.##..#...#...#
#.#...#.#####...###.#.#.#.
.#####.###.#.#.####.#####.
#.#.#.##.#.##...####.#.##.
##....###..#.#..#..#..###.
..##....#.#...##.#.#...###
#.....#.#######..##.##.#..
#.###.#..###.#.#..##.....#
##.#.#.##.#......#####..##
#..##.#.##..###.##.###..##
#..#.###...#.#...#..#.##.#
.#..#.#....###.#.#..##.#.#
#.##.#####..###...#.###.##
#...##..#..##.##.#.##..###
#.#.###.###.....####.##..#
######....#.##....###.#..#
..##.#.####.....###..##.#.
#..#..#...#.####..######..
#####.##...#.#....#....#.#
.#####.##.#.#####..##.#...
#..##..##.#.##.##.####..##
.##..####..#..####.#######
#.#..#.##.#.######....##..
.#.##.##.####......#.##.##
"#;

/// Enumeration representing the type of space
/// on the map.
enum SpaceType {
    Asteroid,
    Empty
}

impl std::convert::TryFrom<char> for SpaceType {
    type Error = ();

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Asteroid),
            _ => Err(())
        }
    }
}

/// An iterator over the asteroids in destruction order.
/// Constructed by destruction_iter on the AsteroidMap.
struct DestructionIter {
    groups: Vec<Vec<(usize, usize)>>,
    next_group_index: usize
}

impl DestructionIter {
    fn new(groups: Vec<Vec<(usize, usize)>>) -> Self {
        let next_group_index = 0;
        Self { groups, next_group_index }
    }
}

impl std::iter::Iterator for DestructionIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.groups.len() == 0 {
            None
        } else {
            let item = self.groups[self.next_group_index].remove(0);
            if self.groups[self.next_group_index].len() == 0 {
                self.groups.remove(self.next_group_index);
            } else {
                self.next_group_index += 1;
                self.next_group_index %= self.groups.len();
            }
            Some(item)
        }
    }
}

/// Represents the entire asteroid map
/// which will just be enum values identifying
/// an empty space or an asteroid. We will store
/// this as a contiguous array for efficiency.
struct AsteroidMap {
    width: usize,
    height: usize,
    data: Box<[SpaceType]>,
    cached_asteroids: Box<[(usize, usize)]>
}

impl AsteroidMap {
    fn asteroids(&self) -> &[(usize, usize)] {
        &self.cached_asteroids
    }

    fn scan_from(&self, source: (usize, usize)) -> AsteroidScan {
        let asteroids = self
            .asteroids()
            .iter()
            .filter(|coord| **coord != source)
            .map(|(x, y)| {
                (*x, *y, *x as i32-source.0 as i32, *y as i32-source.1 as i32)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        AsteroidScan { source, asteroids }
    }

    fn scan_best(&self) -> (usize, usize, u32) {
        let mut all = self
            .asteroids()
            .into_iter()
            .map(|(x, y)| {
                (*x, *y, self
                    .scan_from((*x, *y))
                    .group_by_line()
                    .len() as u32)
            })
            .collect::<Vec<_>>();
        all.sort_by(|item1, item2| item1.2.partial_cmp(&item2.2).unwrap());
        all[all.len() - 1]
    }

    /// Gets an iterator over the asteroids in destruction order, that is starting from
    /// the upwards direction (0 degrees rotation) and rotating clockwise until all asteroids
    /// have been destroyed.
    fn destruction_iter(&self, from: (usize, usize)) -> DestructionIter {
        let grouped = self.scan_from(from).group_by_line();
        let mut groups = grouped
            .into_iter()
            .map(|(dx, dy, nodes)| {
                let mut nodes = nodes
                    .into_iter()
                    .collect::<Vec<_>>();
                nodes.sort_by(|node_1, node_2| {
                    node_1.0.partial_cmp(&node_2.0).unwrap()
                });
                let nodes = nodes.into_iter()
                    .map(|(_, x, y)| {
                        (*x, *y)
                    })
                    .collect::<Vec<_>>();
                (calculate_signed_angle((0.0, -1.0), (*dx as f32, *dy as f32)), nodes)
            })
            .collect::<Vec<_>>();
        groups.sort_by(|group_1, group_2| {
            if (group_1.0 >= 0.0) && (group_2.0 < 0.0) {
                std::cmp::Ordering::Less
            } else if (group_1.0 < 0.0) && (group_2.0 >= 0.0) {
                std::cmp::Ordering::Greater
            } else {
                group_1.0.partial_cmp(&group_2.0).unwrap()
            }
        });
        let groups = groups.into_iter()
            .map(|(angle, nodes)| nodes)
            .collect::<Vec<_>>();
        DestructionIter::new(groups)
    }
}

impl std::str::FromStr for AsteroidMap {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let height = string
            .trim()
            .split("\n")
            .filter(|line| line.trim() != "")
            .count();
        let data = string
            .trim()
            .chars()
            .map(|c| SpaceType::try_from(c))
            .filter(|c| c.is_ok())
            .map(|c| c.unwrap())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let width = data.len() / height;
        let cached_asteroids = (0..width)
            .flat_map(|x| {
                (0..height)
                    .map(move |y| {
                        (x, y)
                    })
                    .filter(|(x, y)| {
                        match data[y * width + x] {
                            SpaceType::Empty => false,
                            SpaceType::Asteroid => true
                        }
                    })
            })
            .collect::<Vec<_>>();
        let cached_asteroids = cached_asteroids.into_boxed_slice();
        Ok(Self { width, height, data, cached_asteroids })
    }
}

/// Represents an asteroid scan from a specific
/// location, this contains a list of all the asteroids
/// and their gradients.
/// Format of the asteroids list: (x, y, dx, dy)
struct AsteroidScan {
    source: (usize, usize),
    asteroids: Box<[(usize, usize, i32, i32)]>
}

impl AsteroidScan {
    /// Runs through all the asteroids in the scan and groups them
    /// by line. The results are the lines provided in parametric form.
    /// That is, each line is defined by a gradient and a time 't'.
    fn group_by_line(&self) -> Box<[(i32, i32, Box<[(f32, usize, usize)]>)]> {
        let mut groups = Vec::<(i32, i32, Vec::<(f32, usize, usize)>)>::new();
        'outer: for (x, y, dx, dy) in self.asteroids.iter() {
            for (gdx, gdy, items) in &mut groups {
                // if the gradients can be divided cleanly (as in, both)
                // dx/gdx and dy/gdy have the same value (which is t). AND
                // the t value is not negative (opposite facing), then record
                // it in the same group, and continue outer loop.
                let t = if (*gdx == 0) && (*dx == 0) {
                    *dy as f32 / *gdy as f32
                } else if (*gdy == 0) && (*dy == 0) {
                    *dx as f32 / *gdx as f32
                } else {
                    let t1 = *dx as f32 / *gdx as f32;
                    let t2 = *dy as f32 / *gdy as f32;
                    if t1 == t2 {
                        t1
                    } else {
                        0.0
                    }
                };
                if t > 0.0 {
                    items.push((t, *x, *y));
                    continue 'outer;
                }
            }

            // we only get here when we haven't found a matching group (as we 'continue other when
            // we find one). Therefore we need to make one with this coordinate as t==1
            groups.push((*dx, *dy, vec![(1.0, *x, *y)]))
        }
        
        // convert the vectors into boxed slices as their sizes now won't change.
        groups
            .into_iter()
            .map(|(dx, dy, items)| {
                (dx, dy, items.into_boxed_slice())
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

/// Given two vectors in tuple form, calculates the angle in degrees between them.
fn calculate_signed_angle(vec_1: (f32, f32), vec_2: (f32, f32)) -> f32 {
    let angle = (vec_1.0 * vec_2.1 - vec_1.1 * vec_2.0).atan2(vec_1.0 * vec_2.0 + vec_1.1 * vec_2.1);
    angle * (180.0 / std::f32::consts::PI)
}

fn main() {
    // Part 1
    let map = INPUT.parse::<AsteroidMap>().unwrap();
    let (x, y, count) = map.scan_best();
    println!("Best location is at: ({}, {}) with {} asteroids", x, y, count);

    // Part 2
    let asteroid_destruction_iterator = map.destruction_iter((x, y));
    println!("200th asteroid to be destroyed is at: {:?}", asteroid_destruction_iterator.skip(199).next().unwrap());
}
