#![allow
(
    unused_imports,
    unused_unsafe,
)]

use std::fmt::Debug;

macro_rules! term
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
