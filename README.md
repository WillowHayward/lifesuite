# LifeSuite
## Because life is sweet

LifeSuite is a set of commandline applications which can be used together or independently to organise you life.

# Applications

LifeSuite is still in the early stages of development, so most of these are going to be in "Planned" stage for a while.

## Hub (Planned)

LifeSuite Hub is a standalone application will be a container for all of the other LifeSuite apps. 

## Journal (Under Development)

LifeSuite Journal is a data-driven life journalling app.

[Readme](./journal-cli/README.md)

[Design](./journal-cli/DESIGN.md)

## Finance (Planned)

LifeSuite Finance is a double-entry bookkeeping app designed for an individual to keep track of their personal finances.

## Lists (Planned)

LifeSuite Lists is a list application featuring support for nested lists.

## Tasks (Planned)

There was no need to develop a new task management app when something as perfect as [Taskwarrior](https://taskwarrior.org/) exists, so LifeSuite Tasks is more of an extended integration between Taskwarrior and other LifeSuite apps.

## Daemon (Planned)

Run syncing and other tasks in the background, while you focus on the more important stuff.

## Server (Planned)

A syncing server and API to interact with all the LifeSuite apps.

# Roadmap

For the moment, I'm trying to exclusively focus on LSJournal. It'll establish a lot of the fundamental aspects that will be used across all the LifeSuite apps.

Daemon and Server will both be started when I start implementing the sync functionality of LSJ.

Once LSJ is at a comfortable place, either Finance or Lists are next. Their designs are both pretty far along, but only exist as notebook pages right now.

Tasks is mostly napkin-level notes, so it's probably the last one I'll start on. It's gonna come in two flavours - integration (limited integration with existing Taskwarrior installations) and substitution (a full replacement of `task` - still TW behind the scenes using the [taskchampion crate](https://crates.io/crates/taskchampion), but with more LifeSuite integration and some minor interface changes to make it more consistent with other LS apps)
