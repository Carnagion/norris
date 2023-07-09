# Norris

**Norris** is the registration bot for the official [University of Nottingham Computer Science](https://www.nottingham.ac.uk/computerscience/) starters Discord server.

## Configuration ⚙️

**Norris** reads secrets and other configuration data from a `norris.toml` file on startup.

<details>
<summary>Configuration file format</summary>

- `bot-token` - **Norris**' Discord bot token
- `database-url` - a MySQL database connection URL in the format `mysql://username:password@host/dbname`
- `guild-id` - the ID of the guild where **Norris** will be running
- `log-path` - a path to a log file where logs during operation will be dumped
- `channels`
  - `arrival-channel-id` - the ID of the channel where new users will first land (aka `#foyer`)
  - `support-channel-id` - the ID of the channel to redirect users to for registration support (aka `#registration-support`)
  - `log-channel-id` - the ID of the channel to log each user's registration process to (aka `#registration-logs`)
  - `nickname-channel-id` - the ID of the channel where nickname requests will be posted for mentors to handle (aka `#nickname-queue`)
  - `undergrad`
    - `main-channel-id` - the ID of the main undergraduate channel containing both students and staff (aka `#atrium-chatter`)
  - `postgrad`
    - `main-channel-id` - the ID of the main postgraduate channel containing both students and staff (aka `#postgrad-atrium`)
    - `common-channel-id` - the ID of the student-only postgraduate channel (aka `#postgrad-common-chatter`)
- `roles`
  - `hierarchy`
    - `undergrad-role-id` - the ID of the role for undergraduate students
    - `postgrad-role-id` - the ID of the role for postgraduate students
    - `mentor-role-id` - the ID of the role for mentors
    - `senior-mentor-role-id` - the ID of the role for senior mentors
    - `honorary-mentor-role-id` - the ID of the role for honorary mentors
    - `undergrad-role-id` - the ID of the role for faculty members
  - `pronouns`
    - `he-him-role-id` - the ID of the "he/him" pronouns role
    - `she-her-role-id` - the ID of the "she/her" pronouns role
    - `they-them-role-id` - the ID of the "they/them" pronouns role
    - `xe-xem-role-id` - the ID of the "xe/xem" pronouns role
    - `any-pronouns-role-id` - the ID of the "any pronouns" role
    - `ask-pronouns-role-id` - the ID of the "ask me" pronouns role
  - `housing`
    - `jc-catered-role-id` - the ID of the role for catered Jubilee halls
    - `jc-self-catered-role-id` - the ID of the role for self-catered halls around Jubilee
    - `up-catered-role-id` - the ID of the role for catered University Park halls
    - `up-self-catered-role-id` - the ID of the role for self-catered halls around University Park
    - `private-house-role-id` - the ID of the role for private housing
</details>

<details>
<summary>Example configuration</summary>

```toml
bot-token = "norris-bot-token"
database-url = "mysql://norris-user:norris-password@localhost/norris-db"
guild-id = 1234567890987654321
log-path = "norris.log"

[channels]
arrival-channel-id = 1234567890987654321
support-channel-id = 1234567890987654321
log-channel-id = 1234567890987654321
nickname-channel-id = 1234567890987654321

[channels.undergrad]
main-channel-id = 1234567890987654321

[channels.postgrad]
main-channel-id = 1234567890987654321
common-channel-id = 1234567890987654321

[roles.hierarchy]
undergrad-role-id = 1234567890987654321
postgrad-role-id = 1234567890987654321
mentor-role-id = 1234567890987654321
senior-mentor-role-id = 1234567890987654321
honorary-mentor-role-id = 1234567890987654321
faculty-role-id = 1234567890987654321

[roles.pronouns]
he-him-role-id = 1234567890987654321
she-her-role-id = 1234567890987654321
they-them-role-id = 1234567890987654321
xe-xem-role-id = 1234567890987654321
any-pronouns-role-id = 1234567890987654321
ask-pronouns-role-id = 1234567890987654321

[roles.housing]
jc-catered-role-id = 1234567890987654321
jc-self-catered-role-id = 1234567890987654321
up-catered-role-id = 1234567890987654321
up-self-catered-role-id = 1234567890987654321
private-house-role-id = 1234567890987654321
```
</details>

## License 📜

## Contributors 👥

## Credits ⭐