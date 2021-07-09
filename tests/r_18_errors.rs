use std::error::Error;
use std::fmt;
use std::process::{ExitCode, Termination};

// panic!("error");

// ____________________________________________________________

#[test]
fn result() {
    fn foo() -> Result<i32, String> {
        fn error_inner() -> Result<i32, String> {
            Result::Ok(1)
        }

        let x = match error_inner() {
            Ok(x) => x,
            Err(error) => return Err(error),
        };

        // Propagating
        let y = error_inner()?;
        Ok(x + y)
    }

    let result = foo();
    let option_ok = foo().ok();
    let option_error = foo().err();

    let value = foo().unwrap();
    let value = foo().expect("error");
}

// ____________________________________________________________
// Custom error

// LowLevelError
#[derive(Debug)]
struct LowLevelError;

impl Error for LowLevelError {}

impl fmt::Display for LowLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LowLevelError")
    }
}

// HighLevelError
#[derive(Debug)]
enum HighLevelError {
    TypeA(LowLevelError),
    TypeB,
}

impl Error for HighLevelError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HighLevelError::TypeA(x) => Some(x),
            HighLevelError::TypeB => None,
        }
    }
}

impl fmt::Display for HighLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HighLevelError::TypeA(x) => "HighLevelError typeA",
            HighLevelError::TypeB => "HighLevelError typeB",
        };
        write!(f, "{}", s)
    }
}

impl From<LowLevelError> for HighLevelError {
    fn from(err: LowLevelError) -> HighLevelError {
        HighLevelError::TypeA(err)
    }
}

#[test]
fn custom_error() {
    fn get_low_level_error(b: bool) -> Result<i32, LowLevelError> {
        if b {
            Ok(1)
        } else {
            Err(LowLevelError {})
        }
    }

    fn get_high_level_error(b: bool) -> Result<i32, HighLevelError> {
        let x = get_low_level_error(b)?;
        Ok(x + 1)
    }

    match get_high_level_error(true) {
        Err(e) => {
            println!("Error Display: {e}");
            if let Some(source) = e.source() {
                println!("Caused by: {source}");
            }
        }
        _ => println!("No error"),
    }

    // Map error
    fn foo() -> Result<i32, String> {
        let x = get_high_level_error(true).map_err(|_e| "Error".to_string())?;
        Ok(x + 1)
    }

    // To dyn Error
    fn foo3() -> Result<i32, Box<dyn Error>> {
        // impl<'a, E: Error + 'a> From<E> for Box<Error + 'a>
        get_high_level_error(true)?;
        Ok(10)
    }
}

// Custom exit codes from main

#[repr(u8)]
pub enum CustomResult {
    A = 0,
    B = 1,
    C = 2,
}

impl Termination for CustomResult {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

fn main() -> CustomResult {
    CustomResult::A
}

// Result implements Termination
// Ok translated to a C EXIT_SUCCESS and Err to EXIT_FAILURE

/*
fn main() -> Result<(), std::io::Error> {
    let _f = std::fs::File::open("bar.txt")?;
    Ok(())
}
*/
