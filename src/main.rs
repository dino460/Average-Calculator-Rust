use std::io::{self, Write};
use colored::*;

mod functions;
mod gui;

fn main() {
    gui::print_title();
    gui::print_header();
    gui::print_utilities_help();
    gui::print_foot();

    let mut input_raw: String = String::new();

    let mut sample_size: usize = 5;
    let mut sample_values: Vec<f32> = Vec::with_capacity(sample_size);
    let mut weight_values: Vec<f32> = Vec::with_capacity(sample_size);

    let mut show_sample_values: bool = true;
    let mut show_weight_values: bool = true;
    let mut show_sample_size  : bool = true;
    let mut show_warnings     : bool = true;


    while input_raw != gui::EXIT_STR {
        input_raw.clear();

        if show_sample_size {
            println!(">>> {} {sample_size}", "Sample size:".dimmed().italic());
        }
        if show_sample_values {
            print!(">>> {} [", "Sample values:".dimmed().italic());
            gui::print_vec(&sample_values);
            println!("]");

            if sample_values.is_empty() && show_warnings {
                println!("{}", "(!) SAMPLE VECTOR IS EMPTY!".bright_red().bold().blink());
                println!("(?) {} {} {}", "Use".italic(), gui::SET_VALUES_STR.green().italic(), "to set sample values".italic());
            }
        }
        if show_weight_values {
            print!(">>> {} [", "Weight values:".dimmed().italic());
            gui::print_vec(&weight_values);
            println!("]");

            if weight_values.is_empty() && show_warnings {
                println!("{}", "(!) WEIGHT VECTOR IS EMPTY!".bright_red().bold().blink());
                println!("(?) {} {} {}", "Use".italic(), gui::SET_WEIGHTS_STR.green().italic(), "to set weight values".italic());
            }
        }

        print!(": ");
        let _ = io::stdout().flush();
        
        match io::stdin().read_line(&mut input_raw) {
            Err(error) => {
                println!(">>> Invalid input.\n error: {error}");
                input_raw = String::from(gui::NULL_INPUT_STR);
            },
            _ => (),
        }

        input_raw.pop();
        input_raw = input_raw.to_lowercase();

        let input_raw_clone: String = input_raw.clone();
        let input_vec: Vec<&str> = input_raw_clone.split(' ').collect();

        if input_vec.len() > 1
        {
            for input in input_vec
            {
                match input {
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
                        input_raw.clear();
                        match io::stdin().read_line(&mut input_raw) {
                            Err(error) => {
                                println!("Invalid input.\n error: {error}");
                                input_raw = String::from("5");
                            },
                            _ => (),
                        }
                        
                        input_raw.pop();
                        println!("{:?}", input_raw);
            
                        sample_size = match input_raw.parse::<usize>() {
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
                        println!("{} {}", "Average =".yellow().bold(), functions::average(&sample_values, sample_size));
                    },
                    gui::WEIGHED_AVERAGE_STR => {
                        functions::check_if_empty(&mut sample_values, sample_size);
                        functions::check_if_empty(&mut weight_values, sample_size);
                        println!("{} {}", "Weighed Average =".yellow().bold(), functions::weighed_average(&sample_values, &weight_values));
                    },
                    gui::STD_DEVIATION_STR => {
                        functions::check_if_empty(&mut sample_values, sample_size);
                        println!("{} {}", "Standard Deviation =".yellow().bold(), functions::standard_deviation(&sample_values, sample_size));
                    },
                    gui::BIG_VALUE_STR => {
                        functions::check_if_empty(&mut sample_values, sample_size);
                        println!("{} {}", "Biggest Value =".yellow().bold(), functions::get_biggest_value(&sample_values));
                    },
                    gui::SMALL_VALUE_STR => {
                        functions::check_if_empty(&mut sample_values, sample_size);
                        println!("{} {}", "Smallest Value =".yellow().bold(), functions::get_smallest_value(&sample_values));
                    },

                    gui::ALL_CALCS_STR => {
                        if sample_values.len() != sample_size {
                            functions::change_vec_values(&mut sample_values, sample_size);
                        }
                        println!("{} {}", "Average =".yellow().bold(), functions::average(&sample_values, sample_size));

                        functions::check_if_empty(&mut sample_values, sample_size);
                        functions::check_if_empty(&mut weight_values, sample_size);
                        println!("{} {}", "Weighed Average =".yellow().bold(), functions::weighed_average(&sample_values, &weight_values));

                        functions::check_if_empty(&mut sample_values, sample_size);
                        println!("{} {}", "Standard Deviation =".yellow().bold(), functions::standard_deviation(&sample_values, sample_size));

                        functions::check_if_empty(&mut sample_values, sample_size);
                        println!("{} {}", "Biggest Value =".yellow().bold(), functions::get_biggest_value(&sample_values));

                        functions::check_if_empty(&mut sample_values, sample_size);
                        println!("{} {}", "Smallest Value =".yellow().bold(), functions::get_smallest_value(&sample_values));
                    }

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

                    gui::HELP_VALUES_STR => 
                    {
                        gui::print_header();
                        gui::print_values_help();
                        gui::print_foot();
                    },
                    gui::HELP_CALCULATIONS_STR => 
                    {
                        gui::print_header();
                        gui::print_calculations_help();
                        gui::print_foot();
                    },
                    gui::HELP_FILE_STR => 
                    {
                        gui::print_header();
                        gui::print_file_help();
                        gui::print_foot();
                    },
                    gui::HELP_VISUAL_STR => 
                    {
                        gui::print_header();
                        gui::print_visuals_help();
                        gui::print_foot();
                    },
                    gui::HELP_UTILS_STR => 
                    {
                        gui::print_header();
                        gui::print_utilities_help();
                        gui::print_foot();
                    },
                    gui::HELP_STR => gui::print_all_commands(),

                    gui::EXIT_STR =>
                    {
                        input_raw = input.to_string();
                        println!("\n{}", "(*) Exiting program...".bright_yellow().bold())
                    },

                    _ => println!("\n{}", "(!) Unknown command.".bright_red().bold())
                }
            }

            continue;
        }


        match input_raw.as_str() {
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
                input_raw.clear();
                match io::stdin().read_line(&mut input_raw) {
                    Err(error) => {
                        println!("Invalid input.\n error: {error}");
                        input_raw = String::from("5");
                    },
                    _ => (),
                }
                
                input_raw.pop();
                println!("{:?}", input_raw);
    
                sample_size = match input_raw.parse::<usize>() {
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
                println!("{} {}", "Average =".yellow().bold(), functions::average(&sample_values, sample_size));
            },
            gui::WEIGHED_AVERAGE_STR => {
                functions::check_if_empty(&mut sample_values, sample_size);
                functions::check_if_empty(&mut weight_values, sample_size);
                println!("{} {}", "Weighed Average =".yellow().bold(), functions::weighed_average(&sample_values, &weight_values));
            },
            gui::STD_DEVIATION_STR => {
                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{} {}", "Standard Deviation =".yellow().bold(), functions::standard_deviation(&sample_values, sample_size));
            },
            gui::BIG_VALUE_STR => {
                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{} {}", "Biggest Value =".yellow().bold(), functions::get_biggest_value(&sample_values));
            },
            gui::SMALL_VALUE_STR => {
                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{} {}", "Smallest Value =".yellow().bold(), functions::get_smallest_value(&sample_values));
            },

            gui::ALL_CALCS_STR => {
                if sample_values.len() != sample_size {
                    functions::change_vec_values(&mut sample_values, sample_size);
                }
                println!("{} {}", "Average =".yellow().bold(), functions::average(&sample_values, sample_size));

                functions::check_if_empty(&mut sample_values, sample_size);
                functions::check_if_empty(&mut weight_values, sample_size);
                println!("{} {}", "Weighed Average =".yellow().bold(), functions::weighed_average(&sample_values, &weight_values));

                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{} {}", "Standard Deviation =".yellow().bold(), functions::standard_deviation(&sample_values, sample_size));

                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{} {}", "Biggest Value =".yellow().bold(), functions::get_biggest_value(&sample_values));

                functions::check_if_empty(&mut sample_values, sample_size);
                println!("{} {}", "Smallest Value =".yellow().bold(), functions::get_smallest_value(&sample_values));
            }

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

            gui::HELP_VALUES_STR => 
            {
                gui::print_header();
                gui::print_values_help();
                gui::print_foot();
            },
            gui::HELP_CALCULATIONS_STR => 
            {
                gui::print_header();
                gui::print_calculations_help();
                gui::print_foot();
            },
            gui::HELP_FILE_STR => 
            {
                gui::print_header();
                gui::print_file_help();
                gui::print_foot();
            },
            gui::HELP_VISUAL_STR => 
            {
                gui::print_header();
                gui::print_visuals_help();
                gui::print_foot();
            },
            gui::HELP_UTILS_STR => 
            {
                gui::print_header();
                gui::print_utilities_help();
                gui::print_foot();
            },
            gui::HELP_STR => gui::print_all_commands(),

            gui::EXIT_STR => println!("\n{}", "(*) Exiting program...".bright_yellow().bold()),

            _ => println!("\n{}", "(!) Unknown command.".bright_red().bold())
        }

        println!("");
    }
}
