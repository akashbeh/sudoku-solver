// Completed entirely on November 28, 2023

use crate::Change;

use crate::utils::Board;
use crate::utils::Reference;
use crate::utils::Value;
use crate::board_utils::BoxCoord;
use crate::utils::Node;
//use crate::utils::ord;
use crate::utils::ordinal;
use crate::board_utils::get_references_for_box;
use crate::utils::sum_bool;
use crate::utils::SudokuError;

// ref means what

impl Board {
	pub fn fill_all_solos(mut self) -> Result<(Self,Vec<Change>),SudokuError> {
		let mut changes: Vec<Change> = Vec::new();
		let mut references: [Reference;9];
		for i in 0..9 {
			references = get_references_for_box(BoxCoord::Ord(i));
			let mut new_changes = self.fill_solo(references);
//			if !new_changes.is_empty() {println!("box {:?}",new_changes);}
			changes.append(&mut new_changes);

			for k in 0..9 {
				references[k] = Reference::XY((i+1,(k+1) as u8));
			}
			let mut new_changes = self.fill_solo(references);
//			if !new_changes.is_empty() {println!("column {:?}",new_changes);}
			changes.append(&mut new_changes);

			for k in 0..9 {
				references[k] = Reference::XY(((k+1) as u8,i+1));
			}
			let mut new_changes = self.fill_solo(references);
//			if !new_changes.is_empty() {println!("row {:?}",new_changes);}
			changes.append(&mut new_changes);
		
//			find_row_solos(i);
//			find_col_solos(i);
		}
		let changes_backup = changes.clone();
//		println!("Fill all solos changes: {:?}", changes_backup);
		self = self.fill_board_x(&changes)?;
		Ok((self,changes_backup))
	}

	fn fill_solo(&self, references: [Reference;9]) -> Vec<Change> {
		let mut changes: Vec<Change> = Vec::new();
		let solos = self.find_solos(references);
		let mut j: usize = 0;
		for space in solos {
			if let Some(x) = space {
				changes.push(Change(references[j],Value::Def(x+1)));
			}
			j += 1;
		}
		changes
	}

	fn find_solos(&self,refs: [Reference;9]) -> [Option<u8>;9] {
		let mut return_array: [Option<u8>;9] = [None;9];
		let converted = self.convert_poss(&refs);
//TEST		if ordinal(&refs[0]) == 0 && ordinal(&refs[8]) == 20 {println!("{:?}",converted);}
		let mut i: u8 = 0; // marks the index of n where n is an element of "converted"
		// thus n represents the possibilities for i in the box

		let mut s: u8;
		let mut truth_index: usize;
		for n in converted {
			s = sum_bool(n);
			if s == 1 {
				truth_index = 0;
				while n[truth_index] == false {
					truth_index += 1;
				} // truth index is where n is true, i.e., the reference where the value i exists
				return_array[truth_index] = Some(i);
			};
			i += 1;
		}
		
	//	if refs[0] == Reference::XY((1,4)) || refs[0] == Reference::XY((1,6)) {
	//		println!("return array : {:?}",return_array);
	//		println!("Converted: {:?}",converted);
	//	}
		
		return_array
	}

//	pub fn find_row_solos(n: u8, refs: [Reference;9]) -> [Option<u8>;9] {
		
//	}

//	pub fn find_col_solos(n: u8, refs: [Reference;9]) -> [Option<u8>;9] {
		
//	}

	fn convert_poss(&self,refs: &[Reference]) -> [[bool;9];9] {
//		let mut return_array: [Vec<u8>;9] = std::array::from_fn(|_| Vec::new());
//		for i in 0..9 {
//			return_array[i] = Vec::with_capacity(spaces.len());
//		}
		let mut return_array: [[bool;9];9] = [[false;9];9];
		
		// for skipping over definites
		// effects box, row, and col elimination
//		let mut definite: [bool;9] = [false;9];

		let mut j: usize = 0;
		for refer in refs {
			let node: Node = self.spaces[ordinal(refer)];
			if j > 8 {panic!("index out of range: over 9 (8)");}
			if let Value::Pos(p) = node.value {
				let mut i: usize = 0;
				for boolean in p {
//					if boolean != p[i] {panic!("Bool not equal to indexed");}
					if boolean {
//						return_array[i].push(j-1)
						return_array[i][j] = true;
					};
					i += 1;
				}
			} 
//			else if let Value::Def(d) = node.value {
//				definite[(d-1) as usize] = true;
//			}
			j += 1;
		}
//		j = 0;
//		for d in definite {
//			if d {
//				return_array[j] = [false;9];
//			}`
//			j +=1;
//		}
		return_array
	}
}
