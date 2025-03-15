# Contributing to Norris

Welcome to the contribution guide for **Norris**!

The purpose of this guide is to serve as comprehensive documentation regarding the repository structure, developing and running the bot, and making contributions to the project.

> [!IMPORTANT]
> As this is developed and used by students at the University of Nottingham, you are expected to follow the University's [Code of Discipline](https://www.nottingham.ac.uk/governance/documents/code-of-discipline-for-students-01082021.pdf) when making contributions or interacting with other contributors.

Use the table of contents icon on the top-right corner of this document to jump to a specific section of this guide quickly.

## Getting started

### Navigating the repository

**Norris** follows a simple repository structure, consisting of three core branches (`main`, `dev`, and `docs`) and any number of additional branches for new features, pull requests, and so on.

<details>
<summary>Branch structure</summary>

- `main` is the main (default) branch, meant for stable release versions of **Norris**.

    > [!TIP]
    > Avoid committing directly to `main`. You should instead commit to another relevant branch and merge into it once finished.

- `dev` is the core development branch, where most development occurs, and is intended to be merged into `main` after thorough reviews and testing.

    > [!NOTE]
    > There is also another development branch, `riir`, used for developing the Rust version of **Norris**. Like `dev`, this is intended to be merged into `main` after all changes are approved.

- `docs` is the branch for external documentation (i.e. *not* inline comment-based documentation) such as [design documents and diagrams](docs/), and is also intended to be merged into `main` after reviews.

- Other branches typically branch out from `dev` or `main`, and may be merged back into their source (`dev` or `main`) when necessary.

</details>

You should follow and enforce this branch structure when making any contributions or when reviewing the contributions of others.

### Multiple codebases

**Norris** has codebases in two different languages - Rust and Python.

The dual codebase structure is a consequence of initial prototyping, but was later kept and maintained together to act as a backup, and so that features could be quickly developed and tested on one version without being held back by bugs on the other.

Both versions function in almost the same manner, and follow a similar folder and code organisation structure.

> [!NOTE]
> The current (running) version of **Norris** uses Rust, due to unresolved bugs in the Python version.

While it is not necessary to port all changes to both versions, you are expected to maintain similar functionality and code structure if you update both codebases.

## Developing locally

### Installing prerequisites

You will need to install toolchains and software for the relevant language to develop or build **Norris**.

<details>
<summary>Setting up a Rust environment</summary>

1. Install the latest version of [Rust](https://www.rust-lang.org/tools/install), preferably using `rustup`.

    > [!TIP]
    > You should preferably install the `default` profile (as the name suggests, this is picked by default), which includes all the necessary components for general Rust development.

2. Install the Rust `nightly` toolchain by running `rustup toolchain install nightly`.

    > [!NOTE]
    > The `nightly` toolchain is required since the formatter configuration uses some `nightly`-only options. Compiling should be done using the `stable` toolchain.

3. Install [`rustfmt`](https://github.com/rust-lang/rustfmt) on the `nightly` toolchain by running `rustup component add rustfmt --toolchain nightly`.

</details>

<details>
<summary>Setting up a Python environment</summary>

1. Install version `3.11.4` of [Python](https://www.python.org/downloads).

    > [!NOTE]
    > Other versions are also acceptable, as long as they do not produce any errors or warnings. Avoid using versions older than `3.11.4`.

2. Install [`ruff`](https://github.com/astral-sh/ruff) by running `pip install ruff --upgrade`.

3. Install all of **Norris**' dependencies by running `pip install --requirement requirements.txt`.

</details>

### Using a development database

**Norris** requires a MySQL database to function, and will attempt to connect to it on startup.

You will therefore need to install one if you are planning to develop and test **Norris** on your local machine (i.e. not on a University-provided virtual machine).

> [!CAUTION]
> Use your local database only for testing. Do not store student data on your machine.

<details>
<summary>Setting up a local MySQL database</summary>

1. Download and install the [MySQL Community Server](https://dev.mysql.com/downloads/mysql).

2. Launch the MySQL client and create a new database.

    > [!IMPORTANT]
    > Note down the database name, server host, login details for the root user and other users - you will require them later.

3. Connect to the newly created database and run some queries to verify that it works.

    > [!IMPORTANT]
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

### Viewing code documentation

You can also generate a documentation website with a complete API reference for **Norris** using auto-documentation tools for the relevant language.

<details>
<summary>Documentation for the Rust version</summary>

1. From the project root, run `cargo doc --open`.

    > [!TIP]
    > You can omit the `--open` flag if you just want to re-generate the documentation without opening a new browser tab. You will need to refresh already open documentation tabs in this case.
</details>

<details>
<summary>Documentation for the Python version</summary>

1. Install [`pdoc3`](https://pdoc3.github.io/pdoc) by running `pip install pdoc3`.

    > [!NOTE]
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

    > [!IMPORTANT]
    > Copy the bot token and store it somewhere safe - you will require it later.

    > [!CAUTION]
    > Do not share the bot token publicly or commit it to Git, as this allows others to log in as the bot.

3. Set the bot application's logo, which can be downloaded from the University's [branding guidelines](https://www.nottingham.ac.uk/brand/visual/logos.aspx).

4. Disable the public bot option.
</details>

**Norris** also requires a set of permissions and gateway intents to function properly, including one *privileged* gateway intent.
These must be set from the Discord developer portal as well as by the bot itself during startup.

<details>
<summary>Granting permissions</summary>

1. In the bot application, navigate to `Bot > Privileged Gateway Intents` and enable the server members intent.

    > [!IMPORTANT]
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

    > [!TIP]
    > While not strictly necessary, it is recommended to use release mode instead of debug mode (the default) so the compiler can perform optimisations.

2. Execute the compiled binary by running `nohup ./target/release/norris &`.

    > [!TIP]
    > Using `nohup` prevents **Norris**' process from being stopped when its shell is terminated or logged out of. Using `&` starts the process in the background.

</details>

<details>
<summary>Starting the bot in Python</summary>

1. Execute the main bot script by running `nohup python main.py &`.

    > [!TIP]
    > Using `nohup` prevents **Norris**' process from being stopped when its shell is terminated or logged out of. Using `&` starts the process in the background.

</details>

<details>
<summary>Running the bot using docker</summary>

1. Git clone the repository 

2. `cd` into the working directory 

3. Build the docker image using the following `docker build -t Carnagion/norris .` 

4. Deploy either using the example `docker-compose.yml` via `docker compose up` or using the following command. 

```bash
$ docker run -d --name norris --network default --restart unless-stopped -v /path/to/norris.toml:/app/norris.toml Carnagion/norris:latest
```

> **Important:** The docker image will require a mysql database in order to function. This can be done by hosting a baremetal instance of MySQL or containerising it as shown in `docker-compose.yml`. **Please note** that docker doesn't like when SSL connections are used, this can be disabled specifically for Norris by attaching `?ssl-mode=DISABLED` to the end of the database URL in `norris.toml`.

> **Important:** The docker image will require a mount bind to a `norris.toml` configuration file in order to work. A path to such file can be specified using the `-v` flag. 

</details>

## Following conventions

### Git conventions

**Norris**' Git conventions are intended to make the repository easy to navigate.
These conventions apply to new branches, commits, files, and directories.

<details>
<summary>Branch names</summary>

- Branch names should be in `kebab-case`, without any uppercase letters.

- Avoid using numbers in branch names to represent new versions, and instead use descriptive (but brief) names.

</details>

<details>
<summary>Commit conventions</summary>

- Begin commit messages with a capital letter.

- Commit message titles should be in the imperative mood (i.e. `"Add new feature"` or `"Fix bugs"` instead of `"Added new feature"` or `"Fixed bugs"`).

- Keep commit message titles short but descriptive and specific.

</details>

<details>
<summary>File and directory names</summary>

- For language- or build tool-specific files and directories, use the naming conventions of the relevant language.

- Files and directories that do not belong to any language and were not generated by any build script or external process should be in `kebab-case`, without any uppercase letters.

</details>

### Code conventions

You should follow the code conventions set out for the relevant language when working on **Norris**, to make the codebase look cohesive and standard.

<details>
<summary>Following Rust code conventions</summary>

- Regularly format code by running `cargo fmt`.

    > [!TIP]
    > Many editors have an option to run a formatter upon saving a file - it is recommended to use such an option, if available.

- Regularly check whether code follows standard Rust conventions and idioms by running `cargo check` and `cargo clippy`.

    > [!TIP]
    > Some (not all) code convention violations can be automatically fixed by running `cargo fix` and `cargo clippy --fix`.
    
    > [!CAUTION]
    > Make sure to commit code to Git before applying automated fixes.

</details>

<details>
<summary>Following Python code conventions</summary>

- Regularly format code and check whether it follows conventions as defined by [PEP 8](https://peps.python.org/pep-0008) by running `ruff check . --fix`.

    > [!TIP]
    > Some code convention violations cannot be automatically fixed by `ruff` and must be fixed manually.
    
    > [!CAUTION]
    > Make sure to commit code to Git before applying automated fixes.

</details>

Continous integration pipelines also run on pushes and pull requests for both languages to ensure that the above conventions are followed.