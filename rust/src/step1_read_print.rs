// support precompiled regexes in reader.rs
#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::rc::Rc;
use types::{MalVal,List,Vector,Int,Nil};
mod readline;
mod types;
mod reader;
mod printer;

// read
fn read(str: String) -> Result<MalVal,String> {
    reader::read_str(str)
}

// eval
fn eval(ast: MalVal) -> Result<MalVal,String> {
    Ok(ast)
}

// print
fn print(exp: MalVal) -> String {
    exp.pr_str(true)
}

fn rep(str: String) -> Result<String,String> {
    match read(str) {
        Err(e) => Err(e),
        Ok(ast) => {
            //println!("read: {}", ast);
            match eval(ast) {
                Err(e)  => Err(e),
                Ok(exp) => Ok(print(exp)),
            }
        }
    }
}

fn main() {
    loop {
        let line = readline::mal_readline("user> ");
        match line { None => break, _ => () }
        match rep(line.unwrap()) {
            Ok(str)  => println!("{}", str),
            Err(str) => println!("Error: {}", str),
        }
    }
}
