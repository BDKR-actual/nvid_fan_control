

# NVIDIA FAN CONTROLLER (nvid_fan_control)

As the title states. A somewhat simple but aggressive controller written for my RTX3090. The driver on the system was rather slow 
about increasing fan speed as the temperature rose. This resulted in the card reaching the allowed temperature cieling on more tha 
one occassion, which is somewhat worrying when you consider how much these things cost. 

Digging led me to [nvidia-settings](https://manpages.ubuntu.com/manpages/focal/man1/nvidia-settings.1.html) which is rather 
powerful and feature rich. The obvious conclusion once I played around with it a bit was "why not use this to query the card for 
temp and set the fan speed based on what comes back"?

And out popped this code. 

I've since been working on the ability to chart the performance of the card (in terms of temperature based on load and time). In a 
nutshell, this involves the use of a single thread (communicating via MPSC / Thanx [Trevor](https://www.youtube.com/trevorsullivan)!) that 
will be responsible for collecting this data. There is also an open Unix Domain file socket that will listen for a command to start and stop charting. This uses the 
[charts-rs](https://crates.io/crates/charts-rs) crate. 

That functionality will be along shortly. 


## Bad code warning! / AKA TODO

This is my second project in Rust! Be cool. That said, I pounded this out initially in a near stream of thought kind of way. 
Believing that development is best done in an iterative way, refactoring is garaunteed. With that in mind, below are some of the 
things I'm already noting for change. 

Here is the list of recent changes. 
* There is a new module titled "control". It supports three cooling regimes. High, medium, and low obviously. 
* Implementation of a hysterisys band as part of the transition downward from high to medium cooling regimes. 
* Passing "--d 1" will get you debugging output. This could be redirected into a file if you need to share this information. 
* A lot of the junk that was in main has been cleaned up. 
* The idea of charting has been jettisoned. 
* Bits of that charting code is still around (or stashed) for use in...
* Logging. This will also result in the addition of more runtime arguments.


## Some bad is good

While I'm new to Rust, i'm not new to programming. So I'm experienced and I've found what I like. In this case they are ...

* Snake case
* [Whitesmiths](https://en.wikipedia.org/wiki/Indentation_style#Whitesmiths) formatting style. 

These are non-negotiables in MY code. Don't bother squawking about it. 

OTOH, if you hire me to write some codez for you, I'll do it in whatever style you desire. 

Drop me a message! :-)
Cheers
