
use std::fmt;   
use std::time::Instant;
use std::time::Duration;


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Clone)]
enum temp_hold_state 
    {
    Active,
    Hold,
    Release,
    }
  
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct cool_down_controller 
    {
    state:              temp_hold_state,
    timer_start:        Option<Instant>,
    cooldown_duration:  Duration,
    load_threshold:     f64,
    hysteresis_band:    f64,
    }

