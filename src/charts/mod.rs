

use std::fmt;
use charts_rs::*;
// use nvid_fan_control::nvid::nvid_data;
// use nvid_fan_control::nvid::nvid_data;


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct chart_initd	
	{ pub initialized:    bool }

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct chart_init
	{ 
	/* State data */
    pub initialized:    bool,
	}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct chart_collect
	{

	/* Config data */

	/* GPU Data */
	pub core_temp:			Vec<u8>,
	pub core_temp_f:		Vec<u8>,
    pub ambient_temp:   	Vec<u8>,
   	pub ambient_temp_f: 	Vec<u8>,   
    pub fan_speed:      	Vec<u8>,        // The only useable portion we'll need is 0-100 as it's a percentage. 
	pub fan_speed_avg:		Vec<u16>,		// The calculated average of the below two elements
   	pub fan1_speed_rpm: 	Vec<u16>,
   	pub fan2_speed_rpm: 	Vec<u16>,
    pub gpu_power_draw: 	Vec<String>,
	pub gpu_power_draw_flt:	Vec<f32>,
	pub timestamp:			Vec<String>
	}


impl chart_collect
	{
	fn display_core_temp(self) -> bool
		{
		dbg!(self.core_temp_f);
		true
		}

	}



#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct chart_status
    {

	/*
    pub core_temp_f:    u8,
    pub ambient_temp:   u8,
    pub ambient_temp_f: u8,   
    pub fan_speed:      u8,         // The only useable portion we'll need is 0-100 as it's a percentage. 
    pub fan1_speed_rpm: u16,
    pub fan2_speed_rpm: u16,
    pub gpu_power_draw: String
	*/
    }



impl chart_status
	{

	fn check_initialized(&mut self) -> bool
		{
		// println!("{}", self.initialized);
		return false;
		}




	}





pub mod charting_actual;


