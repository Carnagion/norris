# Norris

**Norris** is the registration bot for the official [University of Nottingham Computer Science](https://www.nottingham.ac.uk/computerscience) starters Discord server.

## Features

- **User registration** - Users joining the server must go through a [multi-step registration process](docs/registration-process.png) before they are allowed access to most channels

- **Automatic de-registration** - Users leaving the server are automatically de-registered

- **Nickname requests** - Users can request nicknames, which must be approved by a mentor

- **Simple configuration** - Data is deserialized from a [configuration file](CONTRIBUTING.md/#runtime-configuration) at startup rather than using hard-coded values

- **Robust error handling** - Errors and failure cases are handled gracefully, without panics or crashes

- **Log files** - Errors, warnings, and other notable events are logged to files that are created on a daily basis

- **Fully documented** - Comprehensive documentation regarding [design](docs/) and [setup](CONTRIBUTING.md), intended for future maintainers

## Commands

**Norris** supports the following slash commands:

- `/registration add [NAME] [KIND]` - Add a user to the registration whitelist (usable by **administrators only**)

- `/registration restart [USER]` - Restart a user's registration (usable by **mentors and administrators only**)

- `/registration nuke [ROLE]` - Restart the registrations of all users with a particular role, defaulting to all undergraduate and postgraduate students if `ROLE` is not provided (usable by **administrators only**)

- `/nickname [NAME]` - Request a server nickname

- `/count undergrads` - Report the number of registered and total undergraduate students

- `/count postgrads` - Report the number of registered and total postgraduate students

## User data

**Norris** only stores the minimum amount of user data required to perform registration, as seen in its [database schema](docs/database-schema.png).

- Student names and kinds (undergraduate or postgraduate) are obtained from the University via the proper channels.

- Users that join the server have their Discord user ID stored to keep track of their registration state.

- Pronouns and housing information are provided voluntarily by users, can be skipped, and are not stored in the database.

Additionally, access to user data via the database is limited to server administrators only.

## Contributors

- [Indraneel Mahendrakumar](https://github.com/Carnagion)
- [Callum Daniel](https://github.com/Warrior2852)

## Credits

Thanks to:
- [Joe Sieniawski](https://github.com/jozefws) and the University of Nottingham Computer Science Research Support Team, for providing a database and server to host **Norris** on
- [Ben Flynn](https://github.com/Ben5656), for providing the previous year's codebase, which served as a foundation for the current bot

## Frequently asked questions

- #### Why is it called **Norris**?

    Who knows? We certainly don't.