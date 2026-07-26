

extern crate nvml_wrapper;                   // Let's bring in the Nvidia wrapper

use std::fmt;
use std::fmt::Display;
use std::time::Instant;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

pub const LOG_HEADERS: &str = "timestamp,core_temp,core_temp_f,ambient_temp,ambient_temp_f,fan_speed,fan1_speed_rpm,fan2_speed_rpm,Power_Draw,gpu_power_draw";

/* This data structure was initially for charting,but will instead be used for writing to a log file. */
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct nvid_data<'a>
	{
	pub timestamp:				String,
	pub core_temp:				String,
	pub core_temp_f:			String,
	pub ambient_temp:			String,
	pub ambient_temp_f:			String,
	pub fan_speed:				String,			// The only useable portion we'll need is 0-100 as it's a percentage. 
	pub fan1_speed_rpm:			String,
	pub fan2_speed_rpm:			String,
	pub gpu_power_draw: 		String,
	log_string:					String,
	internal_list:				Vec<&'a str>,

	/* Wou;ld love to figure out a way to use these in the future */ /*
	pub command:				String,			// 0: temp control only. 1: chart gpu data. 2: create chart
	pub new_intvl:				String,
	pub new_sleep:				String,
	*/
	}


impl <'a>nvid_data<'_>
	{
	pub fn new() -> Self
		{
		nvid_data 
			{
			timestamp:				"".to_string(),
			core_temp:				"".to_string(),
			core_temp_f:			"".to_string(),
			ambient_temp:			"".to_string(),
			ambient_temp_f:			"".to_string(),
			fan_speed:				"".to_string(),			// The only useable portion we'll need is 0-100 as it's a percentage. 
			fan1_speed_rpm:			"".to_string(),		
			fan2_speed_rpm:			"".to_string(),
			gpu_power_draw: 		"".to_string(),
			log_string:				"".to_string(),
			internal_list:			vec![
										"core_temp", "core_temp_f", "ambient_temp", "ambient_temp_f", "fan_speed",
										"fan1_speed_rpm", "fan2_speed_rpm", "Power_Draw", "gpu_power_draw"
										],

			/* Leave for now */ /*
			command:				"".to_string(),			// 0: temp control only. 1: chart gpu data. 2: create chart
			new_intvl:				"".to_string(),
			new_sleep:				"".to_string()
			*/
			}

		}


    /* Accepts a generic `impl Into<String>` and uses match inside to decide how to handle it.*/
    pub fn set_key(&mut self, mut field_id: &str, value_impl: impl Into<String>) 
		{
        let raw_value = value_impl.into(); // Convert everything into an owned String first
		field_id.replace("'", "").trim();
        
        match field_id 
			{
			"core_temp"					=> self.core_temp 			= raw_value,
			"core_temp_f"				=> self.core_temp_f 		= raw_value,
			"ambient_temp"				=> self.ambient_temp 		= raw_value,
			"ambient_temp_f"			=> self.ambient_temp_f		= raw_value,
			"fan_speed"					=> self.fan_speed 			= raw_value,
			"GPUCurrentFanSpeedRPM:0"	=> self.fan1_speed_rpm 		= raw_value,
			"fan1_speed_rpm"			=> self.fan1_speed_rpm 		= raw_value,
			"GPUCurrentFanSpeedRPM:1"	=> self.fan2_speed_rpm 		= raw_value,
			"fan2_speed_rpm"			=> self.fan2_speed_rpm 		= raw_value,
			"Power Draw"				=> self.gpu_power_draw 		= raw_value,
			"gpu_power_draw"			=> self.gpu_power_draw 		= raw_value,
            _ => {}
        	}
    	}


	pub fn return_data_string(&mut self) -> &String
		{
		self.log_string.clear();
		self.set_time_stamp();
		for ld_key in &self.internal_list
			{
			match *ld_key
				{
				"core_temp"					=> self.log_string.push_str(self.core_temp.as_str()),
				"core_temp_f"				=> self.log_string.push_str(self.core_temp_f.as_str()),
				"ambient_temp"				=> self.log_string.push_str(self.ambient_temp.as_str()),
				"ambient_temp_f"			=> self.log_string.push_str(self.ambient_temp_f.as_str()),
				"fan_speed"					=> self.log_string.push_str(self.fan_speed.as_str()),
				"GPUCurrentFanSpeedRPM:0"	=> self.log_string.push_str(self.fan1_speed_rpm.as_str()),
				"fan1_speed_rpm"			=> self.log_string.push_str(self.fan1_speed_rpm.as_str()),
				"GPUCurrentFanSpeedRPM:1"	=> self.log_string.push_str(self.fan2_speed_rpm.as_str()),
				"fan2_speed_rpm"			=> self.log_string.push_str(self.fan2_speed_rpm.as_str()),
				"Power Draw"				=> self.log_string.push_str(self.gpu_power_draw.as_str()),
				"gpu_power_draw"			=> self.log_string.push_str(self.gpu_power_draw.as_str()),
            	_ => {}
				}

			if(*ld_key=="gpu_power_draw")	{ /* Do nothing */ }
			else							{ self.log_string.push_str(",");  }
			}	

		&self.log_string
		}


	fn set_time_stamp(&mut self)
		{
		let now 		= SystemTime::now();
		let secs 		= (SystemTime::now()).duration_since(UNIX_EPOCH).unwrap().as_secs();
		let t_stamp		= secs.to_string();
		self.timestamp	= t_stamp;

		self.log_string.push_str(&self.timestamp);
		self.log_string.push_str(",");
		}


	} /* end of impl */



#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct nvid_state_data	{ pub current_state: u8, }


pub mod nvid_control;
pub mod nvid_settings;
pub mod nvid_state;
