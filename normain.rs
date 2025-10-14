/*
What person or people; which person or people; asks for the identity of someone; 
used in a direct or indirect question (interrogative) */
#[derive( Clone, Debug )]
pub struct Who { pub id:Identity }
/*

Which, especially which of an open-ended set of possibilities (interrogative) */
#[derive( Clone, Debug )]
pub struct What { pub id:Identity }
/*
conjunction
In, at or to which place or situation */
#[derive( Clone, Debug )]
pub struct Where { pub id:Identity }
/*
What, of those mentioned or implied (interrogative) */
#[derive( Clone, Debug )]
pub struct Which { pub id:Identity }
/*
For what cause, reason, or purpose (interrogative) */
#[derive( Clone, Debug )]
pub struct Why { pub id:Identity }
/*
To what degree or extent (interrogative) */
#[derive( Clone, Debug )]
pub struct When { pub id:Identity }

#[derive( Clone, Debug )]
pub struct How { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Perjorate { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Defense { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Offense { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Animate { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Inanimate { pub id:Identity } 

#[derive( Clone, Debug )]
pub struct Nor { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Or { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Auxiliary { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Mock { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Indicate { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Predicate { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Equal { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Inequal { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Compare { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Instance { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Reference { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Referrer { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Referral { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Refer { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Transgender { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Gender { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Entity { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Pronoun { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Use { pub id:Identity }

pub type Thing<Type> = Option<Box<Type>>;

#[derive( Clone, Debug )]
pub struct Project { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Subject { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Object { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Place { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Time { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Abstract { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Unknown { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Animal { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Is { pub id:Identity }

#[derive( Clone, Debug )]
pub enum Be
{
    Auxiliary( Thing<Verb> ),
    Past( Thing<Verb> ),
    Present( Thing<Verb> ),
    Infinite( Thing<Verb> ),
    Habitual( Thing<Verb> ),
    Copulate( Thing<Verb> ),
    Existant( Thing<Verb> ),
    Indicate( Thing<Verb> ),
    Identicate( Thing<Verb> ),
    Predicate( Thing<Verb> ),
    Subjicate( Thing<Verb> ),
    Preposition( Thing<Verb> ),
    Measure( Thing<Verb> ),
    Temporate( Thing<Verb> ),
    Locate( Thing<Verb> ),
    Specificate( Thing<Verb> ),
    Qualitate( Thing<Verb> ),
    Naturate( Thing<Verb> ),
    Occurance( Thing<Verb> ),
    Conjugate( Thing<Verb> ),
}

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
    Adult( Option<Pronoun> ),
    Child( Option<Pronoun> ),
    Infant( Option<Pronoun> ),
}

pub type Baby = Person;

#[derive( Clone, Debug )]
pub enum Identity
{
    Defense( Thing<Defense> ),
    Offense( Thing<Offense> ),
    Perjorate( Thing<Perjorate> ),
    What( Thing<What> ),
    Who( Thing<Who> ),
    Which( Thing<Which> ),
    Where( Thing<Where> ),
    Why( Thing<Why> ),
    How( Thing<How> ),
    When( Thing<When> ),
    Mock( Thing<Mock> ),
    Inequal( Thing<Equal> ),
    Equal( Thing<Equal> ),
    Predicate( Thing<Predicate> ),
    Compare( Thing<Compare> ),
    Indicate( Thing<Indicate> ),
    Instance( Thing<Instance> ),
    Referral( Thing<Referral> ),
    Refer( Thing<Refer> ),
    Referrer( Thing<Referrer> ),
    Reference( Thing<Reference> ),
    Transgender( Thing<Transgender> ),
    Gender( Thing<Gender> ),
    Person( Thing<Person> ),
    Animal( Thing<Animal> ),
    Animate( Thing<Inanimate> ),
    Inanimate( Thing<Inanimate> ),
    Project( Thing<Project> ),
    Subject( Thing<Subject> ),
    Object( Thing<Object> ),
    Place( Thing<Place> ),
    Never( Thing<Time> ),
    Sometime( Thing<Time> ),
    Time( Thing<Time> ),
    Abstract( Thing<Abstract> ),
    Noun( Thing<Noun> ),
    Unknown( Thing<Unknown> ),
    Is( Thing<Is> ),
    Be( Thing<Be> ),
    It( Thing<It> ),
    To( Thing<To> ),
    Use( Thing<Use> ),
    Nor( Thing<Nor> ),
    Or( Thing<Or> ),
}

#[derive( Clone, Debug )]
pub enum Interrogative
{
    Who(),
    What(),
    Where(),
    When(),
    Why(),
    How(),
    Which(),
}

#[derive( Clone, Debug )]
pub enum PartsOfSpeech
{
    Interrogative( Thing<Interrogative> ),
    Conjunction(),
    Verb( Thing<Verb> ),
    Perfect(),
    Intransitive(),
    Copulative(),
    Habitual(),
    Infinite(),
    Participle(),
    Continuous(),
    Passive(),
    Voice(),
    Adverb(),
    Adjective(),
    Abbreviation(),
    Subject(),
    Object(),
    Project(),
    Noun( Thing<Noun> ),
    Instance(),
    Comparision(),
    Predicate(),
    Preposition(),
    Nominal(),
    Phrase(),
    Conjugate(),
}

#[derive( Clone, Debug )]
pub enum Noun
{
    Person( Thing<Person> ),
    Place( Thing<Place> ),
    Entity( Thing<Entity> ),
}

#[derive( Clone, Debug )]
pub enum Verb
{
    
}
// 00291 //////////////////////////////////////////////////////////////////////////////////////////////////////////////
