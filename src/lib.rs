use std::path::Path;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, LitBool, LitStr};

struct Arguments {
    recursive: bool,
    include_dotfiles: bool,
    include_directories: bool,
    include_symlinks: bool,
    include_files: bool,
    separator: String,
    path: String,
}

impl Parse for Arguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let recursive: LitBool = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let include_dotfiles: LitBool = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let include_directories: LitBool = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let include_symlinks: LitBool = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let include_files: LitBool = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let separator: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let path: LitStr = input.parse()?;

        Ok(Arguments {
            recursive: recursive.value,
            include_dotfiles: include_dotfiles.value,
            include_directories: include_directories.value,
            include_symlinks: include_symlinks.value,
            include_files: include_files.value,
            separator: separator.value(),
            path: path.value(),
        })
    }
}

#[proc_macro]
pub fn get_files(input: TokenStream) -> TokenStream {
    let arguments = parse_macro_input!(input as Arguments);

    let path = Path::new(&arguments.path);
    if !path.is_dir() {
        panic!("The path provided is not a directory");
    }

    let file_names = get_file_names(&arguments, &path);

    generate_tokens(file_names)
}

fn get_file_names(arguments: &Arguments, path: &Path) -> Vec<String> {
    let dir_handle = match path.read_dir() {
        Ok(handle) => handle,
        Err(err) => panic!("Could not read directory: {}", err),
    };

    let mut entries = Vec::new();

    for dir_entry in dir_handle {
        let dir_entry = match dir_entry {
            Ok(entry) => entry,
            Err(err) => panic!("Could not read directory entry: {}", err),
        };

        let entry_type = match dir_entry.file_type() {
            Ok(entry_type) => entry_type,
            Err(err) => panic!("Could not get file type of a directory entry: {}", err),
        };

        let entry_name = match dir_entry.file_name().into_string() {
            Ok(entry_name) => entry_name,
            Err(err) => panic!(
                "Unable to convert file name from OsString to String: {:?}\nIs it valid UTF-8?",
                err
            ),
        };

        if entry_type.is_dir() {
            if entry_name.starts_with('.') && !arguments.include_dotfiles {
                continue;
            }

            if arguments.include_directories {
                entries.push(entry_name.clone());
            }

            if arguments.recursive {
                let sub_path_buf = path.join(&entry_name);
                let sub_path = Path::new(&sub_path_buf);

                let sub_entries = get_file_names(arguments, sub_path);
                for sub_entry in sub_entries {
                    entries.push(format!(
                        "{}{}{}",
                        &entry_name, arguments.separator, sub_entry
                    ));
                }
            }
        } else {
            let should_add_as_symlink = entry_type.is_symlink() && arguments.include_symlinks;
            let should_add_as_file =
                entry_type.is_file() && !entry_name.starts_with('.') && arguments.include_files;
            let should_add_as_dotfile = entry_name.starts_with('.') && arguments.include_dotfiles;
            let should_add = should_add_as_symlink || should_add_as_file || should_add_as_dotfile;

            if should_add {
                entries.push(entry_name);
            }
        }
    }

    entries
}

fn generate_tokens(file_names: Vec<String>) -> TokenStream {
    let gen = quote! {
        vec![#(#file_names),*]
    };

    gen.into()
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{get_file_names, Arguments};

    fn test(arguments: &Arguments, test_names: Vec<&str>) {
        let path = Path::new(&arguments.path);

        let file_names = get_file_names(&arguments, &path);

        eprintln!("Got names: {:?}", file_names);

        assert_eq!(file_names.len(), test_names.len());
        for (index, file_name) in file_names.iter().enumerate() {
            assert_eq!(file_name, test_names[index]);
        }
    }

    #[test]
    fn test_files_all() {
        let arguments = Arguments {
            recursive: true,
            include_dotfiles: true,
            include_directories: true,
            include_symlinks: true,
            include_files: true,
            separator: String::from("/"),
            path: String::from("./test"),
        };

        let test_names = vec![
            "testfile1.test",
            "testfile2.test",
            "zzz",
            "zzz/testfile3.test",
            ".testfile.test",
            "testfile.link",
        ];

        test(&arguments, test_names);
    }

    #[test]
    fn test_files_no_dotfiles() {
        let arguments = Arguments {
            recursive: true,
            include_dotfiles: false,
            include_directories: true,
            include_symlinks: true,
            include_files: true,
            separator: String::from("/"),
            path: String::from("./test"),
        };

        let test_names = vec![
            "testfile1.test",
            "testfile2.test",
            "zzz",
            "zzz/testfile3.test",
            "testfile.link",
        ];

        test(&arguments, test_names);
    }

    #[test]
    fn test_files_no_directories() {
        let arguments = Arguments {
            recursive: true,
            include_dotfiles: true,
            include_directories: false,
            include_symlinks: true,
            include_files: true,
            separator: String::from("/"),
            path: String::from("./test"),
        };

        let test_names = vec![
            "testfile1.test",
            "testfile2.test",
            "zzz/testfile3.test",
            ".testfile.test",
            "testfile.link",
        ];

        test(&arguments, test_names);
    }

    #[test]
    fn test_files_no_symlinks() {
        let arguments = Arguments {
            recursive: true,
            include_dotfiles: true,
            include_directories: true,
            include_symlinks: false,
            include_files: true,
            separator: String::from("/"),
            path: String::from("./test"),
        };

        let test_names = vec![
            "testfile1.test",
            "testfile2.test",
            "zzz",
            "zzz/testfile3.test",
            ".testfile.test",
        ];

        test(&arguments, test_names);
    }

    #[test]
    fn test_files_no_files() {
        let arguments = Arguments {
            recursive: true,
            include_dotfiles: true,
            include_directories: true,
            include_symlinks: true,
            include_files: false,
            separator: String::from("/"),
            path: String::from("./test"),
        };

        let test_names = vec!["zzz", ".testfile.test", "testfile.link"];

        test(&arguments, test_names);
    }
}
