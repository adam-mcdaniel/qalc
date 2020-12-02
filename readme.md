# qalc

A language to quickly build multipurpose tools for calculations.

# Purpose

Effectively programming TI84s is very difficult. Writing **good** programs requires managing `Lbl`s for several different menu items. Additionally, the TI84 makes writing out programs and debugging _incredibly_ difficult.

To fix this, I made a half-baked attempt at writing a language that simplifies some of these problems. The reason the language is not nearly is good as it should be is **_time_**. I made this so I can work on my homework more effectively, _not_ so I can spend all day hacking on the compiler.

# Usage


There are very few commands, but they are very simple.

| Command | Usage | Example |
|----------------------------------|-------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------|
| `menu NAME { ... }` | Creates a menu of items of which you can choose from. Each item will run a different segment of code in response. | ``` menu "MENU" {   "ITEM1" {     print "You chose 1";    }    "ITEM2" {     print "You chose 2";   } }  ``` |
| `print ITEM...` | Prints something to the screen. | ``` print 1 "Hello world";  ``` |
| `input VAR` | Stores user input to a variable. | ``` input i;  ``` |
| `for VAR in FIRST..LAST { ... }` | Creates a for loop. | ``` for i in 0..10 {   print i; } ``` |
| `while ITEM { ... }` | Creates a while loop. | ``` while true {   print "hi"; } ``` |
| `if ITEM { ... }` | Creates an if statement with no else clause. | ``` a = true;  if a {   print "a is true"; }  ``` |
| `if ITEM { ... } else { ... }` | Creates an if statement with an else clause. | ``` a = true; if a {   print "a is true"; } else {   print "a is false"; } ``` |
| `clear` | Clears the screen. | ``` clear;  ``` |
| `stop` | Exits the program. | ``` stop; ``` |
| `pause` | Pauses the program and waits for the user to press ENTER. | ``` pause; ``` |
| `VAR = ITEM` | Stores ITEM into VAR. | ``` a = 5; ``` |
| `# COMMENT` | Adds annotations to code. |  ``` # Im a comment! ``` |

# Installation

Install qalc using cargo.

```
cargo install -f --git https://github.com/adam-mcdaniel/qalc
```
