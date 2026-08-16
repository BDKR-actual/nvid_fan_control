#!/usr/bin/php
<?php

# Check if argument is provided
if ($argc < 2) 
	{
	show_help();
    exit(1);
	}

# Get the command from argument
$command = $argv[1];

# Define valid commands
$valid_commands =
	[
    'force_low',
    'force_normal',
    'release_hold',
    'clamp_low',
    'clamp_high',
    'release_clamp',
	'start_logging',
	'stop_logging',
	'quit',
	];

# Validate the command
trim($command);
if (!in_array($command, $valid_commands)) 
	{
    echo "Error: Invalid command. Please use one of: " . implode(", ", $valid_commands) . "\n";
    exit(1);
	}

# Make sure path to file is setup
$file_path  = '~/.config/nvid_fan_controller/commands';
if(strstr($file_path, '~')) 
	{ $file_path = str_replace('~', $_SERVER['HOME'], $file_path);  }

# Write command to file
$result 	= file_put_contents($file_path, $command);

# C'ya!
if ($result !== false)		{ echo "Command '$command' written successfully to $file_path\n"; }
else						{ echo "Error: Failed to write command to file\n"; exit(1); }


function show_help()
	{
    echo "\nUsage: \nphp ext_com.php <command>\n\n";
    echo "Available commands: 
	force_low 
	force_normal  
	release_hold 
	clamp_low 
	clamp_high 
	release_clamp 
	start_logging 
	stop_logging 
	quit\n\n";
	echo "Examples:
	\"php ext_com.php clamp_low\"\t--> Forces low control range. Will not change until released!
	\"php ext_com.php force_normal\"\t--> Change to the normal state regardless of utilization.\n\n";
	}

?>
