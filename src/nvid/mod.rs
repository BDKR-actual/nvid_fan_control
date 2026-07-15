

extern crate nvml_wrapper;                   // Let's bring in the Nvidia wrapper

use std::fmt;
use std::time::Instant;
use std::time::Duration;


// Command for setting peak power
// sudo nvidia-smi -pl 300

/*
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub enum fan_curve
    {
	l1,
	l2,
	l3,
	l4,
	l5
    }

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct nvid_config
    {
    pub directory:  String,
    pub cmd:        String,
    pub image:      String,
    pub interval:   u32,
	pub fan_count:	i8
    }

impl nvid_data
	{
	fn show_core_temp(&self) -> u8	{ 8 }
	fn show_core_temp_f(&self) -> u8	{ 8 }

	fn show_ambient_temp(&self) -> u8 { 8 }
	fn show_ambient_temp_f(&self) -> u8 { 8 }

	fn show_fan_speed(&self) ->u8 { 8 }
	fn show_fan_speed_rpm(&self) ->u16 { 1526 }	
	}
*/


/* This data structure was initially for charting,but will instead be used for writing to a log file. */
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct nvid_data
	{
	pub core_temp:				u8,
	pub core_temp_f:			u8,
	pub ambient_temp:			u8,
	pub ambient_temp_f:			u8,
	pub fan_speed:				u8,			// The only useable portion we'll need is 0-100 as it's a percentage. 
	pub fan1_speed_rpm:			u16,		
	pub fan2_speed_rpm:			u16,
	pub gpu_power_draw: 		String,
	pub gpu_power_draw_float: 	f32,
	pub command:				u8,			// 0 = temp control only. 1 = chart gpu data. 2 = create chart
	pub new_intvl:				u64,
	pub new_sleep:				u64
	}


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct nvid_state_data
	{
	pub current_state:		u8,
	}


pub mod nvid_control;
pub mod nvid_settings;
pub mod nvid_state;

