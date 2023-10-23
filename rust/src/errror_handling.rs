/// Error handling in Rust vs Python is fundamentally different.
/// Python does not require explicit error handling. It is opt-in. From the code, it is not apparent that a function can raise an exception.
/// Rust requires explicit error handling. It is opt-out. From the code, it is always apparent if a function can return an error.
/// This makes runtime errors in Python harder to debug than in Rust and significanly more common.
/// In Rust, the compiler will not let you forget to handle an error. In Python, you have to remember to handle the error.
/// This does make the code more verbose though.
#[test]
fn error_handling_vs_python() {
    use indoc::indoc;
    use std::error::Error;

    fn get_file_content(file_name: &str) -> Result<String, Box<dyn Error>> {
        let list_of_files = ["file1", "file3"];

        if list_of_files.contains(&file_name) {
            Ok(format!("content{}", file_name.chars().last().unwrap()))
        } else {
            Err(format!("file not found").into())
        }
    }

    // `let file_content: String = get_file_content("file1");` -> expected struct `String` found enum `Result<String, Box<(dyn std::error::Error + 'static)>>

    let file_content: String = get_file_content("file1").unwrap(); // unwrap() opt-in to not handle the error and panic instead

    assert_eq!(file_content, "content1");

    // `match`` to handle the error -> manual error handling
    let file_content: String = match get_file_content("file2") {
        Ok(content) => content,
        Err(error) => {
            println!(
                "Problem opening the file: {} defaulting to empty String",
                error
            );
            "".to_string()
        }
    };

    assert_eq!(file_content, "");

    // boilerplate to run python code
    pyo3::prepare_freethreaded_python();

    pyo3::Python::with_gil(|py| -> pyo3::PyResult<()> {
        py.run(
            indoc! {  // indoc macro to write formatted multiline strings
                "
                def get_file_content(file_name):
                        if file_name == 'file1':
                            return 'content1'
                        elif file_name == 'file2':
                            return 'content2'
                        else:
                            raise Exception('file not found')

                file_content = get_file_content('file1')  # -> no explicit error handling required

                assert file_content == 'content1'

                # file_content = get_file_content('file4')  # -> Exception: file not found | it is not apparent from the code that this function can raise an exception

                # try except block is opt-in to handle the error
                try:
                    file_content = get_file_content('file1')
                except Exception as e:
                    print('Problem opening the file: {}'.format(e))
                "
            },
            None,
            None,
        )?;

        Ok(())
    })
    .unwrap();
}
