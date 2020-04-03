use std::io;
use std::convert::TryInto;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::cmp;
use std::str::FromStr;
extern crate rand;
use rand::Rng;
use std::fmt;
const MAX_X:u8 = 15;
const MAX_Y:u8 = 15;

#[derive( Copy, Clone,Debug)]
enum Direction {
    N,S,E,W,
}

impl FromStr for Direction {
    //type Err = std::num::ParseIntError;
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "N" => Ok(Direction::N),
            "S" => Ok(Direction::S),
            "E" => Ok(Direction::E),
	    "W" => Ok(Direction::W),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
enum Action_type {
    MOVE, SURFACE, TORPEDO, SONAR, SILENCE, MINE, TRIGGER,
}


// --- coordinate
#[derive(PartialEq, Eq, Hash, Copy, Clone,Debug)]
struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    fn dist(&self, c: &Coordinate) -> u8 {
	((self.x as i8 -c.x as i8).abs() + (self.y as i8 -c.y as i8).abs()).try_into().unwrap() 
    }
}

// ----------- Action
#[derive(Debug)]
struct Action {
    ac: Action_type,
    dir: Direction,
    coord: Coordinate,
    sector: u8,
}

/*impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	    match self.ac {
		Action_type::MOVE => write!(f, "MOVE {:?}", self.coord.dir),
		Action_type::SURFACE => write!(f, "SURFACE"),
		Action_type::TORPEDO => write!(f, "TORPEDO {}, {}", self.coord.x, self.coord.y),
		Action_type::SONAR => {},
		Action_type::SILENCE => write!(f, "SILENCE"),
		Action_type::MINE => {},
		Action_type::TRIGGER => {},
	    }
    }
}*/
    
impl Action {
    fn parse_raw(st: &str) -> Vec::<Action> {
	if st == "NA" {
	    return Vec::<Action>::new();
	}

	let mut v_ret = Vec::<Action>::new();
	
	for s in st.split("|"){
	    let vec_split: Vec<&str> = s.split(" ").collect();
	    match vec_split[0]{
		"MOVE" => v_ret.push(Action {ac:Action_type::MOVE,
					     dir:vec_split[1].parse::<Direction>().unwrap(),
					     coord: Coordinate {x:0, y:0},
					     sector: 0}),
		"SURFACE" => v_ret.push(Action {ac:Action_type::SURFACE,
						dir:Direction::N,
						coord: Coordinate {x:0, y:0},
						sector: vec_split[1].parse::<u8>().unwrap()}),
		"TORPEDO" => v_ret.push(Action {ac:Action_type::TORPEDO,
						dir:Direction::N,
						coord: Coordinate {x:vec_split[1].parse::<u8>().unwrap(), y:vec_split[2].parse::<u8>().unwrap()},
						sector: 0}),
		"SONAR" => v_ret.push(Action {ac:Action_type::SONAR,
					      dir:Direction::N,
					      coord: Coordinate {x:0, y:0},
					      sector: vec_split[1].parse::<u8>().unwrap()}),
		"SILENCE" => v_ret.push(Action {ac:Action_type::SILENCE,
						dir:Direction::N,
						coord: Coordinate {x:0, y:0},
						sector: 0}),
		"MINE" => v_ret.push(Action {ac:Action_type::MINE,
					     dir:Direction::N,
					     coord: Coordinate {x:0, y:0},
					     sector: 0}),
		"TRIGGER" => v_ret.push(Action {ac:Action_type::TRIGGER,
						dir:Direction::N,
						coord: Coordinate {x:vec_split[1].parse::<u8>().unwrap(), y:vec_split[2].parse::<u8>().unwrap()},
						sector: 0}),
		_ => panic!("Bad action"),
	    }
	}
	v_ret
    }
}
//---- board


#[derive( Copy, Clone,Debug)]
struct Board {
    grid: [[u8; MAX_X as usize]; MAX_Y as usize]
	
}

impl Board {
    fn get_e(&self, c: &Coordinate) -> u8 {
	self.grid[c.x as usize][c.y as usize]
    }

    fn set_visited(&mut self, c: &Coordinate) {
	self.grid[c.x as usize][c.y as usize] = 1
    }

    fn rem_visited(&mut self) {
	for x in 0..15 {
	    for y in 0..15 {
		if self.grid[x as usize][y as usize] == 1 {
		    self.grid[x as usize][y as usize] = 0
		}
	    }
	}
    }
    fn new(v: &Vec::<String>) -> Board {
	let mut r :[[u8; MAX_X as usize]; MAX_Y as usize] = [[0;MAX_X as usize];MAX_Y as usize];
	for (idx,st) in v.iter().enumerate() {
	    for (jdx,c) in st.chars().enumerate() { 
		if c == '.' {
		    r[jdx][idx] = 0
		}
		else {
		    r[jdx][idx] = 10
		}
		    
	    }
	}	
	Board {grid:r}
    }

    //return the coord following the direction if correct
    fn check_dir(&self, c: &Coordinate, dir: &Direction) -> Option<Coordinate>  {
	let mut xl = c.x as i8;
	let mut yl = c.y as i8;


	match dir {
	    Direction::N => yl -= 1,
	    Direction::S => yl += 1,
	    Direction::W => xl -= 1, 
	    Direction::E => xl += 1,    
	}

	if xl < 0 || xl >= MAX_X as i8|| yl < 0 || yl >= MAX_Y as i8 || self.get_e(&Coordinate {x:xl as u8, y:yl as u8}) != 0 {
	    None
	}
	else {
	    Some(Coordinate {x:xl as u8, y:yl as u8})
	}
	
    }

    fn _rec_num_pos(&self, cur_pos :&Coordinate, hist :&mut HashSet::<Coordinate>) -> u8 {
	hist.insert(*cur_pos);
	let mut sum_a = 1;
	for d in &[Direction::N, Direction::S, Direction::W, Direction::E]{
	    match self.check_dir(cur_pos, d) {
		Some(c_valid) => {
		    if !hist.contains(&c_valid) {
			sum_a += self._rec_num_pos(&c_valid, hist);
		    }
		}
		None    => continue,
	    }
	  
	}
	sum_a
    }
    fn num_avail_pos(&self, cur_pos :&Coordinate) -> [(u8,Direction); 4] {
	let mut arr:[(u8,Direction); 4] = [(0,Direction::N) ;4];
	
	for (idx,d) in [Direction::N, Direction::S, Direction::W, Direction::E].iter().enumerate(){
	    match self.check_dir(cur_pos, d) {
		Some(c_valid) => arr[idx] = (self._rec_num_pos(&c_valid, &mut HashSet::<Coordinate>::new()), *d),
		None    => arr[idx] = (0, *d)
	    }
    
	}
	arr.sort_by_key(|k| Reverse(k.0));
	arr
    }


    fn initial_position(&self) -> Coordinate {
	loop {
	    let numx = rand::thread_rng().gen_range(0, 15);
	    let numy = rand::thread_rng().gen_range(0, 15);
	    
	    if self.get_e(&Coordinate {x:numx as u8, y:numy as u8}) == 0 {
		let c = Coordinate {x:numx as u8,y:numy as u8};
	    	break c
	    }

	}
    }
}


// ------------------------- predictor ------------------
#[derive(Debug)]
struct Path {
    path_coords: Vec::<Vec<Coordinate>>,
    board: Board,
}

impl Path {
    fn new(board: Board) -> Path{
	return Path { path_coords:Vec::<Vec<Coordinate>>::new(),board: board}
    }

    fn process_torpedo(&mut self, co_t :Coordinate) {
	eprintln!("Process torpedo");

	self.path_coords.retain(|ve| {ve.last().unwrap().dist(&co_t) <= 4});
    }

    fn process_surface(&mut self, sector :u8) {
	eprintln!("Process surface");
	let mut rx:u8 = 0;
	let mut ry:u8 = 0;
	match sector {
	    1 => {rx=0; ry=0},
            2 => {rx=5; ry= 0},
            3 => {rx=10; ry= 0},
            4 => {rx=0; ry = 5},
            5 => {rx=5; ry = 5},
            6 => {rx=10; ry = 5},
            7 => {rx=0; ry = 10},
            8 => {rx=5; ry = 10}, 
            9 => {rx=10; ry= 10},
	    _ => panic!("Bad sector"),
	}
	self.path_coords.retain(|ve| {
	    let mut find = false;
	    
	    for x in rx..(rx + 5){
		for y in ry..(ry + 5){
                    if ve.last().unwrap() == &(Coordinate {x:x, y:y}) {
			find = true;
		    }
		}
		if find {
		    break;
		}
	    }
	    find
	})
    }
    
    fn process_silence(&mut self) {
	eprintln!("Process SILENCE");
	let max_search:u8 = 20;
	let mut p_coords_l = Vec::<Vec<Coordinate>>::new();

	let mut hist_set = HashMap::<Coordinate, u8>::new();
	
	for v in self.path_coords.iter() {
	    //add new possible coord for each paths
	    //adv can make a 0 move

	    let count = hist_set.entry(*v.last().unwrap()).or_insert(0);
	    *count += 1;
	    if hist_set.get(v.last().unwrap()).unwrap() < &max_search {
		p_coords_l.push(v.to_vec());
	    }

	   
	    
	    for d in [Direction::N, Direction::S, Direction::W, Direction::E].iter() {
		
		let mut cur_path:Vec::<Coordinate> = v.to_vec();
		let mut cur_pos:Coordinate = *cur_path.last().unwrap();
		
		for i in 1..5 {
		    match self.board.check_dir(&cur_pos, &d) {
			Some(c_valid) =>
			{
			    if !cur_path.contains(&c_valid) {
				//if c_valid is in v, this means a cross between path -> invalid
				
				cur_path.push(c_valid);

				let count = hist_set.entry(c_valid).or_insert(0);
				*count += 1;
				if hist_set.get(&c_valid).unwrap() < &max_search {
				    p_coords_l.push(cur_path.to_vec()); //explicit copy
				}
				
				cur_pos = c_valid;

			    }
			},
			None    => break,
		    }
		     
		}
	    }
	}
	self.path_coords = p_coords_l ;
    }
    
    fn process_move(&mut self, d: Direction) {
	eprintln!("Process MOVE");
	
	if self.path_coords.is_empty() {
	    for x in 0..MAX_X {
		for y in 0..MAX_Y {
		    self.path_coords.push(vec![Coordinate {x:x, y:y}]);
		}
	    }
	}
	else {
	    for p in self.path_coords.iter_mut() {
		match self.board.check_dir(p.last().unwrap(), &d) {
		    Some(c_valid) => {
			p.push(c_valid);
		    }
		
		    None    => p.clear(),
		}
	    }
	    //remove all element empty
	    self.path_coords.retain(|ve| !ve.is_empty())
	}
    }	    
}
#[derive(Debug)]
struct Predictor {
    path: Path,
    op_life: Vec::<u8>,
    cur_co: Coordinate,
    play_board: Board,
    my_life: u8,
    torpedo :u8,
    silence :u8,
    sonar :u8,
    mine: u8,
}

impl  Predictor  {
    fn new(board: Board) -> Predictor{
	return Predictor {path: Path::new(board), op_life:Vec::<u8>::new(), cur_co: Coordinate {x:0,y:0}, my_life:0, play_board:board, torpedo:0, silence:0, sonar:0, mine:0};
    }

    //to do dont print!
    fn get_actions_to_play(&mut self) {

	let mut add_str:String = "".to_string();
	match self.get_possible_pos() {
	    None =>  eprintln!("Actio: no enought confidence"),
            Some(coord) => {
		if coord.dist(&self.cur_co) <=4 {
		    add_str = format!("TORPEDO {} {}|",coord.x, coord.y);
		    self.torpedo = 0;
		}
	    },
	}
	let e = self.play_board.num_avail_pos(&self.cur_co);
	if e[0].0 != 0 {

	    if self.silence == 6 {
		println!("{}SILENCE {:?} 1", add_str,e[0].1);
		self.silence = 0;
	    }
	    
	    else if self.torpedo < 3 {
		println!("{}MOVE {:?} TORPEDO", add_str,e[0].1);
		self.torpedo += 1;
		self.torpedo = cmp::min(self.torpedo,3);
	    }
	    else {
		println!("{}MOVE {:?} SILENCE", add_str,e[0].1);
		self.silence += 1;
		self.silence = cmp::min(self.torpedo,6);
	    }
	}
	else {
	    self.play_board.rem_visited();
	    println!("{}SURFACE",add_str)
	}
    }
    fn update_situation(&mut self,opp_life:u8, my_life:u8, x:u8, y:u8) {
	self.op_life.push(opp_life);
	self.cur_co = Coordinate {x:x,y:y};
	self.play_board.set_visited(&self.cur_co);
	self.my_life = my_life;
    }
    fn process_adv_action(&mut self, v_act:Vec<Action>) {
	for a in v_act {
	    match a.ac {
		Action_type::MOVE => self.path.process_move(a.dir),
		Action_type::SURFACE => self.path.process_surface(a.sector),
		Action_type::TORPEDO =>  self.path.process_torpedo(a.coord),
		Action_type::SONAR => {},
		Action_type::SILENCE => self.path.process_silence(),
		Action_type::MINE => {},
		Action_type::TRIGGER => {},
	    }
	    
	}
    }
    fn get_possible_pos(&self) ->  Option<Coordinate> {
	let mut set = HashSet::<Coordinate>::new();
	
	for v in self.path.path_coords.iter() {
	    set.insert(*v.last().unwrap());
	}

	
	eprintln!("Num possible path {}", self.path.path_coords.len());
	eprintln!("Num possible coord {}", set.len());

	let mut xm:f32 = -1.0;
	let mut ym:f32 = -1.0;
	    
	for el in &set {
	    if xm < 0.0 {
		xm = el.x as f32;
		ym = el.y as f32;
	    }
	    else {
		xm += el.x as f32;
		ym += el.y as f32;
	    }
	    
	}

	xm /= set.len() as f32;
	ym /= set.len() as f32;

	let round_coord = Coordinate {x:xm.round() as u8, y:ym.round() as u8};

	
	eprintln!("round {:?}", round_coord);
	if set.len() < 20
	{
	    for p in &set
	    {
		eprintln!("{:?}",p);
	    }
	}

	if set.len() < 10 {
	    eprintln!("inf 10, on est ~sur");
	    Some(round_coord)
	}
	else {
	    None
	}
	   

    }
}
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {

    let mut vec = Vec::<String>::new();
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);
    let my_id = parse_input!(inputs[2], i32);
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_end().to_string();
	vec.push(line);

    }

	
    let board = Board::new(&vec);  //ok dont use now board because value are copied on predictor
    let mut predictor = Predictor::new(board);
 

    let st = board.initial_position();
    println!("{} {}",st.x,st.y);
    
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let my_life = parse_input!(inputs[2], i32);
        let opp_life = parse_input!(inputs[3], i32);
        let torpedo_cooldown = parse_input!(inputs[4], i32);
        let sonar_cooldown = parse_input!(inputs[5], i32);
        let silence_cooldown = parse_input!(inputs[6], i32);
        let mine_cooldown = parse_input!(inputs[7], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let sonar_result = input_line.trim().to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let opponent_orders = input_line.trim_end().to_string();

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

	predictor.update_situation(opp_life as u8, my_life as u8, x as u8, y as u8);
	predictor.process_adv_action(Action::parse_raw(&opponent_orders));
	predictor.get_possible_pos();

	predictor.get_actions_to_play();

    }
}
