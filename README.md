<div align="center">
    <h1>solana-tx-data-extract-code-challenge</h1>
</div>
<br>
<div align="center">
    <img alt="Crates.io" src="https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square">
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/thebashpotato/vybe-check-challenge/rust.yml?style=flat-square&logo=ubuntu&label=Rust%20Build">
    <br>
    <p>Solana blockchain transaction extractor</p>
</div>

The submission is incomplete, I did a simpler version as I didn't have as much time to work on it as I hoped.
### 1. **Fetch and Process Blockchain Data**
Create an application in Rust that:
- **Interacts with the Solana blockchain** to fetch transaction data from the [Phoenix on-chain orderbook DEX](https://www.phoenix.trade/) program. **DONE**
- **Decodes transactions** and extracts trade fill events. **DONE**
- **Stores the processed data** in a database of your choice (Postgres): **DONE (does not store baseTokenMint, qouteTokenMint) doesn't provide any interval or aggregation**
- Runs **continuously** to fetch new transactions, parse them, and insert the extracted trade fill events into the database in real-time or near real-time: **DONE**

### 2. **Build an API Endpoint**
- The endpoint is simple, offers no querying/intervals etc. It just calculates OHLC based off all entries in database

## Expectations and Time Guidelines

- **Estimated Time**: This challenge is estimated to take approximately **4-6 hours** to complete the core requirements. This is not a hard limit but rather a guideline to help you plan your time. If you find yourself significantly exceeding this estimate, feel free to let us know.
    * Took me longer (6 hours Saturday, 10 hours on Sunday), although this is because I went in knowing nothing about Solana, order-books, trading data etc, and not having touched a database in anyway shape or form in 3 years. Although it was a great challenge and I really enjoyed myself.

## Table of Contents

- [Docs](#docs)
- [Install](#install)
- [Usage](#usage)

## Docs

1. Challenge description is [here](./docs/challenge-description.md)


## Install

> Note that commands should be ran in the root of the project

1. Install rust via [rustup](https://rustup.rs/)
    - Install the just command runner `cargo install just`
    - For development you will need the diesel cli tool: `cargo install diesel_cli --no-default-features --features postgres`

2. Install [PostGreSQL](https://www.postgresql.org/download/) for your platform with the following user and password
    - user: postgres
    - pw: coffee

3. Once your database is setup execute the following commands
    - Create a new database: `psql -U postgres -c "CREATE DATABASE solana_data"`
    - Populate the database you just created with the backup file: `psql -U postgres -d solana_data -f data/backup.sql`

4. The Just command runner
    - Running `just --list` will give you an overview of all commands available
    - Running `just dev` will compile the project with debug symbols
    - Running `just release` will compile in release mode
    - Run the tests: `just test`
    - Run the integration tests (this will delete the database): `just itest`

5. You will need to sign up at [Helius](https://www.helius.dev/) and get an api key

6. As a convenience for yourself you can set the following environment variable to your Helius api key
    - Bash/Zsh for example: `export HELIUS_RPC_KEY="your-api-key"`


## Usage

> Once you have set the database up and compiled you can run the following executables

1. To start the extractor daemon..
    - In a seperate terminal: `./target/debug/vn-extractord --api-key $HELIUS_RPC_KEY --log-level debug`

2. To start the api service..
    - In a seperate terminal: `./target/debug/vn-rest-api --log-level debug`

3. Open `http://127.0.0.1:8080/` in your browser
    - View simple OHLC data `http://127.0.0.1:8080/ohlc` It just uses all the available entries there is no interval support
    - View all raw data: `http://127.0.0.1:8080/trade_fills`

