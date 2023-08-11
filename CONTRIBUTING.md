# Contributing to Norris

## Getting started

### Installing prerequisites

You will need to install toolchains and software for the relevant language to develop or build **Norris**.

<details>
<summary>Setting up a Rust environment</summary>

1. Install the latest version of [Rust](https://www.rust-lang.org/tools/install), preferably using `rustup`.

    > **Note**
    > You should preferably install the `default` profile (as the name suggests, this is picked by default), which includes all the necessary components for general Rust development.

2. Install the Rust `nightly` toolchain by running `rustup toolchain install nightly`.

    > **Note**
    > The `nightly` toolchain is required since the formatter configuration uses some `nightly`-only options. Compiling should be done using the `stable` toolchain.

3. Install [`rustfmt`](https://github.com/rust-lang/rustfmt) on the `nightly` toolchain by running `rustup component add rustfmt --toolchain nightly`.

</details>

<details>
<summary>Setting up a Python environment</summary>

1. Install version `3.11.4` of [Python](https://www.python.org/downloads).

    > **Note**
    > Other versions are also acceptable, as long as they do not produce any errors or warnings. Avoid using versions older than `3.11.4`.

2. Install [`ruff`](https://github.com/astral-sh/ruff) by running `pip install ruff --upgrade`.

3. Install all of **Norris**' dependencies by running `pip install --requirement requirements.txt`.

</details>

### Using a development database

**Norris** requires a MySQL database to function, and will attempt to connect to it on startup.

You will therefore need to install one if you are planning to develop and test **Norris** on your local machine (i.e. not on a University-provided virtual machine).

> **Warning**
> Use your local database only for testing. Do not store student data on your machine.

<details>
<summary>Setting up a local MySQL database</summary>

1. Download and install the [MySQL Community Server](https://dev.mysql.com/downloads/mysql).

2. Launch the MySQL client and create a new database.

    > **Note**
    > Note down the database name, server host, login details for the root user and other users - you will require them later.

3. Connect to the newly created database and run some queries to verify that it works.

    > **Note**
    > Ensure that **Norris** has permissions to create, read from, update, insert into, and delete from tables.

</details>

Additionally, the Rust version of **Norris** requires the [`sqlx`](https://github.com/launchbadge/sqlx) CLI to validate the bot's SQL queries at compile time.

<details>
<summary>Setting up SQLx</summary>

1. Install the SQLx CLI by running `cargo install sqlx-cli`.

2. Through a `.env` file in the project root, set an environment variable named `DATABASE_URL` to the MySQL database connection URL, and `SQLX_OFFLINE` to `true`.

    ```bash
    DATABASE_URL="mysql://username:password@host/database"
    SQLX_OFFLINE=true
    ```

3. Run `cargo sqlx prepare` at regular intervals and commit any changes to the query metadata so that queries can be compiled successfully in [offline mode](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query).

</details>

### Viewing documentation

You can also generate a documentation website with a complete API reference for **Norris** using auto-documentation tools for the relevant language.

<details>
<summary>Documentation for the Rust version</summary>

1. From the project root, run `cargo doc --open`.

    > **Note**
    > You can omit the `--open` flag if you just want to re-generate the documentation without opening a new browser tab. You will need to refresh already open documentation tabs in this case.
</details>

<details>
<summary>Documentation for the Python version</summary>

1. Install [`pdoc3`](https://pdoc3.github.io/pdoc) by running `pip install pdoc3`.

    > **Note**
    > Ensure you install `pdoc3`, not `pdoc`, which is an unmaintained version of the same.

2. From the project root, run `pdoc3 norris --html --force`.

3. Navigate to the generated `html/` directory and open the `index.html` file in your browser.
</details>

## Running the bot

### Creating a bot application

To host a separate instance of **Norris**, you will require a Discord developer account and bot application.

If you are developing for an already existing instance of **Norris**, or already have a bot application set up, you can skip this step.

<details>
<summary>Creating a bot application</summary>

1. [Create a new application](https://discord.com/developers/docs/getting-started#step-1-creating-an-app) in the Discord developer portal.

2. In this application, navigate to `Settings > Bot` and create a new bot.

    > **Note**
    > Copy the bot token and store it somewhere safe - you will require it later.

    > **Warning**
    > Do not share the bot token publicly or commit it to Git, as this allows others to log in as the bot.

3. Set the bot application's logo, which can be downloaded from the University's [branding guidelines](https://www.nottingham.ac.uk/brand/visual/logos.aspx).

4. Disable the public bot option.
</details>

**Norris** also requires a set of permissions and gateway intents to function properly, including one *privileged* gateway intent.
These must be set from the Discord developer portal as well as by the bot itself during startup.

<details>
<summary>Granting permissions</summary>

1. In the bot application, navigate to `Bot > Privileged Gateway Intents` and enable the server members intent.

    > **Note**
    > Without this intent, the bot will not receive events when users join or leave the server.

2. Then navigate to `OAuth2 > URL Generator` and select the following scopes:
    - `bot`
    - `applications.commands`

3. Next, select the following permissions:
    - `Manage Roles`
    - `Manage Nicknames`
    - `Send Messages`

4. You can then use the generated URL to invite the bot to a Discord server.

</details>

### Runtime configuration

**Norris** reads secrets and other configuration data from a `norris.toml` file on startup.
Most of this data is kept around and used throughout its runtime.

> **Warning**
> Do not share the configuration file publicly or commit it to Git, as it contains sensitive information.

<details>
<summary>Breakdown of the configuration format</summary>

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

        - `admin-role-id` - the ID of the role held by server administrators

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
<summary>Example configuration file</summary>

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
admin-role-id = 1234567890987654321

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

### Starting the bot

Once your development environment and runtime configuration has been set up, you can run **Norris** using the relevant commands for the version you are working with.

<details>
<summary>Starting the bot in Rust</summary>

1. Compile the bot in release mode by running `cargo build --release`.

    > **Note**
    > While not strictly necessary, it is recommended to use release mode instead of debug mode (the default) so the compiler can perform optimisations.

2. Execute the compiled binary by running `nohup ./target/release/norris &`.

    > **Note**
    > Using `nohup` prevents **Norris**' process from being stopped when its shell is terminated or logged out of. Using `&` starts the process in the background.

</details>

<details>
<summary>Starting the bot in Python</summary>

1. Execute the main bot script by running `nohup python main.py &`.

    > **Note**
    > Using `nohup` prevents **Norris**' process from being stopped when its shell is terminated or logged out of. Using `&` starts the process in the background.

</details>