
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use filesize::PathExt;
use std::
    {
    io::{prelude::*, BufReader},
    path::Path,
    };
use crate::control::load_controller;


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
		external_commands
			{
			comm_file_path:		"/home/bdkr/.config/nvid_fan_controller/commands".to_string(),
			command_list:		vec![
									"force_low".to_string(),
									"force_normal".to_string(),
									"release_hold".to_string(),
									"clamp_low".to_string(),
									"clamp_high".to_string(),
									"release_clamp".to_string(),
									],
			debug:				0
			}
		}


	pub fn set_debug(&mut self, dbg_val: u8)   { self.debug = dbg_val.clone(); }


	pub fn check_for_commands(&self) -> bool
		{
		let path 		= Path::new(&self.comm_file_path);
		let _metadata 	= path.symlink_metadata();
		let realsize 	= path.size_on_disk().unwrap();

		/* We may re-cast this val later */
		if(realsize > 0 as u64)	{ true }
		else					{ false }
		}


	pub fn execute_ext_command(&self, load_control: &mut load_controller)
		{
		let comm_lines 		= self.get_lines();
		let mut cntr: u8	= 1;

		for cl in comm_lines
			{
			if(cntr < 2)
				{
				if(self.command_list.contains(&cl))		{ load_control.run_external(&cl); }
				else									{ println!("The comand... {} ...is not recognized! Ignoring!\nMove this to error log output!", &cl); }
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
