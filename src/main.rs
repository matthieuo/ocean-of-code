use std::io;
use std::convert::TryInto;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::cmp::Reverse;
use std::cmp;
use std::str::FromStr;
use std::iter::FromIterator;
use std::time::Instant;

use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

const MAX_X:u8 = 15;
const MAX_Y:u8 = 15;

const PATH_INIT:f64 = 1.0;

#[derive( Copy, Clone,Debug)]
enum Direction {
    N,S,E,W,
}

impl Default for Direction {
    fn default() -> Self { Direction::N }
}
impl FromStr for Direction {
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
	(((self.x as i32 -c.x as i32).pow(2) + (self.y as i32 -c.y as i32).pow(2)) as f64).sqrt().floor() as u8
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
		Action_type::MINE => out = format!("{}MINE {:?}", out, a.dir),
		Action_type::TRIGGER => out = format!("{}TRIGGER {} {}", out, a.coord.x, a.coord.y),
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
    grid: [[u8; MAX_X as usize]; MAX_Y as usize],

  
	
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

    fn get_visited_stat(&self) -> (f64,f64) {
	let mut x_s = [false; MAX_X as usize];
	let mut y_s = [false; MAX_Y as usize];
	
	for x in 0..15 {
	    for y in 0..15 {
		if self.grid[x as usize][y as usize] == 1 {
		    x_s[x as usize] = true;
		    y_s[y as usize] = true;
		    
		}
	    }
	}

	

	(x_s.iter().filter(|&n| *n == true).count() as f64, y_s.iter().filter(|&n| *n == true).count() as f64)
    }
    

    fn _rec_torpedo_coord_help(&self,c_init:&Coordinate, x:i32,y:i32,hist :&mut HashSet::<(i32,i32)>, ret_list:&mut Vec::<Coordinate>, num_vi:&mut i32){

	*num_vi +=1;
	    
	hist.insert((x,y));
	
	if x < 0 || x >= MAX_X as i32|| y < 0 || y >= MAX_Y as i32 || self.get_e(&Coordinate {x:(x) as u8, y:(y) as u8}) == 10 {
	    return
	}

	if c_init.dist(&Coordinate {x:(x) as u8, y:(y) as u8}) > 4
	{
	    return
	}
	
	ret_list.push(Coordinate {x:(x) as u8, y:(y) as u8});
	
	if c_init.dist(&Coordinate {x:(x) as u8, y:(y) as u8}) == 4
	{
	    return
	}

	for modif in -1..2 {
	    if !hist.contains(&(x + modif, y)) {
		self._rec_torpedo_coord_help(c_init, x + modif, y, hist, ret_list, num_vi);
	    }
	    if !hist.contains(&(x, y + modif)) {
		self._rec_torpedo_coord_help(c_init, x, y + modif, hist, ret_list, num_vi);
	    }
		
	}
    }

    
    fn get_torpedo_pos_from_coord(&self, c:&Coordinate) -> Vec::<Coordinate> {
	let mut ret = Vec::<Coordinate>::new();
	let mut num_vi = 0;
	self._rec_torpedo_coord_help(&c, c.x as i32, c.y as i32,  &mut HashSet::<(i32,i32)>::new(), &mut ret, &mut num_vi);
	ret
    }
    
    fn get_diag_coord(&self, c:&Coordinate) -> Vec::<Coordinate> {
	let mut ret_v = Vec::<Coordinate>::new();
	let x = c.x as i8;
	let y = c.y as i8;

	for x_a in -1..2 {
	    for y_a in -1..2 {
		if !(x+x_a < 0 || x+x_a >= MAX_X as i8|| y+y_a < 0 || y+y_a >= MAX_Y as i8 || self.get_e(&Coordinate {x:(x +x_a) as u8, y:(y + y_a) as u8}) == 10) {
		    ret_v.push(Coordinate {x:(x +x_a) as u8, y:(y + y_a) as u8});
		}
	    }
	}
	ret_v
	
    }

      fn get_nsew_coord(&self, c:&Coordinate) -> Vec::<Coordinate> {
	let mut ret_v = Vec::<Coordinate>::new();

	  let x = c.x as i8;
	  let y = c.y as i8;
	  for d in &[Direction::N, Direction::S, Direction::W, Direction::E] {
	      let x_a;
	      let y_a;
	      match d {
		  Direction::N => {x_a=0; y_a=-1},
		  Direction::S => {x_a=0; y_a=1},
		  Direction::W => {x_a=1; y_a=0},
		  Direction::E => {x_a=-1; y_a=0},
	      }
	      
	      if !(x+x_a < 0 || x+x_a >= MAX_X as i8|| y+y_a < 0 || y+y_a >= MAX_Y as i8 || self.get_e(&Coordinate {x:(x +x_a) as u8, y:(y + y_a) as u8}) == 10) {
		  ret_v.push(Coordinate {x:(x +x_a) as u8, y:(y + y_a) as u8});
	      }
	  }
	ret_v
	
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
    
 
    
    fn _rec_best_path(&self, cur_pos :&Coordinate, hist: &mut [[bool; MAX_X as usize]; MAX_Y as usize]) -> (u8, LinkedList::<Direction>) {
	//hist.insert(*cur_pos);
	hist[cur_pos.x as usize][cur_pos.y as usize] = true;
	
	let mut sum_a = 1;

	let mut dir_max = LinkedList::<Direction>::new();
	let mut max_val = 0;
	 
	for d in &[Direction::N, Direction::S, Direction::W, Direction::E]{
	    match self.check_dir(cur_pos, d) {
		Some(c_valid) => {
		    if !hist[c_valid.x as usize][c_valid.y as usize] {
			let (val, r_l) = self._rec_best_path(&c_valid, hist);
			sum_a += val;

			if val > max_val {
			    max_val = val;
			    dir_max = r_l.iter().copied().collect();
			    dir_max.push_front(*d);
			}
		    }
		}
		None    => continue,
	    }
	  
	}
	(sum_a,dir_max)
    }
    
    
    fn _rec_num_pos(&self, cur_pos :&Coordinate, hist: &mut [[bool; MAX_X as usize]; MAX_Y as usize]) -> u8 {
	hist[cur_pos.x as usize][cur_pos.y as usize] = true;
	
	let mut sum_a = 1;
	for d in &[Direction::N, Direction::S, Direction::W, Direction::E]{
	    match self.check_dir(cur_pos, d) {
		Some(c_valid) => {
		    if !hist[c_valid.x as usize][c_valid.y as usize] {
			sum_a += self._rec_num_pos(&c_valid, hist);
		    }
		},
		None    => continue,
	    }
	  
	}
	sum_a
    }


    fn initial_position(&self) -> Coordinate {
	loop {
	    let numx = 0; // rand::thread_rng().gen_range(0, 15);
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
struct PathElem {
    freq: f64,
    coords: Vec::<Coordinate>,
    mines: Vec::<Coordinate>,
}

#[derive(Debug)]
struct Path {
    //path_coords: Vec::<(f64, Vec::<Coordinate>)>,
    path_coords: Vec::<PathElem>,
    board: Board,
    reduced:bool,
    mines_m: MinesMng,
}


impl Path {
    fn new(board: Board) -> Path{
	return Path { path_coords:Vec::<PathElem>::new(),board: board, reduced:false, mines_m:MinesMng::new()}
    }

    

    fn update_mines_infos(&mut self) {
	self.mines_m.update_mines_prob(&self.path_coords);
    }

    
    fn process_trigger(&mut self, co_t :Coordinate) {
	eprintln!("Process trigger, {:?}",co_t);
	//remove  all paths which not contain co_t in mines list

	if !self.reduced {
	    eprintln!("OK not reduced, we process trigger");
	    self.path_coords.retain(|pel| pel.mines.contains(&co_t));

	    //remove the mine triggered
	    for pel in self.path_coords.iter_mut() {
		pel.mines.retain(|m| m != &co_t);
	    }
	}
	
    }

    fn process_mine(&mut self) {
	eprintln!("Process MINE");
	//add a mines to all potential path

	if !self.reduced {
	    eprintln!("OK not reduced, we process mine");
	    for pel in self.path_coords.iter_mut(){
		pel.mines.append(&mut self.board.get_nsew_coord(pel.coords.last().unwrap()));
	    }

	}
	
    }
    
	
    fn process_torpedo(&mut self, co_t :Coordinate) {
	eprintln!("Process torpedo");

	self.path_coords.retain(|pel| {pel.coords.last().unwrap().dist(&co_t) <= 4});
    }

    fn process_surface(&mut self, sector :u8) {
	eprintln!("Process surface");
	let rx:u8;
	let ry:u8;
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

	let l_g:&Board = &self.board;
	self.path_coords.retain(|pel| {
	    for x in rx..(rx + 5){
		for y in ry..(ry + 5){
		    if l_g.grid[x as usize][y as usize] == 10 {
			continue; //Do not add the islands
		    }
                    if pel.coords.last().unwrap() == &(Coordinate {x:x, y:y}) {
			return true;
		    }
		}
	    }
	    return false;
	});
	
	//reset the paths, we keep mines 
	for pel in &mut self.path_coords {
	    pel.coords = vec![*pel.coords.last().unwrap()]
	}
	
	    
    }

      
    fn _reduce_search_space(v_coord :&Vec::<PathElem>) -> Vec::<PathElem> {
	//ok reduce search space
	let mut p_coords_reduced = Vec::<PathElem>::new();
	
	let mut frequency: HashMap<&Coordinate, f64> = HashMap::new();
	let mut frequency_mines: HashMap<&Coordinate, HashSet<Coordinate>> = HashMap::new();
	
	
	for pel in v_coord {
	    let freq = pel.freq;
	    let coord = &pel.coords;
	    *frequency.entry(coord.last().unwrap()).or_insert(0.0) += freq;
	    frequency_mines.entry(coord.last().unwrap()).or_insert(HashSet::<Coordinate>::new()).extend(pel.mines.iter());
	}
	
	for (co,freq) in &frequency {
	    p_coords_reduced.push(PathElem {freq:*freq, coords:vec![**co], mines:Vec::from_iter(frequency_mines[co].iter().copied())});
	}
	p_coords_reduced
    }
    
    fn process_silence(&mut self) {
	eprintln!("Process SILENCE");
	let max_search:usize = 1000; //max paths before reduction
	let mut p_coords_l = Vec::<PathElem>::new();

	if self.path_coords.len() > max_search {
	    eprintln!("REDUCE size before : {}", self.path_coords.len());

	    self.path_coords =  Path::_reduce_search_space(&self.path_coords);
	   
	    eprintln!("REDUCE size after : {}", self.path_coords.len());
	    panic!("REDUCED should be avoided");
	}
	
	for pel in &self.path_coords {
	    //add new possible coord for each paths
	    //adv can make a 0 move

	    p_coords_l.push(PathElem {freq:pel.freq, coords:pel.coords.to_vec(), mines:pel.mines.to_vec()});
	   
	    
	    for d in [Direction::N, Direction::S, Direction::W, Direction::E].iter() {
		
		let cur_path:&mut Vec::<Coordinate> = &mut pel.coords.to_vec();
		let mut cur_pos:Coordinate = *cur_path.last().unwrap();
		
		for i in 1..5 {
		    match self.board.check_dir(&cur_pos, &d) {
			Some(c_valid) =>
			{
			    if !cur_path.contains(&c_valid) {
				//if c_valid is in v, this means a cross between path -> invalid
				cur_path.push(c_valid);
				let new_freq:f64 = (pel.freq)*(((10-2*i) as f64)/10.0);

				p_coords_l.push(PathElem {freq:new_freq, coords:cur_path.to_vec(), mines:pel.mines.to_vec()}); //explicit copy
				cur_pos = c_valid;

			    }
			    else {
				break
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
	    eprintln!("+-+-----+++-+ path coords empty, initialize");
	    for x in 0..MAX_X {
		for y in 0..MAX_Y {
		    if self.board.grid[x as usize][y as usize] == 10 {
			continue; //Do not add the islands
		    }
		    self.path_coords.push(PathElem {freq:PATH_INIT, coords:vec![Coordinate {x:x, y:y}], mines:Vec::<Coordinate>::new()});
		}
	    }
	}
	
	for pel in self.path_coords.iter_mut() {
	    match self.board.check_dir(pel.coords.last().unwrap(), &d) {
		Some(c_valid) => {
		    pel.coords.push(c_valid);
		}
		
		None    => pel.coords.clear(), //impossible we clear the path
	    }
	}
	//remove all element empty
	self.path_coords.retain(|pel| !pel.coords.is_empty());
    }


    fn process_actions(&mut self, v_act:&Vec<Action>) {
	for a in v_act {
	    match a.ac {
		Action_type::MOVE => self.process_move(a.dir),
		Action_type::SURFACE => self.process_surface(a.sector),
		Action_type::TORPEDO =>  self.process_torpedo(a.coord),
		Action_type::SONAR => {},
		Action_type::SILENCE => self.process_silence(),
		Action_type::MINE => self.process_mine(),
		Action_type::TRIGGER => {self.process_trigger(a.coord)},
	    }   
	}
    }

    
    fn comp_variance(v_c:&Vec::<PathElem>, max_freq:f64) -> (f64,f64) {
	let mut xm = -1.0;
	let mut ym = -1.0;
	
	let mut tot:f64 = 0.0;

	//comput mean
	for pel in v_c {
	    if pel.freq < max_freq {
		continue;
	    }
	    let el = pel.coords.last().unwrap();
	    
	    if xm < 0.0 {
		xm = pel.freq*el.x as f64;
		ym = pel.freq*el.y as f64;
		tot += pel.freq;
	    }
	    else {
		xm += pel.freq*el.x as f64;
		ym += pel.freq*el.y as f64;
		tot += pel.freq;
	    }
	    
	}

	xm /= tot;
	ym /= tot;

	//comput variance
	let mut x_v:f64 = 0.0;
	let mut y_v:f64 = 0.0;
	
	for pel in v_c {
	    let el = pel.coords.last().unwrap();

	    x_v += (pel.freq as f64)*(el.x as f64 - xm).powi(2);
	    y_v += (pel.freq as f64)*(el.y as f64 - ym).powi(2);
	}

	x_v /= tot as f64;
	y_v /= tot as f64;

	(x_v.sqrt(), y_v.sqrt())
    }
    fn process_previous_actions(&mut self, va_issued:&Vec<Action>, va_opp_issued:&Vec<Action>, diff_life_arg:u8) {

	let mut diff_life = diff_life_arg;
	//ok tricky, if the opponement made an action that reduce it's life we need to take it into account

	//if we have torpedo in both actions, we cannot say anothing
	if diff_life_arg !=0 && va_issued.iter().any(|v| v.ac == Action_type::TORPEDO) && va_opp_issued.iter().any(|v| v.ac == Action_type::TORPEDO) {
	    return
	}

	if va_issued.iter().filter(|&v| v.ac == Action_type::TORPEDO || v.ac == Action_type::TRIGGER).count() > 1 {
	    eprintln!("Torpedo + trigger in previous action, don't update");
	    return
	}
	
	let mut coord_torpedo = Coordinate {x:0,y:0};
	if  va_issued.iter().any(|v| {coord_torpedo = v.coord; v.ac == Action_type::TORPEDO || v.ac == Action_type::TRIGGER}) {

	    if  va_opp_issued.iter().any(|a| a.ac == Action_type::SURFACE) {
		diff_life -= 1;
		eprintln!("== correction with surface");
	    }
	    eprintln!("Found torpedo or trigger previous");
	    match  diff_life {
		1 => {
		    eprintln!("torp touch 1! coord {:?}", coord_torpedo);
		    self.path_coords.retain(|pel| {pel.coords.last().unwrap().l2_dist(&coord_torpedo) == 1});
		    eprintln!("re {:?}", Path::_reduce_search_space(&self.path_coords) );
		},
		2 => {
		    eprintln!("torp touch 2! coord {:?}", coord_torpedo);
		    self.path_coords.retain(|pel| {pel.coords.last().unwrap().dist(&coord_torpedo) == 0});
		    eprintln!("re {:?}", Path::_reduce_search_space(&self.path_coords) );
		},
		0 => {
		    eprintln!("torp NO touch  coord {:?}", coord_torpedo);
		    self.path_coords.retain(|pel| {pel.coords.last().unwrap().l2_dist(&coord_torpedo) > 1});
		},
		_ => panic!("diff life != 1,2,0 val : {}",diff_life),
		    
	    }

	}	
    }

}

// ------------ SIMULATOR
#[derive( Clone,Debug)]
struct Simulator {
    board: Board,
    my_mines: MinesMng,
    adv_mines:MinesMng,
    play_c: Coordinate,
    adv_c: Coordinate,
    torpedo_v: u8,
    silence_v: u8,
    mine_v: u8,
    adv_lost: u8,
    play_lost: u8,
    proba_coord: f64,

    adv_life:u8,
    play_life:u8,
    
}

impl Simulator {
    fn new(l_board:Board,
	   my_mines: MinesMng,
	   adv_mines:MinesMng,
	   play_c:Coordinate,
	   adv_c:Coordinate,
	   torpedo_v:u8,
	   silence_v:u8,
	   mine_v:u8,
	   proba_coord:f64,
	   play_life:u8,
	   adv_life:u8) -> Simulator { Simulator {board:l_board,
						  my_mines:my_mines,
						  adv_mines:adv_mines,
						  play_c:play_c,
						  adv_c:adv_c,
						  silence_v:silence_v,
						  torpedo_v:torpedo_v,
						  mine_v:mine_v,
						  adv_lost:0,
						  proba_coord:proba_coord,
						  play_lost:0,
						  adv_life:adv_life,
						  play_life:play_life}}

    
    
    fn play_ac_l(&self, va:&Vec::<Action>) -> Option<(Simulator)> {
	let mut sim_sim = self.clone();

	for a in va {
	    match a.ac {
		Action_type::MOVE => {
		    match sim_sim.board.check_dir(&sim_sim.play_c, &a.dir) {
			Some(c_valid) => {
			    sim_sim.play_c = c_valid;
			    sim_sim.board.set_visited(&c_valid);
			    
			    match a.ac_load {
				Action_type::TORPEDO => sim_sim.torpedo_v = cmp::min(sim_sim.torpedo_v+1, 3),
				Action_type::SILENCE => sim_sim.silence_v = cmp::min(sim_sim.silence_v+1, 6),
				Action_type::MINE => sim_sim.mine_v = cmp::min(sim_sim.mine_v+1, 3),
				_ => panic!("Error, must be torpedo, silence or mine"),
			    }
			},
			None    => return None,
		    }
		},
	    
		Action_type::SURFACE => {
		    //sim_sim.board.rem_visited();
		    sim_sim.play_life -=1;
		},
		Action_type::TORPEDO => {

		    if sim_sim.torpedo_v < 3 {
			return None;
		    }
		    
		    if sim_sim.play_c.dist(&a.coord) > 4 {
			return None;
		     }
		    
		    let list_torps = sim_sim.board.get_torpedo_pos_from_coord(&sim_sim.play_c);
		    if !list_torps.contains(&a.coord) {
			return None;
		    }

		    sim_sim.torpedo_v = 0;
		    if a.coord.dist(&sim_sim.adv_c) == 0 {
			sim_sim.adv_lost = 2;
		    }
		    else if a.coord.l2_dist(&sim_sim.adv_c) == 1 {
			sim_sim.adv_lost = 1;
		    }

		    if a.coord.dist(&sim_sim.play_c) == 0 {
			sim_sim.play_lost = 2;
		    }
		    else if a.coord.l2_dist(&sim_sim.play_c) == 1 {
			sim_sim.play_lost = 1;
		    }
		    
		}
		Action_type::SONAR => continue,
		Action_type::SILENCE => {
		    if sim_sim.silence_v < 6 {
			return None
		    }

		    let mut loc_pc:Coordinate = sim_sim.play_c;
		    for _ in 0..a.sector {
			match sim_sim.board.check_dir(&loc_pc, &a.dir) {
			    Some(c_valid) => loc_pc = c_valid,
			    None    => return None,
			}
		    }
		    //ok here we tested all coords and it's OK, now write on the board
		    for _ in 0..a.sector {
			match sim_sim.board.check_dir(&sim_sim.play_c, &a.dir) {
			    
			    Some(c_valid) => {
				sim_sim.board.set_visited(&c_valid);
				sim_sim.play_c = c_valid;
			    },
			    None    => panic!("Can't happen !!"),
			}
		    }
		    sim_sim.silence_v = 0;
		    
		},
		Action_type::MINE => {
		    if sim_sim.mine_v < 3 {
			return None;
		    }

		    let mut xl = sim_sim.play_c.x as i8;
		    let mut yl = sim_sim.play_c.y as i8;
		    
		    match a.dir {
			Direction::N => yl -= 1,
			Direction::S => yl += 1,
			Direction::W => xl -= 1, 
			Direction::E => xl += 1,    
		    }
		    
		    if xl < 0 || xl >= MAX_X as i8|| yl < 0 || yl >= MAX_Y as i8 || sim_sim.board.get_e(&Coordinate {x:xl as u8, y:yl as u8}) == 10 { 
			 return None
		    }

		    let mine_coord = Coordinate {x:xl as u8, y:yl as u8};
		    if sim_sim.my_mines.list_mines.contains(&mine_coord) {
			return None
		    }

		    sim_sim.my_mines.list_mines.insert(mine_coord);
		    
		    sim_sim.mine_v = 0;
		},
		Action_type::TRIGGER => continue,
	    }	    
	}
	if sim_sim.play_life as i32 - sim_sim.play_lost as i32 <= 0 { //if we loose, bad action...
	    None
	}
	else {
	    Some(sim_sim)
	}
    }

    fn eval_func(&self) -> f64 {
	if self.play_life as i32 - self.play_lost as i32 <= 0  { //if we loose, bad action...
	    return 0.0;
	}

	if self.proba_coord < 0.5 && self.play_lost > 0 {
	    return 0.0 //do not allow to loose life if we are unsure of the position
	}
	return self.adv_lost as f64 - self.play_lost as f64;
    }

    fn eval_func_move(&self) -> f64 {

	//-- find available positions
	let mut grid: [[bool; MAX_X as usize]; MAX_Y as usize] = [[false; MAX_X as usize]; MAX_Y as usize];
	let po = self.board._rec_num_pos(&self.play_c, &mut grid);

	//path with good shape
	let (mx,my) = self.board.get_visited_stat();


	//maximize the number of ressources
	let maxim =  self.torpedo_v + self.mine_v + self.silence_v;

	//try to have good mines structure
	let mines =  4.0*self.my_mines.list_mines.len() as f64; 


	//try to avoid adv mines
	let dc = self.board.get_diag_coord(&self.play_c);

	let avoid_mines:f64 = dc.iter().map(|&c| self.adv_mines.grid_probas[c.x as usize][c.y as usize]).sum();
	return po as f64 + maxim as f64 - (mx-my).abs() + mines - 30.0*avoid_mines;
    }

    fn compute_best_move_sequence(&self, with_sil:bool,with_move:bool) -> Option<(Vec::<Action>, Simulator)> {
	let mut v_move = Vec::<Action>::new();
	let mut v_sil =  Vec::<Action>::new();
	let mut v_mines =  Vec::<Action>::new();
	
	let mut ret_val_std = None;

	let mut max_op = 0.0;
	
	for d in &[Direction::N, Direction::S, Direction::W, Direction::E] {
	    v_mines.push(Action { ac: Action_type::MINE, dir:*d, ..Default::default() });
	    
	    if with_move {
		v_move.push(Action { ac: Action_type::MOVE, dir:*d, ac_load:Action_type::TORPEDO, ..Default::default() });
		v_move.push(Action { ac: Action_type::MOVE, dir:*d, ac_load:Action_type::SILENCE, ..Default::default() });
		v_move.push(Action { ac: Action_type::MOVE, dir:*d, ac_load:Action_type::MINE, ..Default::default() });
	    }

	    if with_sil {
		for i in 1..5 {
		    v_sil.push(Action { ac: Action_type::SILENCE, dir:*d, sector:i, ..Default::default() });
		}
	    }
	}

	for a in &v_move{
	    let v_try = vec![*a];
	    match self.play_ac_l(&v_try)
	    {
		Some(sim) => {
		    let ev_f = sim.eval_func_move();
		    if ev_f > max_op {
			max_op = ev_f;
			ret_val_std = Some((v_try, sim));
		    }
		}
		None => continue,
	    }
	}
	
	for a in &v_move{
	    for m in &v_mines{
		let v_try = vec![*a, *m];
		match self.play_ac_l(&v_try)
		{
		    Some(sim) => {
			let ev_f = sim.eval_func_move();
			if ev_f > max_op {
			    max_op = ev_f;
			    ret_val_std = Some((v_try, sim));
			}
		    }
		    None => continue,
		}
	    }
	}

	ret_val_std
    }

    
    fn compute_best_sequence(&self, m_play:&MinesMng, m_adv:&MinesMng) -> Option<(Vec::<Action>, Simulator)> {
	
	let mut v_move = Vec::<Action>::new();
	let mut v_sil =  Vec::<Action>::new();
	let mut v_torp =  Vec::<Action>::new();
	
	let mut v_trig =  Vec::<Action>::new();
	
	for d in &[Direction::N, Direction::S, Direction::W, Direction::E] {
	    v_move.push(Action { ac: Action_type::MOVE, dir:*d, ac_load:Action_type::TORPEDO, ..Default::default() });
	    v_move.push(Action { ac: Action_type::MOVE, dir:*d, ac_load:Action_type::SILENCE, ..Default::default() });
	    
	    for i in 1..5 {
		v_sil.push(Action { ac: Action_type::SILENCE, dir:*d, sector:i, ..Default::default() });
	    }
	}

	for a in &self.board.get_diag_coord(&self.adv_c) {
	    v_torp.push(Action { ac: Action_type::TORPEDO, coord:*a, ..Default::default() });
	}

	for c in  &m_play.list_mines {
	    v_trig.push(Action { ac: Action_type::TRIGGER, coord:*c, ..Default::default() });
	}

	let mut max_op = 0.0;
	let mut ret_val:Option::<(Vec::<Action>, Simulator)> = None;

	for a in &v_torp {
	    let v_try = vec![*a];
			match self.play_ac_l(&v_try)
			{
			    Some(sim) => {
				let ev_f = sim.eval_func();
				if ev_f  > max_op {
				    max_op = ev_f;
				    ret_val = Some((v_try, sim));
				}
			    }
			    None => continue,
			}
	}
	if self.proba_coord <= 0.2 && (self.silence_v < 6 || self.mine_v < 3){
	    //proba is to low, need to create silence
	    return None;
	}
	
	if self.proba_coord <= 0.2 && self.silence_v == 6 {
	    eprintln!("Proba <=0.2, only torpedo if assez silence, early return since proba low");
	    return ret_val
	}
	
	if self.proba_coord > 0.2 {
	    //move then torpedo
	    eprintln!("Proba inf > 0.2, move +  torpedo");
	    for a_move in &v_move {
		for a in &v_torp {
		    let v_try = vec![*a_move, *a];
			match self.play_ac_l(&v_try)
			{
			    Some(sim) => {
				let ev_f = sim.eval_func();
				if ev_f  > max_op {
				    max_op = ev_f;
				    ret_val = Some((v_try, sim));
				}
			    }
			    None => continue,
			}
		}
	    }
	}
	
	if self.proba_coord > 0.9 {
	    //move silence then torpedo
	    eprintln!("Proba inf > 0.9, move + silence torpedo");
	    for a_move in &v_move {
		for a_sil in &v_sil {
		    for a in &v_torp {
			let v_try = vec![*a_move,*a_sil, *a];
			match self.play_ac_l(&v_try)
			{
			    Some(sim) => {
				let ev_f = sim.eval_func();
				if ev_f  > max_op {
				    max_op = ev_f;
				    ret_val = Some((v_try, sim));
				}
			    }
			    None => continue,
			}
		    }
		}
	    }
	}
	ret_val
    }
  
}

//MINES Manager
#[derive(Debug,  Clone)]
struct MinesMng {
    list_mines: HashSet::<Coordinate>,

    grid_probas: [[f64; MAX_X as usize]; MAX_Y as usize],
}

impl  MinesMng  {
    fn new() -> MinesMng {
	MinesMng {list_mines:HashSet::<Coordinate>::new(), grid_probas:[[0.0; MAX_X as usize]; MAX_Y as usize]}
    }

    fn add_mine(&mut self, c:&Coordinate) {
	self.list_mines.insert(*c);
    }

    
    fn get_remove_d1(&mut self, c:&Coordinate) -> Option<Coordinate> {

	let mut retval = None;
	self.list_mines.retain(|mc| {
	    if mc.l2_dist(c) <= 1 {
		retval=Some(*mc);
		false
	    } else {
		true
	    }
	});
	retval
    }

    //reduce possible mines to freq to coordinate
    fn update_mines_prob(&mut self, v:&Vec::<PathElem>)  {
	let mut frequency: HashMap<&Coordinate, f64> = HashMap::new();
	for pel in v {
	    for m in &pel.mines {
		*frequency.entry(m).or_insert(0.0) += pel.freq;
	    }
	    
	}

	self.grid_probas = [[0.0; MAX_X as usize]; MAX_Y as usize];
	let max_v = frequency.len() as f64;
	for (co_m,freq) in &frequency {
	    self.grid_probas[co_m.x as usize][co_m.y as usize] = (*freq as f64)/max_v;
	}

	eprintln!("Mines situations");
	eprintln!("Num poss mines {}", frequency.len());
    }

    
}
//------------ PREDICTOR
#[derive(Debug)]
struct Predictor {
    path: Path,
    my_path: Path,
    
    op_life: Vec::<u8>,
    cur_co: Coordinate,
    play_board: Board,
    my_life: Vec::<u8>,
    torpedo :u8,
    silence :u8,
    sonar :u8,
    mines: u8,
    actions_issued: Vec::<Action>,
  
}

impl  Predictor  {
    fn new(board: Board) -> Predictor{
	return Predictor {path: Path::new(board),
			  my_path: Path::new(board),
			  op_life:Vec::<u8>::new(),
			  cur_co: Coordinate {x:0,y:0},
			  actions_issued:Vec::<Action>::new(),
			  my_life:Vec::<u8>::new(),
			  play_board:board,
			  torpedo:0,
			  silence:0,
			  sonar:0,
			  mines:0};
    }


    fn get_possible_pos(&self, path:&Path) ->  (usize, Coordinate, (f64, f64), f64, Coordinate) {
		
	let mut reduced_v = Path::_reduce_search_space(&path.path_coords);
	reduced_v.sort_unstable_by(|pela, pelb| pelb.freq.partial_cmp(&pela.freq).unwrap()); //reverse sort
	
	eprintln!("Num possible coord {}", reduced_v.len());
	eprintln!("Num possible path {}", path.path_coords.len());

	//try to keep only the maximum confidences

	let max_freq:f64 = reduced_v[cmp::min(reduced_v.len()-1, reduced_v.len()-1) as usize].freq;
	eprintln!("Max k-{} max_freq : {}", cmp::min(4, reduced_v.len()-1), max_freq);
	
	let mut xm:f64 = -1.0;
	let mut ym:f64 = -1.0;

	let mut tot:f64 = 0.0;
	for pel in &reduced_v {
	    if pel.freq < max_freq {
		continue;
	    }
	    let el = pel.coords.last().unwrap();
	    
	    if xm < 0.0 {
		xm = pel.freq*el.x as f64;
		ym = pel.freq*el.y as f64;
		tot += pel.freq;
	    }
	    else {
		xm += pel.freq*el.x as f64;
		ym += pel.freq*el.y as f64;
		tot += pel.freq;
	    }
	    
	}

	xm /= tot;
	ym /= tot;

	let round_coord = Coordinate {x:xm.round() as u8, y:ym.round() as u8};

	eprintln!("round {:?}", round_coord);
	if reduced_v.len() < 10
	{
	    for pel in &reduced_v
	    {
		eprintln!("freq : {}, prob {},  val : {:?}",pel.freq, pel.freq/tot, pel.coords.last().unwrap());
	    }
	}

	(reduced_v.len(),round_coord, Path::comp_variance(&reduced_v, max_freq), reduced_v[0].freq/tot, *reduced_v[0].coords.last().unwrap())
    }

    

    fn get_actions_to_play(&mut self) -> Vec::<Action> {
	eprintln!("Torpedo val {}",self.torpedo);
	let mut v_act = Vec::<Action>::new();
	let mut v_act_move = Vec::<Action>::new();
	    
	    let mut proba_my = 0.0;
	    
	    if !self.my_path.path_coords.is_empty() && !self.path.path_coords.is_empty() {
		eprintln!("*** MY possible pos");
		let (my_n_pos_l, _,_, proba_my_loc, _) = self.get_possible_pos(&self.my_path);

		proba_my = proba_my_loc;
		eprintln!("mynpos proba {}", proba_my);
		
		let (n_pos, coord_mean, variance, prob, max_prob_coord) = self.get_possible_pos(&self.path);
		let coord = max_prob_coord;
		eprintln!("*** ADV possible pos np: {}, var: {:?}, prob : {}", n_pos, variance, prob);


		let dc = self.play_board.get_diag_coord(&self.cur_co);
		let avoid_mines:f64 = dc.iter().map(|&c| self.path.mines_m.grid_probas[c.x as usize][c.y as usize]).sum();
		eprintln!("Avoid mines {}",30.0*avoid_mines);
		let (_, dir) = self.play_board._rec_best_path(&self.cur_co, &mut [[false; MAX_X as usize]; MAX_Y as usize]);
		if dir.is_empty() || 30.0*avoid_mines > 100.0 {
		    self.play_board.rem_visited();
		    *(self.my_life.last_mut().unwrap()) -=1;
		    eprintln!("SURFACE, sector {}",self.cur_co.to_surface());
		    v_act.push(Action { ac: Action_type::SURFACE, sector:self.cur_co.to_surface(), ..Default::default() });
		    //println!("{}SURFACE",add_str)
		}
		
	
		let simul = Simulator::new(self.play_board,
					   self.my_path.mines_m.clone(),
					   self.path.mines_m.clone(),
					   self.cur_co,
					   coord,
					   self.torpedo,
					   self.silence,
					   self.mines,
					   prob,
					   *self.my_life.last().unwrap(),
					   *self.op_life.last().unwrap());

		let mut next_sim = simul.clone();
		match simul.compute_best_sequence(&self.my_path.mines_m, &self.path.mines_m) {
		    Some((v,sim)) => {
			eprintln!("FFFFFF {:?} {} {}",v, sim.adv_lost,sim.play_lost);
			v_act = v;
			self.torpedo = sim.torpedo_v;
			self.silence = sim.silence_v;
			self.mines = sim.mine_v;
			self.cur_co = sim.play_c;
			self.play_board = sim.board; //update board, should be ok...
			self.my_path.mines_m = sim.my_mines.clone();
			next_sim = sim;
		    },
		    None => eprintln!("FFFFFF NOT"),
		}
		let has_sil = v_act.iter().any(|&x| x.ac == Action_type::SILENCE);
		let has_move = v_act.iter().any(|&x| x.ac == Action_type::MOVE);
		
		    
		match next_sim.compute_best_move_sequence(!has_sil, !has_move) {
		    Some((v,sim)) => {
			eprintln!("FFFmove {:?} {} {}",v, sim.adv_lost,sim.play_lost);
			v_act_move = v;
			self.torpedo = sim.torpedo_v;
			self.silence = sim.silence_v;
			self.mines = sim.mine_v;
			self.cur_co = sim.play_c;
			self.play_board = sim.board; //update board, should be ok...
			self.my_path.mines_m = sim.my_mines;
		    },
		    None => eprintln!("FFmove NOT"),
		}
		
		
		if prob > 0.9 {
		    match self.my_path.mines_m.get_remove_d1(&max_prob_coord) {
			Some(c) => v_act.push(Action { ac: Action_type::TRIGGER, coord:c, ..Default::default() }),
			None => {} ,
		    }
		}
	    }

	v_act.extend(&v_act_move);
	
	if !v_act.iter().any(|&x| x.ac == Action_type::SILENCE) && self.silence == 6 && proba_my > 0.8 {
	    let (_, dir) = self.play_board._rec_best_path(&self.cur_co, &mut [[false; MAX_X as usize]; MAX_Y as usize]);
	    if !dir.is_empty() {
		let next_dir = *dir.front().unwrap();
		eprintln!("He found me, silence !!");
		v_act.push(Action { ac: Action_type::SILENCE, dir:next_dir, sector:1, ..Default::default() });
		self.silence = 0;
	    }
	}

	if v_act.is_empty() { //for the firsts rounds
	    let (_, dir) = self.play_board._rec_best_path(&self.cur_co, &mut [[false; MAX_X as usize]; MAX_Y as usize]);
	    let next_dir = *dir.front().unwrap();

	    
	    v_act.push(Action { ac: Action_type::MOVE, dir:next_dir, ac_load:Action_type::TORPEDO, ..Default::default() });
	    
	    self.torpedo += 1;
	    self.torpedo = cmp::min(self.torpedo,3);
		
	}
		
	self.actions_issued = v_act.to_vec(); //copy here
	v_act
    }
    fn update_situation(&mut self,opp_life:u8, my_life:u8, x:u8, y:u8, opponent_orders:&Vec::<Action>) {
	self.path.update_mines_infos();
	self.op_life.push(opp_life);
	self.cur_co = Coordinate {x:x,y:y};
	self.play_board.set_visited(&self.cur_co);
	self.my_life.push(my_life);

	if self.op_life.len() > 2 {
	    eprintln!("Update ADV coordinate");
	    let diff = self.op_life[self.op_life.len() - 2] - *self.op_life.last().unwrap();
	    self.path.process_previous_actions(&self.actions_issued, opponent_orders, diff);
	}

	if self.my_life.len() > 2 {
	    eprintln!("Update MY coordinate");
	    let diff = self.my_life[self.my_life.len() - 2] - *self.my_life.last().unwrap();
	    self.my_path.process_previous_actions(opponent_orders, &self.actions_issued, diff);
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

    let board = Board::new(&vec);  //ok dont use this Board since values are copied on the predictor
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

	let start = Instant::now();
	
	predictor.update_situation(opp_life as u8, my_life as u8, x as u8, y as u8, &Action::parse_raw(&opponent_orders));
	predictor.path.process_actions(&Action::parse_raw(&opponent_orders));
	//predictor.path.get_possible_pos();
	let v_acts = predictor.get_actions_to_play();
	predictor.my_path.process_actions(&v_acts);

	let duration = start.elapsed();
	eprintln!("Total duration : {:?}",duration);
	println!("{}",&Action::repr_action_v(&v_acts));

    }
}
