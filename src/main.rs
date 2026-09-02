
#![allow(unused)]
#![allow(deprecated)]
#![warn(non_camel_case_types)]
extern crate nvml_wrapper;							// Let's bring in the Nvidia wrapper

use std::fs::File;
use std::collections::HashMap;
use std::io::{Write};
use std::process::exit;
use std::sync::OnceLock;							
use std::thread;
use std::time::{Duration};
use nvml_wrapper::{*};
mod control;

/* Modules */
use nvid_fan_control::utility::{*};
use nvid_fan_control::utility::utility;
use nvid_fan_control::nvid::{*};
use nvid_fan_control::nvid::nvid_gpu;
use nvid_fan_control::nvid::nvid_control;
use nvid_fan_control::nvid::nvid_data;		
use nvid_fan_control::nvid::nvid_settings::celsius_to_farenheit;

use crate::control::external_command;

/* Super simple logic really */
fn main()-> Result<(), Box<dyn std::error::Error>>
	{
	/* Setup */
	static NVML: OnceLock<Nvml> = OnceLock::new();

	let mut core_temp:  u8					= 0;
	let mut core_temp_i:u32					= 0;
    let mut last_temp:  u8   				= 0;
	let mut fan_target: u8					= 0;
	let mut last_fan_target: u8				= 0;
	let mut dbg_out:	u8					= 0;
	let mut logging:	u8					= 0;
	let mut use_old_fan_rpm: u8				= 0;
	let main_intvl:		u64					= 5;									// u64 based on what's required by thread sleep
    let nvml 								= Nvml::init()?;
    NVML.set(nvml).expect("NVML already initialized");
	let mut init_util:   u32				= 0;
	let mut utilization: u8					= 0;									// This is essentially load
	let mut load_control					= control::load_controller::new();		// The mechanism that will start deciding cooling regimes
	let mut ext_commands					= external_command::external_commands::new();
	let mut logging_data 					= nvid_data::new();
	let mut stp_3_otr: HashMap<String, String>	= HashMap::new();             		// Creating this conditionally would be nice
	let mut conf_data: HashMap<String, String>	= HashMap::new();					// An emapty container to pass to utility config
	let mut gpu_actual						= nvid_gpu 
		{ 
		gpu_dev: NVML.get().unwrap().device_by_index(0).expect(DEVICE_ERROR), 
		num_fans: 0
		}; 

	/* ***************************************************************************************************************************************** */
	/* Initialization */
	utility::read_config(&mut conf_data);																		// Load the config file
	let mut fd  	= File::options().append(true).open(<String as Clone>::clone(&conf_data["LOG_LOCATION"]));	// Open the log file for logging
	use_old_fan_rpm = <String as Clone>::clone(&conf_data["USE_CLI_FAN_RPM"]).parse().unwrap();					// Determine if we are using the wrapper or not
	load_control.set_starting_state( <String as Clone>::clone(&conf_data["DEF_REGIME"]) );						// Set the default cooling regime
	gpu_actual.probe_fans();																					// Probe for the number of fans on the card

	/* Let's make sure the drivers will let us control fan speed manually */
	if( !nvid_gpu::init_manual_control() )
		{
		panic!("Unable to control the GPU manually. Perhaps you need to run this as root?"); 
		exit(0);
		}

	/* Super quick super simple way to catch args */
	utility::read_args(&mut dbg_out, &mut logging);

	/* Now that we know our debug posture (from the line above), send it to the load_controller. */
	if(dbg_out ==1 )
		{
		load_control.set_debug(dbg_out);
		load_control.set_debug_path( <String as Clone>::clone(&conf_data["LOG_LOCATION"]) );
		ext_commands.set_debug(dbg_out);
		}

	/* Let's write the headers to the log file. */
	if(logging==1)
		{ writeln!(&mut fd.as_ref().expect("There was an explosion when trying to open/write to the log file!\n"), "{}", LOG_HEADERS); }

	{ conf_data; }


	/* ***************************************************************************************************************************************** */
	/* Initialization done! Send it! */	
	/* ***************************************************************************************************************************************** */

	/* Now get to work */
	loop
		{
		/* Check for new commands */
		if( ext_commands.check_for_commands() )
			{ ext_commands.execute_ext_command(&mut load_control, &mut logging, &mut fd); }

		/* Let's get the values that matter. */
		core_temp		= gpu_actual.return_core_temp();
		utilization		= gpu_actual.return_utilization();

		if(dbg_out==1)
			{ 
			println!("\n---------------------------------------------------------------------------------------");
			println!("{}c", core_temp);
			println!("{}%", utilization);
			println!("---------------------------------------------------------------------------------------\n");
			}

		if( load_control.clamped == 1 || (core_temp != last_temp) )
			{ fan_target = send_speed_request(&dbg_out, &use_old_fan_rpm, &mut core_temp, &mut last_temp, &mut last_fan_target, &mut gpu_actual, &mut load_control); }
        else
            {
			if(dbg_out==1)	
				{ println!("Core temp is {}. Last temp is {} --> Did not set fan speed!", core_temp, last_temp); }
			}

		/* For the next iteration */
		last_temp = core_temp;
	
		/* Are we logging data? */
        if(logging==1)
            {
			let lds = create_log_entry(&gpu_actual, &mut logging_data, &mut core_temp, &mut fan_target, &dbg_out); 
			writeln!(&mut fd.as_ref().expect("There was an explosion when trying to open/write to the log file!\n"), "{}", &lds );
            }

		/* Sleep for a bit then check again */
		thread::sleep(Duration::from_secs(main_intvl));
		}
	}



/* ------------------------------------------------------------------------------------------------------------------------------------------------------ */ 
/* --------------------------------------------------------------------------------------------------------------------------------------------------- */ /*
Related functions below
*/ /* --------------------------------------------------------------------------------------------------------------------------------------------------- */
/* ------------------------------------------------------------------------------------------------------------------------------------------------------ */


fn create_log_entry(gpa: &nvid_gpu, mut logging_data: &mut nvid_data, core_temp: &mut u8, fan_target: &mut u8, dbg_out: &u8) -> String
	{
	/* These calls to get card daa and power write to the logging_data instance */
	gpa.get_card_data(&mut logging_data);
	gpa.get_card_power(&mut logging_data);
	logging_data.core_temp         = core_temp.to_string();
	logging_data.core_temp_f       = celsius_to_farenheit(*core_temp as f32).to_string();
	logging_data.fan_speed         = fan_target.to_string();
			
	/* And output if requested */
	if(*dbg_out==1)	{ dbg!(&logging_data); }

	/* Now generate and return the string */
	logging_data.return_data_string().to_string()
	}



fn send_speed_request(dbg_out: &u8, uofr: &u8, core_tmp: &u8, last_tmp: &u8, lfn: &mut u8, gpa: &mut nvid_gpu, lc: &mut control::load_controller) -> u8
	{
	let mut fan_target: u8 	= 0;
	let	utilization: u8		= gpa.return_utilization();

    if( lc.clamped == 1)
	    {
        if(*dbg_out==1) { println!("We are clamped! The control level is {}", &lc.clamp_level); }

        match( lc.clamp_level )
    		{
            0               => ( fan_target = nvid_control::cold_range_match(*core_tmp) ),
            1               => ( fan_target = nvid_control::warm_range_match(*core_tmp) ),
            2               => ( fan_target = nvid_control::high_range_match(*core_tmp) ),
            3_u8..=u8::MAX  => todo!(),
            }
        }
	else if( (core_tmp != last_tmp) )
        {
        /* Determine cooling regime */
        lc.check_conditions( &utilization );

        if(*dbg_out==1) { println!("core temp => {} | last temp => {}", core_tmp, last_tmp); }

        match( lc.return_state() )
    		{
            "low"           => ( fan_target = nvid_control::cold_range_match(*core_tmp) ),
            "normal"        => ( fan_target = nvid_control::warm_range_match(*core_tmp) ),
            "high"          => ( fan_target = nvid_control::high_range_match(*core_tmp) ),
            &_              => todo!(),
            }
        }
	       
	if(*dbg_out==1)  { println!("Setting fan(s) speed too {}%.", fan_target); }
        
	/* uofr is set in the config file */
	if(*uofr == 1)	{ gpa.set_fan_speed_ext(fan_target); }  	// Uses nvidia-settings
	else		  	{ gpa.set_fan_speed(fan_target as u32); }	// Uses nvml_wrapper <-- Does'nt work on old drivers: Before 565? 

	/* Of course, lfn is the last_fan_target var initialized in main() */
	*lfn = fan_target;

	fan_target
	}
