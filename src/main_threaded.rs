

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

extern crate getopts;
// extern crate rand;              // Not sure we're going to need this. Leave for now.

// use std::slice::Iter;
use getopts::Options;			// Prefer config files, but let's keep this should we change.
use std::char;
use std::collections::HashMap;
use std::env;					// We'll need this for reading from config files
use std::fmt;
use std::fmt::Display;          // For pretty printing debug output
use std::io;
use std::io::{Write, stderr};
use std::sync::{Arc, Mutex};
// use std::sync::{Arc, Mutex, TryLockError};
use std::sync::{MutexGuard, TryLockError};
use std::thread;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;		// For pausing the application

use std::process;				// For executing commands
use std::process::{Command};    // For executing commands
// use std::ptr::{null, null_mut};
use std::str::Split;            // Used in config::check_command
use system::system_output;      // Used in config::check_command

/* Modules */
use nvid_fan_control::nvid;
use nvid_fan_control::nvid::nvid_control;
use nvid_fan_control::nvid::nvid_data;
use nvid_fan_control::nvid::nvid_settings;
use nvid_fan_control::nvid::nvid_settings::celsius_to_farenheit;


/* Super simple logic really */
fn main()
	{
	/* Setup */
	let mut core_temp:  u8 					= 0;
    let mut last_temp:  u8  				= 0;
	let mut fan_target: u8					= 0;
	let mut charting:	u8					= 1;
	let mut stp_3_otr: HashMap<String, String>= HashMap::new();		// Creating this conditionally would be nice
	let charting:		u8					= 1;
	let dbg_out:		u8					= 0;

	/* Let's now instantiate this struct */
	let mut gpu_settings = Arc::new(Mutex::new(nvid::nvid_data
		{
	    core_temp:      0,
   		core_temp_f:    0,
    	ambient_temp:   0,
    	ambient_temp_f: 0,
	    fan_speed:      0,
   		fan1_speed_rpm: 0,
   		fan2_speed_rpm: 0,
		gpu_power_draw: "0 W".to_string()
		}));

	/* Thread setup */
	if(charting==1)
		{
		let mut clnd_gps = Arc::clone(&gpu_settings);
		let d_thread = spawn(move ||
			{
			// dbg!(&clnd_gps);
			local_test(&clnd_gps);
    		});
		}


	loop
		{
		core_temp           = nvid_settings::check_core_temp();		// core_temp comes back in celsius

        if( (core_temp != last_temp) || (core_temp > 30) )
			{
			match core_temp
				{
				0..=24		=> fan_target = 0,
				25..=27 	=> fan_target = 30,
				28..=30		=> fan_target = 60,
				31..=33		=> fan_target = 75,
				34..=37		=> fan_target = 90,
				38..=255	=> fan_target = 100					// This of course is for when real work is being done. 255 is max for u8.
				};
			if(core_temp != last_temp)	{ nvid_control::set_fan_speed(fan_target); }
			else						{ /* println!("Core temp is {}. Last temp is {} --> Did not set fan speed!", core_temp, last_temp); */ }
			}
        else
            { /* println!("Core temp is {}. Last temp is {} --> Did not set fan speed!", core_temp, last_temp); */ }

		last_temp = core_temp;
		if(dbg_out==1) { println!("Core temp is {}", core_temp); }

        /* I think I can set data here and via arc mutex the data will be available in the child thread */
        if(charting==1)
            {
			nvid_settings::get_card_data(&mut stp_3_otr);						// core_temp comes back in celsius
			let mut main_lock = gpu_settings.try_lock().expect("Uhhh...."); 	// Let's get a lock on the data we want to work with.

			// dbg!(&stp_3_otr);
			println!("\tCelsius is {} | Farenheit is {}", core_temp, celsius_to_farenheit(core_temp as f32));
			main_lock.core_temp 		= core_temp;
			main_lock.core_temp_f		= celsius_to_farenheit(core_temp as f32) as u8;
			main_lock.fan_speed			= fan_target;
			main_lock.fan1_speed_rpm	= stp_3_otr.get("GPUCurrentFanSpeedRPM:0").unwrap().parse().unwrap();
			main_lock.fan2_speed_rpm	= stp_3_otr.get("GPUCurrentFanSpeedRPM:1").unwrap().parse().unwrap();

			nvid_settings::get_card_power(&mut stp_3_otr);
			main_lock.gpu_power_draw	= stp_3_otr.get("Power Draw").unwrap().to_string();

			/* We gotta get outta here! */
			drop(main_lock);
            }

		/* Sleep for a bit then check again */
		thread::sleep(Duration::from_secs(7));
		}

	}




// fn local_test(nvid_data: &Arc<Mutex<nvid::nvid_data>>)
fn local_test(nvid_data: &Arc<Mutex<nvid::nvid_data>>)
	{
	// use std::sync::{Mutex, TryLockError};
	

	loop
		{
		let mut main_lock: Result<MutexGuard<'_, nvid_data>, TryLockError<MutexGuard<'_, nvid_data>>> = nvid_data.try_lock();

	

		dbg!(&main_lock);
		dbg!(*&main_lock.core_temp_f);

		println!("Just taking a little nap...");

		drop(main_lock);
		thread::sleep(Duration::from_secs(15));
		}
	}
