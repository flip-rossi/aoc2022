These are my solutions for the [2022 edition](https://adventofcode.com/2022) of the annual event **Advent of Code**.
They were made in Rust to learn and practice the language, so probably not very good code, but getting there.  
The code for each solution is in [src/bin/](src/bin).

### Helper scripts
I made two helper scripts to prepare solutions and submit answers:  
- [setup_day.sh](setup_day.sh)  
  Creates a new source code file in [src/bin/](src/bin) from a [template](template.rs)
  and downloads the day's input to [inputs/](inputs).
- [submit_answer.sh](submit_answer.sh)  
  Submits the answer for the day and selected puzzle part.
  If no answer is passed as argument, reads from clipboard.

They both need you to have a ".env" file in the root of the project with a line like this: `SESSION_TOKEN=yoursessiontoken`  
You can get your session token by looking at your cookies when logged on in the [AoC website](https://adventofcode.com).
But be careful as to **not let *anyone* have access to your token**.
