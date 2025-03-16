

use std::collections::HashMap;
use std::process;
use std::str::FromStr;
use std::string::String;
use system::system_output;

/* Let's not generate this over and over again */
const CARD_CORE_TEMP: &str	= "nvidia-settings -q GPUCoreTemp";
const CARD_DATA_FULL: &str	= "nvidia-settings -q GPUCoreTemp -q GPUCurrentFanSpeedRPM";
const CARD_DATA_PWR:  &str	= "nvidia-smi -q --display=power";


pub fn check_core_temp() -> u8
	{
	// let cmd		= "nvidia-settings -q GPUCoreTemp".to_string();
	let out		= system_output(CARD_CORE_TEMP).expect("Failed to run nvidia-settings!");

    let so_res  = String::from_utf8_lossy(&out.stdout);
    let b1    	= so_res.split("\n").collect::<Vec<&str>>();
	let b2		= b1[1].split(":").collect::<Vec<&str>>();
	let binding	= b2[3].replace(" ", "").replace(".", "").replace("\"", "");
	let b3		= binding.trim();

	/* Return the temp as an integer */
	u8::from_str(b3).unwrap() as u8
	}


pub fn get_card_data(stp_3: &mut HashMap<String, String>) -> ()
	{
	let out					= system_output(CARD_DATA_FULL).expect("Failed to run nvidia-settings!");
    let so_res  			= String::from_utf8_lossy(&out.stdout);
	let mut stp_2: Vec<&str>= vec![];
	let mut x:	u8			= 0;

	/* Get to work! */
	stp_3.clear();
    let stp_1: Vec<&str>   = so_res.split("\n").collect::<Vec<&str>>();

	/* Now loop over command output */
	for l in &stp_1
		{
		if (l.contains("Attribute"))
			{
			stp_2.push(l);	// May go away

			let l2a			= l.split(" ").collect::<Vec<&str>>();
			let fnl_v		= l2a[5].replace(".", "");
			let mut fnl_k	= l2a[3].replace("'", "");

			/* We may need the fan number */
			if(fnl_k.contains("GPUCurrentFanSpeedRPM"))
				{
				let lcl_bm 			= l2a[4].split(":").collect::<Vec<&str>>();
				let mut fan_number	= lcl_bm[2].to_string(); 
				fan_number			= fan_number.replace("]", "");
				fnl_k 				= fnl_k+":"+&fan_number.to_string();
				fnl_k				= fnl_k.replace(")", "");
 				}

			/* Push our shizzle into the map */
			stp_3.insert(fnl_k, fnl_v.to_string());
			}

		x += 1;		
		}
	}


pub fn get_card_power(stp_3: &mut HashMap<String, String>) -> ()
	{
	let out					= system_output(CARD_DATA_PWR).expect("Failed to run nvidia-settings!");
    let so_res  			= String::from_utf8_lossy(&out.stdout);
	let mut x:	u8			= 0;
	let mut k:	String		= "".to_string();
	let mut v:	String		= "".to_string();
	let mut read: u8		= 0;
	let mut first_line: u8	= 0;

    let stp_1: Vec<&str>   = so_res.split("\n").collect::<Vec<&str>>();
	for mut l in &stp_1
		{
		/* Get to work */
		x = 0;
		if(l.contains("GPU Power Readings"))	{ read = 1; first_line = 1; }
		if(l.contains("Power Samples"))			{ break; }

		if(read==1)
			{
			if(first_line==1)
				{
				first_line=0;
				continue;
				}
			let l_local				= l.trim().replace("\t", "").replace("                        ", "");
			let mut l_local_boom	= l_local.split(":");
			for mut debris in l_local_boom
				{
				debris = debris.trim();						// More cleaning 
				if(x==0)	{ k = debris.to_string(); }		// The key
				else		{ v = debris.to_string(); }
				x += 1;
				}

			stp_3.insert(k.clone(), v.clone());				// Now store in the hash map
			}
		}
	}


/* Parse string and convert numerics to floats */
pub fn convert_power_draw(pwr_str: String) -> f32	{ pwr_str.replace(" W", "").parse().unwrap() }

/* Quick conversion to Farenheit */
pub fn celsius_to_farenheit(celsius: f32) -> f32	{ (celsius * 1.8) + 32.0 }

/* The below is based on two fans being present. Will create something more flexible or dynamic later. */
pub fn ret_fan_speed_avg(f1: &u16, f2: &u16) -> u16
	{
	// if(*f1==0 && *f2==0)	{ return 0 as u16; }
	if(*f1==0 && *f2==0)	{ return 0; }
	if(*f1==0 && (*f2>0))	{ return *f2; }
	if((*f1>0)&& *f2==0)	{ return *f1; }

	return (f1 + f2)/2;		
	}

pub fn ret_fan_speeds_rpm()	{}



