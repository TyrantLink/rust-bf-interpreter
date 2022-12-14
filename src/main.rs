#![allow(arithmetic_overflow)]
use std::collections::HashMap;
use std::io::{stdout, Write};
use console::Term;
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args
{
	/// name of bf file
	#[arg(required_unless_present="string",default_value="")]
	file:String,

	/// directly input bf string
	#[arg(short,long,default_missing_value="++++++++++.",default_value="")]
	string:String,

	/// print the entire tape upon program completion
	#[arg(short,long,default_missing_value="true",default_value="false")]
	tape:bool,
}

fn create_loopmap(program:&str) -> HashMap<u64,u64>
{
	let mut loopmap_start:Vec<u64> = vec![];
	let mut final_loopmap = HashMap::<u64,u64>::new();
	for (index,command) in program.chars().enumerate()
	{
		let index = index as u64;
		match command
		{
			'[' => loopmap_start.push(index),
			']' => {
				let start = loopmap_start.pop().unwrap();
				final_loopmap.insert(start,index);
				final_loopmap.insert(index,start);}
			_   => ()
		}
	}
	return final_loopmap;
}

fn run(program:&str,print_tape:bool)
{
	let mut tape:Vec<u8> = vec![0];
	let loopmap = create_loopmap(&program);
	let mut pgrm_ptr: u64 = 0;
	let mut tape_ptr: u64 = 0;
	let pgrm_len:u64 = program.len() as u64;
	let instructions:Vec<char> = program.chars().collect();
	let input_handler = Term::stdout();

	while pgrm_ptr < pgrm_len
	{
		match instructions[pgrm_ptr as usize]
		{
			'>' =>
			{
				tape_ptr += 1;
				if tape_ptr == tape.len() as u64{tape.push(0);}
			}
			'<' =>
			{
				if tape_ptr == 0{println!("ERROR: you went too far back on character {}",pgrm_ptr)}
				tape_ptr -= 1;
			}
			'+' =>{tape[tape_ptr as usize] += 1}
			'-' =>{tape[tape_ptr as usize] -= 1}
			'[' =>
			{
				if tape[tape_ptr as usize] == 0{pgrm_ptr = loopmap[&pgrm_ptr];}
			}
			']' =>
			{
				if tape[tape_ptr as usize] != 0{pgrm_ptr = loopmap[&pgrm_ptr]}
			}
			'.' =>{print!("{}",tape[tape_ptr as usize] as char);stdout().flush().unwrap()}
			',' =>
			{
				let inp:char = input_handler.read_char().unwrap();
				tape[tape_ptr as usize] = inp as u8;
				print!("{}",inp);
				stdout().flush().unwrap();
			}
			_   => ()
		}
		pgrm_ptr += 1;
	}
	if print_tape{println!("\n");dbg!(&tape);}
}

fn main()
{
	let args:Args = Args::parse();
	let mut program: String = if args.string != "" {args.string} else {fs::read_to_string(&args.file).unwrap()};
	program.retain(|c| "+-<>[].,".contains(c));
	run(&program,args.tape);
}
