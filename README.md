# Horace - A Command Line Life Journalling Tool
## This Is Your Life

Horace is a command line utility to help you keep track of things in your life. It is inspired by [Task Warrior](#) and [Nomie](#).


# Usage
## Don't Be Afraid - Just Knock On The Door

The command to use Horace is `life`. When no command is specified, it will search for logs

## log

The heart of Horace - `life log <whatever>` will add a new log entry, dated at the time the command is entered, containing whatever you put in `<whatever>`.

# Concepts

## Log

A log is a journal entry.

## Tag

A tag is a label for a log that can optionally have a value attached to it. The syntax for this is either `+metric` or `+metric:value`, where `metric` can be any alphanumeric string of text, and `value` is whatever value (text or number) you can to store for that metric in the particular log.

## Person

A person is someone who might be mentioned in a log. The syntax for this is `@name` or `@organisation.name`, where `name` and `organisation` can be any alphanumeric string of text.

## Location

A location is a place that might be mentioned in a log. The syntax for this is `%location`, where `location` can be any alphanumeric string of text
