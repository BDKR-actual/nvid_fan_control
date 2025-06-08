

use crate::charts;
use crate::nvid;
use crate::nvid::{*};
// use crate::nvid;
// use crate::nvid::nvid_data;
use crate::utility::{*};
use epoch_timestamp::Epoch;
use std::sync::mpsc::{*};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// use crate::nvid_fan_control::nvid::nvid_data;      //

/*
use nvid_fan_control::nvid;   
use nvid_fan_control::nvid::nvid_control;
*/

pub fn init_chart()
	{
	}

pub fn store_data_points() 
	{
	println!("We've gathered some data!");
	}


pub fn collect_chart_data(rx: Receiver<nvid::nvid_data>, mut dbg: u8)
    {
    /* Get setup */
    let mut av: u16         = 0;    let mut sys_time        = 0;
    let mut t_now: u64      = 0;    let mut t_new: u64      = 0;
    let mut t_accum: u64    = 0;    let mut t_elapsed: u64  = 0;
    let mut loop_val:u64    = 0;    let mut sleep_val: u64  = 1000;
    // let mut chart_data  = nvid_fan_control::charts::chart_collect
    let mut chart_data  	= charts::chart_collect
        {
        core_temp:          vec![],     core_temp_f:        vec![],
        ambient_temp:       vec![],     ambient_temp_f:     vec![],
        fan_speed:          vec![],     fan_speed_avg:      vec![],
        fan1_speed_rpm:     vec![],     fan2_speed_rpm:     vec![],
        gpu_power_draw:     vec![],     gpu_power_draw_flt: vec![],
        timestamp:          vec![]
        };
    // let mut timer_data = nvid_fan_control::utility::timer
    let mut timer_data 		= crate::utility::timer
        {
        now:        0,      new:        0,
        accum:      0,      elapsed:    0
        };

    /* Initialize our timer */
    timer::init_timer(&mut timer_data, Epoch::now());

    /* Now off to work with you */
    loop
        {
        /* Cool! We're in. Now let's push data into the chart_dat struct per ieration. */
        let new_data: Result<nvid::nvid_data, RecvTimeoutError> = rx.recv_timeout(Duration::from_millis(sleep_val));    // Short wait for data before giving up
        if(dbg==1)              { println!("\tChannel receive result is {}", new_data.is_ok()); }                       // Dbg output
        if(!new_data.is_ok())   { continue; }                                                                           // If no data, go to next iteration
        else
            {
            let unwr= new_data.unwrap();                                                                                // Get our data set
            av = nvid_settings::ret_fan_speed_avg(&unwr.fan1_speed_rpm as &u16, &unwr.fan2_speed_rpm as &u16);          // Generate a fan speed average

            /* */
            loop_val = unwr.new_intvl;
            if(sleep_val != unwr.new_sleep)
                { sleep_val = unwr.new_sleep; }

            /* Let's now store our collected data */
            chart_data.core_temp.push( unwr.core_temp );
            chart_data.core_temp_f.push( unwr.core_temp_f );
            chart_data.ambient_temp.push( 0 );
            chart_data.ambient_temp_f.push( 0 );
            chart_data.fan_speed.push( unwr.fan_speed );                                                                // Fan speed as a percentage
            chart_data.fan_speed_avg.push(av);                                                                          // Average fan speed. Assuming two fans for now. 
            chart_data.gpu_power_draw.push ( unwr.gpu_power_draw );
            chart_data.gpu_power_draw_flt.push( unwr.gpu_power_draw_float );
            chart_data.timestamp.push( timer::return_elapsed_time(&mut timer_data).to_string() );                       // This is calling an impl method defined elsewhere
            }

        /* Now sleep */
        // thread::sleep(Duration::from_secs(1));
        thread::sleep(Duration::from_secs(loop_val));
        }
    }

