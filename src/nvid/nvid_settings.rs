



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


