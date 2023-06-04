pub use std::io;

mod functions;
mod gui;

fn main() {
    gui::print_title();
    gui::print_commands();

    let mut input: String = String::new();

    let mut sample_size: usize = 5;
    let mut sample_values: Vec<f32> = Vec::with_capacity(sample_size);
    let mut weight_values: Vec<f32> = Vec::with_capacity(sample_size);


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
        println!("{}", input);

        match input.as_str() {
            gui::SET_VALUES_STR => (),
            gui::SET_WEIGHTS_STR => (),
            gui::SET_SAMPLE_SIZE_STR => (),
            
            gui::RESET_ALL_STR => (),
            gui::RESET_VALUE_STR => (),
            gui::RESET_WEIGHTS_STR => (),

            gui::VALUES_FROM_FILE_STR => (),
            gui::WEIGHTS_FROM_FILE_STR => (),
            
            gui::AVERAGE_STR => println!("{}", functions::average(&sample_values, sample_size)),
            gui::WEIGHED_AVERAGE_STR => println!("{}", functions::weighed_average(&sample_values, &weight_values, sample_size)),
            gui::STD_DEVIATION_STR => println!("{}", functions::standard_deviation(&sample_values, sample_size)),
            gui::BIG_VALUE_STR => println!("{}", functions::get_biggest_value(&sample_values)),
            gui::SMALL_VALUE_STR => println!("{}", functions::get_smallest_value(&sample_values)),

            gui::RESET_SCREEN_STR => (),
            gui::SHOW_ALL_STR => (),
            gui::SHOW_SAMPLE_SIZE_STR => (),
            gui::SHOW_VALUES_STR => (),
            gui::SHOW_WEIGHTS_STR => (),

            gui::HIDE_WARNINGS_STR => (),
            gui::EXIT_STR => (),
            gui::HELP_STR => (),

            _ => ()
        }
    }
}
