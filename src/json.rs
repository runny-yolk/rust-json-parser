#![allow(warnings)]
use std::collections::HashMap;
mod reader; use reader::*;

static null: char = '\0';
static wsarray: [char; 11] = [' ','\n','\r','\t','\u{000B}','\u{000C}','\u{0085}','\u{200E}','\u{200F}','\u{2028}','\u{2029}'];
static escarray: [char; 7] = ['\\', '"', 'b', 'f', 'n', 'r', 't'];

#[derive(Debug)]
pub enum JSONdata {
	Object(HashMap<String, JSONdata>),
	Array(Vec<JSONdata>),
	Str(String),
	Num(f64),
	Bool(bool),
	Null,
	Undefined
}
#[derive(Debug)]
pub struct JSON {
	pub data: JSONdata
}
impl JSON {
	pub fn parse(string: &String) -> JSON {
		let mut source = Reader::new(string);
		JSON {
			data: readValue(&mut source)
		}
	}
	pub fn get(&self, key: &str) -> JSONdata {
		
	}
}

fn charVec(s: &str) -> Vec<char> {
	s.to_string().chars().collect()
}

fn readStr(source: &mut Reader) -> Option<String> {
	source.o = 0;
	if(source.peeko() != '"') { return None }

	let mut charbuf = Vec::<char>::new();
	// charbuf.push(source.peeko());
	source.o += 1;
	while source.peeko() != '"' {
		if(source.peeko() == null) { panic!("uh oh") }

		// handle escapes
		charbuf.push(source.peeko()); 
		source.o += 1;
	};
	// charbuf.push(source.peeko());
	source.o += 1;
	source.consume(None);
	Some(charbuf.into_iter().collect())
}
fn readNum(source: &mut Reader) -> Option<f64> {
	let mut charbuf = Vec::<char>::new();
	if(source.peeko() == '-') {
		charbuf.push(source.peeko());
		source.o += 1;
	}

	let mut dotcount = 0;
	while match source.peeko() {
		'.' => {dotcount += 1; true}
		'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'  => true,
		_ => false
	} {
		if(dotcount > 1) {return None}
		charbuf.push(source.peeko());
		source.o += 1;
	};
	source.consume(None);
	
	if(charbuf.len() == 0) {None}
	else {Some(charbuf.into_iter().collect::<String>().parse().unwrap())}
}
fn readIntStr(source: &mut Reader) -> Option<String> {
	let mut charbuf = Vec::<char>::new();

	while match source.peeko() {
		'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'  => true,
		_ => false
	} {
		charbuf.push(source.peeko());
		source.o += 1;
	};
	source.consume(None);
	
	if(charbuf.len() == 0) {None}
	else {Some(charbuf.into_iter().collect::<String>())}
}
fn consumeWS(source: &mut Reader){
	while wsarray.contains(&source.peek(0)){
		source.consume(Some(1))
	}
}
fn readNullData(source: &mut Reader) -> bool {
	source.checkString(charVec("null"))
}
fn readBool(source: &mut Reader) -> Option<bool> {
	if source.checkString(charVec("true")) {Some(true)}
	else if source.checkString(charVec("false")) {Some(false)}
	else {None}
}
fn readValue(source: &mut Reader) -> JSONdata {
	consumeWS(source);
	if readNullData(source) { JSONdata::Null }
	else if let Some(b) = readBool(source) { JSONdata::Bool(b) }
	else if let Some(s) = readStr(source) { JSONdata::Str(s) }
	else if let Some(n) = readNum(source) { JSONdata::Num(n) }
	else if let Some(o) = readObject(source) { JSONdata::Object(o) }
	else if let Some(a) = readArray(source) { JSONdata::Array(a) }
	else { panic!("invalid syntax {} char: {}", source.peek(0), source.i) }
}
fn readObject(source: &mut Reader) -> Option<HashMap<String, JSONdata>> {
	if source.peek(0) != '{' {return None}
	source.consume(Some(1));

	let mut hm = HashMap::<String, JSONdata>::new();
	
	loop {
		consumeWS(source);
		if source.peek(0) == '}' {break}
		let key = if let Some(k) = readStr(source) {k}
		else if let Some(k) = readIntStr(source) {k}
		else {return None};

		consumeWS(source);
		if source.peek(0) != ':' {return None}
		source.consume(Some(1));

		consumeWS(source);
		hm.insert(key, readValue(source));
		consumeWS(source);
		if source.peek(0) != ',' {break}
		source.consume(Some(1));
	}

	Some(hm)
}
fn readArray(source: &mut Reader) -> Option<Vec<JSONdata>> {
	let mut v = Vec::<JSONdata>::new();
	if source.peek(0) != '[' {return None}
	source.consume(Some(1));
	
	loop {
		consumeWS(source);
		if source.peek(0) == ']' {break}
		v.push(readValue(source));
		consumeWS(source);
		if source.peek(0) != ',' {break}
		source.consume(Some(1));
	}

	Some(v)
}
