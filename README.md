# Exchange Platform

## Overview
This repository contains the source code for a limit order book trading system built entirely with Rust. The project is my take on the limit order book project series from [Manning](https://www.manning.com/liveprojectseries/fintech-platform-ser).

In this project I demonstrate:
- Implementing a matching engine to process existing and incoming limit orders
- Setting up a basic http server with actix-web
- Creating a command line interface to interact with the trading server with the reqwest library
- Implementing unit tests to ensure modules are working as expected
- Using traits and generics to create a flexible and reusable codebase
- Organizing a project into crates and modules within a cargo workspace
- ...and much more


## Usage
This project is still in development but you can clone this repo and run cargo build to compile the project and play with the trading server.
