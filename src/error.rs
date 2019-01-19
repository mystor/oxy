use std::fmt;

macro_rules! error_type {
    (
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident {
            $(
                $variant:ident ( $err:ty )
            ),*
            $(,)*
        }
    ) => {
        $( #[$attr] )*
        $vis enum $name {
            $( $variant($err) ),*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(
                        $name::$variant(err) => err.fmt(f),
                    )*
                }
            }
        }

        $(
            impl From<$err> for $name {
                fn from(err: $err) -> Self {
                    $name::$variant(err)
                }
            }
        )*
    };
}

error_type! {
    #[derive(Debug)]
    pub enum OxyError {
        ParseError(syn::Error),
        InterpretError(InterpretError),
    }
}

#[derive(Debug)]
pub enum InterpretError {
    Unsupported(String),
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use InterpretError::*;
        match self {
            Unsupported(s) => write!(f, "Unsupported: {}", s),
        }
    }
}
