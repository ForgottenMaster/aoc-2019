const INPUT: &'static str = r#"
    <x=-4, y=-14, z=8>
    <x=1, y=-8, z=10>
    <x=-15, y=2, z=1>
    <x=-17, y=-17, z=16>
"#;

const TIME_STEPS: usize = 1000;

mod algebra {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Vector3 {
        x: i32,
        y: i32,
        z: i32
    }
    
    impl Vector3 {
        pub fn new(x: i32, y: i32, z: i32) -> Self {
            Self { x, y, z }
        }
        
        pub fn x(&self) -> i32 {
            self.x
        }
        
        pub fn y(&self) -> i32 {
            self.y
        }
        
        pub fn z(&self) -> i32 {
            self.z
        }
        
        pub fn sum_absolute(&self) -> i32 {
            self.x.abs() + self.y.abs() + self.z.abs()
        }
    }
    
    impl std::ops::AddAssign for Vector3 {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }
}

mod physics {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Position(super::algebra::Vector3);
    
    impl Position {
        pub fn new(x: i32, y: i32, z: i32) -> Self {
            Self(super::algebra::Vector3::new(x, y, z))
        }
        
        pub fn x(&self) -> i32 {
            self.0.x()
        }
        
        pub fn y(&self) -> i32 {
            self.0.y()
        }
        
        pub fn z(&self) -> i32 {
            self.0.z()
        }
        
        pub fn sum_absolute(&self) -> i32 {
            self.0.sum_absolute()
        }
    }
    
    impl std::ops::AddAssign<&Velocity> for Position {
        fn add_assign(&mut self, rhs: &Velocity) {
            self.0 += rhs.into();
        }
    }
    
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Velocity(super::algebra::Vector3);
    
    impl Velocity {
        pub fn new(x: i32, y: i32, z: i32) -> Self {
            Self(super::algebra::Vector3::new(x, y, z))
        }
        
        pub fn sum_absolute(&self) -> i32 {
            self.0.sum_absolute()
        }
        
        pub fn x(&self) -> i32 {
            self.0.x()
        }
        
        pub fn y(&self) -> i32 {
            self.0.y()
        }
        
        pub fn z(&self) -> i32 {
            self.0.z()
        }
    }
    
    impl std::ops::AddAssign<VelocityDelta> for Velocity {
        fn add_assign(&mut self, rhs: VelocityDelta) {
            self.0 += rhs.into();
        }
    }
    
    impl std::convert::Into<super::algebra::Vector3> for &Velocity {
        fn into(self) -> super::algebra::Vector3 {
            self.0.clone()
        }
    }
    
    #[derive(Debug)]
    pub struct VelocityDelta(super::algebra::Vector3);
    
    impl VelocityDelta {
        pub fn new(x: i32, y: i32, z: i32) -> Self {
            Self(super::algebra::Vector3::new(x, y, z))
        }
    }
    
    impl std::convert::Into<super::algebra::Vector3> for VelocityDelta {
        fn into(self) -> super::algebra::Vector3 {
            self.0
        }
    }
}

mod astronomy {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Moon {
        position: super::physics::Position,
        velocity: super::physics::Velocity
    }
    
    impl Moon {
        pub fn new(x: i32, y: i32, z: i32) -> Self {
            let position = super::physics::Position::new(x, y, z);
            let velocity = super::physics::Velocity::new(0, 0, 0);
            Self { position, velocity }
        }
        
        pub fn determine_velocity_deltas_between(first: &Self, second: &Self) -> (super::physics::VelocityDelta, super::physics::VelocityDelta) {
            let test_func = |first, second| {
                if first < second {
                    1
                } else if first > second {
                    -1
                } else {
                    0
                }
            };
            (
                super::physics::VelocityDelta::new(
                    test_func(first.position.x(), second.position.x()),
                    test_func(first.position.y(), second.position.y()),
                    test_func(first.position.z(), second.position.z())
                ),
                super::physics::VelocityDelta::new(
                    test_func(second.position.x(), first.position.x()),
                    test_func(second.position.y(), first.position.y()),
                    test_func(second.position.z(), first.position.z())
                )
            )
        }
        
        pub fn apply_velocity(&mut self) {
            self.position += &self.velocity;
        }
        
        fn potential_energy(&self) -> i32 {
            self.position.sum_absolute()
        }
        
        fn kinetic_energy(&self) -> i32 {
            self.velocity.sum_absolute()
        }
        
        pub fn total_energy(&self) -> i32 {
            self.potential_energy() * self.kinetic_energy()
        }
        
        fn get_x_cross_section(&self) -> (i32, i32) {
            (self.position.x(), self.velocity.x())
        }
        
        fn get_y_cross_section(&self) -> (i32, i32) {
            (self.position.y(), self.velocity.y())
        }
        
        fn get_z_cross_section(&self) -> (i32, i32) {
            (self.position.z(), self.velocity.z())
        }
    }
    
    impl std::ops::AddAssign<super::physics::VelocityDelta> for Moon {
        fn add_assign(&mut self, rhs: super::physics::VelocityDelta) {
            self.velocity += rhs;
        }
    }
    
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct MoonSet {
        moons: Vec<Moon>
    }
    
    impl MoonSet {
        pub fn new(moons: Vec<Moon>) -> Self {
            Self { moons }
        }
        
        pub fn apply_time_step(&mut self) {
            // update velocity due to gravity.
            let count = self.moons.len();
            for first in 0..count-1 {
                for second in first+1..count {
                    let (first_update, second_update) = Moon::determine_velocity_deltas_between(&self.moons[first], &self.moons[second]);
                    self.moons[first] += first_update;
                    self.moons[second] += second_update;
                }
            }
            
            // update position due to velocity.
            for moon in self.moons.iter_mut() {
                moon.apply_velocity();
            }
        }
        
        pub fn total_energy(&self) -> i32 {
            self.moons.iter().map(|moon| moon.total_energy()).sum()
        }
        
        pub fn get_x_cross_section(&self) -> MoonSetCrossSection {
            MoonSetCrossSection::new(self.moons.iter().map(|moon| {
                moon.get_x_cross_section()
            }).collect())
        }
        
        pub fn get_y_cross_section(&self) -> MoonSetCrossSection {
            MoonSetCrossSection::new(self.moons.iter().map(|moon| {
                moon.get_y_cross_section()
            }).collect())
        }
        
        pub fn get_z_cross_section(&self) -> MoonSetCrossSection {
            MoonSetCrossSection::new(self.moons.iter().map(|moon| {
                moon.get_z_cross_section()
            }).collect())
        }
    }
    
    /// Represents a cross section of the moon positions and velocities.
    /// This is just a wrapper around a vector (entry per moon) of pairs
    /// where each pair contains the position and velocity component in
    /// question.
    #[derive(Eq, PartialEq)]
    pub struct MoonSetCrossSection(Vec<(i32, i32)>);
    
    impl MoonSetCrossSection {
        fn new(inner: Vec<(i32, i32)>) -> Self {
            Self(inner)
        }
    }
}

mod parsing {
    pub struct MoonParser {
        moon: super::astronomy::Moon
    }
    
    impl MoonParser {
        fn new(moon: super::astronomy::Moon) -> Self {
            Self { moon }
        }
        
        pub fn unwrap(self) -> super::astronomy::Moon {
            self.moon
        }
    }
    
    impl std::str::FromStr for MoonParser {
        type Err = &'static str;
        
        fn from_str(string: &str) -> Result<Self, Self::Err> {
            let string = string.trim();
            if string.starts_with("<") {
                if string.ends_with(">") {
                    let string = &string[1..string.len()-1];
                    let splits = string
                        .split(",")
                        .flat_map(|part| part.trim().split("=").map(|inner| inner.trim()))
                        .collect::<Vec<_>>();
                    if splits.len() == 6 {
                        if splits[0] == "x" {
                            if splits[2] == "y" {
                                if splits[4] == "z" {
                                    if let Ok(x) = splits[1].parse::<i32>() {
                                        if let Ok(y) = splits[3].parse::<i32>() {
                                            if let Ok(z) = splits[5].parse::<i32>() {
                                                Ok(Self::new(super::astronomy::Moon::new(x, y, z)))
                                            } else {
                                                Err("MoonParser input format requires z to be a valid number")
                                            }
                                        } else {
                                            Err("MoonParser input format requires y to be a valid number")
                                        }
                                    } else {
                                        Err("MoonParser input format requires x to be a valid number")
                                    }
                                } else {
                                    Err("MoonParser input format requires z component third")
                                }
                            } else {
                                Err("MoonParser input format requires y component second")
                            }
                        } else {
                            Err("MoonParser input format requires x component first")
                        }
                    } else {
                        Err("MoonParser input format requires three parts, each with an equals symbol")
                    }
                } else {
                    Err("MoonParser input format requires a '>' as the last character")
                }
            } else {
                Err("MoonParser input format requires a '<' as the first character")
            }
        }
    }
    
    pub struct MoonSetParser {
        moon_set: super::astronomy::MoonSet
    }
    
    impl MoonSetParser {
        fn new(moon_set: super::astronomy::MoonSet) -> Self {
            Self { moon_set }
        }
    
        pub fn unwrap(self) -> super::astronomy::MoonSet {
            self.moon_set
        }
    }
    
    impl std::str::FromStr for MoonSetParser {
        type Err = &'static str;
        
        fn from_str(string: &str) -> Result<Self, Self::Err> {
            let mut moon_set: Vec<super::astronomy::Moon> = Vec::new();
            for line in string.trim().split("\n") {
                moon_set.push(line.trim().parse::<MoonParser>()?.unwrap())
            }
            Ok(Self::new(super::astronomy::MoonSet::new(moon_set)))
        }
    }
}

fn greatest_common_divisor(first: usize, second: usize) -> usize {
    // the greatest common divisor must be equal or lower than
    // the lowest of the two provided numbers. If it were higher
    // it would not divide into it.
    let lowest = std::cmp::min(first, second);
    for i in (1..=lowest).rev() {
        if ((first % i) == 0) && ((second % i) == 0) {
            return i
        }
    }
    0
}

fn lowest_common_multiple(first: usize, second: usize) -> usize {
    let gcd = greatest_common_divisor(first, second);
    (first / gcd) * second
}

fn main() {
    // get the moon set.
    let mut moons = INPUT.parse::<parsing::MoonSetParser>().unwrap().unwrap();
    
    // in order to be more efficient with determining how many time steps are
    // required to bring the moons into a previous state we can leverage the fact
    // that the three axes are updated completely independently. This lets us
    // determine the required time steps to bring each single axis back to its
    // initial state and we can then calculate from that the lowest common multiplier
    // of the three periods.
    let initial_x_cross_section = moons.get_x_cross_section();
    let initial_y_cross_section = moons.get_y_cross_section();
    let initial_z_cross_section = moons.get_z_cross_section();
    
    let mut x_repeat_time_step: i32 = -1;
    let mut y_repeat_time_step: i32 = -1;
    let mut z_repeat_time_step: i32 = -1;
    let mut time_steps_passed = false;
    
    // iterate "forever"
    for i in 1.. {
        // apply the time step
        moons.apply_time_step();
        
        // check if the time step is the required one to measure total energy
        if i == TIME_STEPS {
            println!("After {} time steps, the total energy in the system is {}", TIME_STEPS, moons.total_energy());
            time_steps_passed = true;
        }
        
        // if the x repeat time is less than 0, and the current cross-section equals initial, set the
        // x repeat time step
        if (x_repeat_time_step < 0) && (moons.get_x_cross_section() == initial_x_cross_section) {
            x_repeat_time_step = i as i32;
        }
        
        // if the y repeat time is less than 0, and the current cross-section equals initial, set the
        // y repeat time step
        if (y_repeat_time_step < 0) && (moons.get_y_cross_section() == initial_y_cross_section) {
            y_repeat_time_step = i as i32;
        }
        
        // if the z repeat time is less than 0, and the current cross-section equals initial, set the
        // z repeat time step
        if (z_repeat_time_step < 0) && (moons.get_z_cross_section() == initial_z_cross_section) {
            z_repeat_time_step = i as i32;
        }
        
        if time_steps_passed && (x_repeat_time_step >= 0) && (y_repeat_time_step >= 0) && (z_repeat_time_step >= 0) {
            break;
        }
    }
    
    // now we have the orbital periods of the moons on the various axes, we can determine the timestep at which
    // the system will be back to its initial state by finding the lowest common multiple of the three
    let confluence_time_step = lowest_common_multiple(lowest_common_multiple(x_repeat_time_step as usize, y_repeat_time_step as usize), z_repeat_time_step as usize);
    println!("The system will return to its initial state after {} time steps", confluence_time_step);
}
