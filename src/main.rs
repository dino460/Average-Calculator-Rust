use std::io::{self, Write};
use colored::*;

mod functions;
mod gui;

fn main() {
    gui::print_title();
    gui::print_commands();

    let mut input: String = String::new();

    let mut sample_size: usize = 5;
    let mut sample_values: Vec<f32> = Vec::with_capacity(sample_size);
    let mut weight_values: Vec<f32> = Vec::with_capacity(sample_size);

    let mut show_sample_values: bool = true;
    let mut show_weight_values: bool = true;
    let mut show_sample_size  : bool = true;
    let mut show_warnings     : bool = true;


    while input != gui::EXIT_STR {
        input.clear();

        if show_sample_size {
            println!(">>> {} {sample_size}", "Sample size:".dimmed().italic());
        }
        if show_sample_values {
            print!(">>> {} [", "Sample values:".dimmed().italic());
            gui::print_vec(&sample_values);
            println!("]");

            if sample_values.is_empty() && show_warnings {
                println!("{}", "(!) SAMPLE VECTOR IS EMPTY!".bright_red().bold().blink());
                println!("(?) {} {} {}", "Use".italic(), "set".green().italic(), "to set sample values".italic());
            }
        }
        if show_weight_values {
            print!(">>> {} [", "Weight values:".dimmed().italic());
            gui::print_vec(&weight_values);
            println!("]");

            if weight_values.is_empty() && show_warnings {
                println!("{}", "(!) WEIGHT VECTOR IS EMPTY!".bright_red().bold().blink());
                println!("(?) {} {} {}", "Use".italic(), "stw".green().italic(), "to set weight values".italic());
            }
        }

        print!(": ");
        let _ = io::stdout().flush();
        
        match io::stdin().read_line(&mut input) {
            Err(error) => {
                println!(">>> Invalid input.\n error: {error}");
                input = String::from(gui::NULL_INPUT_STR);
            },
            _ => (),
        }

        input.pop();
        input = input.to_lowercase();

        match input.as_str() {
            gui::SET_VALUES_STR => {
                println!("\n{} {sample_size} {}", "Type the".italic(), "new sample values:".italic());
                sample_values.clear();
                functions::change_vec_values(&mut sample_values, sample_size);
            },
            gui::SET_WEIGHTS_STR => {
                println!("\n{} {sample_size} {}", "Type the".italic(), "new weight values:".italic());
                weight_values.clear();
                functions::change_vec_values(&mut weight_values, sample_size);
            },
            gui::SET_SAMPLE_SIZE_STR => {
                println!("\n{}", "Type the new sample size:".italic());
                input.clear();
                match io::stdin().read_line(&mut input) {
                    Err(error) => {
                        println!("Invalid input.\n error: {error}");
                        input = String::from("5");
                    },
                    _ => (),
                }
                
                input.pop();
                println!("{:?}", input);
    
                sample_size = match input.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => 5
                };

                functions::change_vec_size(&mut sample_values, sample_size);
                functions::change_vec_size(&mut weight_values, sample_size);
                println!("new sample size: {sample_size}");
            },
            
            gui::RESET_ALL_STR => {
                sample_size = 5;
                functions::reset_vec_values(&mut sample_values, sample_size);
                functions::reset_vec_values(&mut weight_values, sample_size);
            },
            gui::RESET_VALUE_STR => {
                sample_size = 5;
                functions::reset_vec_values(&mut sample_values, sample_size);
            },
            gui::RESET_WEIGHTS_STR => {
                sample_size = 5;
                functions::reset_vec_values(&mut weight_values, sample_size);
            },

            gui::VALUES_FROM_FILE_STR => (),
            gui::WEIGHTS_FROM_FILE_STR => (),
            
            gui::AVERAGE_STR => {
                if sample_values.len() != sample_size {
                    functions::change_vec_values(&mut sample_values, sample_size);
                }
                println!("{}", functions::average(&sample_values, sample_size));
            },
            gui::WEIGHED_AVERAGE_STR => {
                functions::check_if_empty(&mut sample_values, sample_size);
                functions::check_if_empty(&mut weight_values, sample_size);

                println!("{}", functions::weighed_average(&sample_values, &weight_values));
            },
            gui::STD_DEVIATION_STR => {
                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{}", functions::standard_deviation(&sample_values, sample_size));
            },
            gui::BIG_VALUE_STR => {
                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{}", functions::get_biggest_value(&sample_values));
            },
            gui::SMALL_VALUE_STR => {
                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{}", functions::get_smallest_value(&sample_values));
            },

            gui::RESET_SCREEN_STR => gui::reset_and_print_all(),
            gui::CLEAR_SCREEN_STR => print!("{esc}c", esc = 27 as char),
            gui::SHOW_ALL_STR => {
                show_sample_size = true;
                show_sample_values = true;
                show_weight_values = true;
            },
            gui::SHOW_SAMPLE_SIZE_STR => show_sample_size = !show_sample_size,
            gui::SHOW_VALUES_STR => show_sample_values = !show_sample_values,
            gui::SHOW_WEIGHTS_STR => show_weight_values = !show_weight_values,

            gui::HIDE_WARNINGS_STR => show_warnings = !show_warnings,
            gui::HELP_STR => gui::print_commands(),
            gui::EXIT_STR => println!("\n{}", "(*) Exiting program...".bright_yellow().bold()),

            _ => println!("\n{}", "(!) Unknown command.".bright_red().bold())
        }

        println!("");
    }
}
