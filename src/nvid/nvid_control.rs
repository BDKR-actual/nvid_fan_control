

use std::process;
use std::str::FromStr;
use std::string::String;
use system::system_output;

const COM_START: &str	= "nvidia-settings -a [fan:";						// This and the below should be set once!
const COM_END: &str		= "]/GPUTargetFanSpeed=100";


pub fn cold_range_match(core_temp: u8) -> u8
    {
    return match core_temp
        {
        0..=30      => 0,
		31..=40		=> 60,
        41..=50     => 75,
        51..=65     => 85,
        66..=69     => 95,
        70..=255    => 100,
        }
    }


pub fn warm_range_match(core_temp: u8) -> u8
    {
    return match core_temp
        {
        0..=45      => 0,
        46..=50     => 65,
        51..=65     => 85,
        66..=69     => 95,
        70..=255    => 100,
        }
    }


pub fn high_range_match(core_temp: u8) -> u8
	{
    return match core_temp
        {
        0..=60      => 0,
        61..=65     => 85,
        66..=69     => 95,
        70..=255    => 100,
        }
	}


pub fn set_fan_speed(fan_speed: u8) -> bool
	{
	/* setup */
	let num_fans: u8		= 2;
	let mut com_end_local	= COM_END.to_string();
	let fs_str				= fan_speed.to_string();
	com_end_local 			= com_end_local.replace("100", &fs_str);

	/* Loop and set */	
	for x in 0..num_fans
		{
		let com = format!("{}{}{}", COM_START, x, com_end_local);	
		let out	= system_output(&com).expect("Failed to run nvidia-settings!");
		}

	/* Return the good news */
	true
	}


