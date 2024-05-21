# mpac v0.1.1

## Overview

**mpac** (short for "manage packages") is a simple command-line utility written in Rust. It helps users manage cloned repositories on their local systems, particularly those that are unlikely to have merge conflicts (e.g., tools you use).

## Current Features
- **List Repos:** Display all repositories under management.
- **Asynchronous Updating:** Utilizes Tokio for fast, asynchronous updates of all managed repos, saving time over manual `git pull` commands.
- **Simple CLI:** Offers an easy-to-use command-line interface with Clap, including robust error checking.
- **Add/Remove Repos:** Easily add or remove repositories from management.

## Upcoming Features
- Integration with `git2`
- Additional Git utilities
- Conditional updates (only pull when updates are available)
- More enhancements (see TODO comments)

## Contribution
- Open issues for feature requests or submit pull requests.

## Usage
- `mpac help`: Display the help menu
- `mpac list`: List all managed repositories
- `mpac update`: Update all managed repositories
- `mpac add --repo <path>`: Add a repository to be managed
- `mpac rm --index <num>`: Remove a repository from management

## Motivation
Updating manually cloned repositories can be tedious and time-consuming. mpac simplifies this process by automating the checking and updating of these repositories.
