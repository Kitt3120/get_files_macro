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
pub fn list_files(input: TokenStream) -> TokenStream {
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

        if entry_name.starts_with('.') && !arguments.include_dotfiles {
            continue;
        }

        if entry_type.is_dir() {
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
        } else if entry_type.is_symlink() {
            if arguments.include_symlinks {
                entries.push(entry_name);
            }
        } else if entry_type.is_file() {
            if arguments.include_files {
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

    #[test]
    fn test_files() {
        let arguments = Arguments {
            recursive: true,
            include_dotfiles: true,
            include_directories: true,
            include_symlinks: true,
            include_files: true,
            separator: String::from("/"),
            path: String::from("./test"),
        };

        let path = Path::new(&arguments.path);

        let file_names = get_file_names(&arguments, &path);

        let test_names = vec![
            "testfile1.test",
            "testfile2.test",
            "zzz",
            "zzz/testfile3.test",
        ];

        for (index, file_name) in file_names.iter().enumerate() {
            assert_eq!(file_name, test_names[index]);
        }
    }
}
