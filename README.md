# Dashberry

Dashberry is a dashboard app to monitor many aspects of your raspberry pi.

Following features are currently implemented:

- disk usage
- memory usage
- docker containers
- system info
- temperature

The design is based on [Raspi Dash](https://raspidash.com/).

## Build

Build the project:

```bash
dx bundle --web
```

On your raspberry, run `./target/dx/dashberry`.

## Development

The project is based on [Dioxus](https://dioxuslabs.com/), a Rust framework for building web, desktop and mobile apps.

It is recommanded to follow the [Getting Started](https://dioxuslabs.com/learn/0.7/getting_started/) tutorial.

Project structure

```none
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ cards/ # card components
│  ├─ backend.rs # backend API endpoints
│  ├─ main.rs # main.rs is the entry point of the application
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for the project
```

## Serving Dashberry

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform web
```
