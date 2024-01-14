# Rusty Timetracker

Small timetracker written in Rust with two purposes:
- Tracking the working time
- Helping me learn writing stuff in Rust

## Features

### Local storage

Every single data tracked/logged will be stored locally.  
If I ever implement a way to track this per a server, the local storage will be default and anything else will be opt-in and in addition to the local one.

### Compatible with multiple OS

Since this project is written in Rust, it should be compatible with multiple OS, given that it is compiled per command on your host system.

### Full control over the tracking

I disliked the 'break detection' offered by the default tool I used at work.  
With that in mind (and the fact that everything is stored locally), I want to give the user full control over his tracked/logged times.  
It is already stored locally and may be changed, also you probably use this tool on your own free will. A 'cheating' user will only lie to himself with his times.  
Purpose of this tool is to have a peace of its own mind over his own working time.

## Future plans

- [ ] Adding pictures to this README
- [ ] Adding a forgotten break over the UI
- [ ] Splitting the logged time data in more convenient structures:
  - [ ] Having a separate log file, with each day as its own entry
  - [ ] Adding a meta file that I could use to store stuff like data structure versions
    - [ ] ... to add the possibility to auto-migrate data after an update to a newer structure version
- [ ] Doing some fancy statistics with the tracked time
  - [ ] This may be accompanied by some cool graphs
- [ ] Calculating the time still needed to work
- [ ] Adding more customization to data, time tracking, working hours, ...
- [ ] Auto-compiling via GitHub actions
- [ ] Auto-Updates over the UI (only works after auto-compiling)
