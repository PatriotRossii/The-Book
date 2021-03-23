use std::io::{self, Read, Write};
use fprint::fprint;

use temperature_converter::{Celsius, Fahrenheit};

fn main() {
	let mut buffer = String::new();
	let mut value_buffer = String::new();
	let mut value: f64 = 0.0;
    loop {
		buffer.clear();
		println!("1) F->C\n2) C->F\n3) Exit\n");
		
		fprint!(">> ");
		io::stdin().read_line(&mut buffer).expect("Failed to read to string");
		
		let command = buffer.trim();		

		match command {
			"1" | "2" => {
				loop {
					value_buffer.clear();
					println!("\nEnter value to convert");
					fprint!(">> ");
					io::stdin().read_line(&mut value_buffer).expect("Failed to read a line");
					match value_buffer.trim().parse() {
						Ok(e) => {
							value = e;
							break;
						},
						Err(_) => {
							println!("Enter correct number!");
						}
					}
				}
				println!("\nTranslated value: {}\n", match command {
					"1" => Celsius::from(Fahrenheit(value)).0,
					"2" => Fahrenheit::from(Celsius(value)).0,
					_ => unreachable!()
				});
			},
			"3" => {
				break
			},
			_ => println!("Invalid input"),
		}
	}
}
