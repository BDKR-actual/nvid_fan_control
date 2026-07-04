

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

use epoch_timestamp::Epoch;					// Straightforward Unix Epoch as seconds
use std::char;
use std::collections::HashMap;
use std::env;								// We'll need this for reading from config files
use std::env::args;							// We explicitly need the arg function
use std::fmt;
use std::fmt::Display;		      		    // For pretty printing debug output
use std::fs;
use std::io;
use std::io::{Write, stderr};
use std::path::Path;
use std::process;							// For executing commands
use std::process::{Command};    			// For executing commands
use std::str::Split;            			// Used in config::check_command
use std::sync::mpsc::{*};
use std::thread;
use std::thread::sleep;
use std::thread::spawn;
use std::time::{Duration, SystemTime};
use std::os::unix::net::UnixStream;			// For listening over a file socket (Unix Domain Socket)
use std::os::unix::net::UnixListener;		// Kind of like above
use std::io::prelude::*;
use system::system_output;      			// Used in config::check_command

/* Modules */
use nvid_fan_control::nvid;
use nvid_fan_control::nvid::nvid_control;
use nvid_fan_control::nvid::nvid_data;		// let (chnnl_tx: Sender, chnnl_rx: Reciever)	=	 mpsc::channel();
use nvid_fan_control::nvid::nvid_settings;
use nvid_fan_control::nvid::nvid_settings::celsius_to_farenheit;
use nvid_fan_control::utility::timer;

const SOCK_PATH: &str     = "/tmp/chart_comm";

/* Super simple logic really */
fn main()
	{
	/* Setup */
	let mut core_temp:  u8 					= 0;
    let mut last_temp:  u8  				= 0;
	let mut fan_target: u8					= 0;
	let mut last_fan_target: u8				= 0;
	let mut main_intvl:	u64					= 8;					// u64 based on what's required by thread sleep
	let dbg_out:		u8					= 0;

	/* Now get to work */
	loop
		{
		core_temp           = nvid_settings::check_core_temp();										// core_temp comes back in celsius
		if(dbg_out==1)
			{
			println!("---------------------------------------------------------------------------------------");
			println!("{}", core_temp);
			println!("---------------------------------------------------------------------------------------");
			}

        // if( (core_temp != last_temp) || (core_temp > 41) )
        if( (core_temp != last_temp) )
			{
			if(dbg_out==1) { println!("core temp => {} | last temp => {}", core_temp, last_temp); }
			match core_temp
				{
                0..=45      => fan_target = 0,
                46..=50     => fan_target = 65,
                51..=65     => fan_target = 85,
                66..=69     => fan_target = 95,
                70..=255    => fan_target = 100,
				};
			// if( (core_temp != last_temp) && (fan_target != last_fan_target) )	
			if( (core_temp != last_temp) )	
				{ 
				if(dbg_out==1) 	{ println!("core temp is {}. Setting fan(s) speed too {}%.", core_temp, fan_target); }
				nvid_control::set_fan_speed(fan_target); 
				last_fan_target	= fan_target;
				}
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


