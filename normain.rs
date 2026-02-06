
pub trait Syntax
{

}

#[allow( bare_trait_objects )]
pub struct Word( pub Syntax );

pub struct Sentence<'a>( pub Vec<&'a Word> );

pub struct Is<'a>( pub Vec<Sentence<'a>> );




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
pub struct Thing { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Pronoun { pub id:Identity }

#[derive( Clone, Debug )]
pub struct Use { pub id:Identity }

pub type Event<Type> = Option<Box<Type>>;
pub type Entity<Type> = Option<Box<Type>>;

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
pub enum Be
{
    Auxiliary( Entity<Verb> ),
    Past( Entity<Verb> ),
    Present( Entity<Verb> ),
    Infinite( Entity<Verb> ),
    Habitual( Entity<Verb> ),
    Copulate( Entity<Verb> ),
    Existant( Entity<Verb> ),
    Indicate( Entity<Verb> ),
    Identicate( Entity<Verb> ),
    Predicate( Entity<Verb> ),
    Subjicate( Entity<Verb> ),
    Preposition( Entity<Verb> ),
    Measure( Entity<Verb> ),
    Temporate( Entity<Verb> ),
    Locate( Entity<Verb> ),
    Specificate( Entity<Verb> ),
    Qualitate( Entity<Verb> ),
    Naturate( Entity<Verb> ),
    Occurance( Entity<Verb> ),
    Conjugate( Entity<Verb> ),
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
    Defense( Entity<Defense> ),
    Offense( Entity<Offense> ),
    Perjorate( Entity<Perjorate> ),
    What( Entity<What> ),
    Who( Entity<Who> ),
    Which( Entity<Which> ),
    Where( Entity<Where> ),
    Why( Entity<Why> ),
    How( Entity<How> ),
    When( Entity<When> ),
    Mock( Entity<Mock> ),
    Inequal( Entity<Equal> ),
    Equal( Entity<Equal> ),
    Predicate( Entity<Predicate> ),
    Compare( Entity<Compare> ),
    Indicate( Entity<Indicate> ),
    Instance( Entity<Instance> ),
    Referral( Entity<Referral> ),
    Refer( Entity<Refer> ),
    Referrer( Entity<Referrer> ),
    Reference( Entity<Reference> ),
    Transgender( Entity<Transgender> ),
    Gender( Entity<Gender> ),
    Person( Entity<Person> ),
    Animal( Entity<Animal> ),
    Animate( Entity<Inanimate> ),
    Inanimate( Entity<Inanimate> ),
    Project( Entity<Project> ),
    Subject( Entity<Subject> ),
    Object( Entity<Object> ),
    Place( Entity<Place> ),
    Never( Entity<Time> ),
    Sometime( Entity<Time> ),
    Time( Entity<Time> ),
    Abstract( Entity<Abstract> ),
    Noun( Entity<Noun> ),
    Unknown( Entity<Unknown> ),
    Is( Entity<Is> ),
    Be( Entity<Be> ),
    It( Entity<It> ),
    To( Entity<To> ),
    Use( Entity<Use> ),
    Nor( Entity<Nor> ),
    Or( Entity<Or> ),
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
    Interrogative( Entity<Interrogative> ),
    Conjunction(),
    Verb( Entity<Verb> ),
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
    Noun( Entity<Noun> ),
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
    Person( Entity<Person> ),
    Place( Entity<Place> ),
    Thing( Entity<Thing> ),
}

#[derive( Clone, Debug )]
pub enum Verb
{
    
}
// 00292 //////////////////////////////////////////////////////////////////////////////////////////////////////////////
