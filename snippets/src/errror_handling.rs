//! https://doc.rust-lang.org/book/ch09-00-error-handling.html
//!
//! Rust has recoverable and unrecoverable errors. Unrecoverable errors are called panics.
//! Rust doesn't have exceptions. It has the `Result<T, E>` enum.
//! The `Result` enum is typically used when more than one thing could go wrong. `Option` is used when something could be something or nothing.
//! Functionally `Result<T, E>` and `Option<T>` are very similar.

#[test]
fn error_handling() {
    use std::error::Error;

    fn mult_add(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
        let x = a
            .checked_add(b)
            .ok_or("Overflow while adding".into())
            .and_then(|x| x.checked_mul(b).ok_or("Overflow while multiplying".into()));

        x
    }

    let result = mult_add(1, 2); // -> Result<u8, Box<dyn Error>> NOT u8

    // let y = x + 1; // -> error: cannot add `{integer}` to `Result<u8, Box<dyn std::error::Error>>`

    // Manual error handling
    let x = match result {
        Ok(x) => x,
        Err(_) => 0, // -> default value or anything else (that matches the type of x or returns from the function)
    };

    assert_eq!(x, 6);

    let x = result.as_ref().unwrap(); // `unwrap()` opt-in to not handle the error and panic instead (`unwrap()` moves the value -> `as_ref()`)

    assert_eq!(x, &6);

    let x = result.as_ref().expect("Error while multiplying or adding"); // `expect()` opt-in to not handle the error and panic instead with a custom error message

    assert_eq!(x, &6);

    let result = mult_add(16, 16); // -> Result<u8, Box<dyn Error>> NOT u8

    let x = result.as_ref().unwrap_or(&0); // `unwrap_or()` opt-in to not handle the error and return a default value instead

    assert_eq!(x, &0);

    // This is not a comprehensive list of all the methods on `Result`. Other means of error handling exist.
}

/// Error handling in Rust vs Python is fundamentally different.
/// Python does not require explicit error handling. It is opt-in. From the code, it is not apparent that a function can raise an exception.
/// Rust requires explicit error handling. It is opt-out. From the code, it is always apparent if a function can return an error.
/// This makes runtime errors in Python harder to debug than in Rust and significanly more common.
/// In Rust, the compiler will not let you forget to handle an error. In Python, you have to remember to handle the error.
/// This does make the code more verbose though.
#[cfg(feature = "python")]
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

/// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?#propagating-errors
///
/// The `?` operator can be used to propagate errors.
/// It can only be used in functions that return `Result` or `Option`.
/// As a strong recommendation it should be used in conjunction with the `anyhow` crate.
#[test]
fn question_mark_operator() {
    use std::collections::HashMap;

    use anyhow::{anyhow, Context, Result};

    const VALID_IDENTIFIERS: [&str; 2] = ["Fe", "Cu"];

    fn request(webpage: &str) -> Result<&str> {
        if webpage == "https://en.wikipedia.org/" {
            Ok("200")
        } else {
            Err(anyhow!("404"))
        }
    }

    fn get_content(response: &str) -> Result<&str> {
        if response == "200" {
            Ok("Iron")
        } else {
            Err(anyhow!("Could not get content from the response"))
        }
    }

    fn parse(content: &str) -> Result<HashMap<&str, &str>> {
        let mut information = HashMap::new();

        let mut content = content.split_whitespace();

        information.insert("name", content.next().ok_or(anyhow!("No name"))?);

        information.insert("weight", content.next().ok_or(anyhow!("No weight"))?);

        Ok(information)
    }

    fn get_information_about_element(identifier: &str) -> Result<HashMap<&str, &str>> {
        let webpage = "https://en.wikipedia.org/"; // + identifier..;

        let response = request(&webpage)?;

        let content = get_content(response)?;

        let information = parse(content)?;

        Ok(information)
    }

    #[derive(Debug)]
    struct Element {
        pub name: String,
        pub weight: f32,
    }

    impl Element {
        fn new(identifier: &str) -> Result<Self> {
            if VALID_IDENTIFIERS.contains(&identifier) {
                let information = get_information_about_element(identifier)
                    .context("Could not get information about element")?;

                let name = information.get("name").unwrap().to_string();

                let weight = information.get("weight").unwrap().parse::<f32>()?;

                let element = Element { name, weight };

                Ok(element)
            } else {
                Err(anyhow!("Invalid identifier"))
            }
        }
    }

    match Element::new("Fe").context("Could not create element") {
        Ok(element) => {
            assert_eq!(element.name, "Iron");
            assert_eq!(element.weight, 55.845);
        }
        Err(error) => {
            let mut chain = error.chain();

            assert_eq!(
                chain.next().unwrap().to_string(),
                "Could not create element"
            );

            assert_eq!(
                chain.next().unwrap().to_string(),
                "Could not get information about element"
            );

            assert_eq!(chain.next().unwrap().to_string(), "No weight");
        }
    };
}
