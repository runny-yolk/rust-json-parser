static null: char = '\0';

pub struct Reader {
	pub i: usize,
	pub o: usize,
	source: Vec<char>
}
impl Reader {
	pub fn new(source: &String) -> Reader {
		Reader {
			i: 0,
			o: 0,
			source: source.chars().collect()
		}
	}
	pub fn peek(&self, k: usize) -> char {
		let n = self.i + k;
		if n >= self.source.len() { null }
		else { self.source[n] }
	}
	pub fn peeko(&self) -> char {
		let n = self.i + self.o;
		if n >= self.source.len() { null }
		else { self.source[n] }
	}
	pub fn consume(&mut self, k: Option<usize>) {
		let toconsume = k.unwrap_or(self.o);
		self.i += toconsume;
		self.o = 0;
	}
	pub fn checkString(&mut self, comp: Vec<char>) -> bool {
		let entryo = self.o;
		for c in comp {
			if self.peeko() != c {self.o = entryo; return false}
			self.o += 1;
		};
		true
	}
}