# Mald Manager
Mald Manager is your one-stop-shop for tracking those salty users in your discord server!
- Have a user who rages everytime they fall off the ship in Sea of Thieves?
- Maybe unit testing makes them lose their cool?
- Counter Strike makes them break their keyboard?

Fret not! Mald Manager is here to track and shame all their mald'ings!

Mald Manager is a state-of-the-art bot that allows for tracking mald-levels across time and space, all from the comfort of your discord server!

## Available Commands
`!mald [@User]` - Increments a user's mald level for today.

`!demald [@User]` - Decrements a user's mald level for today.

`!mald_hist [@User]` - View the user's mald history.

`!mald_help` - View this help prompt.

## Development Guide
1. To get started running MaldManager for development, you need to [install Rust](https://www.rust-lang.org/tools/install) by following the instructions found at that link.

2. After installing Rust, clone the repo:
``` bash
git clone git@github.com:SylvanB/mald_manager.git
```

3. Navigate into the newly created folder contianing the source
``` bash
cd mald_manager
```

4. Now create a `.env` file in the root directory of the `mald_manager` folder with the following contents (replace the values wrapped in `<xxx>`) with the actual value.
``` bash
DISCORD_TOKEN=<DISCORD_BOT_USER_TOKEN>
MALD_LOCATION=<WHERE_TO_STORE_MALD_HISTORY>
``` 




