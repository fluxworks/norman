#![allow
(
    unused_imports,
    unused_unsafe,
)]

use std::fmt::Debug;

#[macro_export] #[macro_use] macro_rules! term
{
    ($($i:ident), * ) =>
    {
        #[derive( Clone, Debug )]
        pub enum Entity
        {
            $( $i, )*
        }
        

        #[derive( Clone, Debug )]
        pub enum Ident
        {
            $( $i( Definitions ), )*
        }

        pub type Definition = Vec<Entity>;
        pub type Definitions = Vec<Definition>;
    };
}

#[macro_export] #[macro_use] macro_rules! definition
{
    ($($i:ident)* ) =>
    {
        vec!
        [
            $( Entity::$i, )*
        ]
    };
}

pub unsafe fn domain()
{
    unsafe
    {
        let when = Ident::when();
        println!( r#"when::synonyms( {:?} )"#, when );
    }
}

pub fn main()
{
    unsafe
    {
        domain();        
    }
}
