

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



