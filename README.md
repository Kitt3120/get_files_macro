# get_files_macro

A Rust macro that can resolve a directory's file names before compile-time.

# Usage

```rust
// This scenario is meant to be executed from this repo's root

let file_names = get_files!(true, true, true, true, true, "/", "./test");
// ^^^^^^^^^^^ -> Vec<&str>

let test_names = vec![
    "testfile1.test",
    "testfile2.test",
    "zzz",
    "zzz/testfile3.test",
    ".testfile.test",
    "testfile.link",
];

assert_eq!(file_names.len(), test_names.len());
for (index, file_name) in file_names.into_iter().enumerate() {
    assert_eq!(file_name, test_names[index]);
}

println!("All tests passed!");
```

# Status

Deployment status: [![Deploy](https://github.com/Kitt3120/get_files_macro/actions/workflows/deploy.yml/badge.svg)](https://github.com/Kitt3120/get_files_macro/actions/workflows/deploy.yml)

# Why?

I wanted to extend sqlx with a function that can check if a database's migrations are up-to-date. To do that, I needed to get the names of all files in a directory before compile-time. I couldn't find a way to do that, so I made this macro.
