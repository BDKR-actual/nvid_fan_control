
use std::time::{Duration, Instant};


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Clone)]
enum load_state 
	{
    low,
    normal,
    high,
	}


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Clone)]
pub struct load_controller 
	{
    state: 					load_state,
    threshold_normal:		u8,
    threshold_high: 		u8,
    hysteresis_width: 		Duration,

	in_hysteresis:			u8,
	in_normal:				u8,	

    current_load: 			u8,
    last_transition_time:	Instant,
	inactivity:				Duration,
	}



impl load_controller 
	{

    pub fn new() -> Self 
		{
        load_controller 
			{
            state: 					load_state::normal,					// Set initial state
            threshold_normal:		25,									// Don't really care about this. Leave for now.
            threshold_high:			75,									// When we consider the load high
            hysteresis_width:		Duration::from_secs(60 * 10),		// How long to remain in high/hysteresis

			in_hysteresis:			0,
			in_normal:				0,	

            current_load: 			0,									// Obvious really
            last_transition_time: 	Instant::now(),						// When a state transistion occured 
			inactivity:				Duration::from_secs(60 * 30),		// How long before transitioning to a lower power state (May need to refer to logged data)
        	}
    	}


	pub fn check_conditions(&mut self, new_load: u8)
		{
        self.current_load = new_load;
		
		/* Check and set in mode high / in_hysterisis */
		if(self.current_load > self.threshold_high)
			{
			println!("\nSetting high!\n");
			self.state 					= load_state::high;
			self.last_transition_time	= Instant::now();
			self.in_hysteresis			= 1;
			}
		/* Check and set if in normal / in_normal */
		else if( (self.current_load > self.threshold_normal) && self.in_hysteresis == 0 )
			{
			println!("\nSetting normal!\n");
			self.state 					= load_state::normal;
	
			if(self.in_normal == 0)
				{
				self.last_transition_time	= Instant::now();
				self.in_normal				= 1;
				}
			}	


		/* Check for a downward transition from high to normal */
		if(self.state == load_state::high)
			{
			if(self.last_transition_time.elapsed().as_secs() > self.hysteresis_width.as_secs())
				{
				self.state 					= load_state::normal;
				self.last_transition_time	= Instant::now();
				self.in_hysteresis			= 0;
				self.in_normal				= 1;
				}
			}


		/* Check for a downward transition from normal to low*/
		if(self.state == load_state::normal)
			{
			if(self.last_transition_time.elapsed().as_secs() > self.inactivity.as_secs())
				{
				println!("\nSetting low!\n");
				self.state 					= load_state::low;
				
				if(self.in_normal == 1)
					{
					self.last_transition_time	= Instant::now();
					self.in_hysteresis			= 0;												// <--- Just in case
					self.in_normal				= 0;
					}
				}
			}


		/* Some debugging ouutput. Can't hurt. */
		println!("\n\n");
		dbg!(&self);
		}


    pub fn return_state(&self) -> &str 
		{ 
		let mut ret_val:	&str = "";
		match(self.state) 
			{
			load_state::low		=> ( ret_val = "low" ),
			load_state::normal	=> ( ret_val = "normal" ),
			load_state::high	=> ( ret_val = "high" ),
			}

		ret_val
		}

	}

