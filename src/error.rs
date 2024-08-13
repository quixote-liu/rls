use core::fmt;

pub enum Error {
    SubOption {sub: String, msg: String},
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubOption { sub, msg } => {
                write!(f, "sub option is error:\noption={sub}\nerror_msg={msg}")
            },
        }
    }
}

pub fn new_sub_opt(sub: String, msg: String) -> Error {
    Error::SubOption { sub, msg }
}