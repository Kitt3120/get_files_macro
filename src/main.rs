use get_files_macro::get_files;

pub fn main() {
    let file_names = get_files!(true, true, true, true, true, "/", "./test");

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
}
