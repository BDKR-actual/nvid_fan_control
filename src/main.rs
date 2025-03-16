

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
// #![allow(deprecated)]
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
use nvid_fan_control::charts;
use nvid_fan_control::charts::chart_init;
use nvid_fan_control::charts::chart_collect;
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
	let mut charting:		u8				= 0;
	let mut charting_alive:	u8				= 0;
	let arguments:		Vec<String>	= args().collect();	
	if( arguments.len() >= 2 )
		{
        match arguments.get(1).unwrap().as_str()
            {
            "chart"		=> { charting = 1; }
			_			=> { charting = 0; }
            }
		}
	let mut core_temp:  u8 					= 0;
    let mut last_temp:  u8  				= 0;
	let mut fan_target: u8					= 0;
	let mut last_fan_target: u8				= 0;
	let mut main_intvl:	u64					= 8;					// u64 based on what's required by thread sleep
	let dbg_out:		u8					= 0;

	/* Charting related vars | I'll figure out a way to move this out of main later. */
	let (chnnl_tx, chnnl_rx)				= channel();			// Create the mpsc channels. F'3n @w35om3!
	let mut snd_err_cnt:u16					= 0;					// To keep count of times there was an error sending data on the channel
	let mut stp_3_otr: HashMap<String, String>= HashMap::new();		// Creating this conditionally would be nice
	let mut gpu_settings = nvid::nvid_data							// Instantiate the data carrier. 
		{
	    core_temp:      		0,
		core_temp_f:    		0,
   		ambient_temp:   		0,
    	ambient_temp_f: 		0,
	    fan_speed:      		0,
		fan1_speed_rpm: 		0,
   		fan2_speed_rpm: 		0,
		gpu_power_draw: 		"0 W".to_string(),
		gpu_power_draw_float:	0.0,
		command:				0,									/* 0 = temp control only. 1 = chart gpu data. 2 = create chart */
		new_intvl:				4,
		new_sleep:				750
		};

	/* Thread setup if we are charting data */
	if(charting==1)
		{
		main_intvl				= 2;												// Set the new interval for the main thread
		gpu_settings.new_intvl	= 1;												// Pass the new thread interval
		gpu_settings.command	= 1;												// This tells the thread to start collecting GPU data
		charting_alive			= 1;
		}
	let d_thread	= spawn(move || { collect_chart_data(chnnl_rx, dbg_out); });	// Spwan the thread handing it the reciever

	/* Now get to work */
	loop
		{
		core_temp           = nvid_settings::check_core_temp();						// core_temp comes back in celsius
		if(dbg_out==1)
			{
			println!("---------------------------------------------------------------------------------------");
			println!("{}", core_temp);
			println!("---------------------------------------------------------------------------------------");
			}

        if( (core_temp != last_temp) || (core_temp > 30) )
			{
			if(dbg_out==1) { println!("core temp => {} | last temp => {}", core_temp, last_temp); }
			match core_temp
				{
				0..=30		=> fan_target = 0,
				31..=35 	=> fan_target = 30,
				36..=40		=> fan_target = 60,
				41..=43		=> fan_target = 75,
				44..=48		=> fan_target = 90,
 				49..=255	=> fan_target = 100										// This of course is for when real work is being done. 255 is max for u8.
				};
			if( (core_temp != last_temp) && (fan_target != last_fan_target) )	
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

        /* If we are charting, generate gpu data and push onto the pipe. */
        if(charting==1)
            {
            nvid_settings::get_card_data(&mut stp_3_otr);                       // core_temp comes back in celsius

			/* Assign the vals from above */
            if(dbg_out==1) { println!("\tCelsius is {} | Farenheit is {}", core_temp, celsius_to_farenheit(core_temp as f32)); }
            gpu_settings.core_temp         		= core_temp;
            gpu_settings.core_temp_f       		= celsius_to_farenheit(core_temp as f32) as u8;
            gpu_settings.fan_speed         		= fan_target;
            gpu_settings.fan1_speed_rpm    		= stp_3_otr.get("GPUCurrentFanSpeedRPM:0").unwrap().parse().unwrap();
            gpu_settings.fan2_speed_rpm    		= stp_3_otr.get("GPUCurrentFanSpeedRPM:1").unwrap().parse().unwrap();

            nvid_settings::get_card_power(&mut stp_3_otr);
            gpu_settings.gpu_power_draw    		= stp_3_otr.get("Power Draw").unwrap().to_string();
            gpu_settings.gpu_power_draw_float   = nvid_settings::convert_power_draw( stp_3_otr.get("Power Draw").unwrap().to_string() );

			/* Put the data in the pipe */
			let tx_res: Result<(), SendError<nvid::nvid_data>> = chnnl_tx.send(gpu_settings.clone());
			if(!tx_res.is_ok())		{ snd_err_cnt += 1; }
			if(snd_err_cnt > 50)	{ /* println!("It appears the reciever is dead!"); */ charting=0; }
            }

		/* Sleep for a bit then check again */
		thread::sleep(Duration::from_secs(main_intvl));
		}
	}



fn collect_chart_data(rx: Receiver<nvid::nvid_data>, mut dbg: u8)
	{
	/* Get setup */
	let mut av: u16			= 0; 	let mut sys_time		= 0;
	let mut t_now: u64		= 0;	let mut t_new: u64		= 0;
	let mut t_accum: u64	= 0;	let mut t_elapsed: u64	= 0;
	let mut loop_val:u64	= 0;	let mut sleep_val: u64 	= 1000;	
	let mut chart_data  = nvid_fan_control::charts::chart_collect
		{ 
		core_temp: 			vec![], 	core_temp_f: 		vec![], 
		ambient_temp: 		vec![], 	ambient_temp_f: 	vec![], 
		fan_speed: 			vec![], 	fan_speed_avg:		vec![], 
		fan1_speed_rpm: 	vec![], 	fan2_speed_rpm: 	vec![], 
		gpu_power_draw: 	vec![],		gpu_power_draw_flt:	vec![],
		timestamp:			vec![]
		};
	let mut timer_data = nvid_fan_control::utility::timer
		{
	    now:        0,		new:        0,
	    accum:      0,		elapsed:    0
		};

	/* Initialize our timer */
	timer::init_timer(&mut timer_data, Epoch::now());

	/* Now off to work with you */
	loop
		{
		/* Cool! We're in. Now let's push data into the chart_dat struct per ieration. */
		let new_data: Result<nvid::nvid_data, RecvTimeoutError> = rx.recv_timeout(Duration::from_millis(sleep_val));	// Short wait for data before giving up
		if(dbg==1) 				{ println!("\tChannel receive result is {}", new_data.is_ok()); }						// Dbg output
		if(!new_data.is_ok()) 	{ continue; }																			// If no data, go to next iteration
		else
			{
			let unwr= new_data.unwrap();																				// Get our data set
			av = nvid_settings::ret_fan_speed_avg(&unwr.fan1_speed_rpm as &u16, &unwr.fan2_speed_rpm as &u16);			// Generate a fan speed average

			/* */
			loop_val = unwr.new_intvl;
			if(sleep_val != unwr.new_sleep)	
				{ sleep_val = unwr.new_sleep; }

			/* Let's now store our collected data */
			chart_data.core_temp.push( unwr.core_temp );
			chart_data.core_temp_f.push( unwr.core_temp_f );
			chart_data.ambient_temp.push( 0 ); 
			chart_data.ambient_temp_f.push( 0 ); 
			chart_data.fan_speed.push( unwr.fan_speed );																// Fan speed as a percentage
			chart_data.fan_speed_avg.push(av);																			// Average fan speed. Assuming two fans for now. 
			chart_data.gpu_power_draw.push ( unwr.gpu_power_draw );
			chart_data.gpu_power_draw_flt.push( unwr.gpu_power_draw_float );
			chart_data.timestamp.push( timer::return_elapsed_time(&mut timer_data).to_string() );						// This is calling an impl method defined elsewhere
			}

		/* Now sleep */
		// thread::sleep(Duration::from_secs(1));
		thread::sleep(Duration::from_secs(loop_val));
		}
	}



