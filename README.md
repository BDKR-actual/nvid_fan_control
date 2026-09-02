

# NVIDIA FAN CONTROLLER (nvid_fan_control)

As the title states. A somewhat simple but aggressive controller written for my RTX3090. The driver on the system was rather slow 
about increasing fan speed as the temperature rose. This resulted in the card reaching the allowed temperature cieling on more tha 
one occassion, which is somewhat worrying when you consider how much these things cost. 

Digging led me to [nvidia-settings](https://manpages.ubuntu.com/manpages/focal/man1/nvidia-settings.1.html) which is rather 
powerful and feature rich. The obvious conclusion once I played around with it a bit was "why not use this to query the card for 
temp and set the fan speed based on what comes back"?

And out popped this code. 

HOWEVER, since there are issues related to thermal cycling, differening load states and the concept of a hysteresis band have been 
added. This allows us to also controll how quickly a system cools down depending on it's utilization. But even in the high load_state, 
this controller is still more agressive than the OE drivers, as seen by the fan speeds accellerating to 100% before 70C (controlling the 
peak).  

Here is a good link talking about [thermal cycling](https://ansys.synopsys.com/blog/thermal-cycling-failure-in-electronics).

## Bad code warning! / DONE and TODO

This is my second project in Rust! Be cool. That said, I pounded this out initially in a near stream of thought kind of way. 
Believing that development is best done in an iterative way, refactoring is garaunteed. With that in mind, below are some of the 
recent changes and ideas that are brewing in my head. 

Here is the list of recent changes. 
* 09:02:26 A new fan probe utility added after finding out NVML isn't return the number of fans on a card.
* 09:02:26 Fan speed set and gpu power draw (current) is done using the NVML wrapper. 
* 08:30:26 Now setup to run as root. Required for NVML contol functionality.
* 08:30:26 New location of config, logging, and command files (etc/gpufanconf/)
* 08:30:26 Fixed bug that occurred as a result of not truncating the comm file after receiving a quit command. 
* 08:24:26 config file example now in place. 
* 08:20:26 Now issues command to driver to allow manual control of fan speeds. 
* 08:19:26 nvml_wrapper::set_fan_speed_rpm set to unimplemented until further testing. 
* 08:19:26 Fixed issues related to missing files and lack of "~" expansion. 
* 08:16:26 Can now respond to external commands via a super simple file based IPC mechansim. See ext_com.php. 
* 08:16:26 Via the mechanism mentioned above, logging can be started and stopped and cooling regimes can be selected. 
* 08:16:26 A new file, simply title commands is now in the .config/nvid_fan_controller directory. This facilitates exteranl commands. 
* 08:16:26 More logic moved out of main() proper. 
* 08:16:26 TODO!!!! Still need to create a sample config file!
* There is now a config file. It's placed in "/home/your_user_name/.config/nvid_fan_controller/ and named config. 
* In the same directory above is the log file. gpu_perf_log! 
* More cli arguments. Use --h to see. 
* There is a new module titled "control". It supports three cooling regimes. High, medium, and low obviously. 
* Implementation of a hysterisys band as part of the transition downward from high to medium cooling regimes. 


## Install

This utility (now) runs as root. To facilitate this make sure that there is an /etc/gpufanconf/ directory. In that directory, 
place the config_example file, but rename it too config. Also place two empty files named "command" and "gpu_fan_perf_log". 
Make sure these are writable!!!

Additionally, this also means that at this time, ext_com.php will need to be run as root. 

## Some bad is good / Style guide.

While I'm new to Rust, i'm not new to programming. So I'm experienced and I've found what I like. In this case they are ...

* Snake case
* [Whitesmiths](https://en.wikipedia.org/wiki/Indentation_style#Whitesmiths) formatting style. 

In other words, if you would like to contribute, these are the current guide lines. They are non-negotiables. 


Drop me a message! :-)
Cheers
