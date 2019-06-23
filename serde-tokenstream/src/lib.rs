use serde::Serialize;
use proc_macro2::TokenStream;

//TODO: Move to error module as shown here: https://serde.rs/error-handling.html
#[derive(Debug)]
pub enum Error {}
pub type Result<T> = std::result::Result<T, Error>;

pub fn to_token_stream<T: Serialize>(value: &T) -> Result<TokenStream> {
    unimplemented!() //TODO
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use quote::quote;

    use super::to_token_stream;

    #[derive(Debug, Serialize, PartialEq)]
    struct Foo<'a> {
        bar: Bar,
        value: &'a str,
        spam: Spam<'a>,
    }

    #[derive(Debug, Serialize, PartialEq)]
    struct Bar {
        x: i32,
        y: usize,
        z: f64,
        t: (i32, i32),
    }

    #[derive(Debug, Serialize, PartialEq)]
    struct Spam<'a>(&'a str, u128);

    #[test]
    fn serialize_nested() -> super::Result<()> {
        let value = Foo {
            bar: Bar {
                x: -12,
                y: 84,
                z: 0.57694,
                t: (5, -104),
            },
            value: "test 123 MANISH",
            spam: Spam("hi", 1234596849),
        };

        let expected = quote! {
            Foo {
                bar: Bar {
                    x: -12,
                    y: 84,
                    z: 0.57694,
                    t: (5, -104),
                },
                value: "test 123 MANISH",
                spam: Spam("hi", 1234596849),
            }
        };

        let tokens = to_token_stream(&value)?;

        Ok(())
    }
}
