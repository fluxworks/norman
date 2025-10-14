// Type your code here, or load an example.

// As of Rust 1.75, small functions are automatically
// marked as `#[inline]` so they will not show up in
// the output when compiling with optimisations. Use
// `#[unsafe(no_mangle)]` or `#[inline(never)]` to
// work around this issue.
// See https://github.com/compiler-explorer/compiler-explorer/issues/5939
#[unsafe(no_mangle)]
pub fn square(num: i32) -> i32 {
    num * num
}

// If you use `main()`, declare it as `pub` to see it in the output:
// pub fn main() { ... }

/*
*/
#[derive( Clone, Debug )]
pub struct Inanimate { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Reference { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Refer { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Entity { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Pronoun { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Use { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Object { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Abstract { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Animal { pub id:Identity }

#[derive( Clone, Debug )]
pub struct It { pub id:Identity }

impl It
{
    pub fn new() -> Self
    {
        It { id:Identity::Person( Some( Person::Third( None ).into() ) ) }
    }
}

#[derive( Clone, Debug )]
pub struct To { pub id:Identity }

#[derive( Clone, Debug )]
pub enum Person
{
    Pronoun( Option<Pronoun> ),
    Neutral( Option<Pronoun> ),
    Singular( Option<Pronoun> ),
    Third( Option<Pronoun> ),
}

#[derive( Clone, Debug )]
pub enum Identity
{
    Refer( Option<Box<Refer>> ),
    Reference( Option<Box<Reference>> ),
    Person( Option<Box<Person>> ),
    Animal( Option<Box<Animal>> ),
    Object( Option<Box<Object>> ),
    Inanimate( Option<Box<Inanimate>> ),
    Abstract( Option<Box<Abstract>> ),
    It( Option<Box<It>> ),
    To( Option<Box<To>> ),
    Use( Option<Box<Use>> ),
}
