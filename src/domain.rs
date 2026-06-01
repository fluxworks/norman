/*! mod | v26.3.20 */
#![allow
(
    static_mut_refs,
    unused_attributes,
    unused_unsafe,
)]

#![feature
(

)]

/*
pub mod _
{
    pub use std::_::{*};
}

pub mod _
{
    /*!*/
    use ::
    {
        *,
    };
}
*/
pub static mut ARGUMENTS:Vec<String> = vec![];

pub mod env
{
    pub use std::env::{*};
}

pub mod error
{
    pub use std::error::{*};
}

unsafe fn domain() -> Result<(), Box<dyn crate::error::Error>>
{
    unsafe
    {
        let arguments = crate::env::args().collect::<Vec<String>>();

        for argument in arguments
        {
            ARGUMENTS.push(argument);
        }

        println!( r#"{:?}"#, ARGUMENTS );

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn crate::error::Error>>
{
    unsafe
    {
        domain()
    }
}
