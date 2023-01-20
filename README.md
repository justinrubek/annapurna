# annapurna

This is currently very experimental but I will expand it over time.

## cli

From within the cli directory (`crates/cli`) you can run the program: `cargo run --bin cli`.
This will read recipes and current inventory from the `facts` directory and print out what can be created, and what ingredients are missing.
The file format is subject to immediate change and is not documented, but it is currently simple enough that it can easily be changed.
