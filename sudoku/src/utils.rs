//use crate::Change;

#[derive(Debug)]
pub struct SudokuError;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Node {
	pub x: u8,
	pub y: u8,
	pub value: Value,
}
// x and y are counted from the top-left

impl Node {
	pub fn make_empty_node() -> Self {
		Node {x: 0, y: 0, value: Value::Pos([true;9])} // Self?
	}

	pub fn change_value(mut self,new_value: Value) -> Result<Self,SudokuError> {
		if let Value::Def(d) = self.value {if self.value == new_value {println!("Duplicate change");} else {panic!("Tried to change node at {},{} with old value {:?} to {:?}",self.x,self.y,d, new_value);}}
		if new_value == Value::Pos([false;9]) {
			// panic!("Attempted to falsify node at {},{} with old value {:?}", self.x, self.y, self.value);
			return Err(SudokuError);
		}
		self.value = new_value;
		Ok(self)
	}
}

//impl Copy for Node {
	
//}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Value {
	Pos([bool;9]), // u8
	Def(u8),
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Reference {
	Ord(usize),
	XY((u8,u8)),
	// Box, etc
}

pub fn ordinal(reference: &Reference) -> usize {
	match reference {
		Reference::Ord(n) => *n,
		Reference::XY((x,y)) => ordinal2(x,y), // .into()
		// y-1 is important; else we go too high
	}
	// Instead of using pointer: let var = ...; var
}

pub fn ordinal2(x: &u8, y: &u8) -> usize {
//	println!("Doing ordinal2 with x: {}, y: {}",x,y);
	((y-1)*9 + x-1) as usize
}

pub fn xy(reference: &Reference) -> (u8,u8) {
	match reference {
		Reference::Ord(n) => ((n%9 + 1) as u8, (n/9 + 1) as u8),
		Reference::XY(t) => *t,
	}
}

pub fn is_pos(node_value: &Value, n: &u8) -> bool {
	let n2 = n.clone() as usize;
	match node_value {
		Value::Pos(pos_list) => if n2 != 0 && n2 < 10 {
			pos_list[n2 - 1]
		} else {false},
		Value::Def(_) => false,
	}
}

pub fn match_value(value: &Value, exception: u8) -> u8 {
	match value {
		Value::Pos(_) => exception,
		Value::Def(x) => *x,
	}
}

#[derive(PartialEq,Copy,Clone)]
pub struct Board {pub spaces: [Node;81]}



pub fn ord(number: &u8) -> usize {
	(number -1) as usize
}

pub fn sum_bool(poss: [bool;9]) -> u8 {
	let mut sum: u8 = 0;
	for each in poss {
		if each {sum += 1;}
	}
	sum
}

pub fn sum_bool_3(poss: [bool;3]) -> u8 {
	let mut sum: u8 = 0;
	for each in poss {
		if each {sum += 1;}
	}
	sum
}

pub fn sum_option<T>(anything: &[Option<T>]) -> u8 {
	let mut answer: u8 = 0;
	for a in anything {
		match a {
			Some(_) => answer += 1,
			None => continue,
		}
	}
	answer
}

pub fn poss_as_vec(poss: [bool;9]) -> Vec<u8> {
	let mut answer: Vec<u8> = Vec::new();
	for (i, &tf) in poss.iter().enumerate() {
		if tf == true {
			answer.push((i+1) as u8);
		}
	}
	answer
}
