
extern crate dirs;
use std::fs::File;
use std::fs::OpenOptions;
use filesize::PathExt;
use std::
    {
    io::{prelude::*, BufReader, Error},
    path::Path,
    };
use std::process::exit;

use crate::control::load_controller;

pub const LOG_HEADERS_LOCAL:  &str    = "timestamp,core_temp,core_temp_f,ambient_temp,ambient_temp_f,fan_speed,fan1_speed_rpm,fan2_speed_rpm,Power_Draw,gpu_power_draw";

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Clone)]
pub struct external_commands
	{
	comm_file_path: 	String,
	command_list:		Vec<String>,
	debug:				u8,
	}


impl external_commands
	{
	pub fn new() -> Self
		{
        /* Setup the config file path. */
		let conf_path: String   = "/etc/gpufanconf/commands".to_string();

		external_commands
			{
            comm_file_path:     conf_path,
			command_list:		vec![
									"force_low".to_string(),
									"force_normal".to_string(),
									"release_hold".to_string(),
									"clamp_low".to_string(),
									"clamp_high".to_string(),
									"release_clamp".to_string(),
									],
			//logging_list:		vec![
			//						"start_logging".to_string(),
			//						"stop_logging".to_string(),
			//						],
			debug:				0
			}
		}


	pub fn set_debug(&mut self, dbg_val: u8)   { self.debug = dbg_val.clone(); }


	pub fn check_for_commands(&self) -> bool
		{
		let path 		= Path::new(&self.comm_file_path);
		let _metadata 	= path.symlink_metadata();
		let realsize 	= path.size_on_disk().unwrap_or(0);

		/* We may re-cast this val later */
		if(realsize > 0 as u64)	{ true }
		else					{ false }
		}


	pub fn execute_ext_command(&self, load_control: &mut load_controller, logger: &mut u8, fd: &mut Result::<File, Error>)
		{
		let comm_lines 		= self.get_lines();
		let mut cntr: u8	= 1;

		for cl in comm_lines
			{
			cl.trim();			
			println!("The command is {}.", &cl);
			if(cntr < 2)
				{
				if(cl=="stop_logging")						{ *logger = 0; }
				else if(cl=="start_logging")				
					{ 
					*logger = 1; 
					writeln!(&mut fd.as_ref().expect("There was an explosion when trying to open/write to the log file!\n"), "{}", LOG_HEADERS_LOCAL);					
					}
				else if(cl=="quit")
					{ 
					println!("Quit command recieved! Exit..."); 
					self.truncate_comm_file();
					exit(0); 
					}
				else if(self.command_list.contains(&cl))	{ load_control.run_external(&cl); }
				else										{ println!("The comand... {} ...is not recognized! Ignoring!\nMove this to error log output!", &cl); }
				}

			/* The easy way to stop multiple commands from being pushed in at once. */
			cntr += 1;
			}

		&self.truncate_comm_file();
		}


	/* Opens to write then truncates! */
	pub fn truncate_comm_file(&self)
		{ let _file = OpenOptions::new().write(true).truncate(true).open(&self.comm_file_path); }


	pub fn get_lines(&self) -> Vec<String>
		{
	    let file = File::open(&self.comm_file_path).expect("no such file");
    	let buf = BufReader::new(file);
		let final_form: Vec<String> = buf.lines().map(|l| l.expect("Uhhh...")).collect();
		final_form
    	}
	}
