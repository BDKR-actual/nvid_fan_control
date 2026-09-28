
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::exit;

const APP_DIR: 		&str = "/etc/gpufanconf/"; 
const INIT_FAILURE:	&str = "\nApp initialization error!";
const NFNW_START: 	&str = "The file you are interested in: ";
const NFNW_END: 	&str = " : does not exist";

pub fn config_dir_exists()
	{

    let path = Path::new(APP_DIR);
    if path.is_dir() 
		{ /* Carry on! */ } 
	else 
		{
		println!("\n************************************************************");
		println!("             		ERROR!!!!!!                       ");
		println!("************************************************************");
        println!("The config directory does not exist! As root, create the /etc/gpufanconf directory or run the install file!\n\n");
		exit(0);
    	}
	}

/* Checks if the file exists first then checks if it's readable */
pub fn check_files_exist_and_writable() 
	{
	for file in vec!["commands", "config", "gpu_fan_error_log", "gpu_fan_perf_log"]	
		{
		let local_path = (APP_DIR.to_owned())+file; 
		let f_path = Path::new( &local_path );
		if(!f_path.exists())
			{
			println!("{}", INIT_FAILURE);
			println!("Unable to open file: {}. Make sure this exists and is writable.\n", file);
			exit(0);
			}

		// let metadata 	= fs::metadata( (APP_DIR.to_owned())+file );
		let metadata 	= fs::metadata( local_path );
		if( metadata.expect("Unable to open file. It likely doesn't exists!").permissions().readonly() == true ) 
			{
			println!("Can't write to this file!"); 
			println!("{}{}{}", NFNW_START, file, NFNW_END);
			exit(0);			
			}
		else
			{ /* Good stuff! */ }
		}
	}





