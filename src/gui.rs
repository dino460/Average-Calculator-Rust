use colored::*;


// Change values
pub const SET_VALUES_STR     : &str = "set";
pub const SET_WEIGHTS_STR    : &str = "stw";
pub const SET_SAMPLE_SIZE_STR: &str = "sps";
pub const RESET_VALUE_STR    : &str = "rsv";
pub const RESET_WEIGHTS_STR  : &str = "rsw";
pub const RESET_ALL_STR      : &str = "rsa";

// File manipulation
pub const VALUES_FROM_FILE_STR : &str = "rvf";
pub const WEIGHTS_FROM_FILE_STR: &str = "rwf";

// Display values
pub const SHOW_SAMPLE_SIZE_STR: &str = "shs";
pub const SHOW_VALUES_STR     : &str = "shv";
pub const SHOW_WEIGHTS_STR    : &str = "shw";
pub const SHOW_ALL_STR        : &str = "sha";
pub const HIDE_ALL_STR        : &str = "hia";

// Do calculations
pub const AVERAGE_STR        : &str = "avg";
pub const WEIGHED_AVERAGE_STR: &str = "wav";
pub const STD_DEVIATION_STR  : &str = "std";
pub const BIG_VALUE_STR      : &str = "bvl";
pub const SMALL_VALUE_STR    : &str = "svl";

// General purpose commands
pub const HIDE_WARNINGS_STR: &str = "hid";
pub const RESET_SCREEN_STR : &str = "rsc";
pub const CLEAR_SCREEN_STR : &str = "clr";
pub const EXIT_STR         : &str = "q";
pub const HELP_STR         : &str = "h";

pub const NULL_INPUT_STR   : &str = "nul";


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
	println!("\\____/\\__,_/_/\\___/\\__,_/_/\\__,_/\\__/\\____/_/  v1.2.0.rs");
	println!("{}\n", "~By Raphael Zoega~".italic());
}

pub fn print_commands() {
    println!(" ______________________________________________________________________");
	println!("|                                                                      |");
	println!("| {}                                                       {}  |", "FUNCTION".bright_blue().bold(), "CODE".bright_red().bold());
	println!("|                                                                      |");

	println!("| {} ======================================================= |", "Values setup".magenta().bold().underline());
	println!("| {} ---------------------------------------------- {} |",       "SET SAMPLE VALUES".bright_blue()  , SET_VALUES_STR.bright_red().italic()      );
	println!("| {} ---------------------------------------------- {} |",       "SET WEIGHT VALUES".bright_blue()  , SET_WEIGHTS_STR.bright_red().italic()     );
	println!("| {} ------------------------------------------------ {} |",     "SET SAMPLE SIZE".bright_blue()    , SET_SAMPLE_SIZE_STR.bright_red().italic() );
	println!("| {} -------------------------------------------- {} |",         "RESET SAMPLE VALUES".bright_blue(), RESET_VALUE_STR.bright_red().italic()     );
	println!("| {} -------------------------------------------- {} |",         "RESET WEIGHT VALUES".bright_blue(), RESET_WEIGHTS_STR.bright_red().italic()   );
	println!("| {} ----------------------------------------------- {} |",      "RESET ALL VALUES".bright_blue()   , RESET_ALL_STR.bright_red().italic()       );
	
	println!("|                                                                      |");

	println!("| {} ======================================================= |",     "Calculations".magenta().bold().underline());
	println!("| {} -------------------------------------------------------- {} |", "AVERAGE".bright_blue()           , AVERAGE_STR.bright_red().italic()         );
	println!("| {} ----------------------------------------------- {} |",          "WEIGHTED AVERAGE".bright_blue()  , WEIGHED_AVERAGE_STR.bright_red().italic() );
	println!("| {} --------------------------------------------- {} |",            "STANDARD DEVIATION".bright_blue(), STD_DEVIATION_STR.bright_red().italic()   );
	println!("| {} ------------------------------------------------- {} |",        "GREATEST VALUE".bright_blue()    , BIG_VALUE_STR.bright_red().italic()       );
	println!("| {} ------------------------------------------------- {} |",        "SMALLEST VALUE".bright_blue()    , SMALL_VALUE_STR.bright_red().italic()     );
	
	println!("|                                                                      |");

	println!("| {} ================================================== |", "File manipulation".magenta().bold().underline());
	println!("| {} ------------------------------------------ {} |",      "READ VALUES FROM FILE".bright_blue() , VALUES_FROM_FILE_STR.bright_red().italic()  );
	println!("| {} ----------------------------------------- {} |",       "READ WEIGHTS FROM FILE".bright_blue(), WEIGHTS_FROM_FILE_STR.bright_red().italic() );
	
	println!("|                                                                      |");

	println!("| {} ============================================================ |", "Visuals".magenta().bold().underline());
	println!("| {} --------------------------------------------- {} |", "TOGGLE SHOW VALUES".bright_blue()     , SHOW_VALUES_STR.bright_red().italic()   );
	println!("| {} -------------------------------------------- {} |",  "TOGGLE SHOW WEIGHTS".bright_blue()    , SHOW_WEIGHTS_STR.bright_red().italic()  );
	println!("| {} ---------------------------------------- {} |",      "SHOW VALUES AND WEIGHTS".bright_blue(), SHOW_ALL_STR.bright_red().italic()      );
	println!("| {} ---------------------------------------- {} |",      "HIDE VALUES AND WEIGHTS".bright_blue(), HIDE_ALL_STR.bright_red().italic()      );
	println!("| {} ------------------------------------------- {} |",   "TOGGLE HIDE WARNINGS".bright_blue()   , HIDE_WARNINGS_STR.bright_red().italic() );
	
	println!("|                                                                      |");

	println!("| {} ========================================================== |", "Utilities".magenta().bold().underline());
	println!("| {} -------------------------------------------------- {} |",      "RESET CONSOLE".bright_blue()    , RESET_SCREEN_STR.bright_red().italic() );
	println!("| {} -------------------------------------------------- {} |",      "CLEAR CONSOLE".bright_blue()    , CLEAR_SCREEN_STR.bright_red().italic() );
	println!("| {} ----------------------------------------------------- {} |",   "REQUEST HELP".bright_blue()     , HELP_STR.bright_red().italic()         );
	println!("| {} ------------------------------------------------ {} |",        "TERMINATE PROGRAM".bright_blue(), EXIT_STR.bright_red().italic()         );
	println!("|______________________________________________________________________|\n");
}


pub fn print_vec(vec: & Vec<f32>) {
	for index in 0..vec.len() {
		print!("{}, ", vec[index]);
	}
}


pub fn reset_and_print_all()
{
	print!("{esc}c", esc = 27 as char);
    print_title();
    print_commands();
}
