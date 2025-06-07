use crate::Change;

use crate::utils::Node;
use crate::utils::Value;
//use crate::utils::ordinal;
//use crate::utils::ordinal2;
use crate::utils::ord;
//use crate::utils::xy;
//use crate::utils::is_pos;
//use crate::utils::match_value;
use crate::utils::sum_bool;
use crate::utils::sum_bool_3;
//use crate::utils::sum_option;
//use crate::utils::poss_as_vec;
use crate::utils::SudokuError;


use crate::utils::Board;
use crate::utils::Reference;
use crate::board_utils::BoxCoord;
use crate::board_utils::get_references_for_box;
impl Board {
	fn certain_elimination(self) -> Result<(Self,Vec<Change>),SudokuError> { // UNUSED
		let (rows,columns) = self.get_rows_and_columns();
		
		// We have to create 9 different sets of certainties
		// make mut stuff

		let mut changes: Vec<Change> = Vec::new();
		let mut i = 0;
		for node in self.spaces {
			i += 1;
			match node.value {
				Value::Def(_) => continue,
				Value::Pos(p) => {
					let mut new_value = p;
					for number in 0..9 {
						if rows[number][ord(&node.y)] || columns[number][ord(&node.x)] {
							new_value[number] = false;
							// println!("Marking {},{} as false for {}", node.x,node.y, number+1);
						}
					}
					let refer = Reference::Ord(i-1);
					let val = Value::Pos(new_value);
			//		self = self.fill_board(&refer,val);
					changes.push(Change(refer,val))
				},
			};
		}
		Ok((self,changes))
	}

	fn fill_solos(mut self) -> Result<(Self,Vec<Change>),SudokuError> { // UNUSED
		let mut changes: Vec<Change> = Vec::new();
		for i_box in 0..9 {
			let references = get_references_for_box(BoxCoord::Ord(i_box));
			let mut nodes = [Node::make_empty_node(); 9];
			for i in 0..9 {
				nodes[i] = self.return_space(&references[i]);
//				println!("Node value = {:?}",nodes[i].value);
			}
//			let nodes = nodes;
			let (rows_poss, cols_poss, definites) = get_possibilities(&nodes[..]);
//			let possibilities = get_possibilities(&self.spaces[..]);
//			println!("Definites: {:?}",definites);

			// This excludes a value from being repeated in a box. Requires mut rows_poss, mut cols_poss
//			for Definite(a,b,c) in definites.iter() {
//				let n = ord(a);
//				let x = ord(b);
//				let y = ord(c);
//				rows_poss[n] = [false; 9];
//				rows_poss[n][y] = true;
//				cols_poss[n] = [false; 9];
//				cols_poss[n][x] = true;
//			}
//			println!("Possibilities for 1: rows {:?}, columns {:?}", rows_poss[0], cols_poss[0]);
//			println!("Possibilities for 5: rows {:?}, columns {:?}", rows_poss[4], cols_poss[4]);
//			println!("Possibilities for 6: rows {:?}, columns {:?}", rows_poss[5], cols_poss[5]);
			let certainties = get_certainties((rows_poss,cols_poss));
//			println!("CERTAINTIES: {:?}", certainties);
			let solos = check_solos(&certainties);
//			println!("SOLOS: {:?}",solos);
			for n in 0..9 {
				let mut skip = false;
				for def in definites.iter() {if n == ord(&def.0) {skip = true;}}
				if skip {continue;}
				match solos[n] {
					None => continue,
					Some((x,y)) => {
						let refer = Reference::XY((x,y));
						let val = Value::Def((n+1) as u8);
						self = self.fill_board(&refer,val).unwrap();
						changes.push(Change(refer,val));
					},
				};
			}
//			self = self.fill_board_x(changes);
		}
		Ok((self,changes))
	}
	
	pub fn final_quantum(mut self) -> Result<(Self,Vec<Change>),SudokuError> {
		let mut changes: Vec<Change> = Vec::new();
		let mut all_cert: [Certainties;9] = [Certainties::initialize(); 9];
		for n in 0..9 {
			let references = get_references_for_box(BoxCoord::Ord(n as u8));
			let mut nodes = [Node::make_empty_node();9];
			for i in 0..9 {
				nodes[i] = self.return_space(&references[i]);
			}
			let (rows_poss, cols_poss, _definites) = get_possibilities(&nodes[..]);
			all_cert[n] = get_certainties((rows_poss,cols_poss));
		}
		for x in 0..3 {
			for y in 0..3 { // the double for loops iterate over each box
				let references = get_references_for_box(BoxCoord::XY((x as u8,y as u8)));
				let mut nodes = [Node::make_empty_node();9];
				for i in 0..9 {
					nodes[i] = self.return_space(&references[i]);
				}
				let b = 3*y + x;
				let (filled_rows,filled_cols) = Certainties::sum_except(&all_cert,b);
//				if x == 2 && y == 2 {println!("filled_rows(1) = {:?}",filled_rows[0]);println!("filled_cols(8)={:?}",filled_cols[7]);}
				let filled_rows = cut_down(filled_rows,3*y);
				let filled_cols = cut_down(filled_cols,3*x);
				
				let mut j: usize = 0;
				for node in nodes {
					if let Value::Pos(p) = node.value {
						let inner_x = j % 3;
						let inner_y = j / 3; // both 0-2
						if inner_x > 2 || inner_y > 2 {panic!("inner x or y > 2");}
						if 3*x + inner_x > 9 || 3*y + inner_y > 9 {panic!("outer x or y > 9");}
						let mut new_p = p; // copy
						for n in 0..9 {
							if filled_rows[n][inner_y] || filled_cols[n][inner_x] {
								new_p[n] = false;
							}
						}
						if new_p != p {
//							if new_p == [false;9] {println!("x and y: {x},{y}"); println!("node= {:?}",node); println!("filled rows: {:?}", filled_rows); println!("filled cols: {:?}", filled_cols); panic!("Why?");}
							changes.push(Change (references[j],Value::Pos(new_p)));
							self = self.fill_board(&references[j],Value::Pos(new_p))?;
						}
						
					}
					j += 1;
				}
			}
		}
		Ok((self,changes))
	}
	
	// Done on December 7-8. Testing at 7:25pm Dec 8
	// 7:45 switched indices on all_rows/cols in the 'v for-loop
	// Computer then correctly identified all quantum doubles.
	pub fn quantum_double(mut self) -> Result<(Self,Vec<Change>),SudokuError> {
		let mut changes: Vec<Change> = Vec::new();
		let mut double_possible_r = [[true;3];9];
		let mut double_possible_c = [[true;3];9];
		
		
		let mut all_rows = [[[false;3];9];9]; // initializing
		let mut all_cols = [[[false;3];9];9]; // outermost index: box; middle index: value; innermost index: row
		for n in 0..9 {
			let references = get_references_for_box(BoxCoord::Ord(n as u8));
			let mut nodes = [Node::make_empty_node();9];
			for i in 0..9 {
				nodes[i] = self.return_space(&references[i]);
			}
			let (rows_poss, cols_poss, definites) = get_possibilities(&nodes[..]);
			
			let box_x = n % 3;
			let box_y = n / 3;
			for Definite(v,_x,_y) in definites.iter() {
				double_possible_r[ord(v)][box_y] = false;
				double_possible_c[ord(v)][box_x] = false;
			}
			let certainties = get_certainties((rows_poss,cols_poss));
			for v in 0..9 {
				if certainties.rows[v] != 0 {
					double_possible_r[v][box_y] = false;
				}
				if certainties.cols[v] != 0 {
					double_possible_c[v][box_x] = false;
				}
			}
			
			all_rows[n] = cut_down(rows_poss,3*box_y);
			all_cols[n] = cut_down(cols_poss,3*box_x);
			//println!("All_rows[{n}] = {:?}",all_rows[n]);
			//println!("All_cols[{n}] = {:?}",all_cols[n]);
			// Works 
		}
		
		let mut changes_stackable: [[Option<[bool;9]>;9];9] = [[None;9];9]; // index outer is x, then inner is y
		for v in 0..9 {
			'y: for y in 0..3 { //checking box-row #y
				if !double_possible_r[v][y] {continue 'y;}
				'b: for b in 0..2 { // checking the last box is redundant
					let first_rows = all_rows[3*y + b][v];
					if sum_bool_3(first_rows) != 2 {continue 'b;}
					for k in 0..2 {
						let a = (b+k+1) % 3;
						let second_rows = all_rows[3*y + a][v];
						if second_rows == first_rows {
							let c = 3 - (a+b);
							// c identifies the box which we will change. Its box-x is c, and box-y is y.
							for inner_row in 0..3 {
								let start_number = 27*y + 9*inner_row + 3*c;
								for node_n in start_number..(start_number+3) {
									if first_rows[inner_row] {
										// license to change
										let node = self.spaces[node_n];
										if let Value::Pos(p) = node.value {
											if p[v] { // if false, no need to change
												let ord_x = ord(&node.x);
												let ord_y = ord(&node.y);
												if changes_stackable[ord_x][ord_y] == None {
													changes_stackable[ord_x][ord_y] = Some(p);
												}
												// changes_stackable[node.x][node.y][v] = false;
												// this is better than simply {let mut new_p = p; new_p[v] = false; ... = new_p} because that would override other changes made
												let mut old_cs = changes_stackable[ord_x][ord_y].unwrap();
												old_cs[v] = false; // ironically now it's "new_cs"
												changes_stackable[ord_x][ord_y] = Some(old_cs);
												// sure this might not always flip a bit if there was already a change made, so yeah if I wanted to unwrap and check none etc etc i could put this with "if p[v]", but that's not worth it
											}
										}
									}
								}
							}
							break 'b;
						}
					}
				}
				
				//if v == 8 {println!("Row changes as of 9, with y={y}: {:?}", changes_stackable);}
				
				// now do all that but pretend that y is x
				let x = y;
				if !double_possible_c[v][x] {continue 'y;}
				'b2: for b in 0..2 { // checking the last box is redundant
					let first_cols = all_cols[3*x + b][v];
					if sum_bool_3(first_cols) != 2 {continue 'b2;}
					for k in 0..2 {
						let a = (b+k+1) % 3;
						let second_cols = all_cols[3*x + a][v];
						if second_cols == first_cols {
							let c = 3 - (a+b);
							// c identifies the box which we will change. Now its box-y is c, and its box-x is x.
							for inner_row in 0..3 {
								let start_number = 27*c + 3*x + 9*inner_row;
								for node_n in start_number..(start_number+3) {
									let node = self.spaces[node_n];
									if first_cols[ord(&node.x) % 3] { // Notice that above, in the y section, inner_row == ord(node.y)(0-8) % 3
										// license to change
										let node = self.spaces[node_n];
										if let Value::Pos(p) = node.value {
											if p[v] { // if false, no need to change
												let ord_x = ord(&node.x);
												let ord_y = ord(&node.y);
												if changes_stackable[ord_x][ord_y] == None {
													changes_stackable[ord_x][ord_y] = Some(p);
												}
												let mut old_cs = changes_stackable[ord_x][ord_y].unwrap();
												old_cs[v] = false;
												changes_stackable[ord_x][ord_y] = Some(old_cs);
											}
										}
									}
								}
							}
							break 'b2;
						}
					}
				}
			}
		}
		
		for (i,&c1) in changes_stackable.iter().enumerate() {
			for (j,&c2) in c1.iter().enumerate() {
				if let Some(new_p) = c2 {
					changes.push(Change(Reference::Ord(9*j+i),Value::Pos(new_p)) );
					let node = self.spaces[9*j+i];
					self.spaces[9*j+i] = node.change_value(Value::Pos(new_p))?;
				}
			}
		}
		Ok((self,changes))
	}
}

fn get_possibilities(board_slice: &[Node]) -> ([[bool;9];9],[[bool;9];9],Vec<Definite>) {
	let mut rows = [[false;9];9];
	let mut columns = [[false;9];9];
	let mut definites: Vec<Definite> = Vec::new();
	for node in board_slice {
//		println!("Node x and y: {},{}",node.x,node.y);
		match node.value {
			Value::Def(n) => {
//if n==1 {				println!("Definite value {} at {},{}",n,node.x,node.y);}
				definites.push(Definite(n,node.x,node.y));
//				for t in 0..9 {
//					rows[t][ord(node.y)] = if t == ord(n) {true} else {false};
//					columns[t][ord(node.x)] = if t == ord(n) {true} else {false};
//				} // This attempt to exclude all other things from a square already occupied was stupid as it excluded them from entire rows


//				for t in 0..9 { // Here we exclude the same number from the rest of the box
//					rows[ord(&n)][t] = if t == ord(&node.y) {true} else {false};
//					columns[ord(&n)][t] = if t == ord(&node.x) {true} else {false}
//				}
			},
			Value::Pos(p) => 
//				{println!("Possibilities: {:?}",p);
				for n in 0..9 {
					if p[n] {
						rows[n][ord(&node.y)] = true;
						columns[n][ord(&node.x)] = true
					}
//					println!("Rows[{}]: {:?}",n, rows[n]);
//					println!("Cols[{}]: {:?}",n, columns[n]);
//				}
				},
		};
	}
	(rows,columns,definites)
}

fn check_solos_with_slice(board_slice: &[Node]) -> [Option<(u8,u8)>;9] {
	let possibilities = get_possibilities(board_slice);
	let certainties = get_certainties((possibilities.0,possibilities.1));
	check_solos(&certainties)
}

fn check_solos(certainties: &Certainties) -> [Option<(u8,u8)>;9] { // formerly "check only spaces" i.e. one space only possible
//	let (rows_cert,cols_cert) = certainties;
	let mut answer: [Option<(u8,u8)>;9] = [None;9];
	for n in 0..9 {
		let certainty = ((certainties.cols)[n],(certainties.rows)[n]);
		if certainty.0 != 0 && certainty.1 != 0 {
			answer[n] = Some(certainty);
		}
	}
	answer
}

fn get_certainties(possibilities: ([[bool;9];9],[[bool;9];9])) -> Certainties {
	let mut rows = [0u8;9];
	let mut cols = [0u8;9];
	let rows_poss = possibilities.0;
	let cols_poss = possibilities.1;
	for n in 0..9 {
		if sum_bool(rows_poss[n]) == 1 {
			for (i, &tf) in rows_poss[n].iter().enumerate() {
				if tf {
					rows[n] = (i+1) as u8;
				}
			}
		}
		if sum_bool(cols_poss[n]) == 1 {
			for (i, &tf) in cols_poss[n].iter().enumerate() {
				if tf {
					cols[n] = (i+1) as u8;
				}
			}
		}
	}
	Certainties{rows,cols}
}

fn get_def_refs(board_slice: &[Node]) -> Vec<Definite> { // UNUSED
	let mut def_refs: Vec<Definite> = Vec::with_capacity(board_slice.len());
	for &node in board_slice {
		if let Value::Def(d) = node.value {
			def_refs.push(Definite(d,node.x,node.y));
		}
	}
	def_refs
}

fn get_defs(board_slice: &[Node]) -> [Option<(u8,u8)>;9] { // UNUSED
	let mut defs = [None;9];
	for &node in board_slice {
		if let Value::Def(d) = node.value {
			defs[ord(&d)] = Some((node.x,node.y));
		}
	}
	defs
}

fn def_bools(definites: &[Option<(u8,u8)>;9]) -> [bool;9] { // UNUSED
	let mut answer = [false;9];
	for i in 0..9 {
		answer[i] = match definites[i] {
			Some(_) => true,
			None => false, 
		};
	}
	answer
}
// fn get_quantum_certainties or dualities across 2 boxes or a row or column or boxes i.e 3 rows or columns


#[derive(Debug,Copy,Clone)]
struct Certainties {
	rows: [u8;9],
	cols: [u8;9],
}

impl Certainties {
	fn initialize() -> Self {
		Certainties{rows: [0u8;9],cols:[0u8;9]}
	}

	fn sum_except(array: &[Self;9],k: usize) -> ([[bool;9];9],[[bool;9];9]) { // [n][x/y] true indicates that n is occupied in that column/row
		let mut rows = [[false;9];9];
		let mut cols = [[false;9];9];
		let mut b: usize = 0;
		for a in array {
			if b != k {
				for i in 0..9 {
					if a.rows[i] != 0 {
						rows[i][ord(&a.rows[i])] = true;
					}
					if a.cols[i] != 0 {
						cols[i][ord(&a.cols[i])] = true;
					}
				}
			}
			b += 1;
		}
		(rows,cols)
	}
}
	
fn cut_down(poss_array: [[bool;9];9], start: usize) -> [[bool;3];9] {
	let mut new = [[false;3];9];
	for i in 0..9 {
		for j in 0..9 {
			if j >= start && j < start+3 {
				new[i][j-start] = poss_array[i][j];
			}
		}
	}
	new
}

#[derive(Debug)]
struct Definite(u8,u8,u8);

// This shows that the crate works despite routing to board_utils thru main
//pub fn test_reset_board(board: Board) -> bool {
//	let new_board = board.reset_possibilities();
//	new_board.spaces[0].value == Value::Def(0)
//}
