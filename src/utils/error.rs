use snafu::{ErrorCompat, ResultExt, Snafu};

use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not read {}: {}", filename.display(), source))]
    ReadErr {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not write {}: {}", filename.display(), source))]
    WriteErr {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Vector lengths do not match {} != {}", a, b))]
    LengthErr {
        a: usize,
        b: usize,
    },
    #[snafu(display("Please assert  {} has been set prior to {} call", a, b))]
    SetErr{
        a: String,
        b: String,
    },
    #[snafu(display("Please assert  {} has been set prior to {} call", a, b))]
    SusQueryErr{
        a: String,
        b: String,
    },
    #[snafu(display("Please assert  {} has been set prior to {} call", a, b))]
    SusCompErr{
        a: String,
        b: String,
    },


}
