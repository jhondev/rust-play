use failure::{Backtrace, Fail};
use quick_error::ResultExt as ResultQuickExt;
use serde::Deserialize;
use serde_json;
use std::{env, error::Error, fmt, fs::File, fs::OpenOptions, io, result};

//***********************************************Handling Result */
#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

pub fn handle_result() {
    let first = serde_json::from_str::<Person>(
        r#"{
        "name": "Margaret Hamilton",
    }"#,
    );

    let first_inner = match first {
        Ok(inner) => inner,
        Err(_) => Person {
            name: "unknown".to_owned(), // it will enter cause the comma in the json
        },
    };

    println!("first's name = {:?}", first_inner.name);
}

//**********************************************Generalizing error */
// use box to propagate different error types
fn num_threads() -> result::Result<usize, Box<dyn Error>> {
    let s = env::var("NUM_THREADS")?; // error propagation with ?
    let n: usize = s.parse()?; // multiple error propagation with ?
    Ok(n + 1)
}

pub fn box_error() {
    match num_threads() {
        Ok(num) => println!("the number of threads is {}", num),
        Err(e) => println!("error getting num threads: {}", e),
    }
}

//**********************************************Custom Errors */
#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

pub type Result<T> = result::Result<T, DocumentServiceError>;

impl Error for DocumentServiceError {}

impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(f, "Limit exceeded"),
            Io(ref io) => write!(f, "I/O error: {}", io),
        }
    }
}

impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

pub fn create_document(filename: &str) -> Result<File> {
    let last_min_docs = 5;
    let max_docs = 3;
    if last_min_docs > max_docs {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?; // `?` uses from to convert the error type

    Ok(file)
}

//*************************************************Using Crates [QuickError] */
quick_error! {
    #[derive(Debug)]
    pub enum DocumentServiceErrorQuick {
        RateLimitExceeded {
            display("Limit exceeded [QuickError]")
        }
        Io(filename: String, cause: io::Error) {
            display("I/O error: {} for filename {}", cause, filename)
            context(filename: &'a str, cause: io::Error)
                -> (filename.to_string(), cause)
        }
        // Io(cause: io::Error) {
        //     display("I/O error: {}", cause)
        //     from()
        // }
    }
}

pub type ResultQuick<T> = result::Result<T, DocumentServiceErrorQuick>;
pub fn create_document_quick_error(filename: &str) -> ResultQuick<File> {
    let last_min_docs = 5;
    let max_docs = 3;
    if last_min_docs > max_docs {
        return Err(DocumentServiceErrorQuick::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .context(filename)?;

    // let file = OpenOptions::new()
    //     .write(true)
    //     .create_new(true)
    //     .open(filename)?;

    Ok(file)
}

//*************************************************Using Crates [ErrorChain] */
pub mod errors {
    error_chain! {
        errors {
            RateLimitExceeded {
                display("Limit exceeded [ErrorChain]")
            }
        }
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::ResultExt;
pub fn create_document_error_chain(filename: &str) -> errors::Result<File> {
    let last_min_docs = 5;
    let max_docs = 3;
    if last_min_docs > max_docs {
        bail!(errors::ErrorKind::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .chain_err(|| format!("could not open {}", filename))?;

    Ok(file)
}
// TODO: support backtrace

//*************************************************Using Crates [FailureDerive] */
#[derive(Debug, Fail)]
pub enum DocumentServiceErrorFD {
    #[fail(display = "Limit exceeded [FailureDerive]")]
    RateLimitExceeded(Backtrace),
    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error, Backtrace),
}

impl From<io::Error> for DocumentServiceErrorFD {
    fn from(error: io::Error) -> Self {
        DocumentServiceErrorFD::Io(error, Backtrace::new())
    }
}

pub type ResultFD<T> = result::Result<T, DocumentServiceErrorFD>;

pub fn create_document_failure_derive(filename: &str) -> ResultFD<File> {
    let last_min_docs = 5;
    let max_docs = 3;
    if last_min_docs > max_docs {
        return Err(DocumentServiceErrorFD::RateLimitExceeded(Backtrace::new()));
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?; // `?` uses from to convert the error type

    Ok(file)
}

// backtrace use
// Err(e) => {
//     println!("Project creation failed: {}", e);
//     if let Some(backtrace) = e.backtrace() {
//         if !backtrace.to_string().trim().is_empty() {
//             println!("backtrace: {:?", backtrace);
//         }
//     }
// }
