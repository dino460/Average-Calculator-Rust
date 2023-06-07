use std::io;

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


    while input != gui::EXIT_STR {
        input.clear();

        match io::stdin().read_line(&mut input) {
            Err(error) => {
                println!("Invalid input.\n error: {error}");
                input = String::from(gui::NULL_INPUT_STR);
            },
            _ => (),
        }

        input.pop();
        input = input.to_lowercase();

        match input.as_str() {
            gui::SET_VALUES_STR => {
                sample_values.clear();
                sample_values = functions::change_vec_values(sample_values, sample_size);
                gui::print_vec(&sample_values, sample_size);
            },
            gui::SET_WEIGHTS_STR => {
                weight_values.clear();
                weight_values = functions::change_vec_values(weight_values, sample_size);
                gui::print_vec(&weight_values, sample_size)
            },
            gui::SET_SAMPLE_SIZE_STR => {
                input.clear();
                io::stdin().read_line(&mut input);
                input.pop();
                println!("{:?}", input);
    
                sample_size = match input.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => 5
                };
                println!("new sample size: {sample_size}");
            },
            
            gui::RESET_ALL_STR => {
                sample_size = 5;
                sample_values = functions::change_vec_values(vec![0.0, 0.0, 0.0, 0.0, 0.0], 5);
                weight_values = functions::change_vec_values(vec![0.0, 0.0, 0.0, 0.0, 0.0], 5);
            },
            gui::RESET_VALUE_STR => {
                sample_size = 5;
                sample_values = functions::change_vec_values(vec![0.0, 0.0, 0.0, 0.0, 0.0], 5);
            },
            gui::RESET_WEIGHTS_STR => {
                sample_size = 5;
                weight_values = functions::change_vec_values(vec![0.0, 0.0, 0.0, 0.0, 0.0], 5);
            },

            gui::VALUES_FROM_FILE_STR => (),
            gui::WEIGHTS_FROM_FILE_STR => (),
            
            gui::AVERAGE_STR => println!("{}", functions::average(&sample_values, sample_size)),
            gui::WEIGHED_AVERAGE_STR => println!("{}", functions::weighed_average(&sample_values, &weight_values, sample_size)),
            gui::STD_DEVIATION_STR => println!("{}", functions::standard_deviation(&sample_values, sample_size)),
            gui::BIG_VALUE_STR => println!("{}", functions::get_biggest_value(&sample_values)),
            gui::SMALL_VALUE_STR => println!("{}", functions::get_smallest_value(&sample_values)),

            gui::RESET_SCREEN_STR => (),
            gui::SHOW_ALL_STR => {
                show_sample_size = !show_sample_size;
                show_sample_values = !show_sample_values;
                show_weight_values = !show_weight_values;
            },
            gui::SHOW_SAMPLE_SIZE_STR => show_sample_size = !show_sample_size,
            gui::SHOW_VALUES_STR => show_sample_values = !show_sample_values,
            gui::SHOW_WEIGHTS_STR => show_weight_values = !show_weight_values,

            gui::HIDE_WARNINGS_STR => (),
            gui::HELP_STR => gui::print_commands(),
            gui::EXIT_STR => println!("Exiting program..."),

            _ => println!("Unknown command.")
        }
    }
}
