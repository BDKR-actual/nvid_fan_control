


use epoch_timestamp::Epoch;             // Straightforward Unix Epoch as seconds
use crate::utility::timer;

impl timer
	{
	pub fn init_timer(&mut self, now_init: u64)
		{ self.now = now_init; }
	
	pub fn return_elapsed_time(&mut self) -> u64
		{
		self.new       = Epoch::now();
       	self.elapsed   = (self.new - self.now) + self.accum;

        /* Note that below operations are setting current meauserments and operations up 
   	    to be compared against or added too in the next iteration. */
       	self.now       = self.new;
        self.accum     = self.elapsed;

		return self.elapsed;
		}

	}







