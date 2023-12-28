# get_files_macro

A Rust macro that can resolve a directory's file names before compile-time.

# Status

Deployment status: [![Deploy](https://github.com/Kitt3120/get_files_macro/actions/workflows/deploy.yml/badge.svg)](https://github.com/Kitt3120/get_files_macro/actions/workflows/deploy.yml)

# Why?

I wanted to extend sqlx with a function that can check if a database's migrations are up-to-date. To do that, I needed to get the names of all files in a directory before compile-time. I couldn't find a way to do that, so I made this macro.
