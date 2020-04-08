use std::io;
use std::convert::TryInto;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::cmp::Reverse;
use std::cmp;
use std::str::FromStr;
//use itertools::Itertools;
//use std::collections::VecDeque;

//extern crate rand;
use rand::Rng;
//use std::fmt;
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


thread_local! {
    static grid_cache_torpedo:HashMap::<Coordinate, Vec::<Coordinate> > = HashMap::<Coordinate, Vec::<Coordinate> >::new();
}
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



    fn _rec_torpedo_coord_help(&self,c_init:&Coordinate, x:i32,y:i32,hist :&mut HashSet::<(i32,i32)>, ret_list:&mut Vec::<Coordinate>, num_vi:&mut i32){

	*num_vi +=1;
	    
	hist.insert((x,y));
	
	if x < 0 || x >= MAX_X as i32|| y < 0 || y >= MAX_Y as i32 || self.get_e(&Coordinate {x:(x) as u8, y:(y) as u8}) == 10 {
	    return
	}

	if c_init.dist(&Coordinate {x:(x) as u8, y:(y) as u8}) > 4
	{
	    //eprintln!("sup {:?} {:?} dist {}",c_init, Coordinate {x:(x) as u8, y:(y) as u8},c_init.dist(&Coordinate {x:(x) as u8, y:(y) as u8}) );
	    return
	}
	
	ret_list.push(Coordinate {x:(x) as u8, y:(y) as u8});
	
	if c_init.dist(&Coordinate {x:(x) as u8, y:(y) as u8}) == 4
	{
	    //eprintln!("sup {:?} {:?} dist {}",c_init, Coordinate {x:(x) as u8, y:(y) as u8},c_init.dist(&Coordinate {x:(x) as u8, y:(y) as u8}) );
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
	//eprintln!("Num vi {}", num_vi);
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
    
    /*fn _bfs_best_path(&self, cur_pos :&Coordinate) { //-> (u8, LinkedList::<Direction>) {
	let mut visited = [[false;MAX_X as usize];MAX_Y as usize];

	let mut queue = VecDeque::<Coordinate>::new();
	queue.push_back(*cur_pos) ;
        visited[cur_pos.x as usize][cur_pos.y as usize] = true;
	
	while !queue.is_empty() {
	    let c = queue.pop_back().unwrap();

	    
	    
	}
	
    }*/


    
    fn _rec_best_path(&self, cur_pos :&Coordinate, hist :&mut HashSet::<Coordinate>) -> (u8, LinkedList::<Direction>) {
	hist.insert(*cur_pos);
	let mut sum_a = 1;

	let mut dir_max = LinkedList::<Direction>::new();
	let mut max_val = 0;
	    
	for d in &[Direction::N, Direction::S, Direction::W, Direction::E]{
	    match self.check_dir(cur_pos, d) {
		Some(c_valid) => {
		    if !hist.contains(&c_valid) {
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
	    let numx = 0; //rand::thread_rng().gen_range(0, 15);
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
    path_coords: Vec::<(f64, Vec::<Coordinate>)>,

    board: Board,
}

impl Path {
    fn new(board: Board) -> Path{
	return Path { path_coords:Vec::<(f64, Vec::<Coordinate>)>::new(),board: board}
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
	self.path_coords.retain(|(_,ve)| {
	    for x in rx..(rx + 5){
		for y in ry..(ry + 5){
                    if ve.last().unwrap() == &(Coordinate {x:x, y:y}) {
			return true;
		    }
		}
	    }
	    return false;
	});
	//reset the paths
	for e in &mut self.path_coords {
	    *e = (e.0, vec![*e.1.last().unwrap()]);
	}
	
	    
    }

    fn _reduce_search_space(v_coord :&Vec::<(f64,Vec::<Coordinate>)>) -> Vec::<(f64,Vec::<Coordinate>)> {
	//ok reduce search space
	let mut p_coords_reduced = Vec::<(f64,Vec::<Coordinate>)>::new();
	
	let mut frequency: HashMap<&Coordinate, f64> = HashMap::new();
	
	for (freq, coord) in v_coord { 
	    *frequency.entry(coord.last().unwrap()).or_insert(0.0) += *freq;
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
	let mut p_coords_l = Vec::<(f64,Vec::<Coordinate>)>::new();

	if self.path_coords.len() > max_search {
	    eprintln!("REDUCE size before : {}", self.path_coords.len());

	    self.path_coords =  Path::_reduce_search_space(&self.path_coords);
	    
	    eprintln!("REDUCE size after : {}", self.path_coords.len());
	}
	
	for (freq,v) in self.path_coords.iter() {
	    //add new possible coord for each paths
	    //adv can make a 0 move

	    p_coords_l.push((*freq, v.to_vec()));
	   
	    
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

				//p_coords_l.push((PATH_INIT - 2*(i-1),cur_path.to_vec())); //explicit copy
				let new_freq:f64 = (*freq)*(((10-2*i) as f64)/10.0);
				p_coords_l.push((new_freq,cur_path.to_vec())); //explicit copy

				
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
		    self.path_coords.push((PATH_INIT,vec![Coordinate {x:x, y:y}]));
		}
	    }
	}
	else {
	    for (_,p) in self.path_coords.iter_mut() {
		match self.board.check_dir(p.last().unwrap(), &d) {
		    Some(c_valid) => {
			p.push(c_valid);
		    }
		
		    None    => p.clear(), //impossible we clear the path
		}
	    }
	    //remove all element empty
	    self.path_coords.retain(|(_, ve)| !ve.is_empty())
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

    
    fn comp_variance(v_c:&Vec::<(f64,Vec::<Coordinate>)>, max_freq:f64) -> (f64,f64) {
	let mut xm = -1.0;
	let mut ym = -1.0;
	
	let mut tot:f64 = 0.0;

	//comput mean
	for (freq, el_v) in v_c {
	    if *freq < max_freq {
		continue;
	    }
	    let el = el_v.last().unwrap();
	    
	    if xm < 0.0 {
		xm = *freq*el.x as f64;
		ym = *freq*el.y as f64;
		tot += *freq;
	    }
	    else {
		xm += *freq*el.x as f64;
		ym += *freq*el.y as f64;
		tot += *freq;
	    }
	    
	}

	xm /= tot;
	ym /= tot;

	//comput variance
	let mut x_v:f64 = 0.0;
	let mut y_v:f64 = 0.0;
	
	for (freq, el_v) in v_c {
	    let el = el_v.last().unwrap();

	    x_v += (*freq as f64)*(el.x as f64 - xm).powi(2);
	    y_v += (*freq as f64)*(el.y as f64 - ym).powi(2);
	}

	x_v /= tot as f64;
	y_v /= tot as f64;

	(x_v.sqrt(), y_v.sqrt())
    }


    //arg will be sorted !!
   /* fn sorted_top_k(v:&mut Vec::<(f64,Vec::<Coordinate>)>, k:usize) -> Option<Vec::<f64>> {
	v.sort_unstable_by(|(a,_), (b,_)| b.partial_cmp(a).unwrap()); //reverse sort
	
        if v.len() <= k {
	    None
        } else {
            let mut result = vec![0.0; k];
	    result[0] = v[0].0;
	    let mut cur_idx = 0;
	    for (e,_) in v.iter().skip(1) {
		if *e != result[cur_idx] {
		    cur_idx +=1;
		    result[cur_idx] = *e;
		    if cur_idx == k-1 {
			break;
		    }
		}
	    }
	    Some(result)
        }
    
    }*/

    fn get_possible_pos(&self) ->  (usize, Coordinate, (f64, f64), f64, Coordinate) {
		
	let mut reduced_v = Path::_reduce_search_space(&self.path_coords);
	reduced_v.sort_unstable_by(|(a,_), (b,_)| b.partial_cmp(a).unwrap()); //reverse sort
	
	eprintln!("Num possible coord {}", reduced_v.len());
	eprintln!("Num possible path {}", self.path_coords.len());

	//try to keep only the maximum confidences
	//let (max_freq, _) =  reduced_v.iter().max_by_key(|(x,_)| x).unwrap(); 


	let max_freq:f64 = reduced_v[cmp::min(reduced_v.len()-1, reduced_v.len()-1) as usize].0;
	eprintln!("Max k-{} max_freq : {}", cmp::min(4, reduced_v.len()-1), max_freq);
	
	
	let mut xm:f64 = -1.0;
	let mut ym:f64 = -1.0;

	let mut tot:f64 = 0.0;
	for (freq, el_v) in &reduced_v {
	    if *freq < max_freq {
		continue;
	    }
	    let el = el_v.last().unwrap();
	    
	    if xm < 0.0 {
		xm = *freq*el.x as f64;
		ym = *freq*el.y as f64;
		tot += *freq;
	    }
	    else {
		xm += *freq*el.x as f64;
		ym += *freq*el.y as f64;
		tot += *freq;
	    }
	    
	}

	xm /= tot;
	ym /= tot;

	let round_coord = Coordinate {x:xm.round() as u8, y:ym.round() as u8};

	
	eprintln!("round {:?}", round_coord);
	if reduced_v.len() < 40
	{
	    for (f,v_p) in &reduced_v
	    {
		eprintln!("freq : {}, prob {},  val : {:?}",f, f/tot, v_p.last().unwrap());
	    }
	}

	(reduced_v.len(),round_coord, Path::comp_variance(&reduced_v, max_freq), reduced_v[0].0/tot, *reduced_v[0].1.last().unwrap())
    }
    fn process_previous_actions(&mut self, va_issued:&Vec<Action>, va_opp_issued:&Vec<Action>, diff_life_arg:u8) {

	let mut diff_life = diff_life_arg;
	//ok tricky, if the opponement made an action that reduce it's life we need take it into account
	
	for a in va_opp_issued {
	    match a.ac {
		Action_type::MOVE => {},
		Action_type::SURFACE => {
		    diff_life -= 1;
		    eprintln!("== correction with surface");
		},
		Action_type::TORPEDO =>  {},
		Action_type::SONAR => {},
		Action_type::SILENCE => {},
		Action_type::MINE => {},
		Action_type::TRIGGER => {},
	    }   
	}
	
	let mut coord_torpedo = Coordinate {x:0,y:0};
	if  va_issued.iter().any(|v| {coord_torpedo = v.coord; v.ac == Action_type::TORPEDO || v.ac == Action_type::TRIGGER}) {
	    eprintln!("Found torpedo or trigger previous");
	    //let diff = self.op_life[self.op_life.len() - 2] - *self.op_life.last().unwrap();
	    match  diff_life {
		1 => {
		    eprintln!("torp touch 1! coord {:?}", coord_torpedo);
		    self.path_coords.retain(|(_freq, ve)| {ve.last().unwrap().l2_dist(&coord_torpedo) == 1});
		    eprintln!("re {:?}", Path::_reduce_search_space(&self.path_coords) );
		},
		2 => {
		    eprintln!("torp touch 2! coord {:?}", coord_torpedo);
		    self.path_coords.retain(|(_freq, ve)| {ve.last().unwrap().dist(&coord_torpedo) == 0});
		    eprintln!("re {:?}", Path::_reduce_search_space(&self.path_coords) );
		},
		_ => {
		    eprintln!("torp NO touch  coord {:?}", coord_torpedo);
		    self.path_coords.retain(|(_freq, ve)| {ve.last().unwrap().l2_dist(&coord_torpedo) > 1});
		}
		    
	    }

	}	
    }

}

// ------------ SIMULATOR
#[derive( Copy, Clone,Debug)]
struct Simulator {
    board: Board,
    play_c: Coordinate,
    adv_c: Coordinate,
    torpedo_v: u8,
    silence_v: u8,
    adv_lost: u8,
    play_lost: u8,
    proba_coord: f64,
    
}

impl Simulator {
    fn new(l_board:Board,
	   play_c:Coordinate,
	   adv_c:Coordinate,
	   torpedo_v:u8,
	   silence_v:u8,
	   proba_coord:f64) -> Simulator { Simulator {board:l_board,
						      play_c:play_c,
						      adv_c:adv_c,
						      silence_v:silence_v,
						      torpedo_v:torpedo_v,
						      adv_lost:0,
						      proba_coord:proba_coord,
						      play_lost:0}}

    fn play_ac_l(&self, va:&Vec::<Action>) -> Option<Simulator> {
	let mut sim_sim = *self;
	for a in va {
	    match a.ac {
		Action_type::MOVE => {
		    match sim_sim.board.check_dir(&sim_sim.play_c, &a.dir) {
			Some(c_valid) => {
			    sim_sim.play_c = c_valid;
			    sim_sim.board.set_visited(&c_valid);
			    if a.ac_load == Action_type::TORPEDO {
				sim_sim.torpedo_v = cmp::min(sim_sim.torpedo_v+1, 3);
			    }
			    else {
				sim_sim.silence_v = cmp::min(sim_sim.silence_v+1, 6);
			    }		    
			},
			None    => return None,
		    }
		},
	    
		Action_type::SURFACE => return None,
		Action_type::TORPEDO => {

		    //eprintln!("Torp val co {:?}, vec {:?}", a.coord, sim_sim.board.get_torpedo_pos_from_coord(&a.coord));



		    if sim_sim.play_c.dist(&a.coord) > 4 {
			//first verif
			//eprintln!("to long {:?}", a.coord);
			return None;
		     }
		    
		    let list_torps = sim_sim.board.get_torpedo_pos_from_coord(&sim_sim.play_c);
		    //eprintln!("Size list {:?} {}", sim_sim.play_c, list_torps.len());
		    //if sim_sim.play_c.dist(&a.coord) > 4 {
		    if !list_torps.contains(&a.coord) {
			//eprintln!("to long {:?}", a.coord);
			return None;
		    }

		    if sim_sim.torpedo_v < 3 {
			//eprintln!("no torpedo {}", sim_sim.torpedo_v);
			return None;
		    }
		    eprintln!("ok torpedo {:?} {:?} {:?} {} {}", a.coord, sim_sim.adv_c, sim_sim.play_c, a.coord.dist(&sim_sim.adv_c),a.coord.l2_dist(&sim_sim.adv_c) );
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
		Action_type::SONAR => return None,
		Action_type::SILENCE => {
		    if sim_sim.silence_v < 6 {
			return None
		    }

		    let mut loc_pc:Coordinate = sim_sim.play_c;
		    for i in 0..a.sector {
			//eprintln!("beg Action pass {}",i);
			match sim_sim.board.check_dir(&loc_pc, &a.dir) {
			    Some(c_valid) => loc_pc = c_valid,
			    None    => return None,
			}
		    }
		    
		    //ok here we tested all coords and it's OK, now write on the board
		    for i in 0..a.sector {
			//eprintln!("beg Action OK {}",i);
			match sim_sim.board.check_dir(&sim_sim.play_c, &a.dir) {
			    
			    Some(c_valid) => {
				sim_sim.board.set_visited(&c_valid);
				sim_sim.play_c = c_valid;
				//eprintln!("Action OK {}",i);
			    },
			    None    => panic!("Can't happen !!"),
			}
		    }
		    sim_sim.silence_v = 0;
		},
		Action_type::MINE => return None,
		Action_type::TRIGGER => return None,	
	    }	    
	}
	//eprintln!("ret val {}",sim_sim.adv_lost);
	Some(sim_sim)
    }

    fn compute_best_sequence(&self) -> Option<(Vec::<Action>, Simulator)> {
	
	//let mut v_ret =  Vec::<Action>::new();
	
	let mut v_move = Vec::<Action>::new();
	let mut v_sil =  Vec::<Action>::new();
	let mut v_torp =  Vec::<Action>::new();

	
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


	let mut max_op = 0;
	let mut ret_val:Option::<(Vec::<Action>, Simulator)> = None;




	if self.proba_coord <= 0.2 || self.silence_v == 6 {
	    eprintln!("Proba inf >= 0.2 <=0.3, only torpedo if assez silence");
	    for a in &v_torp {
		let v_try = &vec![*a];
		match self.play_ac_l(v_try)
		{
		    Some(sim) => {
			if (sim.adv_lost as i32 - sim.play_lost as i32)  > max_op {
			    max_op = sim.adv_lost as i32 - sim.play_lost as i32;
			    ret_val = Some((v_try.to_vec(), sim));
			}
		    }
		    None => continue,
		}
	    }
	}
	
	if self.proba_coord > 0.2 {
	    //move then torpedo
	    eprintln!("Proba inf < 0.8, move +  torpedo");
	    for a_move in &v_move {
		for a in &v_torp {
		    let v_try = &vec![*a_move, *a];
		    match self.play_ac_l(v_try)
		    {
			Some(sim) => {
			    if sim.adv_lost as i32 - sim.play_lost as i32  > max_op {
				max_op = sim.adv_lost as i32 - sim.play_lost as i32;
				ret_val = Some((v_try.to_vec(), sim));
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
			let v_try = &vec![*a_move,*a_sil, *a];
			match self.play_ac_l(v_try)
			{
			    Some(sim) => {
				if (sim.adv_lost as i32 - sim.play_lost as i32).abs()  > max_op {
				    max_op = (sim.adv_lost as i32 - sim.play_lost as i32).abs();
				    ret_val = Some((v_try.to_vec(), sim));
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
			  my_life:Vec::<u8>::new(),
			  play_board:board,
			  torpedo:0,
			  silence:0,
			  sonar:0,
			  mine:0};
    }

    //to do dont print!
    fn get_actions_to_play(&mut self) -> Vec::<Action> {
	eprintln!("Torpedo val {}",self.torpedo);
	let mut v_act = Vec::<Action>::new();
	//let e = self.play_board.num_avail_pos(&self.cur_co);
	let (_, dir) = self.play_board._rec_best_path(&self.cur_co, &mut HashSet::<Coordinate>::new());
	
	//let e = (dir.len(), dir.front());

	let possible_move = dir.len();

	if possible_move != 0 {
	    let next_dir = *dir.front().unwrap();
	    let mut proba_my = 0.0;
	    
	    if !self.my_path.path_coords.is_empty() && !self.path.path_coords.is_empty() {


		eprintln!("*** MY possible pos");
		let (my_n_pos_l, _,_, proba_my_loc, _) = self.my_path.get_possible_pos();

		proba_my = proba_my_loc;
		eprintln!("mynpos proba {}", proba_my);
		
	
		let (n_pos, coord_mean, variance, prob, max_prob_coord) = self.path.get_possible_pos();
		let coord = max_prob_coord;
		eprintln!("*** ADV possible pos np: {}, var: {:?}, prob : {}", n_pos, variance, prob);


	
		let simul = Simulator::new(self.play_board,
					   self.cur_co,
					   coord,
					   self.torpedo,
					   self.silence,
					   prob);
			
		match simul.compute_best_sequence() {
		    Some((v,sim)) => {
			eprintln!("FFFFFF {:?} {} {}",v, sim.adv_lost,sim.play_lost);
			v_act = v;
			self.torpedo = sim.torpedo_v;
			self.silence = sim.silence_v;
			self.play_board = sim.board; //update board, should be ok...
		    },
		    None => eprintln!("FFFFFF NOT"),
		}


	    }
	    
	    
	    if !v_act.iter().any(|&x| x.ac == Action_type::SILENCE) && self.silence == 6 && proba_my > 0.8 {
		eprintln!("He found me, silence !!");
		v_act.push(Action { ac: Action_type::SILENCE, dir:next_dir, sector:1, ..Default::default() });
		self.silence = 0;
	    }
	    
	    else if !v_act.iter().any(|&x| x.ac == Action_type::MOVE) && self.torpedo < 3 {
		    v_act.push(Action { ac: Action_type::MOVE, dir:next_dir, ac_load:Action_type::TORPEDO, ..Default::default() });
		
		    self.torpedo += 1;
		    self.torpedo = cmp::min(self.torpedo,3);
		
	    }
	    else if !v_act.iter().any(|&x| x.ac == Action_type::MOVE){
		    v_act.push(Action { ac: Action_type::MOVE, dir:next_dir, ac_load:Action_type::SILENCE, ..Default::default() });
		
		    self.silence += 1;
		    self.silence = cmp::min(self.silence,6);
	    }
	}
	else {
	    self.play_board.rem_visited();
	    eprintln!("SURFACE, sector {}",self.cur_co.to_surface());
	    v_act.push(Action { ac: Action_type::SURFACE, sector:self.cur_co.to_surface(), ..Default::default() });
	    //println!("{}SURFACE",add_str)
	}
	self.actions_issued = v_act.to_vec(); //copy here
	v_act
    }
    fn update_situation(&mut self,opp_life:u8, my_life:u8, x:u8, y:u8, opponent_orders:&Vec::<Action>) {
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

	predictor.update_situation(opp_life as u8, my_life as u8, x as u8, y as u8, &Action::parse_raw(&opponent_orders));
	predictor.path.process_actions(&Action::parse_raw(&opponent_orders));
	//predictor.path.get_possible_pos();
	let v_acts = predictor.get_actions_to_play();
	predictor.my_path.process_actions(&v_acts);
	println!("{}",&Action::repr_action_v(&v_acts));

    }
}
