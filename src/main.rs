

#![allow(unused)]
#![allow(deprecated)]
#![warn(non_camel_case_types)]
extern crate nvml_wrapper;					// Let's bring in the Nvidia wrapper

use std::fs::File;
use std::collections::HashMap;
use std::env;								
use std::env::args;
use std::fs;
use std::io::{Write, stderr};
use std::path::Path;
use std::process::exit;							
use std::process::{Command};
use std::str::Split;            			
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, Instant};
use system::system_output;      			// Used in config::check_command
use nvml_wrapper::Nvml;

/* Modules */
use nvid_fan_control::control;
use nvid_fan_control::utility::{*};
use nvid_fan_control::nvid::{*};
use nvid_fan_control::nvid::nvid_control;
use nvid_fan_control::nvid::nvid_data;		
use nvid_fan_control::nvid::nvid_settings;
use nvid_fan_control::nvid::nvid_settings::celsius_to_farenheit;
use nvid_fan_control::utility::timer;

/* Super simple logic really */
fn main()
	{
	/* Setup */
	let mut core_temp:  u8					= 0;
	let mut core_temp_i:u32					= 0;
    let mut last_temp:  u8   				= 0;
	let mut fan_target: u8					= 0;
	let mut last_fan_target: u8				= 0;
	let mut dbg_out:	u8					= 0;
	let mut logging:	u8					= 0;
	let main_intvl:		u64					= 5;									// u64 based on what's required by thread sleep
	let nvml 								= Nvml::init();
	let binding 							= nvml.expect("Uh on... We didn't get our nvml rep!");
	let gpu_bound 							= binding.device_by_index(0);
	let mut init_util:   u32				= 0;
	let mut utilization: u8					= 0;									// This is essentially load
	let mut load_control					= control::load_controller::new();		// The mechanism that will start deciding cooling regimes
	let mut logging_data 					= nvid_data::new();
	let args: Vec<String> 					= env::args().collect();				// This will do. We are only interested in one arg.
	let mut stp_3_otr: HashMap<String, String>	= HashMap::new();             		// Creating this conditionally would be nice
	let mut conf_data: HashMap<String, String>	= HashMap::new();					// An emapty container to pass to utility config

	/* ***************************************************************************************************************************************** */
	/* Initialization */
	utility::read_config(&mut conf_data);																		// Load the config file
	let mut fd  = File::options().append(true).open(<String as Clone>::clone(&conf_data["LOG_LOCATION"]));		// Open the log file for logging
	load_control.set_starting_state( <String as Clone>::clone(&conf_data["DEF_REGIME"]) );						// Set the default cooling regime

	/* Super quick super simple way to catch args */
	for arg in args
		{
		match(arg.as_str())
			{
			"--d"	=> dbg_out	= 1,
			"--l"	=> logging	= 1,
			"--h"	=> show_help(),
			_ 		=> {},
			}
		}	

	/* Now that we know our debug posture, send it to the load_controller. */
	if(dbg_out ==1 )
		{
		load_control.set_debug(dbg_out);
		load_control.set_debug_path( <String as Clone>::clone(&conf_data["LOG_LOCATION"]) );
		}

	/* Let's write the headers to the log file. */
	if(logging==1)
		{ writeln!(&mut fd.as_ref().expect("There was an explosion when trying to open/write to the log file!\n"), "{}", LOG_HEADERS); }

	/* Initialization done! Send it! */
	/* ***************************************************************************************************************************************** */

	/* Now get to work */
	loop
		{
		/* Let's get the values that matter. */
		core_temp_i		= nvid_settings::return_core_temp( (gpu_bound.as_ref().expect("Give us that core bro!")) ); 
    	init_util     	= nvid_settings::return_utilization( (gpu_bound.as_ref().expect("Give us that core bro!")) );

		/* Conver the values from above to u8 */
		core_temp		= core_temp_i as u8;
		utilization		= init_util as u8;

		/* Determine cooling regime */
		load_control.check_conditions( &utilization );

		if(dbg_out==1)
			{ 
			println!("\n---------------------------------------------------------------------------------------");
			println!("{}c", core_temp);
			println!("{}%", utilization);
			println!("---------------------------------------------------------------------------------------\n");
			}

        if( (core_temp != last_temp) )
			{
			if(dbg_out==1) { println!("core temp => {} | last temp => {}", core_temp, last_temp); }

			match( load_control.return_state() )
				{
				"low" 		=> ( fan_target = nvid_control::cold_range_match(core_temp) ),
				"normal" 	=> ( fan_target = nvid_control::warm_range_match(core_temp) ),
				"high" 		=> ( fan_target = nvid_control::high_range_match(core_temp) ),
				&_ 			=> todo!(),
				}

			if(dbg_out==1) 	{ println!("Setting fan(s) speed too {}%.", fan_target); }
			nvid_control::set_fan_speed(fan_target); 
			last_fan_target	= fan_target;
			}
        else
            {
			if(dbg_out==1)	
				{ println!("Core temp is {}. Last temp is {} --> Did not set fan speed!", core_temp, last_temp); }
			}
		last_temp 		= core_temp;
	
        if(logging==1)
            {
			/* These calls to get card daa and power write to the logging_data instance */
            nvid_settings::get_card_data(&mut logging_data);
            nvid_settings::get_card_power(&mut logging_data);
            logging_data.core_temp         = core_temp.to_string();
            logging_data.core_temp_f       = celsius_to_farenheit(core_temp as f32).to_string();
            logging_data.fan_speed         = fan_target.to_string();

			/* And output if requested */
			if(dbg_out==1)	{ dbg!(&logging_data); }

			/* Now write to the log file */
			writeln!(&mut fd.as_ref().expect("There was an explosion when trying to open/write to the log file!\n"), "{}", logging_data.return_data_string());
            }

		/* Sleep for a bit then check again */
		thread::sleep(Duration::from_secs(main_intvl));
		}
	}



pub fn show_help()
	{
	println!
		(
		"\nNVID Fan Control usage...
		\t--d	: Turns on debugging output.
		\t--l	: Turns on logging output.
		\t--h	: Show usage | List arguments.\n\n"
		);
	exit(0);
	}
