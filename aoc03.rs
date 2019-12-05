// input for the two wires.
const WIRE_1_INPUT: &'static str = "R1002,U407,R530,D268,R516,U937,L74,U838,R784,D684,L912,U746,R189,U192,R868,D345,L972,D492,R942,U631,L559,U634,L80,U513,L746,D997,L348,D160,L655,U949,R717,U396,R549,D167,R591,U469,L22,U977,L167,D856,L320,D920,L396,U490,L895,U180,R661,D828,R864,U189,R307,U402,R409,U445,L101,D418,R812,U419,R319,U75,L813,D46,L491,U39,R737,U11,R177,U311,L278,U254,R475,U166,L515,D105,L694,D437,L298,U169,L613,D234,L999,U380,L711,D758,R932,D27,L951,D529,L935,D189,R816,D176,R98,D320,R965,D333,L367,U622,R18,U83,R275,D205,L960,U177,R750,D466,R442,U797,R355,D717,L569,D578,R384,U863,R541,U405,R527,D658,L514,U168,L64,D918,R947,D11,L189,D875,R599,U201,L165,U772,L679,U566,L195,U660,R896,D622,R678,U390,L984,D900,R889,D714,R557,U848,L176,U541,R518,D699,L904,D23,L55,D886,L206,D621,L48,D197,R502,D259,L779,D72,L183,U747,L424,U452,L603,U561,L430,D942,R515,D378,R962,U508,R230,D650,R804,D453,R899,D813,R484,U798,R456,D231,L316,U117,R630,D436,L985,D283,L393,D370,R158,U957,L914,D455,L875,U536,R889,U400,R347,U712,R487,D455,R428,U590,R127,D132,L202,U377,R138,U654,L760,D46,R213,D225,L817,U455,L612,U543,L525,U979,R591,D940,R446,U786,R750,U244,R325,U928,L44,U551,L955,U221,L986,U516,L916,D242,L280,D71,R80,U849,L271,U626,R737,D646,R82,U120,R646,U569,R463,D94,R570,U456,L548,D687,R221,D759,L606,D818,L859,U218,R682,U299,R818,D966,R407,U605,L859,D378,L53,D722,L216,D221,R639,U485,L865,D620,R99,D988,R944,D323,R540,U372,L470,U106,L804,D92,L177,U518,R277,U670,R451,D194,L695,D502,L601,U596,R374,U682,L19,D54,L156";
const WIRE_2_INPUT: &'static str = "L1003,U22,R594,D241,L215,D906,R733,D831,L556,U421,L780,D242,R183,U311,R46,D52,R124,D950,L18,U985,R999,D528,R850,U575,L138,D62,L603,U467,R641,U155,L165,D63,L489,U4,R635,D460,L446,D938,R983,U494,L491,D433,L722,U427,L469,U832,L712,U798,R906,U804,R646,U721,R578,D194,L726,U803,L985,D934,R943,U198,R726,U341,R583,U998,L992,D401,L132,D681,L363,U949,L814,D977,R840,D145,L177,D291,L652,D396,R330,D951,L363,U813,R847,D374,R961,D912,R516,D178,R495,U49,R340,D395,R632,D991,R487,D263,R320,D948,R456,D142,L783,D888,R589,D999,L159,U686,R402,D586,L425,U946,R56,D979,L534,U313,R657,D401,L666,D504,R712,D232,L557,D620,R193,D670,L134,D975,R837,D901,R813,U459,L499,U450,L87,U84,L582,U310,R795,D280,L730,D458,L727,D196,R95,U210,R498,U760,R778,U325,R715,U479,R275,U557,L450,D196,L60,U115,R475,D265,L611,D372,R60,U935,L717,U809,L344,D854,L386,U473,R72,U968,L816,U900,R866,U172,R965,U383,R576,D774,R753,U788,L781,D237,L401,U786,R873,U331,R609,D232,L603,U685,L494,U177,L982,D173,L673,U772,L7,U7,R517,U573,R212,D413,L124,D810,L223,U137,L576,U95,R128,U896,L91,U932,L875,U917,R106,U911,L208,D507,L778,D59,L71,D352,R988,U708,L58,D429,L122,U771,L713,D801,R188,U661,R752,D374,R312,D848,L504,D540,R334,U517,R343,D739,L727,D552,L555,U580,L857,U474,R145,U188,L789,U698,R735,U131,L494,U162,L27,D849,L140,D849,R615,U798,R160,U492,R884,U521,L542,D729,R498,D853,R918,U565,R65,U32,L607,U552,L38,D822,L77,U490,L190,D93,L104,U268,R702,D112,L917,D876,L631,D139,L989,U810,R329,U253,L498,D767,L550,U666,L549,U616,R376";

//-----------------------------------------------------------------------------------------------------------------------------------

// Provide a nice wrapper for infallible parses which will do the appropriate
// levels of unwrapping and just parse as a T rather than a Result.
trait StrRefExtensions { 
    fn infallible_parse<T: std::str::FromStr<Err=std::convert::Infallible>>(self) -> T;
}

// implement the extensions for &str so we can use it.
impl StrRefExtensions for &str {
    fn infallible_parse<T: std::str::FromStr<Err=std::convert::Infallible>>(self) -> T {
        self.parse::<T>()
            .ok()
            .unwrap()
    }
}

//------------------------------------------------------------------------------------------------------------------------------------

// The direction in which the segment is moving.
#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

// Implement FromStr for Direction to map a character direction to a variant.
impl std::str::FromStr for Direction {
    type Err = std::convert::Infallible;
    
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => panic!()
        }
    }
}

//------------------------------------------------------------------------------------------------------------------------------------

// The raw unprocessed wire segment, this is just a direction and
// distance and will be from the previous coordinates.
#[derive(Debug)]
struct UnprocessedSegment(Direction, i32);

impl UnprocessedSegment {
    fn delta_coordinate(&self) -> Coordinate {
        match self.0 {
            Direction::Left => Coordinate::new(-self.1, 0),
            Direction::Right => Coordinate::new(self.1, 0),
            Direction::Up => Coordinate::new(0, self.1),
            Direction::Down => Coordinate::new(0, -self.1)
        }
    }

    fn process_from(&self, from: &ProcessedSegment) -> ProcessedSegment {
        self.process_from_coordinate(from.to()) // process this direction applied to the endpoint of the processed segment
    }
    
    fn process_from_coordinate(&self, from: &Coordinate) -> ProcessedSegment {
        let to = from + &self.delta_coordinate();
        ProcessedSegment::new((*from).clone(), to)
    }
}

// implement FromStr for UnprocessedSegment so we can convert a component into a struct.
impl std::str::FromStr for UnprocessedSegment {
    type Err = std::convert::Infallible;
    
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (direction, size) = string.split_at(1);
        Ok(Self(direction.infallible_parse(), size.parse().unwrap()))
    }
}

//------------------------------------------------------------------------------------------------------------------------------------

// The raw unprocessed wire, which is just a thin wrapper around
// a vector of segments.
struct UnprocessedWire(Vec<UnprocessedSegment>);

impl UnprocessedWire {
    fn as_processed(&self, starting_coordinate: &Coordinate) -> ProcessedWire {
        let mut processed_segments = Vec::with_capacity(self.0.len());
        processed_segments.push(self.0[0].process_from_coordinate(starting_coordinate)); // seed the initial from raw coordinate.
        for i in 1..self.0.len() {
            processed_segments.push(self.0[i].process_from(&processed_segments[i-1]));
        }
        ProcessedWire::new(processed_segments)
    }
}

// implement FromStr for UnprocessedWire so we can easily convert the comma-separated input into
// a vector of segments.
impl std::str::FromStr for UnprocessedWire {
    type Err = std::convert::Infallible;
    
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self(string.split(",").map(|segment| segment.infallible_parse()).collect()))
    }
}

//-----------------------------------------------------------------------------------------------------------------------------------

// a coordinate which is just a pair of integers.
#[derive(Clone, Debug, PartialEq)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
    
    fn x(&self) -> i32 {
        self.0
    }
    
    fn y(&self) -> i32 {
        self.1
    }
    
    fn manhattan(&self, other: &Self) -> u32 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as u32
    }
}

impl std::ops::Add for &Coordinate {
    type Output = Coordinate;
    
    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.0 + other.0, self.1 + other.1)
    }
}

//-----------------------------------------------------------------------------------------------------------------------------------

// a processed version of a segment, which is just a line between two coordinates.
#[derive(Debug)]
struct ProcessedSegment(Coordinate, Coordinate);

impl ProcessedSegment {
    fn new(from: Coordinate, to: Coordinate) -> Self {
        Self(from, to)
    }
    
    fn from(&self) -> &Coordinate {
        &self.0
    }
    
    fn to(&self) -> &Coordinate {
        &self.1
    }
    
    fn is_horizontal(&self) -> bool {
        self.from().x() != self.to().x() // horizontal if the x component is different from and to
    }
    
    fn intersect(&self, other: &Self) -> Option<Coordinate> {
        let self_horizontal = self.is_horizontal();
        let other_horizontal = other.is_horizontal();
        if self_horizontal == other_horizontal {
            None
        } else {
            let horizontal = if self_horizontal { self } else { other };
            let vertical = if self_horizontal { other } else { self };
            let (left_x, right_x, central_x) = (std::cmp::min(horizontal.from().x(), horizontal.to().x()), std::cmp::max(horizontal.from().x(), horizontal.to().x()), vertical.from().x());
            let (top_y, bottom_y, central_y) = (std::cmp::max(vertical.from().y(), vertical.to().y()), std::cmp::min(vertical.from().y(), vertical.to().y()), horizontal.from().y());
            
            if (left_x < central_x) && (right_x > central_x) && (top_y > central_y) && (bottom_y < central_y) {
                Some(Coordinate::new(central_x, central_y))
            } else {
                None
            }
        }
    }
    
    fn steps(&self) -> u32 {
        ((self.from().x() - self.to().x()).abs() + (self.from().y() - self.to().y()).abs()) as u32
    }
    
    fn steps_to(&self, coordinate: &Coordinate) -> Option<u32> {
        let (min_x, max_x, coord_x) = (std::cmp::min(self.from().x(), self.to().x()), std::cmp::max(self.from().x(), self.to().x()), coordinate.x());
        let (min_y, max_y, coord_y) = (std::cmp::min(self.from().y(), self.to().y()), std::cmp::max(self.from().y(), self.to().y()), coordinate.y());
        
        let intersects = ((min_x < coord_x) && (max_x > coord_x) && (min_y == coord_y)) || ((min_y < coord_y) && (max_y > coord_y) && (min_x == coord_x));
        
        if intersects {
            Some(((self.from().x() - coord_x).abs() + (self.from().y() - coord_y).abs()) as u32)
        } else {
            None
        }
    }
}

//-----------------------------------------------------------------------------------------------------------------------------------

// a processed version of a wire which is a wrapper around a sequence of segments.
#[derive(Debug)]
struct ProcessedWire(Vec<ProcessedSegment>);

impl ProcessedWire {
    fn new(segments: Vec<ProcessedSegment>) -> Self {
        Self(segments)
    }
    
    fn intersections(&self, other: &Self) -> Vec<Coordinate> {
        let mut results = Vec::new();
        for segment_1 in &self.0 {
            for segment_2 in &other.0 {
                if let Some(coordinate) = segment_1.intersect(segment_2) {
                    results.push(coordinate);
                }
            }
        }
        results
    }
    
    fn steps_to(&self, coordinate: &Coordinate) -> u32 {
        let mut steps = 0;
        for segment in &self.0 {
            if let Some(s) = segment.steps_to(coordinate) {
                steps += s;
                break;
            } else {
                steps += segment.steps();
            }
        }
        steps
    }
}

//-----------------------------------------------------------------------------------------------------------------------------------

fn main() {
    // convert the input into unprocessed wires (sequence of direction/distances).
    let wire_1 = WIRE_1_INPUT.infallible_parse::<UnprocessedWire>();
    let wire_2 = WIRE_2_INPUT.infallible_parse::<UnprocessedWire>();
    
    // the coordinate of the central box.
    let central = Coordinate::new(0, 0);
    
    // map the unprocessed wires into processed wires (segments have fixed coordinates).
    let wire_1 = wire_1.as_processed(&central);
    let wire_2 = wire_2.as_processed(&central);
    
    // determine the intersection coordinates for the wires.
    let intersections = wire_1.intersections(&wire_2);
    
    // determine the smallest manhattan distance from the central coordinate.
    let min = intersections
        .into_iter()
        .map(|coord| wire_1.steps_to(&coord) + wire_2.steps_to(&coord))
        .min()
        .unwrap();
        
    println!("Result is {}", min);
}
