

/* ***********************************************************************************************
The two commands below allow me to alter fan speed. On this sytem there are two fans. The below
commands set each to 100%. Will will of course drive these percentages based on temperature.

nvidia-settings -a ‘[fan:0]/GPUTargetFanSpeed=100’
nvidia-settings -a ‘[fan:1]/GPUTargetFanSpeed=100’


This command will return core temp


nvidia-settings -q GPUCoreTemp

----

Fome: https://forums.developer.nvidia.com/t/how-to-set-fanspeed-in-linux-from-terminal/72705/9

In terminal you can set the power curve:
sudo nvidia-smi -i 0 -pl 150
Where ‘0’ is your primary GPU (and 1 would be your secondary, and so on…)
and ‘150’ is your capped wattage.


*********************************************************************************************** */

#![allow(unused)]
#![allow(deprecated)]
#![warn(non_camel_case_types)]
extern crate nvml_wrapper;					// Let's bring in the Nvidia wrapper

// use std::char;
// use std::collections::HashMap;
// use std::fmt;
// use std::fmt::Display;		      		// For pretty printing debug output
use std::env;								// We'll need this for reading from config files
use std::env::args;							// We explicitly need the arg function
use std::fs;
use std::io::{Write, stderr};
use std::path::Path;
use std::process;							// For executing commands
use std::process::{Command};    			// For executing commands
use std::str::Split;            			// Used in config::check_command
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, Instant};
use system::system_output;      			// Used in config::check_command
use nvml_wrapper::Nvml;

/* Modules */
use nvid_fan_control::control;
use nvid_fan_control::nvid;
use nvid_fan_control::nvid::nvid_control;
use nvid_fan_control::nvid::nvid_data;		// let (chnnl_tx: Sender, chnnl_rx: Reciever)	=	 mpsc::channel();
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
	let main_intvl:		u64					= 8;									// u64 based on what's required by thread sleep
	let nvml 								= Nvml::init();
	let binding 							= nvml.expect("Uh on... We didn't get our nvml rep!");
	let gpu_bound 							= binding.device_by_index(0);
	let mut init_util:   u32				= 0;
	let mut utilization: u8					= 0;									// This is essentially load
	let mut load_control					= control::load_controller::new();		// The mechanism that will start deciding cooling regimes
	let args: Vec<String> 					= env::args().collect();				// This will do. We are only interested in one arg.

	/* There is only one arg we are after so we'll make it simple. */
	if(args.len() as u8 == 3) 
		{ 
		if(args[1].clone() as String == "--d")
			{
			if(args[2].clone() as String == "1")
				{ dbg_out = 1;  }
			}
		}

	/* Now that we know our debug posture, send it to the load_controller. */
	load_control.set_debug(dbg_out);

	/* Bye! */
	drop(args);

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
		load_control.check_conditions( utilization.clone() );

		if(dbg_out==1)
			{
			println!("\n---------------------------------------------------------------------------------------");
			println!("{}", core_temp);
			println!("{}", utilization);
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
			if(dbg_out==1)	{ println!("Core temp is {}. Last temp is {} --> Did not set fan speed!", core_temp, last_temp); }
			}
		last_temp 		= core_temp;

		/* Sleep for a bit then check again */
		thread::sleep(Duration::from_secs(main_intvl));
		}
	}

