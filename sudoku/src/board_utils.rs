use crate::Change;

use std::fmt::Display;
use std::fmt;

use crate::utils::Node;
use crate::utils::Value;
use crate::utils::Reference;
use crate::utils::ordinal;
//use crate::utils::ordinal2;
use crate::utils::xy;
use crate::utils::is_pos;
use crate::utils::match_value;
use crate::utils::Board;
use crate::utils::ord;
//use crate::utils::sum_bool;
//use crate::utils::sum_option;
//use crate::utils::poss_as_vec;
use crate::utils::SudokuError;

impl Board {
	pub fn create_board() -> Self {
		let mut new_spaces: [Node; 81] = [Node::make_empty_node();81];
		for i in 0..81 {
			let nth_node = Node {
				x: u8::try_from((i % 9) + 1).unwrap(),
				y: u8::try_from((i / 9) + 1).unwrap(), // The thing is panic!
				value: Value::Pos([true;9]), // This being set to all false caused a glitch that only applied when going row by row 
				// where the program would stop with all remaining nodes being all false, and numbers being placed next to each other
			};
			new_spaces[i] = nth_node;
		}
		let new_board = Board {
			spaces: new_spaces,
		};
		new_board
	}

	pub fn return_space_ord(&self, n: usize) -> Node {
		self.spaces[n]
	}

	pub fn return_space(&self,reference: &Reference) -> Node {
		// self.return_space_ord(ordinal(&reference))
		self.spaces[ordinal(&reference)]
	}
//	fn return_box(&self,x,y) -> set[Node] {
//	}

//	pub fn return_ref_space(&self, reference: &Reference) -> &Node {
//		&self.spaces[ordinal(&reference)]
//	}

	pub fn fill_board(mut self,reference: &Reference,value: Value) -> Result<Self,SudokuError> {
		if let Value::Def(_d) = self.return_space(reference).value {println!("Changing definite on reference {:?}",reference.clone());}
		self.spaces[ordinal(reference)] = self.return_space(reference).change_value(value)?; // Had to declare this mutable here, not under create_board;
//		match value {Value::Def(x) => println!("Changing {},{} to {}",(ordinal(&reference) % 9) + 1,(ordinal(&reference) / 9) + 1,x), Value::Pos(_) => (),};
		Ok(self)
	}

//	fn fill_row(mut self,y: u8, values: [u8;9]) {
//		
//	}

	pub fn fill_board_x(mut self, changes: &Vec<Change>) -> Result<Self,SudokuError> {
		for Change(reference,value) in changes.iter() {
			let new_node = self.return_space(&reference).change_value(*value)?;
			self.spaces[ordinal(&reference)] = new_node;
		}
		Ok(self)
	}
// Which is faster?
	fn fill_board_x2(mut self, changes: Vec<Change>) -> Result<Self,SudokuError> {
		for Change(reference,value) in changes.iter() {
			self = self.fill_board(reference,*value)?;
		}
		Ok(self)
	}

	fn fill_board_x3(mut self, changes: Vec<Change>) -> Result<Self,SudokuError> {
		for node in self.spaces {
			for Change(reference,value) in changes.iter() {
				if self.spaces[ordinal(&reference)] == node {
					let new_node = node.change_value(*value)?;
					self.spaces[ordinal(&reference)] = new_node;
				}
			}
		}
		// something about self.spaces being accessed first
		Ok(self)
	}
// get_spaces and poss moved to independent functions
	pub fn get_spaces_with(&self,n: u8) -> Vec<Reference> {
		get_spaces_with(&self.spaces,n)
	}

	fn get_spaces_with_poss(&self,n: u8) -> Vec<Reference> {
		get_spaces_with_poss(&self.spaces,n)
	}

	pub fn get_rows_and_columns(&self) -> ([[bool;9];9],[[bool;9];9]) {
		get_rows_and_columns(&self.spaces)
	}

//	fn get_certainties_box(&self, n_box: u8) -> (Certainties,Vec<Definite>) {
//	}


	pub fn get_def_exists(&self, board_slice: [Node;9]) -> [bool; 9] {
		let mut def_exists = [false;9];
		for j in 0..9 {
			if let Value::Def(d) = board_slice[j].value {
				def_exists[ord(&d)] = true;
			}
		}
		def_exists
	}
}

pub fn get_spaces_with(board_slice: &[Node], n: u8) -> Vec<Reference> {
	let mut answer = Vec::new();
	for node in board_slice {
		let definite_value = match_value(&node.value,0);
		if definite_value == n {
			let new_reference = Reference::XY((node.x,node.y));
			answer.push(new_reference)
		}
	}
	answer
}

pub fn get_spaces_with_poss(board_slice: &[Node], n: u8) -> Vec<Reference> {
	let mut answer = Vec::new();
	for node in board_slice {
		if is_pos(&node.value,&n) {
			answer.push(Reference::XY((node.x,node.y)));
		}
	}
	answer
}

// OBSOLETE
//fn get_rows_and_columns(board_slice: &[Node]) -> ([[bool;9];9],[[bool;9];9]) {
//	let mut rows = [[false;9];9]; // the wider index refers to the number, the inner refers to the row. So if 1 was done, it would be [[true;9],[false;9]...]
//	let mut columns = [[false;9];9];
//	for node in board_slice {
//		let definite_value = match_value(&node.value,0);
//		if definite_value != 0 {
//			rows[(definite_value - 1) as usize][(node.y - 1) as usize] = true;
//			columns[(definite_value - 1) as usize][(node.x - 1) as usize] = true;
//		}
//	}
//	(rows,columns)
//}

pub fn get_rows_and_columns(board_slice: &[Node]) -> ([[bool;9];9],[[bool;9];9]) {
	let mut rows = [[false;9];9];
	let mut columns = [[false;9];9];
	for node in board_slice {
		match node.value {
			Value::Pos(_) => continue,
			Value::Def(n) => {
				rows[ord(&n)][ord(&node.y)] = true;
				columns[ord(&n)][ord(&node.x)] = true
			},
		};
	}
	(rows,columns)
}

// struct Changes
pub fn filter_changes(changes: &Vec<Change>) -> (Vec<String>,bool) {
	let mut any_poss = false;
	let mut new_changes: Vec<String> = Vec::new();
	for change in changes.iter() {
		match change.1 {
			Value::Def(x) => new_changes.push(format!("{:?} to {}",xy(&change.0),x)),
			Value::Pos(_) => any_poss = true,
		};
	}
	(new_changes,any_poss)
}

impl Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//		let mut all_spaces = [0u8; 81];
//		for (i, &item) in self.spaces.iter().enumerate() { // Enumerate is useful thing from references chapter
//			all_spaces[i] = match item.value {
//				Pos(_) => 0,
//				Def(n) => n,
//			};
//		}
		let mut rows = Vec::<String>::with_capacity(9); // Using a vector here and an iterable later also solves having to copy the String as when using [String::new();9] lol
		for y in 0..9 {
			let mut this_row = String::new();
			for x in 0..9 {
				let space = self.return_space(&Reference::XY((x+1,y+1))); //Without the +1, we are looking for invalid spaces and as a result the thread panicks (panics?) in subtracting
				this_row = format!("{}{}",this_row, match space.value {
					Value::Def(n) => n.to_string(),
					Value::Pos(_) => "_".to_string(),
				});
			}
			rows.push(this_row);
		}
		let mut answer = String::new();
		for line in rows.iter() {
			answer = format!("{}\n{}",answer,line);
		}
		// answer = answer.trim().to_string();
		// ^ This removes the leading \n, but we use it ultimately anyway
		write!(f, "{}", answer)
	}
}

struct SBox {
	pub coord: BoxCoord,
	pub spaces: [Node;9],
	pub definites: [Option<(u8,u8)>;9],
	
}

pub enum BoxCoord {
	Ord(u8), // 0-8. To get to space coords 0-80: y*27 + x*3
	XY((u8,u8)), // = n%3,n/3 Note: 0-2, 0-2
}

pub fn get_references_for_box(boxc: BoxCoord) -> [Reference;9] {
	let (box_x,box_y) = match boxc {
		BoxCoord::Ord(n) => (n%3,n/3),
		BoxCoord::XY(xy) => xy,
	};
	let mut answer = [Reference::Ord(0);9];
	for i in 0..9 {
		let inner_x = (i % 3 + 1) as u8;
		let inner_y = (i / 3 + 1) as u8;
		answer[i] = Reference::XY((box_x*3 + inner_x, box_y*3 + inner_y));
	}
	answer
}
