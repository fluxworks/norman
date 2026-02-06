/*
pub type Event<Type> = Option<Box<Type>>;
pub type Entity<Type> = Option<Box<Type>>;
*/

pub type Definition = Vec<Entity>;
pub type Definitions = Vec<Definition>;

#[derive( Clone, Debug )]
pub enum Entity<T=()>
{
    Defense,
    Offense,
    Perjorate,
    What,
    Who,
    Whose,
    Which,
    Where,
    Why,
    How,
    When,
    Mock,
    Inequal,
    Equal,
    Predicate,
    Compare,
    Indicate,
    Instance,
    Referral,
    Refer,
    Referrer,
    Reference,
    Transgender,
    Gender,
    Person,
    People,
    Animal,
    Animate,
    Inanimate,
    Project,
    Subject,
    Object,
    Place,
    Never,
    Sometime,
    Time,
    Abstract,
    Noun,
    Unknown,
    Is,
    Be,
    Of,
    It,
    To,
    Use,
    Nor,
    Or,
    Inside,
    Ask,
    For,
    The,
    Identity,
    Someone,
    Used,
    In,
    A,
    Direct,
    Indirect,
    Question,
    Introduces,
    Relative,
    Clause,
    Having,
    Human,
    Antecedent,
    __(T)
}



impl Entity
{
    pub fn who() -> Definitions
    {
        vec!
        [
            vec!
            [
                Entity::What,
                Entity::Person,
                Entity::Or,
                Entity::People,
            ],

            vec!
            [
                Entity::Which,
                Entity::Person,
                Entity::Or,
                Entity::People,
            ],

            vec!
            [
                Entity::Ask,
                Entity::For,
                Entity::The,
                Entity::Identity,
                Entity::Of,
                Entity::Someone,
            ],

            vec!
            [
                
                Entity::Used,
                Entity::In,
                Entity::A,
                Entity::Direct,
                Entity::Or,
                Entity::Indirect,
                Entity::Question,
            ],

            vec!
            [
                Entity::Whose,
            ],

            vec!
            [
                
                Entity::Introduces,
                Entity::A,
                Entity::Relative,
                Entity::Clause,
                Entity::Having,
                Entity::A,
                Entity::Human,
                Entity::Antecedent,
            ],
        ]
    }
}

#[derive( Clone, Debug )]
pub enum Identity
{
    Defense( Vec<Definition> ),
    Offense( Vec<Definition> ),
    Perjorate( Vec<Definition> ),
    What( Vec<Definition> ),
    Who( Vec<Definition> ),
    Whose( Vec<Definition> ),
    Which( Vec<Definition> ),
    Where( Vec<Definition> ),
    Why( Vec<Definition> ),
    How( Vec<Definition> ),
    When( Vec<Definition> ),
    Mock( Vec<Definition> ),
    Inequal( Vec<Definition> ),
    Equal( Vec<Definition> ),
    Predicate( Vec<Definition> ),
    Compare( Vec<Definition> ),
    Indicate( Vec<Definition> ),
    Instance( Vec<Definition> ),
    Referral( Vec<Definition> ),
    Refer( Vec<Definition> ),
    Referrer( Vec<Definition> ),
    Reference( Vec<Definition> ),
    Transgender( Vec<Definition> ),
    Gender( Vec<Definition> ),
    Person( Vec<Definition> ),
    People( Vec<Definition> ),
    Animal( Vec<Definition> ),
    Animate( Vec<Definition> ),
    Inanimate( Vec<Definition> ),
    Project( Vec<Definition> ),
    Subject( Vec<Definition> ),
    Object( Vec<Definition> ),
    Place( Vec<Definition> ),
    Never( Vec<Definition> ),
    Sometime( Vec<Definition> ),
    Time( Vec<Definition> ),
    Abstract( Vec<Definition> ),
    Noun( Vec<Definition> ),
    Unknown( Vec<Definition> ),
    Is( Vec<Definition> ),
    Be( Vec<Definition> ),
    It( Vec<Definition> ),
    To( Vec<Definition> ),
    Use( Vec<Definition> ),
    Nor( Vec<Definition> ),
    Or( Vec<Definition> ),
    Ask( Vec<Definition> ),
    For( Vec<Definition> ),
    The( Vec<Definition> ),
    Identity( Vec<Definition> ),
    Of( Vec<Definition> ),
    Someone( Vec<Definition> ),
    Used( Vec<Definition> ),
    In( Vec<Definition> ),
    A( Vec<Definition> ),
    Direct( Vec<Definition> ),
    Indirect( Vec<Definition> ),
    Question( Vec<Definition> ),
    Introduces( Vec<Definition> ),
    Relative( Vec<Definition> ),
    Clause( Vec<Definition> ),
    Having( Vec<Definition> ),
    Human( Vec<Definition> ),
    Anteceden( Vec<Definition> ),
    __(),
}

impl Identity
{
    pub fn who() -> Self
    {
        Self::Who
        (
            Entity::who()       
        )
    }
}
