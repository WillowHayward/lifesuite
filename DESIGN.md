# Horace - A Life Journal

This is a living document detailing the design of Horace

## Contents
1 - Introduction
    1.1 - Terminology
2 - Concepts
    2.1 - Journals
    2.2 - Logs
    2.3 - Tags, Entities, and Contexts
        2.3.1 - Values
        2.3.2 - Constraints
    2.4 - Mods
    2.5 - Templates
    2.6 - Events
    2.7 - Reports
3 - Commands
    3.1 - journal
        3.1.1 - add
        3.1.2 - edit
        3.1.3 - delete
        3.1.4 - list
    3.2 - log
    3.3 - edit
    3.4 - modify
    3.5 - logs
        3.5.1 - add
        3.5.2 - edit
        3.5.3 - delete
    3.6 - tag
        3.5.1 - add
        3.5.2 - edit
        3.5.3 - delete
    3.7 - entity
        3.5.1 - add
        3.5.2 - edit
        3.5.3 - delete
    3.8 - context
        3.5.1 - add
        3.5.2 - edit
        3.5.3 - delete
    3.9 - template
        3.5.1 - add
        3.5.2 - edit
        3.5.3 - delete
    3.10 - event
        3.5.1 - add
        3.5.2 - edit
        3.5.3 - delete
        3.5.4 - start
        3.5.5 - end
    3.11 - report
    3.12 - sync
4 - Storage
5 - Syncing
    5.1 - LifeSync
    5.2 - Sync Modes
6 - Roadmap

# 1 - Introduction

Horace is a command line application for data-driven life journalling.

## 1.1 - Terminology
// TODO: Short form of 2.* (1-2 sentences each)
    logs - A timestamped journal entry which can optionally have tags attached to it
    tags - A named attribute associated with one or more logs, can optionally have a value attached to it (one for each associated log)
    entities, contexts - Subtypes of tags (this is probably the least clean part of the design at the moment, I'll need to see them in action but I'm considering chopping them)
    constraints - A mechanism to enforce structure in tags (such as requiring a value, or only accepting values of a certain type). Logs which violate the constraints of any of their tags will be rejected.
    journals - A named set of logs. Every log is attached to one (and only one) journal, with the default name of default
# 2 - Concepts
## 2.1 - Journals

A journal is a named collection of logs, tags, . 

A journal is automatically created when its first log is created, or can be manually created with the `journal create` command.

A journal has the following properties.

 - id - a uuid generated on the creation of the journal
 - created - the timestamp of when this journal was created
 - name - 
 - description - 
 - mods - an array of mod ids that have been applied to the journal

## 2.2 - Logs

A log is a timestamped journal entry. 

Logs are created with the `log` command.

A log has the following properties.

 - id - a uuid generated on the creation of the journal
 - created - the timestamp of when this log was created
 - date - the timestamp of when this log is dated (e.g. A log created with `ago:1week` would have a `date` one week before `created`)
 - entry - the text of the journal entry
 - journal - the id of the journal the log belongs to
 - tags - an map of tag ids and the connected to that tag for this log. This is generated from the entry on log creation, and can be manually edited with the `edit` command
 - mods - an array of mod ids that have been applied to the log

Any log property aside from `id` can be edited with the `edit` command. When the entry property is edited in this way, tags will not be generated from the updated entry, however tags can be edited from the same command.

## 2.3 - Tags, Entities, and Contexts

A tag is a named attribute associated with any number of logs, optionally with a value for each individual log it is associated with. The same tag can be associated with 1 log multiple times.

Tags can be manually created with the `tag add` command, and are automatically created when a log is created whose entry contains text following one of these patterns:

 - `+<name>`
 - `+<name>:<value>`
 - `+<name>::<value map>`

`<name>` can be a single word composed of alphanumeric characters and underscores (`_`). If it includes periods (`.`), it will generate multiple tags, separated by the period. The rightmost tag, the only one that will be directly associated with the log, has its parent tag to its immediate left, and so on, up to the leftmost tag.

See 2.3.1 for more information about acceptable values.

Entities and contexts are subtypes of tags. An entity is a type of tag denoting a specific subject (e.g. a person, pet, or mysterious figure in the corner of your eye), and a context is a type of tag denoting a specific domain (e.g. a location, or a broad association like "work"). Entities and contexts share all other features with tags (including values), except that instead of being denoted with `+`, entities are denoted with `@` (`@<name>`) and contexts with `%` (`%<name>`). Unless otherwise noted, anything that applied to a log will also apply to the subtypes of logs.

A tag has the following properties. NOTE: These are the values of the tag itself, values of a tag instance are stored as part of the `tag` property on a log.

 - id - a uuid generated on the creation of the tag
 - created - the timestamp of when this tag was created
 - parent - the id of the parent tag, or the id journal if there is no parent tag
 - name - the full name of the tag, including the signifier (`+`/`@`/`%`) and all parent tags (joined by `.`)
 - type - tag, entity, or context
 - constraints - the constraints associated with this tag (see 2.3.2 for more information about constraints)
 - mods - an array of mod ids that have been applied to the tag

### 2.3.1 - Values

There are several different value types for tags.

 - number - Any number of digits. Can be negative. Can start with +, which will be [figure out if it'll be removed or treated as relative]
 - string - Any number of utf-8 characters. Can be quoted to allow shell-unsafe characters.
 - boolean - true or false.
`<value>` can be any of formats/types
 - date - [TODO] - Modified ISO 8601 (replacing `:` with `-`, allow partial values, allow US/AU dates, etc), 24hr time, am/pm
 - list - An array of scalar values, separated with `:` (e.g. `value1:value2:value3`). Lists of a single item are denoted by suffixing them with `:` (e.g. `:value:`).
 - map - A set of key-value pairs, separated by `:` (e.g. `key1:value1:key2:value2`). Maps are denoted in a log by being preceded separated from the tag with `::`.

Numbers, strings, and boolean values are collectively referred to as "scalar values".

Number, string, boolean, and date values are all stored as strings in the format they were originally entered as, and are parsed by reporting commands as whatever report filter they fit, unless a `type` constraint is set in which case parsing will be narrowed down by that constraint.

### 2.3.2 - Constraints

A constraint is a rule or set of rules applied to tags that must be obeyed for a log to be created with that tag.

A constraint has the following properties, stored in the `constaints` property of the tag they apply to.
 - required - a boolean value representing if a value must be attached to instances of this tag
 - default - the value assigned to an instance of this tag if one is not specified
 - prompt - a boolean value representing if the user should be prompted to provide a value
 - type - the type of the tag  - number, string, boolean, date, scalar (for any scalar type), list, or map.
 - values - acceptable values for this tag. Depending on the type of the tag, this may come in multiple different formats.
    - for scalar values, this will be an array of values that can be accepted
 - subconstraints - additional constraints applied to list and map types only (see below)
 - values - a list of acceptable values for this tag (scalar only)
 - link - a list of ids of a corresponding tags (including entities and contexts) that must be present on logs where this tag is present. Not mutual - a link constraint must be present on both tags to be mutual.

list subconstraints
 - minlength - the minimum length of the list
 - maxlength - the maximum length of the list
 - type - acceptable types for list values (scalar types only). Can be single value or list of values.
 - items - further constraints to be applied to particular items in the list
     - index - the index of the list this subsubconstraint will target. This must be present.
     - default // TODO: Consider edge cases of this?
     - prompt(?) // TODO: As above
     - type - the type for the list value (scalar types only, overrides `type` of parent for this index)
     - values - a list of acceptable values for this index (scalar only)

list item

map item subconstraints
Map subconstraints are a list of objects with the following properties
 - key - the map key this subconstraint will target. This must be present.
 - required
 - default
 - prompt - if this key isn't specificy, should the user be prompted?
 - type - the type of this value. Can be list. Must be scalar.
 - values - a list of acceptable values for this index (scalar only)
 - link(?)

## 2.4 - Mods

A mod is an immutable record of a change to a journal, log, tag, constraint, template, or event.

Mods are created when a change is made by any `edit` command

Properties:
 - id - a uuid generated on creation
 - created - the timestamp of when this mod was created
 - target - the id of the target journal, log, tag, constraint, template, or event
 - changes - an array of changes in the form of objects with the following properties
    - target - the property this change is targeting (e.g. "entry" for a log, "required" for a constraint). For changes to subproperties, this will include a list of all ancestory property changes separated by `.` (e.g. `grandparent.parent.property`), and for changes to individual
    - type - What type of change this was (create, delete, or modify)
    - old (delete and modify only) -  the value of the target property prior to this change
    - new (create and modify only) - the value of the target property after this change

## 2.5 - Templates

A template is a pre-defined structure for logs to be created with.

Properties:
 - id
 - created

## 2.6 - Events

An event is a time-bound occurrence with defined start and end points. Events can recur, but not overlap.

Events 

Properties:
 - id
 - created

TODO: `^` for events.

## 2.7 - Reports

A report is a generic term for displaying information derived from data about logs, including tags and tag values.

# 3 - Commands

Commands are triggered with the `life` binary, in the format `life <command> <args>`

## 3.1 - log <opts?> <entry?> <opts?>

Create a log with the provided entry and options. 

 - <entry?> - Any number of words consistent of utf-8 characters. Can be quoted to allow shell-unsafe characters. If no entry is provided, `$EDITOR` will be opened and the entry will be populated by the contents of that file. If the resulting entry is empty, the command will display a message and do nothing.
 - <opts?> - A space-separated list of any number of options from the list below in the format `option:value`. Options can both precede and follow the entry.

### 3.1.1 - Options

 - journal - The name of the journal this entry belongs to. If not provided, will use `default`

## edit <target>

Open `$EDITOR` to modify the <word other than entity or rename entities> attached to the given id.

Creates a mod derived from any changes found when `$EDITOR` is closed.

Journals and tags will be translated to their name for readability.

Mods are never shown on the edit screen.

 - <target> - The id or name of the target journal, log, tag, constraint, template, or event.

## mod(ify)

Create a mod without $EDITOR. Probably divided by type like w/ edit

## journal <subcommand>

### 3.1.1 - add 

Creates a journal with the name and description provided. 

`add <name> <description>`
 - <name>  - A single word consisting of any number of alphanumeric characters, as well as the following characters: `-`, `_`, and `.`. `<name>` cannot be `default`, `add`, `edit`, `delete`, or `FreeBird`.
 - <description> - Any number of words consisting of utf-8 characters. Can be quoted to allow shell-unsafe characters.

### edit

## tag
### add <target?> <opts?>
### edit <target>

## entity
### add <target?> <opts?>

An alias of `tag create` that only accepts the ids and names of entities (with or without `@`)

### edit <target>

## context
### add <target?> <opts?>

An alias of `tag create` that only accepts the ids and names of contexts (with or without `@`)

### edit <target>

## template
### add
### edit
## event
### add
### edit

## retry

Retry the last failed command. For failed `edit` commands, will reopen $EDITOR with attempted changes in place and error highlighted at the top

# 4 - Storage

For the moment, storage will mostly be in the form of json files. This might not scale well, and will likely be revised prior to 1.0

Under the directory `~/.life`, the following files and subdirectories will be created

 - journals
    - 

# 5 - Syncing

## 5.1 - LifeSync

## 5.2 - Sync Modes

 - Sync every time (probably slowest)
 - Sync every n times
 - Daemon (for people with a fast computer and slow internet)
    - Stray idea: This is how you could handle TaskWarrior sync without implementing their api? Set up a hook or a file watcher and then export/import manually. Food for thought.
# 6 - Roadmap

0.2.0
    - Commands
        - log add
        - journal add
        - tag add
    - No constraints
    - No templates
    - No events
    - No mods
    - Filter by tag names and journal
    - Scalar types only
0.3.0
    - Able to edit journals, logs, tags
    - Mods
    - Implement Tag constraints
0.4.0
    - date, list, and map types for tag values
0.5.0
    - Implement templates
    - Advanced filters
0.6.0
    - Implement events
0.7.0
    - Start on server
0.8.0
    - Start on daemon


------------------------------------
COMMANDS
journal
    add
    edit
    delete
log
edit
modify
logs
    add
    edit
    delete
tag
    add
    edit
    delete
entity
    add
    edit
    delete
context
    add
    edit
    delete
template
    add
    edit
    delete
event
    add
    edit
    delete
report
sync
    init
    configure
