use oxy::{Context, populate, eval_fn};

macro_rules! eval_program {
    ({ $($program:tt)* } $func:ident ( $($args:expr),* $(,)* )) => {
        {
            let mut ctx = Context::default();
            populate(&mut ctx, quote!($(program)*))?;
            eval_fn(&mut ctx, stringify!($func), &[ $(quote!($args)),* ])?
        }
    };
}

#[test]
fn simple_program() -> Result<(), OxyError> {
    let result = eval_program! {
        {
            fn add_one(x: ()) {
                x + 1
            }
        }
        add_one(12)
    };
    assert_eq!(result, 13);

    Ok(())
}
