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

impl Default for Direction {
    fn default() -> Self { Direction::N }
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
#[derive(Debug,Copy, Clone,PartialEq, Eq)]
enum Action_type {
    MOVE, SURFACE, TORPEDO, SONAR, SILENCE, MINE, TRIGGER,
}

impl Default for Action_type {
    fn default() -> Self { Action_type::MOVE }
}

// --- coordinate
#[derive(PartialEq, Eq, Hash, Copy, Clone,Debug,  Default)]
struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    fn dist(&self, c: &Coordinate) -> u8 {
	((self.x as i8 -c.x as i8).abs() + (self.y as i8 -c.y as i8).abs()).try_into().unwrap() 
    }

    fn l2_dist(&self, c: &Coordinate) -> u8 {
	(((self.x as i8 -c.x as i8).pow(2) + (self.y as i8 -c.y as i8).pow(2)) as f64).sqrt().floor() as u8
    }

    fn to_surface(&self) -> u8 {
	if self.x < 5 && self.y < 5 {1}
	else if self.x < 10 && self.y < 5 {2}
	else if self.x < 15 && self.y < 5 {3}
	else if self.x < 5 && self.y < 10 {4}
	else if self.x < 10 && self.y < 10 {5}
	else if self.x < 15 && self.y < 10 {6}
	else if self.x < 5 && self.y < 15 {7}
	else if self.x < 10 && self.y < 15 {8}
	else if self.x < 15 && self.y < 15 {9}
	else {panic!("Bad sector")}
    }
}

// ----------- Action 
#[derive(Debug, Default,Copy, Clone)]
struct Action {
    ac: Action_type,
    dir: Direction,
    coord: Coordinate,
    sector: u8,
    ac_load: Action_type, //only for load
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
    fn repr_action_v(va :&Vec::<Action>) -> String{
	let mut out:String = "".to_string();
	
	for (idx, a) in va.iter().enumerate() {
	    if idx > 0 {
		out = format!("{}|", out);
	    }
	    match a.ac {
		Action_type::MOVE => out = format!("{}MOVE {:?} {:?}",out, a.dir, a.ac_load),
		Action_type::SURFACE => out = format!("{}SURFACE", out),
		Action_type::TORPEDO => out = format!("{}TORPEDO {} {}", out, a.coord.x, a.coord.y),
		Action_type::SONAR => out = format!("{}SONAR {}", out, a.sector),
		Action_type::SILENCE => out = format!("{}SILENCE {:?} {}", out, a.dir, a.sector),
		Action_type::MINE => panic!("no mines"),
		Action_type::TRIGGER => panic!("no trig"),
	    }
	}
	out
    }
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
					     sector: 0,
					     ac_load:Action_type::TORPEDO}),
		"SURFACE" => v_ret.push(Action {ac:Action_type::SURFACE,
						dir:Direction::N,
						coord: Coordinate {x:0, y:0},
						sector: vec_split[1].parse::<u8>().unwrap(),
						ac_load:Action_type::TORPEDO}),
		"TORPEDO" => v_ret.push(Action {ac:Action_type::TORPEDO,
						dir:Direction::N,
						coord: Coordinate {x:vec_split[1].parse::<u8>().unwrap(), y:vec_split[2].parse::<u8>().unwrap()},
						sector: 0,
						ac_load:Action_type::TORPEDO}),
		"SONAR" => v_ret.push(Action {ac:Action_type::SONAR,
					      dir:Direction::N,
					      coord: Coordinate {x:0, y:0},
					      sector: vec_split[1].parse::<u8>().unwrap(),
					      ac_load:Action_type::TORPEDO}),
		"SILENCE" => v_ret.push(Action {ac:Action_type::SILENCE,
						dir:Direction::N,
						coord: Coordinate {x:0, y:0},
						sector: 0,
						ac_load:Action_type::TORPEDO}),
		"MINE" => v_ret.push(Action {ac:Action_type::MINE,
					     dir:Direction::N,
					     coord: Coordinate {x:0, y:0},
					     sector: 0,
					     ac_load:Action_type::TORPEDO}),
		"TRIGGER" => v_ret.push(Action {ac:Action_type::TRIGGER,
						dir:Direction::N,
						coord: Coordinate {x:vec_split[1].parse::<u8>().unwrap(), y:vec_split[2].parse::<u8>().unwrap()},
						sector: 0,
						ac_load:Action_type::TORPEDO}),
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
    path_coords: Vec::<(u32, Vec::<Coordinate>)>,

    board: Board,
}

impl Path {
    fn new(board: Board) -> Path{
	return Path { path_coords:Vec::<(u32, Vec::<Coordinate>)>::new(),board: board}
    }

    fn process_torpedo(&mut self, co_t :Coordinate) {
	eprintln!("Process torpedo");

	self.path_coords.retain(|(_freq, ve)| {ve.last().unwrap().dist(&co_t) <= 4});
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
	self.path_coords.retain(|(_freq,ve)| {
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

    fn _reduce_search_space(v_coord :&Vec::<(u32,Vec::<Coordinate>)>) -> Vec::<(u32,Vec::<Coordinate>)> {
	//ok reduce search space
	let mut p_coords_reduced = Vec::<(u32,Vec::<Coordinate>)>::new();
	
	let mut frequency: HashMap<&Coordinate, u32> = HashMap::new();
	
	for (freq, coord) in v_coord { 
	    *frequency.entry(coord.last().unwrap()).or_insert(0) += *freq;
	}
	//eprintln!("FREQ {:?}", frequency);
	
	//update the path
	//self.path_coords.clear();
	for (co,freq) in &frequency {
	    p_coords_reduced.push((*freq,vec![**co]));
	}
	p_coords_reduced
    }
    
    fn process_silence(&mut self) {
	eprintln!("Process SILENCE");
	let max_search:usize = 500;
	let mut p_coords_l = Vec::<(u32,Vec::<Coordinate>)>::new();

	if self.path_coords.len() > max_search {
	    eprintln!("REDUCE size before : {}", self.path_coords.len());

	    self.path_coords =  Path::_reduce_search_space(&self.path_coords);
	    
	    eprintln!("REDUCE size after : {}", self.path_coords.len());
	}
	
	for (freq,v) in self.path_coords.iter() {
	    //add new possible coord for each paths
	    //adv can make a 0 move

	    p_coords_l.push((1, v.to_vec()));
	   
	    
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

				p_coords_l.push((1,cur_path.to_vec())); //explicit copy

				
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
		    self.path_coords.push((1,vec![Coordinate {x:x, y:y}]));
		}
	    }
	}
	else {
	    for (feq,p) in self.path_coords.iter_mut() {
		match self.board.check_dir(p.last().unwrap(), &d) {
		    Some(c_valid) => {
			p.push(c_valid);
		    }
		
		    None    => p.clear(),
		}
	    }
	    //remove all element empty
	    self.path_coords.retain(|(freq, ve)| !ve.is_empty())
	}
    }

    fn process_actions(&mut self, v_act:&Vec<Action>) {
	for a in v_act {
	    match a.ac {
		Action_type::MOVE => self.process_move(a.dir),
		Action_type::SURFACE => self.process_surface(a.sector),
		Action_type::TORPEDO =>  self.process_torpedo(a.coord),
		Action_type::SONAR => {},
		Action_type::SILENCE => self.process_silence(),
		Action_type::MINE => {},
		Action_type::TRIGGER => {},
	    }   
	}
    }

        
    fn get_possible_pos(&self) ->  (usize, Coordinate) {
		
	let reduced_v = Path::_reduce_search_space(&self.path_coords);
	
	eprintln!("Num possible coord {}", reduced_v.len());
	eprintln!("Num possible path {}", self.path_coords.len());

	let mut xm:f32 = -1.0;
	let mut ym:f32 = -1.0;

	let mut tot:u32 = 0;
	for (freq, el_v) in &reduced_v {
	    let el = el_v.last().unwrap();
	    
	    if xm < 0.0 {
		xm = (*freq*el.x as u32) as f32;
		ym = (*freq*el.y as u32) as f32;
		tot += freq;
	    }
	    else {
		xm += (*freq*el.x as u32) as f32;
		ym += (*freq*el.y as u32) as f32;
		tot += *freq;
	    }
	    
	}

	xm /= tot as f32;
	ym /= tot as f32;

	let round_coord = Coordinate {x:xm.round() as u8, y:ym.round() as u8};

	
	eprintln!("round {:?}", round_coord);
	if reduced_v.len() < 20
	{
	    for (f,v_p) in &reduced_v
	    {
		eprintln!("freq : {}, val : {:?}",f, v_p.last().unwrap());
	    }
	}

	(reduced_v.len(),round_coord)
    }

}
#[derive(Debug)]
struct Predictor {
    path: Path,
    my_path: Path,
    
    op_life: Vec::<u8>,
    cur_co: Coordinate,
    play_board: Board,
    my_life: u8,
    torpedo :u8,
    silence :u8,
    sonar :u8,
    mine: u8,
    actions_issued: Vec::<Action>,
}

impl  Predictor  {
    fn new(board: Board) -> Predictor{
	return Predictor {path: Path::new(board),
			  my_path: Path::new(board),
			  op_life:Vec::<u8>::new(),
			  cur_co: Coordinate {x:0,y:0},
			  actions_issued:Vec::<Action>::new(),
			  my_life:0,
			  play_board:board,
			  torpedo:0,
			  silence:0,
			  sonar:0,
			  mine:0};
    }

    //to do dont print!
    fn get_actions_to_play(&mut self) -> Vec::<Action> {

	let mut v_act = Vec::<Action>::new();
	eprintln!("*** MY possible pos");
	let (my_n_pos, _) = self.my_path.get_possible_pos();
	eprintln!("mynpos {}", my_n_pos);
	
	eprintln!("*** ADV possible pos");
	let (n_pos, coord) = self.path.get_possible_pos();
	if n_pos > 10 {
	    eprintln!("Action: no confidence");
	}
	else {
	    if coord.dist(&self.cur_co) <=4 && coord.dist(&self.cur_co) > 1 && self.torpedo == 3 {
		v_act.push(Action { ac: Action_type::TORPEDO, coord:coord, ..Default::default() });
		self.torpedo = 0;
	    }
	}
	
	
	let e = self.play_board.num_avail_pos(&self.cur_co);
	if e[0].0 != 0 {

	    if self.silence == 6 && my_n_pos < 10 {
		v_act.push(Action { ac: Action_type::SILENCE, dir:e[0].1, sector:1, ..Default::default() });
		self.silence = 0;
	    }
	    
	    else if self.torpedo < 3 {
		v_act.push(Action { ac: Action_type::MOVE, dir:e[0].1, ac_load:Action_type::TORPEDO, ..Default::default() });
		self.torpedo += 1;
		self.torpedo = cmp::min(self.torpedo,3);
	    }
	    else {
		v_act.push(Action { ac: Action_type::MOVE, dir:e[0].1, ac_load:Action_type::SILENCE, ..Default::default() });
		self.silence += 1;
		self.silence = cmp::min(self.silence,6);
	    }
	}
	else {
	    self.play_board.rem_visited();
	    v_act.push(Action { ac: Action_type::SURFACE, sector:self.cur_co.to_surface(), ..Default::default() });
	    //println!("{}SURFACE",add_str)
	}
	self.actions_issued = v_act.to_vec(); //copy here
	v_act
    }
    fn update_situation(&mut self,opp_life:u8, my_life:u8, x:u8, y:u8) {
	self.op_life.push(opp_life);
	self.cur_co = Coordinate {x:x,y:y};
	self.play_board.set_visited(&self.cur_co);
	self.my_life = my_life;

	let mut coord_torpedo = Coordinate {x:0,y:0};
	
	if self.op_life.len() > 2 && self.actions_issued.iter().any(|v| {coord_torpedo = v.coord; v.ac == Action_type::TORPEDO}) {
	    eprintln!("Found torpedo previous");
	    let diff = self.op_life[self.op_life.len() - 2] - *self.op_life.last().unwrap();
	    match diff {
		1 => {
		    eprintln!("torp touch 1! coord {:?}", coord_torpedo);
		    self.path.path_coords.retain(|(_freq, ve)| {ve.last().unwrap().l2_dist(&coord_torpedo) == 1});
		    eprintln!("re {:?}", Path::_reduce_search_space(&self.path.path_coords) );
		},
		2 => {
		    eprintln!("torp touch 2! coord {:?}", coord_torpedo);
		    self.path.path_coords.retain(|(_freq, ve)| {ve.last().unwrap().dist(&coord_torpedo) == 0});
		    eprintln!("re {:?}", Path::_reduce_search_space(&self.path.path_coords) );
		},
		_ => {
		    eprintln!("torp NO touch  coord {:?}", coord_torpedo);
		    self.path.path_coords.retain(|(_freq, ve)| {ve.last().unwrap().l2_dist(&coord_torpedo) > 1});
		}
		    
	    }

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
	predictor.path.process_actions(&Action::parse_raw(&opponent_orders));
	//predictor.path.get_possible_pos();
	let v_acts = predictor.get_actions_to_play();
	predictor.my_path.process_actions(&v_acts);
	println!("{}",&Action::repr_action_v(&v_acts));

    }
}
