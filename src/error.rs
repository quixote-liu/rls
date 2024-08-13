use core::fmt;
use std::process;

pub enum Error {
    SubOption {sub: String, msg: String},
    Common {msg: String},
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubOption { sub, msg } => {
                write!(f, "sub option is error:\noption={sub}\nerror_msg={msg}")
            },
            Self::Common { msg } => {
                write!(f, "{}", msg)
            },
        }
    }
}

pub fn thrown_subopt_err(sub: String, msg: String) {
    let e = Error::SubOption { sub, msg };
    println!("{}", e);
    process::exit(0);
}

pub fn thrown_common_err(msg: String) {
    let e = Error::Common { msg };
    println!("{}", e);
    process::exit(0);
}