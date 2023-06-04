// Change values
const SET_VALUES_STR     : &str = "set";
const SET_WEIGHTS_STR    : &str = "stw";
const SET_SAMPLE_SIZE_STR: &str = "sps";
const RESET_VALUE_STR    : &str = "rsv";
const RESET_WEIGHTS_STR  : &str = "rsw";
const RESET_ALL_STR      : &str = "rsa";

// File manipulation
const VALUES_FROM_FILE_STR : &str = "rvf";
const WEIGHTS_FROM_FILE_STR: &str = "rwf";

// Display values
const SHOW_SAMPLE_SIZE_STR: &str = "shs";
const SHOW_VALUES_STR     : &str = "shv";
const SHOW_WEIGHTS_STR    : &str = "shw";
const SHOW_ALL_STR        : &str = "sha";

// Do calculations
const AVERAGE_STR        : &str = "avg";
const WEIGHED_AVERAGE_STR: &str = "wav";
const STD_DEVIATION_STR  : &str = "std";
const BIG_VALUE_STR      : &str = "bvl";
const SMALL_VALUE_STR    : &str = "svl";

// General purpose commands
const HIDE_WARNINGS_STR: &str = "hid";
const RESET_SCREEN_STR : &str = "rsc";
const EXIT_STR         : &str = "q";
const HELP_STR         : &str = "h";


pub fn print_title() {
    println!("    __  ___      ____  _       ______                 __  _         ");  
	println!("   /  |/  /_  __/ / /_(_)     / ____/_  ______  _____/ /_(_)___  ____ ");
	println!("  / /|_/ / / / / / __/ /_____/ /_  / / / / __ \\/ ___/ __/ / __ \\/ __ \\ ");
	println!(" / /  / / /_/ / / /_/ /_____/ __/ / /_/ / / / / /__/ /_/ / /_/ / / / /");
	println!("/_/  /_/\\__,_/_/\\__/_/     /_/    \\__,_/_/ /_/\\___/\\__/_/\\____/_/ /_/ ");
	println!("   ______      __           __      __                                ");
	println!("  / ____/___ _/ /______  __/ /___ _/ /_____  _____                    ");
	println!(" / /   / __ `/ / ___/ / / / / __ `/ __/ __ \\/ ___/                    ");
	println!("/ /___/ /_/ / / /__/ /_/ / / /_/ / /_/ /_/ / /                        ");
	println!("\\____/\\__,_/_/\\___/\\__,_/_/\\__,_/\\__/\\____/_/  v1.0.0.rs");
	println!("\n\n~By Raphael Zoega~");
}

pub fn print_commands() {
    println!(" ______________________________________________________________________");
	println!("|                                                                      |");
	println!("| FUNCTION                                                       CODE  |");
	println!("| ==================================================================== |");
	println!("| SET SAMPLE VALUES -------------------------------------------- '{}' |", SET_VALUES_STR);
	println!("|    > Prompts user input for sample values                            |");
	println!("| SET WEIGHT VALUES -------------------------------------------- '{}' |", SET_WEIGHTS_STR);
	println!("|    > Prompts user input for weight values                            |");
	println!("| SET SAMPLE SIZE ---------------------------------------------- '{}' |", SET_SAMPLE_SIZE_STR);
	println!("|    > Prompts user input for new sample/weight size                   |");
	println!("| RESET SAMPLE VALUES ------------------------------------------ '{}' |", RESET_VALUE_STR);
	println!("| RESET WEIGHT VALUES ------------------------------------------ '{}' |", RESET_WEIGHTS_STR);
	println!("| RESET ALL VALUES --------------------------------------------- '{}' |", RESET_ALL_STR);
	println!("| ==================================================================== |");
	println!("| AVERAGE ------------------------------------------------------ '{}' |", AVERAGE_STR);
	println!("| WEIGHTED AVERAGE --------------------------------------------- '{}' |", WEIGHED_AVERAGE_STR);
	println!("|    > Prompts user input for weight values                            |");
	println!("| STANDARD DEVIATION ------------------------------------------- '{}' |", STD_DEVIATION_STR);
	println!("| GREATEST VALUE ----------------------------------------------- '{}' |", BIG_VALUE_STR);
	println!("| SMALLEST VALUE ----------------------------------------------- '{}' |", SMALL_VALUE_STR);
	println!("| ==================================================================== |");
	println!("| READ VALUES FROM FILE ---------------------------------------- '{}' |", VALUES_FROM_FILE_STR);
	println!("| READ WEIGHTS FROM FILE --------------------------------------- '{}' |", WEIGHTS_FROM_FILE_STR);
	println!("| ==================================================================== |");
	println!("| TOGGLE HIDE WARNINGS ----------------------------------------- '{}' |", HIDE_WARNINGS_STR);
	println!("| RESET CONSOLE ------------------------------------------------ '{}' |", RESET_SCREEN_STR);
	println!("| REQUEST HELP --------------------------------------------------- '{}' |", HELP_STR);
	println!("| TERMINATE PROGRAM ---------------------------------------------- '{}' |", EXIT_STR);
	println!("|______________________________________________________________________|\n");
}
