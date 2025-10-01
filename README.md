# Pitbox - 🏎️ A Rust F1 Stats CLI
**Pitbox** is a CLI written in Rust to display F1 standings, results, and other stats in pretty formatted tables using data from the [jolpica-f1](https://github.com/jolpica/jolpica-f1) API.

This project is still in the early stages, and may change significantly over time.
## Features
- Display F1 driver and constructor standings after any round for any season in F1 history.
- View race results from any season.
- Compare driver performances through computed stats over a single season.
## Installation
#### From Crates.io (Recommended)
To install the latest release version, you will need Rust and Cargo installed
```bash
cargo install pitbox
```
## Usage
The different output types are separated into different subcommands to the main app, with each subcommand having it's own set of optional flags to use. Each subcommand has a shorthand version that is just one letter (usually the first letter of the command), e.g., For `standings` you can just use `s` for a shorter version.

`pitbox <command> [options]`
#### Standings
`pitbox standings <-c|-d> [options]`
You can specify constructor standings with `-c`, or driver standings with `-d`. Output will default to the latest standings of the current F1 season. However that behavior can be overridden using the season flag `-s` and/or the round flag `-r` to specify a round to display the standings relative to, e.g., if you wanted to show the driver standings for 2016 at round 13 you would use `pitbox standings -d -s 2016 -r 13`.
#### Race Results
`pitbox results [options]`
This command will display the race results of one race. No other part of the weekend is included in this output, only the race. The default race if no options are provided is the most recent race of the current season. This can be overridden by the season flag `-s` and/or the round flag `-r` to show the results from a particular season and a particular race.
#### Driver Results
`pitbox driver -n <names> [options]`
This command will display stats over a season for one or multiple drivers. Given you a way to compare driver performance. Currently these are the stats that are displayed for each driver:
- Total races
    - This includes classified finishes and retirements but does not include "Did Not Start" or "Disqualified" classifications.
- Best grid position
- Best finishing position
- Average grid position
- Average finishing position
- Total number of poles
- Total number of wins
- Total number of retirements
- Points from races (Does not include points from sprint races)
You can optionally pass a season flag `-s` to display a driver's race results from any season, otherwise it will default to results from the current season.
## Roadmap
This roadmap is very subject to change and includes a mix of big and small additions I'd like to add.
- [ ] Make some tests.
- [ ] Add a PKGBUILD to distribute through the AUR.
- [ ] Add a way to provide a circuit name or country through a flag to race results.
- [ ] Figure out some way to allow search of valid drivers/circuits for a given season with an easy way to select them to be used for a command.
- [ ] Add a teammate flag to the driver results command to quickly compare teammates over a season.
- [ ] For the standings output, add the gap to the leader as a new column, maybe as an optional flag to not clutter the table too much.
- [ ] Add a new subcommand to display qualifying results over a weekend. This could also be added as a flag to the race results command.
- [ ] Create a display to show a driver's performance at a specific track over their career or a custom range of seasons.
- [ ] Show polesitters and winners at a given track over a range of seasons.
- [ ] Cache API responses in a local SQL database. This is a really big feature and originates from my interest in learning more about databases.
## License
Pitbox is licensed under the [MIT License](LICENSE).
