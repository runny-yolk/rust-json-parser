# rust-json-parser
My first Rust project - a JSON parser.

It's not perfect - it doesn't handle strings exactly correctly (ignores newlines and transcribes escape characters literally), and it doesn't have functions for reading/writing JSON data easily yet. It is fast though - by my tests, using a 60MB JSON file, about 3 times faster than the JSON parser in NodeJS.
