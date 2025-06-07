use crate::Change;

//use std::cmp;
use crate::utils::Value;
use crate::utils::Reference;
use crate::board_utils::get_references_for_box;
use crate::board_utils::BoxCoord;
use crate::utils::Board;
use crate::utils::Node;
use crate::utils::sum_bool;
//use crate::utils::sum_option;
use crate::utils::SudokuError;

#[derive(Debug,Copy,Clone,PartialEq)]
struct BitSet {
	c: [bool; 9],
	size: u8,
}

impl BitSet {
	fn create(bit_list: [bool; 9]) -> Self {
//		let mut b_iter = bit_list.iter();
//		let mut u_iter = used.iter();
//		let mut c: [Option<bool>;9] = [None;9];
//		let mut leng: usize = 0;
//		let mut i: usize = 0;
//		for u in used {
//			c[i] = if u {
//				capacity += 1;
//				let b = bit_list[i];
//				if b {leng +=1};
//				Some(b)
//			} else {None};
//			i += 1;
			
//			let this_bit = b_iter.next().unwrap();
//			let this_u = u_iter.next().unwrap();
//			c[i] = if this_u == &true {
//				leng += 1;
//				Some(*this_bit)
//			} else {
//				None
//			};
//		}
		Self {c: bit_list, size: sum_bool(bit_list)}
	}
	
	fn empty() -> Self {
		Self::create([false;9])
	}
	
	fn full() -> Self {
		Self::create([true;9])
	}
	
	fn add(&mut self, i: usize) {
		self.c[i] = true;
		self.size += 1;
	}
	
	fn remove(&mut self, i: usize) {
		self.c[i] = false;
		self.size -= 1;
	}

	fn move_one_from(&mut self, other: &mut Self) {
		for i in 0..9 {
			if !self.c[i] && other.c[i] {
				self.add(i);
				other.remove(i);
				break;
			}
		}
	}
	
//	fn add_from(&mut self, other: &mut Self) {
//		for k in 0..9 {
//			if !self.c[k] && other.c[k] {
//				self.add(k);
//				break;
//			}
//		}
//	}
	
	fn move_from(&mut self, other: &mut Self, n: usize) -> Self {
		let mut new = self.clone();
		new.add(n);
		other.remove(n);
		new
	}
	
	fn union_of(self, other: &Self) -> Self {
		let mut new = self;
		for i in 0..9 {
			let original = self.c[i];
			new.c[i] = new.c[i] || other.c[i];
			if original && !new.c[i] {
				new.size -= 1;
			} else if !original && new.c[i] {
				new.size += 1;
			}
		}
		new
	}
	
//	fn undo_move(&mut self, other: &mut Self, n: usize) {
//		self.remove(n);
//		other.add(n);
//	}
	
	fn find_group(mapping: [Option<Self>;9], minimum_depth: u8) -> Option<(Self,Self)> {
	// returns the values which make up the group
	// The greater function will then try to make a change
	// If no change, it continues with a greater minimum_depth, which starts as 2
	// Breadth-first search
	
		let mut capacity = 0;
	//	println!("capacity = {capacity}");
		let mut i = 0;
		let mut values = Self::empty();
		for m in mapping {
			if let Some(_) = m {
				capacity += 1;
				values.c[i] = true;
			}
			i += 1;
		}
		if capacity < 4 {return None;}
		values.size = capacity as u8;
		let values = values; // non-mutable
		// let values = Self::create(mapping.iter().map(|x| if let Some(_) = x {true}).collect::<Vec<bool>>().try_into().unwrap());
		
//		println!("Cap {capacity}, values {:?}",values);
		
		let mut branches: Vec<(Self,Self)> = vec![(Self::empty(),Self::empty())];
		let mut d: u8 = 0;
		while (d as usize) < capacity-1 {
			let mut new_branches: Vec<(Self,Self)> = Vec::new();
			while !branches.is_empty() {
				let mut unfilled_values = values;
				let (mut b, fb) = branches.pop().unwrap();
				if d >= minimum_depth { // goal test
					if b.size == fb.size {
						return Some((b,fb));
					}
				}
				
				for v in 0..9 {
					// println!("v = {v}, unfilled_values = {:?}",unfilled_values);
					if unfilled_values.c[v] && !b.c[v] {
						let new_b = b.move_from(&mut unfilled_values,v);
						new_branches.push((
							new_b,
							fb.union_of(&mapping[v].unwrap())
						));
					}
				}
			}
			d += 1;
			branches = new_branches;
			
			//if print {println!("branches: {:?}",branches);}
		}
		
		None
	}
}

//pub fn test() {
//	let mut empty_set = BitSet::empty();
//	let mut full_set = BitSet::full();
//	empty_set.move_one_from(&mut full_set);
//	println!("{:?}",empty_set);
//	println!("{:?}",full_set);
//}

impl Board {
	pub fn find_all_groups(mut self) -> Result<(Self,Vec<Change>),SudokuError> {
		let mut all_changes: Vec<Change>;
		let mut changes: Vec<Change>;
		(self,changes) = self.find_groups(None)?;
		all_changes = changes;
		if all_changes.is_empty() {
			(self,changes) = self.find_groups(Some(true))?; // row
			all_changes.append(&mut changes);
			if all_changes.is_empty() {
				(self,changes) = self.find_groups(Some(false))?; // column
				all_changes.append(&mut changes);
			}
		}
		Ok((self,all_changes))
	}
	fn find_groups(mut self,how: Option<bool>) -> Result<(Self, Vec<Change>), SudokuError> {
		let mut changes: Vec<Change> = Vec::new();
		for i in 0..9 {
			let references = match how {
				None => get_references_for_box(BoxCoord::Ord(i as u8)),
				Some(true) => [
					Reference::Ord(9*i),
					Reference::Ord(9*i+1),
					Reference::Ord(9*i+2),
					Reference::Ord(9*i+3),
					Reference::Ord(9*i+4),
					Reference::Ord(9*i+5),
					Reference::Ord(9*i+6),
					Reference::Ord(9*i+7),
					Reference::Ord(9*i+8)
				],
				Some(false) => [
					Reference::Ord(i),
					Reference::Ord(i+9),
					Reference::Ord(i+18),
					Reference::Ord(i+27),
					Reference::Ord(i+36),
					Reference::Ord(i+45),
					Reference::Ord(i+54),
					Reference::Ord(i+63),
					Reference::Ord(i+72)
				],
			};
			let mut board_slice: [Node; 9] = [Node::make_empty_node();9];
			for j in 0..9 {
				board_slice[j] = self.return_space(&references[j]);
			}
//			let def_exists = self.get_def_exists(board_slice);
			
			let mut to_be_mapping: [BitSet;9] = [BitSet::empty();9];
			let mut r: usize = 0;
			for node in board_slice {
				if let Value::Pos(p) = node.value {
					for j in 0..9 {
						if p[j] {to_be_mapping[j].add(r);} // If say j=1 is valid in space r=2 (with r being the index of the node whose value is p), then the map adds f(j) => r
					}
				}
				r += 1;
			}
//			if how == None {println!("TO BE MAPPING: {:?}",to_be_mapping);}
			let mut mapping: [Option<BitSet>;9] = [None;9];
			for (k,&m) in to_be_mapping.iter().enumerate() {
				mapping[k] = match m.size {
					0 => None,
					1 => panic!("Size of bitset is 1, indicating failed solo. how = {:?}, i = {i}, m = {:?}, k= {k}.", how, m),
					_ => Some(m),
				}
			}
			
			let mut minimum_depth: u8 = 2;
			let mut these_changes: Vec<Change> = Vec::new();
			while these_changes.is_empty() {
				if let Some((t,ft)) = BitSet::find_group(mapping, minimum_depth) {
				//	println!("i={i},how={:?},minimum_depth={minimum_depth}",how);
				//	println!("Grouping found with set {:?} and {:?}", t, ft);
					for j in 0..9 {
						if ft.c[j] {
							let node = board_slice[j];
							let p = match node.value {
								Value::Def(_) => panic!("Definite marked as possible for grouping"),
								Value::Pos(x) => x,
							};
							let mut new_p = p; // this initialization happens to not be redundant
							for k in 0..9 {
								if t.c[k] == false {
									new_p[k] = false;
								}
							}
							if new_p != p {
								these_changes.push(Change(references[j],Value::Pos(new_p)));
								self = self.fill_board(&references[j],Value::Pos(new_p))?;
							}
						}
					}
				}
				if these_changes.is_empty() {
					minimum_depth += 1;
					if minimum_depth > 8 {break;}
				//	println!("Another round of grouping for i={i}");
				}
			}
			//if !these_changes.is_empty() {println!("Found changes for i={i}, how = {:?}", how);}
			changes.append(&mut these_changes);
		}
		Ok((self,changes))
	}
}
