

# NVIDIA FAN CONTROLLER (nvid_fan_control)

As the title states. A somewhat simple but aggressive controller written for my RTX3090. The driver on the system was rather slow 
about increasing fan speed as the temperature rose. This resulted in the card reaching the allowed temperature cieling on more tha 
one occassion, which is somewhat worrying when you consider how much these things cost. 

Digging led me to [nvidia-settings](https://manpages.ubuntu.com/manpages/focal/man1/nvidia-settings.1.html) which is rather 
powerful and feature rich. The obvious conclusion once I played around with it a bit was "why not use this to query the card for 
temp and set the fan speed based on what comes back"?

And out popped this code. 

## Bad code warning! / AKA TODO

This is my second project in Rust! Be cool. That said, I pounded this out initially in a near stream of thought kind of way. 
Believing that development is best done in an iterative way, refactoring is garaunteed. With that in mind, below are some of the 
recent changes and ideas that are brewing in my head. 

Here is the list of recent changes. 
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
* A lot of the junk that was in main has been cleaned up. 
* The idea of charting has been jettisoned for now. 
* Bits of that charting code is still around (or stashed) for use in...


## Some bad is good

While I'm new to Rust, i'm not new to programming. So I'm experienced and I've found what I like. In this case they are ...

* Snake case
* [Whitesmiths](https://en.wikipedia.org/wiki/Indentation_style#Whitesmiths) formatting style. 

These are non-negotiables in MY code. Don't bother squawking about it. 

OTOH, if you hire me to write some codez for you, I'll do it in whatever style you desire. 

Drop me a message! :-)
Cheers
