
# Cmds

A simple rust program that lets you create a keybind activated cmd prompt and configure it (HYPRLAND ONLY)


## Guide
- After a successful install press SUPER+SEMICOLON 
- Type in a command (Next section is all about commands) 
- Click ";" to sudmit and run the command
## Command Documentation

### Cmds that come with the program:
- hello => prints You typed 'hello'! (only if you run the cmds command in the terminal does it show up if you used keybind it does not)
- exit => prints Exiting... (same deal as hello)
Give more ideas for commands I could add
#### Add more commands:
1. Open the main.rs file and add a rust function of the thing u want it to do.
2. Go to the match statement under the process_input function and add the command you want to correspond to your function as a statement with a semicolon.
3. Compile using the following command:
``` bash
cargo build --release && sudo cp target/release/cmds /usr/local/bin/
```


## Installation

### Step 1
Clone this repo using
``` bash 
git clone https://quotequack/cmd_prompt/ && cd cmd_prompt
```
### Step 2
Compile and add it as a command
``` bash
cargo build --release && sudo cp target/release/cmds /usr/local/bin/
```
### Step 3
Navigate to your hyprland.conf with
```
cd ~/.config/hypr/
```
And using a text editor add the following line
```
bind = $mainMod, semicolon, exec, cmds
```
    
