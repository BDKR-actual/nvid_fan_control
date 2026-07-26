

use std::fmt;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct timer
	{
	pub now:		u64,
	pub new:		u64,
	pub accum:		u64,
	pub elapsed:	u64
	}


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct log_file_data
	{
	pub now:			u64,
	pub new:			u64,
	pub accum:			u64,
	pub elapsed:		u64,

	pub file_name:		String,
	pub file_path:		String,
	}


pub mod utility;

