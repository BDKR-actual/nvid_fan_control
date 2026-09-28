

extern crate dirs;
use epoch_timestamp::Epoch;             // Straightforward Unix Epoch as seconds
use crate::utility::timer;
use std::collections::HashMap;

use std::fs::File;
use std::fs::exists;
use std::env;
use std::
    {
    io::{prelude::*, BufReader},
    path::Path,
    };
use std::process::exit;


/* This is really a sub-function of read_config. Does as named and returns the file as lines in a vector */
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String>
    {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
    }


pub fn read_config(conf_store: &mut HashMap<String, String>)
    {
    let conf_path: String                       = "/etc/gpufanconf/config".to_string();

    /* Let's now open the file, iterate, and assign pertinent values */
    let lines_ref   = lines_from_file(conf_path);   // Get each line from the config file as an entry in a vector
    let lines       = lines_ref.clone();
    for l in &lines
        {
        if(l.contains("//") || l.chars().count()==0) { /* Do nothing */ }   /* Yes, I could do this another way, but this feels explicit. */
        else
            {
            if(l.contains(":"))
                {
                let mut b2: Vec<&str>       = l.split(":").collect::<Vec<&str>>();
                // I know the correct config format should only result in two elements. If more, I'll ignore the third + element(s). 
                // Parse the string and return it to it's own value 
                let mut b2_00: String = b2[0].replace("\t", "").trim().to_string();
                let mut b2_01: String = b2[1].replace("\t", "").trim().to_string();

                // Now push those good strings into the HashMap 
                conf_store.insert(String::from(b2_00), String::from(b2_01));
                }
            }
        }
    }


pub fn read_args(dbg_out: &mut u8, logging: &mut u8)
    {
    let args:       Vec<String>                 = env::args().collect();
    for arg in args
        {
        match(arg.as_str())
            {
            "--d"   => *dbg_out = 1,
            "--l"   => *logging = 1,
            "--h"   => show_help(),
            _       => {},
            }
        }
    }


pub fn show_help()
    {
    println!
        (
        "\nNVID Fan Control usage...
        \t--d   : Turns on debugging output.
        \t--l   : Turns on logging output.
        \t--h   : Show usage | List arguments.\n\n"
        );
    exit(0);
    }



