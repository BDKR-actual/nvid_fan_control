

extern crate nvml_wrapper;                   // Let's bring in the Nvidia wrapper

use nvml_wrapper::{*};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{ Nvml, device::Device };
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use std::fmt;
use std::fmt::Display;
use std::sync::OnceLock;
use std::time::Instant;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use system::system_output;                  // Used in config::check_command


pub const LOG_HEADERS:	&str	= "timestamp,core_temp,core_temp_f,ambient_temp,ambient_temp_f,fan_speed,fan1_speed_rpm,fan2_speed_rpm,Power_Draw,gpu_power_draw";
pub const DEVICE_ERROR: &str	= "Failure attemtping to acquire a descriptor on the GPU!";
const COM_START: &str   		= "nvidia-settings -a [fan:";
const COM_END: &str     		= "]/GPUTargetFanSpeed=100";
const CARD_CORE_TEMP: &str  	= "nvidia-settings -q GPUCoreTemp";
const CARD_DATA_FULL: &str  	= "nvidia-settings -q GPUCoreTemp -q GPUCurrentFanSpeedRPM";
const CARD_DATA_PWR:  &str  	= "nvidia-smi -q --display=power";


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct nvid_gpu	{ pub gpu_dev: 	Device<'static>, }

impl nvid_gpu
	{
	pub fn return_core_temp(&self) -> u8						{ self.gpu_dev.temperature(TemperatureSensor::Gpu).unwrap() as u8 }
	pub fn return_utilization(&self) -> u8						{ self.gpu_dev.utilization_rates().unwrap().gpu as u8 }
	pub fn return_power_usage(&self) -> f32						{ self.gpu_dev.power_usage().unwrap() as f32 }

	/* The next two items use nvml, BUT it won't work on drivers prior to 565.x I believe. In this case, use the variant */
	/* with _ext on the end. */
	pub fn return_fan_speed_rpm(&self, fan_number: u32) -> u32	{ self.gpu_dev.fan_speed_rpm(fan_number).unwrap() }
	pub fn set_fan_speed(&self, fan_speed: u8) -> bool 			{ true }

	/* At this point, this is actually taken care of in the method "get_card_data" */
	pub fn return_fan_speed_ext(&self) -> u32 					{ 22 }

	pub fn set_fan_speed_ext(&self, fan_speed: u8) -> bool	
		{
	    /* setup */
    	let num_fans: u8        = 2;
	    let mut com_end_local   = COM_END.to_string();
    	let fs_str              = fan_speed.to_string();

	    /* If fan speed is set to 100, there is no need to alter the string */
    	if(fan_speed < 100)
        	{ com_end_local = com_end_local.replace("100", &fs_str); }

	    /* Loop and set */
    	for x in 0..num_fans
        	{
	        let com = format!("{}{}{}", COM_START, x, com_end_local);
    	    let out = system_output(&com).expect("Failed to run nvidia-settings!");
        	}

	    /* Return the good news */
    	true
		}


	pub fn get_card_data(&self, stp_3: &mut nvid_data) -> ()
		{
	    let out                 = system_output(CARD_DATA_FULL).expect("Failed to run nvidia-settings!");
    	let so_res              = String::from_utf8_lossy(&out.stdout);
	    let mut stp_2: Vec<&str>= vec![];
    	let mut x:  u8          = 0;

	    /* Get t"o work! */
    	let stp_1: Vec<&str>   = so_res.split("\n").collect::<Vec<&str>>();

	    /* Now loop over command output */
    	for l in &stp_1
        	{
	        if (l.contains("Attribute"))
    	        {
        	    stp_2.push(l);  // May go away
	            let l2a         = l.split(" ").collect::<Vec<&str>>();
    	        let fnl_v       = l2a[5].replace(".", "");
        	    let mut fnl_k   = l2a[3].replace("'", "");

	            if(fnl_k.contains("GPUCurrentFanSpeedRPM"))
    	            {
        	        let lcl_bm          = l2a[4].split(":").collect::<Vec<&str>>();
            	    let mut fan_number  = lcl_bm[2].to_string();
                	fan_number          = fan_number.replace("]", "");
	                fnl_k               = fnl_k+":"+&fan_number.to_string();
    	            fnl_k               = fnl_k.replace(")", "");
        	        }

            	stp_3.set_key(&fnl_k, fnl_v.to_string());
	            }

	        x += 1;
    	    }
		}


	pub fn get_card_power(&self, stp_3: &mut nvid_data) -> ()
    	{
	    let out                 = system_output(CARD_DATA_PWR).expect("Failed to run nvidia-settings!");
    	let so_res              = String::from_utf8_lossy(&out.stdout);
	    let mut x:  u8          = 0;
    	let mut k:  String      = "".to_string();
	    let mut v:  String      = "".to_string();
    	let mut read: u8        = 0;
	    let mut first_line: u8  = 0;

	    let stp_1: Vec<&str>   = so_res.split("\n").collect::<Vec<&str>>();
    	for mut l in &stp_1
        	{
	        /* Get to work */
    	    x = 0;
        	if(l.contains("GPU Power Readings"))    { read = 1; first_line = 1; }
	        if(l.contains("Power Samples"))         { break; }

	        if(read==1)
    	        {
        	    if(first_line==1)
            	    {
                	first_line=0;
	                continue;
    	            }
        	    let l_local             = l.trim().replace("\t", "").replace("                        ", "");
            	let mut l_local_boom    = l_local.split(":");
	            for mut debris in l_local_boom
    	            {
        	        debris = debris.trim();                     // More cleaning 
            	    if(x==0)    { k = debris.to_string(); }     // The key
                	else        { v = debris.to_string(); }
	                x += 1;
    	            }

				// println!("\t{} -> {}\n", &k, &v);
	            stp_3.set_key(&k.clone(), v.clone());           // Now store in the hash map
    	        }
        	}
    	}


	} // End of impl



/* This data structure was initially for charting,but will instead be used for writing to a log file. */
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
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
