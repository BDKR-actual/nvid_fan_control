

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

* Main is too big, much like my first project. LOL?
* There is a lot here that's supporting functionality I'm working on and not yet releasing. This also means that allow unused is on. 
* While I'm down with the borrow checker (we drink and go clubbing together), condtional creation of charting related threads is a no until the functionality is moved into a module of it's own. 
* Matching the core_temp to a target fan speed needs to also be moved into module of it's own. In doing this, we should also be able to gain the ability to add and select custom match statements based on allowable temperature floors or ceilings.
* Arguments to this are simplistic and few. It can be started in charting mode at this time, but this is largely for testing until other things are complete. The idea being that a different script will be called to send a message through the file socket telling the process to begin charting. Another message will be sent in the same fashing telling it to stop collecting data and render the chart. 


## Some bad is good

While I'm new to Rust, i'm not new to programming. So I'm experienced and I've found what I like. In this case they are ...

* Snake case
* [Whitesmiths](https://en.wikipedia.org/wiki/Indentation_style#Whitesmiths) formatting style. 

These are non-negotiables in MY code. Don't bother squawking about it. 

OTOH, if you hire me to write some codez for you, I'll do it in whatever style you desire. 

Drop me a message! :-)
Cheers
