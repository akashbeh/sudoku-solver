use std::io;

pub mod utils;
use crate::utils::Node;
use crate::utils::Value;
use crate::utils::Reference;
use crate::utils::ordinal;
// use crate::utils::ordinal2;
//use crate::utils::xy;
//use crate::utils::is_pos;
//use crate::utils::match_value;
use crate::utils::Board;
use crate::utils::ord;
//use crate::utils::sum_bool;
//use crate::utils::sum_option;
use crate::utils::poss_as_vec;
use crate::utils::SudokuError;

pub mod board_utils;
//use crate::board_utils::create_board;
//use crate::board_utils::return_space_ord;
//use crate::board_utils::return_space;
//use crate::board_utils::fill_board;
//use crate::board_utils::fill_board_x;
//use crate::board_utils::fill_board_x2;

//use crate::board_utils::get_spaces_with;
//use crate::board_utils::get_spaces_with_poss;
//use crate::board_utils::get_rows_and_columns;
use crate::board_utils::filter_changes;
//use crate::board_utils::SBox;
use crate::board_utils::BoxCoord;
use crate::board_utils::get_references_for_box;

pub mod quantum_utils;
//use crate::quantum_utils::check_solos_with_slice;
//use crate::quantum_utils::get_def_refs;
//use crate::quantum_utils::get_defs;
//use crate::quantum_utils::def_bools;
//use crate::quantum_utils::Certainties;
//use crate::quantum_utils::Definite;

pub mod solos;
//use crate::solos::fill_all_solos;

pub mod disclusion;

#[derive(Debug,Clone,PartialEq)]
pub struct Change(Reference,Value);
// struct Changes(Vec<Change>);

impl Board {
	fn initialize(mut self) -> Self {
		let mut changes: Vec<Change> = Vec::new();
		let mut alt_input = false;
		for y in 0..9 {
			let mut input = String::new();
			println!("Enter row {}. Type 0 for nothing",y+1);
			io::stdin().read_line(&mut input).expect("Failed to read input");
			let input: Vec<char> = input.trim().to_string().chars().collect(); // to_string instead of .parse::<String>().expect("..."). Figured parse is used below anyway
			if input[0] == 'z' {alt_input = true; break;}
			if input.len() > 9 {continue;}
			for (i,&character) in input.iter().enumerate() {
				changes.push(Change(
					Reference::XY(((i+1) as u8,y+1)),
					match character.to_digit(10) { // Lazy solution was to_string().parse() lol
						Some(x) => if x == 0 {Value::Pos([true;9])} else {Value::Def(x as u8)},
						None => Value::Pos([true;9]),
					}
				));
			}
			
		}
		if alt_input {
			println!("Enter the whole board:");
			changes.clear();
			let mut input = String::new();
			io::stdin().read_line(&mut input).expect("Failed to read input2");
			let input: Vec<char> = input.trim().to_string().chars().collect();
			for (i,&character) in input.iter().enumerate() {
				changes.push(Change(
					Reference::Ord(i),
					match character.to_digit(10) {
						Some(x) => if x ==0 {Value::Pos([true;9])} else {Value::Def(x as u8)},
						None => Value::Pos([true;9]),
					} // No need to remove middle "whitespace" apparently; if you try to paste with paragraph breaks, it just thinks you're hitting enter. Just paste one long string.
				));
			}
		}
		self = self.fill_board_x(&changes).unwrap();
		//println!("Board during initialization: {:?}", self);
//		for node in self.spaces {
			//println!("Problematic node x and y: {},{}",node.x,node.y);
//			let definite_value = match_value(&node.value,10);
//			if definite_value == 0 {
//				self.spaces[ordinal2(&node.x,&node.y)] = node.change_value(Value::Pos([true;9])); // INITIALIZING
//			}
//		}
		self
	}



	fn direct_elimination(mut self) -> Result<(Self,Vec<Change>),SudokuError> {
		let (rows,columns) = self.get_rows_and_columns(); // Change this to get_certainties in the next function eliminating certainties
//		println!("Rows and columns for 2: rows {:?},columns {:?}",rows[1],columns[1]);
//		println!("ROWS: {:?}",rows);
//		println!("COLUMNS: {:?}",columns);
		let mut changes: Vec<Change> = Vec::new();
		let mut i = 0;
		for node in self.spaces {
			i += 1;
			match node.value {
				Value::Def(_) => continue,
				Value::Pos(p) => {
					let mut any_changed = false;
					let mut new_value = p;
					for number in 0..9 {
						if rows[number][ord(&node.y)] || columns[number][ord(&node.x)] {
							if new_value[number] == true {any_changed = true;}
							new_value[number] = false;
							// println!("Marking {},{} as false for {}", node.x,node.y, number+1);
						}
					}
					if any_changed {
						let refer = Reference::Ord(i-1);
						let val = Value::Pos(new_value);
						self = self.fill_board(&refer,val )?;
						changes.push(Change(refer,val))
					}
				},
			};
		}
		Ok((self,changes))
	}

	fn depossibilize(mut self) -> Result<(Self,Vec<Change>),SudokuError> {
//		println!("24 is {:?}",self.spaces[24]);
		let mut changes: Vec<Change> = Vec::new();
		let mut i = 0;
		for node in self.spaces {
			i += 1;
			match node.value {
				Value::Def(_) => continue,
				Value::Pos(p) => {
//					if i==24 {println!("13 is {:?}",self.spaces[i]);}
//					println!("p = {:?}",p);
					let poss_vec = poss_as_vec(p);
//					println!("poss_vec number {} is = {:?}", i, poss_vec);
					if poss_vec.len() == 1 {
						let refer = Reference::Ord(i-1);
						let val = Value::Def(poss_vec[0]);
						self = self.fill_board(&refer,val )?;
						changes.push(Change(refer,val))
					}
				},
			};
		}
		Ok((self,changes))
	}

//	fn update(board: &mut Self, changes: &mut bool, b_function: (Self,bool)) {
//		let (new_board,new_changes) = b_function;
//		*board = new_board;
//		*changes = *changes || new_changes;
//	}

	fn reset_possibilities(mut self) -> Self {
		for i in 0..81 {
			let node = self.spaces[i];
			match node.value {
				Value::Def(_) => continue,
				Value::Pos(_) => self.spaces[i] = node.change_value(Value::Pos([true;9])).unwrap(),
			};
		}
		self
	}

	fn box_elimination(mut self) -> Result<(Self,Vec<Change>),SudokuError> {
		let mut changes: Vec<Change> = Vec::new();
		for i in 0..9 {
			let references = get_references_for_box(BoxCoord::Ord(i));
			let mut board_slice: [Node; 9] = [Node::make_empty_node();9];
			for j in 0..9 {
				board_slice[j] = self.return_space(&references[j]);
			}
			let def_exists = self.get_def_exists(board_slice);
//			let box_poss = def_bools(&get_defs(&board_slice)); // THIS WILL BECHANGED TO A TRAIT OF THE BOX THAT SAYS WHETHER ANY # is FILLED
//			println!("{:?}",def_refs);
//			println!("{:?}",box_poss);
			// replaced by def_exists
			for j in 0..9 {
				let node = board_slice[j];
				if let Value::Pos(p) = node.value {
					let mut new_p = p;
					for k in 0..9 {
						new_p[k] = p[k] && !def_exists[k]; // If p is true and box_poss is true, it returns true
					}
					if new_p != p {
						let number = ordinal(&references[j]);
						self.spaces[number] = node.change_value(Value::Pos(new_p))?;
						changes.push(Change (Reference::Ord(number),Value::Pos(new_p)) );
					}
				}
			}
		}
		Ok((self,changes))
	}

//	fn get_box_info(&self) -> [Vec<Certainties>] {
//		
//	}

	fn full_processing(mut self) -> Result<(Self,Vec<Change>),SudokuError> {
		let mut changes: Vec<Change>;
		let mut all_changes: Vec<Change>;
		(self,all_changes) = self.direct_elimination()?;
//		println!("Changes from direct elim: {:?}",filter_changes(&all_changes).0);
		(self,changes) = self.box_elimination()?;
//		println!("Changes from box elim: {:?}",changes);
		all_changes.append(&mut changes);
		(self,changes) = self.depossibilize()?;
//		println!("Changes from deposs: {:?}",changes);
		all_changes.append(&mut changes);
//		if input == "k" {
//		println!("board: {}",self);

		print_filter(&all_changes);
		if all_changes.is_empty() {println!("Entering tier two...");} else {return Ok((self,all_changes));}
		
	//	println!("weird square: {:?}", board.spaces[29].value);
		(self,changes) = self.fill_all_solos()?;
		println!("Tier two changes: {:?}",changes);
		all_changes.append(&mut changes);
//		}
//		if filtered.len() == 0 {
//			println!("All changes: {:?}",all_changes);
//			if input[0] == 'r' {println!("resetting"); board = board.reset_possibilities();}
//		}
//		println!("input == r {}", input.chars().collect::<Vec<char>>()[0] =='r');
		if all_changes.is_empty() {println!("Entering tier three...");} else {return Ok((self,all_changes));}
		(self, changes) = self.find_all_groups()?;
		
	//	if !changes.is_empty() {
	//		let mut k = 0;
	//		for n in self.spaces {
	//			println!("Value {k}: {:?}",n.value);
	//			k += 1;
	//		}
	//	}
		println!("Tier three changes:");
		print_filter(&all_changes);
		
		
		all_changes.append(&mut changes);
		if all_changes.is_empty() {println!("Entering tier four...");} else {return Ok((self,all_changes));}
		(self,changes) = self.final_quantum()?;
		println!("Tier four changes:");
		print_filter(&all_changes);
		all_changes.append(&mut changes);
		
		if all_changes.is_empty() {println!("Entering tier five...");} else {return Ok((self,all_changes));}
		(self,changes) = self.quantum_double()?;
		println!("Tier five changes:");
		print_filter(&all_changes);
		all_changes.append(&mut changes);
		Ok((self,all_changes))
	}
	
	
	fn simulate(&self) -> Self {
		let mut change_attempts: Vec<Change> = Vec::new();
		loop {
			println!("Attempting simulation...");
			let change_option = match self.find_change(&change_attempts) { // Instead of using find_change, could simply try anything
											// and if hypothesis_board.fill_board fails, then continue the loop
											// but that shouldn't be necessary
				Some(c) => c,
				None => break,
			};
			let mut hypothesis_board = self.clone();
			hypothesis_board = hypothesis_board.fill_board(&change_option.0,change_option.1.clone()).unwrap();
			loop {
				let result_process = hypothesis_board.full_processing();
				match result_process {
					Ok(b) => {hypothesis_board = b.0; if hypothesis_board.get_spaces_with(0).len() == 0 {return hypothesis_board;}},
					Err(SudokuError) => {change_attempts.push(change_option); break;}, // How is this unreachable??
				}
			}
		}
		panic!("Nothing found");
	}
	
	fn find_change(&self,change_attempts: &Vec<Change>) -> Option<Change> {
		for node in self.spaces {
			match node.value {
				Value::Def(_d) => continue,
				Value::Pos(p) => {
					'val: for i in 0..9 {
						if p[i] {
							let potential_change = Change(Reference::XY((node.x,node.y)),Value::Def((i+1) as u8));
							for c in change_attempts.iter() {
								if &potential_change == c {
									continue 'val;
								}
							}
							return Some(potential_change);
						}
					}
				},
			}
		}
		None
	}
}
// Note: In order to show a board is impossible, we just need to take the "tried to falsify node at __ " error from fill_board and move it up the chain


fn print_filter(all_changes: &Vec<Change>) {
	let (filtered,poss) = filter_changes(all_changes);
	println!("Changes: {:?}{}",filtered, match poss {true => ", with changes in possibilities", false => "."});
}

use std::fmt::Display;
fn print_board(board: &Board) {
	let mut n = 0;
	for space in board.spaces {
		println!("Value {n}: {}", space.value);
		n += 1;
	}
}

use std::fmt;
impl Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Value::Pos(p) => {
				let mut text = String::new();
				for (i,&boolean) in p.iter().enumerate() {
					if boolean {text = format!("{}{}",text,i+1);}
				}
				text
			},
			Value::Def(d) => format!("{d}"),
		})
	}
}

fn main() {
	let mut board = Board::create_board();
//	println!("board: {:?}",board);
//	board = board.fill_board(Reference::Ord(25),Value::Def(5));
	println!("Setting up board...");
	board = board.initialize();
//	println!("{}",board.spaces.len());
	let mut i: usize = 0;
	let mut auto = false;
	if board.get_spaces_with(0).len() == 0 {println!("Finished! board: {}", board); return;}
	loop {
//		disclusion::test();
		println!("board: {}",board);
		let old_board = board.clone();
		i+=1;
		println!("\n Step {}",i);
		println!("Type anything to continue");
		let mut input = String::new();
		if !auto {io::stdin().read_line(&mut input).expect("Couldn't read input");}
		let input: Vec<char> = input.chars().collect();
		if !auto {
			if input[0] == 'a' {auto = true;}
			if input[0] == 'p' {print_board(&board);}
		}
		let all_changes: Vec<Change>;
		(board,all_changes) = board.full_processing().unwrap();
		
		if board.get_spaces_with(0).len() == 0 {println!("Finished! board: {}", board); break;}
		
		if all_changes.is_empty() {
			println!("Brute force hypothesis required");
			if board != old_board {
				panic!("Changes not recorded");
			}
			board = board.simulate();
		}
		if board.get_spaces_with(0).len() == 0 {println!("Finished! board: {}", board); break;}
	}
}



