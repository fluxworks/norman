#![allow
(
    unused_imports,
    unused_unsafe,
)]
/*
pub type Event<Type> = Option<Box<Type>>;
pub type Entity<Type> = Option<Box<Type>>;
*/
pub type Definition = Vec<Entity>;
pub type Definitions = Vec<Definition>;

#[derive( Clone, Debug )]
pub enum Entity
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
    Noun,
    Verb,
    Adverb,
    Adjective,
    Pronoun,
    Person,
    Place,
    Thing,
    People,
    Animal,
    Animate,
    Inanimate,
    Project,
    Subject,
    Object,
    Interrogative,
    Relative,
    FusedRelative,
    Interjection,
    Indexable,
    Noteable,
    Determiner,    
    Conjunction,
    Exclamative,
    Ambiguous,
    Truncation,
    Rhetorical,    
    Never,
    Sometime,
    Time,
    Abstract,
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
    Clause,
    Having,
    Human,
    Antecedent,
    Test,
    Abandon,
    Ability,
    Able,
    Abortion,
    About,
    Above,
    Abroad,
    Absence,
    Absolute,
    Absolutely,
    Absorb,
    Abuse,
    Academic,
    Accept,
    Access,
    Accident,
    Accompany,
    Accomplish,
    According,
    Account,
    Accurate,
    Accuse,
    Achieve,
    Achievement,
    Acid,
    Acknowledge,
    Acquire,
    Across,
    Act,
    Action,
    Active,
    Activist,
    Activity,
    Actor,
    Actress,
    Actual,
    Actually,
    Ad,
    Adapt,
    Add,
    Addition,
    Additional,
    Address,
    Adequate,
    Adjust,
    Adjustment,
    Administration,
    Administrator,
    Admire,
    Admission,
    Admit,
    Adolescent,
    Adopt,
    Adult,
    Advance,
    Advanced,
    Advantage,
    Adventure,
    Advertising,
    Advice,
    Advise,
    Adviser,
    Advocate,
    Affair,
    Affect,
    Afford,
    Afraid,
    African,
    AfricanAmerican,
    After,
    Afternoon,
    Again,
    Against,
    Age,
    Agency,
    Agenda,
    Agent,
    Aggressive,
    Ago,
    Agree,
    Agreement,
    Agricultural,
    Ah,
    Ahead,
    Aid,
    Aide,
    Aids,
    Aim,
    Air,
    Aircraft,
    Airline,
    Airport,
    Album,
    Alcohol,
    Alive,
    All,
    Alliance,
    Allow,
    Ally,
    Almost,
    Alone,
    Along,
    Already,
    Also,
    Alter,
    Alternative,
    Although,
    Always,
    Am,
    Amazing,
    American,
    Among,
    Amount,
    Analysis,
    Analyst,
    Analyze,
    Ancient,
    And,
    Anger,
    Angle,
    Angry,
    Anniversary,
    Announce,
    Annual,
    Another,
    Answer,
    Anticipate,
    Anxiety,
    Any,
    Anybody,
    Anymore,
    Anyone,
    Anything,
    Anyway,
    Anywhere,
    Apart,
    Apartment,
    Apparent,
    Apparently,
    Appeal,
    Appear,
    Appearance,
    Apple,
    Application,
    Apply,
    Appoint,
    Appointment,
    Appreciate,
    Approach,
    Appropriate,
    Approval,
    Approve,
    Approximately,
    Arab,
    Architect,
    Area,
    Argue,
    Argument,
    Arise,
    Arm,
    Armed,
    Army,
    Around,
    Arrange,
    Arrangement,
    Arrest,
    Arrival,
    Arrive,
    Art,
    Article,
    Artist,
    Artistic,
    As,
    Asian,
    Aside,
    Asleep,
    Aspect,
    Assault,
    Assert,
    Assess,
    Assessment,
    Asset,
    Assign,
    Assignment,
    Assist,
    Assistance,
    Assistant,
    Associate,
    Association,
    Assume,
    Assumption,
    Assure,
    At,
    Athlete,
    Athletic,
    Atmosphere,
    Attach,
    Attack,
    Attempt,
    Attend,
    Attention,
    Attitude,
    Attorney,
    Attract,
    Attractive,
    Attribute,
    Audience,
    Author,
    Authority,
    Auto,
    Available,
    Average,
    Avoid,
    Award,
    Aware,
    Awareness,
    Away,
    Awful,
    Baby,
    Back,
    Background,
    Bad,
    Badly,
    Bag,
    Bake,
    Balance,
    Ball,
    Ban,
    Band,
    Bank,
    Bar,
    Barely,
    Barrel,
    Barrier,
    Base,
    Baseball,
    Basic,
    Basically,
    Basis,
    Basket,
    Basketball,
    Bathroom,
    Battery,
    Battle,
    Beach,
    Bean,
    Bear,
    Beat,
    Beautiful,
    Beauty,
    Because,
    Become,
    Bed,
    Bedroom,
    Beer,
    Before,
    Begin,
    Beginning,
    Behavior,
    Behind,
    Being,
    Belief,
    Believe,
    Bell,
    Belong,
    Below,
    Belt,
    Bench,
    Bend,
    Beneath,
    Benefit,
    Beside,
    Besides,
    Best,
    Bet,
    Better,
    Between,
    Beyond,
    Bible,
    Big,
    Bike,
    Bill,
    Billion,
    Bind,
    Biological,
    Bird,
    Birth,
    Birthday,
    Bit,
    Bite,
    Black,
    Blade,
    Blame,
    Blanket,
    Blind,
    Block,
    Blood,
    Blow,
    Blue,
    Board,
    Boat,
    Body,
    Bomb,
    Bombing,
    Bond,
    Bone,
    Book,
    Boom,
    Boot,
    Border,
    Born,
    Borrow,
    Boss,
    Both,
    Bother,
    Bottle,
    Bottom,
    Boundary,
    Bowl,
    Box,
    Boy,
    Boyfriend,
    Brain,
    Branch,
    Brand,
    Bread,
    Break,
    Breakfast,
    Breast,
    Breath,
    Breathe,
    Brick,
    Bridge,
    Brief,
    Briefly,
    Bright,
    Brilliant,
    Bring,
    British,
    Broad,
    Broken,
    Brother,
    Brown,
    Brush,
    Buck,
    Budget,
    Build,
    Building,
    Bullet,
    Bunch,
    Burden,
    Burn,
    Bury,
    Bus,
    Business,
    Busy,
    But,
    Butter,
    Button,
    Buy,
    Buyer,
    By,
    Cabin,
    Cabinet,
    Cable,
    Cake,
    Calculate,
    Call,
    Camera,
    Camp,
    Campaign,
    Campus,
    Can,
    Canadian,
    Cancer,
    Candidate,
    Cap,
    Capability,
    Capable,
    Capacity,
    Capital,
    Captain,
    Capture,
    Car,
    Carbon,
    Card,
    Care,
    Career,
    Careful,
    Carefully,
    Carrier,
    Carry,
    Case,
    Cash,
    Cast,
    Cat,
    Catch,
    Category,
    Catholic,
    Cause,
    Ceiling,
    Celebrate,
    Celebration,
    Celebrity,
    Cell,
    Center,
    Central,
    Century,
    Ceo,
    Ceremony,
    Certain,
    Certainly,
    Chain,
    Chair,
    Chairman,
    Challenge,
    Chamber,
    Champion,
    Championship,
    Chance,
    Change,
    Changing,
    Channel,
    Chapter,
    Character,
    Characteristic,
    Characterize,
    Charge,
    Charity,
    Chart,
    Chase,
    Cheap,
    Check,
    Cheek,
    Cheese,
    Chef,
    Chemical,
    Chest,
    Chicken,
    Chief,
    Child,
    Childhood,
    Chinese,
    Chip,
    Chocolate,
    Choice,
    Cholesterol,
    Choose,
    Christian,
    Christmas,
    Church,
    Cigarette,
    Circle,
    Circumstance,
    Cite,
    Citizen,
    City,
    Civil,
    Civilian,
    Claim,
    Class,
    Classic,
    Classroom,
    Clean,
    Clear,
    Clearly,
    Client,
    Climate,
    Climb,
    Clinic,
    Clinical,
    Clock,
    Close,
    Closely,
    Closer,
    Clothes,
    Clothing,
    Cloud,
    Club,
    Clue,
    Cluster,
    Coach,
    Coal,
    Coalition,
    Coast,
    Coat,
    Code,
    Coffee,
    Cognitive,
    Cold,
    Collapse,
    Colleague,
    Collect,
    Collection,
    Collective,
    College,
    Colonial,
    Color,
    Column,
    Combination,
    Combine,
    Come,
    Comedy,
    Comfort,
    Comfortable,
    Command,
    Commander,
    Comment,
    Commercial,
    Commission,
    Commit,
    Commitment,
    Committee,
    Common,
    Communicate,
    Communication,
    Community,
    Company,
    Comparison,
    Compete,
    Competition,
    Competitive,
    Competitor,
    Complain,
    Complaint,
    Complete,
    Completely,
    Complex,
    Complicated,
    Component,
    Compose,
    Composition,
    Comprehensive,
    Computer,
    Concentrate,
    Concentration,
    Concept,
    Concern,
    Concerned,
    Concert,
    Conclude,
    Conclusion,
    Concrete,
    Condition,
    Conduct,
    Conference,
    Confidence,
    Confident,
    Confirm,
    Conflict,
    Confront,
    Confusion,
    Congress,
    Congressional,
    Connect,
    Connection,
    Consciousness,
    Consensus,
    Consequence,
    Conservative,
    Consider,
    Considerable,
    Consideration,
    Consist,
    Consistent,
    Constant,
    Constantly,
    Constitute,
    Constitutional,
    Construct,
    Construction,
    Consultant,
    Consume,
    Consumer,
    Consumption,
    Contact,
    Contain,
    Container,
    Contemporary,
    Content,
    Contest,
    Context,
    Continue,
    Continued,
    Contract,
    Contrast,
    Contribute,
    Contribution,
    Control,
    Controversial,
    Controversy,
    Convention,
    Conventional,
    Conversation,
    Convert,
    Conviction,
    Convince,
    Cook,
    Cookie,
    Cooking,
    Cool,
    Cooperation,
    Cop,
    Cope,
    Copy,
    Core,
    Corn,
    Corner,
    Corporate,
    Corporation,
    Correct,
    Correspondent,
    Cost,
    Cotton,
    Couch,
    Could,
    Council,
    Counselor,
    Count,
    Counter,
    Country,
    County,
    Couple,
    Courage,
    Course,
    Court,
    Cousin,
    Cover,
    Coverage,
    Cow,
    Crack,
    Craft,
    Crash,
    Crazy,
    Cream,
    Create,
    Creation,
    Creative,
    Creature,
    Credit,
    Crew,
    Crime,
    Criminal,
    Crisis,
    Criteria,
    Critic,
    Critical,
    Criticism,
    Criticize,
    Crop,
    Cross,
    Crowd,
    Crucial,
    Cry,
    Cultural,
    Culture,
    Cup,
    Curious,
    Current,
    Currently,
    Curriculum,
    Custom,
    Customer,
    Cut,
    Cycle,
    Dad,
    Daily,
    Damage,
    Dance,
    Danger,
    Dangerous,
    Dare,
    Dark,
    Darkness,
    Data,
    Date,
    Daughter,
    Day,
    Dead,
    Deal,
    Dealer,
    Dear,
    Death,
    Debate,
    Debt,
    Decade,
    Decide,
    Decision,
    Deck,
    Declare,
    Decline,
    Decrease,
    Deep,
    Deeply,
    Deer,
    Defeat,
    Defend,
    Defendant,
    Defensive,
    Deficit,
    Define,
    Definitely,
    Definition,
    Degree,
    Delay,
    Deliver,
    Delivery,
    Demand,
    Democracy,
    Democrat,
    Democratic,
    Demonstrate,
    Demonstration,
    Deny,
    Department,
    Depend,
    Dependent,
    Depending,
    Depict,
    Depression,
    Depth,
    Deputy,
    Derive,
    Describe,
    Description,
    Desert,
    Deserve,
    Design,
    Designer,
    Desire,
    Desk,
    Desperate,
    Despite,
    Destroy,
    Destruction,
    Detail,
    Detailed,
    Detect,
    Determine,
    Develop,
    Developing,
    Development,
    Device,
    Devote,
    Dialogue,
    Die,
    Diet,
    Differ,
    Difference,
    Different,
    Differently,
    Difficult,
    Difficulty,
    Dig,
    Digital,
    Dimension,
    Dining,
    Dinner,
    Direction,
    Directly,
    Director,
    Dirt,
    Dirty,
    Disability,
    Disagree,
    Disappear,
    Disaster,
    Discipline,
    Discourse,
    Discover,
    Discovery,
    Discrimination,
    Discuss,
    Discussion,
    Disease,
    Dish,
    Dismiss,
    Disorder,
    Display,
    Dispute,
    Distance,
    Distant,
    Distinct,
    Distinction,
    Distinguish,
    Distribute,
    Distribution,
    District,
    Diverse,
    Diversity,
    Divide,
    Division,
    Divorce,
    Dna,
    Do,
    Doctor,
    Document,
    Dog,
    Domestic,
    Dominant,
    Dominate,
    Door,
    Double,
    Doubt,
    Down,
    Downtown,
    Dozen,
    Draft,
    Drag,
    Drama,
    Dramatic,
    Dramatically,
    Draw,
    Drawing,
    Dream,
    Dress,
    Drink,
    Drive,
    Driver,
    Drop,
    Drug,
    Dry,
    Due,
    During,
    Dust,
    Duty,
    Each,
    Eager,
    Ear,
    Early,
    Earn,
    Earnings,
    Earth,
    Ease,
    Easily,
    East,
    Eastern,
    Easy,
    Eat,
    Economic,
    Economics,
    Economist,
    Economy,
    Edge,
    Edition,
    Editor,
    Educate,
    Education,
    Educational,
    Educator,
    Effect,
    Effective,
    Effectively,
    Efficiency,
    Efficient,
    Effort,
    Egg,
    Eight,
    Either,
    Elderly,
    Elect,
    Election,
    Electric,
    Electricity,
    Electronic,
    Element,
    Elementary,
    Eliminate,
    Elite,
    Else,
    Elsewhere,
    Email,
    Embrace,
    Emerge,
    Emergency,
    Emission,
    Emotion,
    Emotional,
    Emphasis,
    Emphasize,
    Employ,
    Employee,
    Employer,
    Employment,
    Empty,
    Enable,
    Encounter,
    Encourage,
    End,
    Enemy,
    Energy,
    Enforcement,
    Engage,
    Engine,
    Engineer,
    Engineering,
    English,
    Enhance,
    Enjoy,
    Enormous,
    Enough,
    Ensure,
    Enter,
    Enterprise,
    Entertainment,
    Entire,
    Entirely,
    Entrance,
    Entry,
    Environment,
    Environmental,
    Episode,
    Equally,
    Equipment,
    Era,
    Error,
    Escape,
    Especially,
    Essay,
    Essential,
    Essentially,
    Establish,
    Establishment,
    Estate,
    Estimate,
    Etc,
    Ethics,
    Ethnic,
    European,
    Evaluate,
    Evaluation,
    Even,
    Evening,
    Event,
    Eventually,
    Ever,
    Every,
    Everybody,
    Everyday,
    Everyone,
    Everything,
    Everywhere,
    Evidence,
    Evolution,
    Evolve,
    Exact,
    Exactly,
    Examination,
    Examine,
    Example,
    Exceed,
    Excellent,
    Except,
    Exception,
    Exchange,
    Exciting,
    Executive,
    Exercise,
    Exhibit,
    Exhibition,
    Exist,
    Existence,
    Existing,
    Expand,
    Expansion,
    Expect,
    Expectation,
    Expense,
    Expensive,
    Experience,
    Experiment,
    Expert,
    Explain,
    Explanation,
    Explode,
    Explore,
    Explosion,
    Expose,
    Exposure,
    Express,
    Expression,
    Extend,
    Extension,
    Extensive,
    Extent,
    External,
    Extra,
    Extraordinary,
    Extreme,
    Extremely,
    Eye,
    Fabric,
    Face,
    Facility,
    Fact,
    Factor,
    Factory,
    Faculty,
    Fade,
    Fail,
    Failure,
    Fair,
    Fairly,
    Faith,
    Fall,
    False,
    Familiar,
    Family,
    Famous,
    Fan,
    Fantasy,
    Far,
    Farm,
    Farmer,
    Fashion,
    Fast,
    Fat,
    Fate,
    Father,
    Fault,
    Favor,
    Favorite,
    Fear,
    Feature,
    Federal,
    Fee,
    Feed,
    Feel,
    Feeling,
    Fellow,
    Female,
    Fence,
    Few,
    Fewer,
    Fiber,
    Fiction,
    Field,
    Fifteen,
    Fifth,
    Fifty,
    Fight,
    Fighter,
    Fighting,
    Figure,
    File,
    Fill,
    Film,
    Final,
    Finally,
    Finance,
    Financial,
    Find,
    Finding,
    Fine,
    Finger,
    Finish,
    Fire,
    Firm,
    First,
    Fish,
    Fishing,
    Fit,
    Fitness,
    Five,
    Fix,
    Flag,
    Flame,
    Flat,
    Flavor,
    Flee,
    Flesh,
    Flight,
    Float,
    Floor,
    Flow,
    Flower,
    Fly,
    Focus,
    Folk,
    Follow,
    Following,
    Food,
    Foot,
    Football,
    Force,
    Foreign,
    Forest,
    Forever,
    Forget,
    Form,
    Formal,
    Formation,
    Former,
    Formula,
    Forth,
    Fortune,
    Forward,
    Found,
    Foundation,
    Founder,
    Four,
    Fourth,
    Frame,
    Framework,
    Free,
    Freedom,
    Freeze,
    French,
    Frequency,
    Frequent,
    Frequently,
    Fresh,
    Friend,
    Friendly,
    Friendship,
    From,
    Front,
    Fruit,
    Frustration,
    Fuel,
    Full,
    Fully,
    Fun,
    Function,
    Fund,
    Fundamental,
    Funding,
    Funeral,
    Funny,
    Furniture,
    Furthermore,
    Future,
    Gain,
    Galaxy,
    Gallery,
    Game,
    Gang,
    Gap,
    Garage,
    Garden,
    Garlic,
    Gas,
    Gate,
    Gather,
    Gay,
    Gaze,
    Gear,
    Gene,
    General,
    Generally,
    Generate,
    Generation,
    Genetic,
    Gentleman,
    Gently,
    German,
    Gesture,
    Get,
    Ghost,
    Giant,
    Gift,
    Gifted,
    Girl,
    Girlfriend,
    Give,
    Given,
    Glad,
    Glance,
    Glass,
    Global,
    Glove,
    Go,
    Goal,
    God,
    Gold,
    Golden,
    Golf,
    Good,
    Government,
    Governor,
    Grab,
    Grade,
    Gradually,
    Graduate,
    Grain,
    Grand,
    Grandfather,
    Grandmother,
    Grant,
    Grass,
    Grave,
    Gray,
    Great,
    Greatest,
    Green,
    Grocery,
    Ground,
    Group,
    Grow,
    Growing,
    Growth,
    Guarantee,
    Guard,
    Guess,
    Guest,
    Guide,
    Guideline,
    Guilty,
    Gun,
    Guy,
    Habit,
    Habitat,
    Hair,
    Half,
    Hall,
    Hand,
    Handful,
    Handle,
    Hang,
    Happen,
    Happy,
    Hard,
    Hardly,
    Hat,
    Hate,
    Have,
    He,
    Head,
    Headline,
    Headquarters,
    Health,
    Healthy,
    Hear,
    Hearing,
    Heart,
    Heat,
    Heaven,
    Heavily,
    Heavy,
    Heel,
    Height,
    Helicopter,
    Hell,
    Hello,
    Help,
    Helpful,
    Her,
    Here,
    Heritage,
    Hero,
    Herself,
    Hey,
    Hi,
    Hide,
    High,
    Highlight,
    Highly,
    Highway,
    Hill,
    Him,
    Himself,
    Hip,
    Hire,
    His,
    Historian,
    Historic,
    Historical,
    History,
    Hit,
    Hold,
    Hole,
    Holiday,
    Holy,
    Home,
    Homeless,
    Honest,
    Honey,
    Honor,
    Hope,
    Horizon,
    Horror,
    Horse,
    Hospital,
    Host,
    Hot,
    Hotel,
    Hour,
    House,
    Household,
    Housing,
    However,
    Huge,
    Humor,
    Hundred,
    Hungry,
    Hunter,
    Hunting,
    Hurt,
    Husband,
    Hypothesis,
    I,
    Ice,
    Idea,
    Ideal,
    Identification,
    Identify,
    Ie,
    If,
    Ignore,
    Ill,
    Illegal,
    Illness,
    Illustrate,
    Image,
    Imagination,
    Imagine,
    Immediate,
    Immediately,
    Immigrant,
    Immigration,
    Impact,
    Implement,
    Implication,
    Imply,
    Importance,
    Important,
    Impose,
    Impossible,
    Impress,
    Impression,
    Impressive,
    Improve,
    Improvement,
    Incentive,
    Incident,
    Include,
    Including,
    Income,
    Incorporate,
    Increase,
    Increased,
    Increasing,
    Increasingly,
    Incredible,
    Indeed,
    Independence,
    Independent,
    Index,
    Indian,
    Indication,
    Individual,
    Industrial,
    Industry,
    Infant,
    Infection,
    Inflation,
    Influence,
    Inform,
    Information,
    Ingredient,
    Initial,
    Initially,
    Initiative,
    Injury,
    Inner,
    Innocent,
    Inquiry,
    Insight,
    Insist,
    Inspire,
    Install,
    Instead,
    Institution,
    Institutional,
    Instruction,
    Instructor,
    Instrument,
    Insurance,
    Intellectual,
    Intelligence,
    Intend,
    Intense,
    Intensity,
    Intention,
    Interaction,
    Interest,
    Interested,
    Interesting,
    Internal,
    International,
    Internet,
    Interpret,
    Interpretation,
    Intervention,
    Interview,
    Into,
    Introduce,
    Introduction,
    Invasion,
    Invest,
    Investigate,
    Investigation,
    Investigator,
    Investment,
    Investor,
    Invite,
    Involve,
    Involved,
    Involvement,
    Iraqi,
    Irish,
    Iron,
    Islamic,
    Island,
    Israeli,
    Issue,
    Italian,
    Item,
    Its,
    Itself,
    Jacket,
    Jail,
    Japanese,
    Jet,
    Jew,
    Jewish,
    Job,
    Join,
    Joint,
    Joke,
    Journal,
    Journalist,
    Journey,
    Joy,
    Judge,
    Judgment,
    Juice,
    Jump,
    Junior,
    Jury,
    Just,
    Justice,
    Justify,
    Keep,
    Key,
    Kick,
    Kid,
    Kill,
    Killer,
    Killing,
    Kind,
    King,
    Kiss,
    Kitchen,
    Knee,
    Knife,
    Knock,
    Know,
    Knowledge,
    Lab,
    Label,
    Labor,
    Laboratory,
    Lack,
    Lady,
    Lake,
    Land,
    Landscape,
    Language,
    Lap,
    Large,
    Largely,
    Last,
    Late,
    Later,
    Latin,
    Latter,
    Laugh,
    Launch,
    Law,
    Lawn,
    Lawsuit,
    Lawyer,
    Lay,
    Layer,
    Lead,
    Leader,
    Leadership,
    Leading,
    Leaf,
    League,
    Lean,
    Learn,
    Learning,
    Least,
    Leather,
    Leave,
    Left,
    Leg,
    Legacy,
    Legal,
    Legend,
    Legislation,
    Legitimate,
    Lemon,
    Length,
    Less,
    Lesson,
    Let,
    Letter,
    Level,
    Liberal,
    Library,
    License,
    Lie,
    Life,
    Lifestyle,
    Lifetime,
    Lift,
    Light,
    Like,
    Likely,
    Limit,
    Limitation,
    Limited,
    Line,
    Link,
    Lip,
    List,
    Listen,
    Literally,
    Literary,
    Literature,
    Little,
    Live,
    Living,
    Load,
    Loan,
    Local,
    Locate,
    Location,
    Lock,
    Long,
    LongTerm,
    Look,
    Loose,
    Lose,
    Loss,
    Lost,
    Lot,
    Lots,
    Loud,
    Love,
    Lovely,
    Lover,
    Low,
    Lower,
    Luck,
    Lucky,
    Lunch,
    Lung,
    Machine,
    Mad,
    Magazine,
    Mail,
    Main,
    Mainly,
    Maintain,
    Maintenance,
    Major,
    Majority,
    Make,
    Maker,
    Makeup,
    Male,
    Mall,
    Man,
    Manage,
    Management,
    Manager,
    Manner,
    Manufacturer,
    Manufacturing,
    Many,
    Map,
    Margin,
    Mark,
    Market,
    Marketing,
    Marriage,
    Married,
    Marry,
    Mask,
    Mass,
    Massive,
    Master,
    Match,
    Material,
    Math,
    Matter,
    May,
    Maybe,
    Mayor,
    Me,
    Meal,
    Mean,
    Meaning,
    Meanwhile,
    Measure,
    Measurement,
    Meat,
    Mechanism,
    Media,
    Medical,
    Medication,
    Medicine,
    Medium,
    Meet,
    Meeting,
    Member,
    Membership,
    Memory,
    Mental,
    Mention,
    Menu,
    Mere,
    Merely,
    Mess,
    Message,
    Metal,
    Meter,
    Method,
    Mexican,
    Middle,
    Might,
    Military,
    Milk,
    Million,
    Mind,
    Mine,
    Minister,
    Minor,
    Minority,
    Minute,
    Miracle,
    Mirror,
    Miss,
    Missile,
    Mission,
    Mistake,
    Mix,
    Mixture,
    MmHmm,
    Mode,
    Model,
    Moderate,
    Modern,
    Modest,
    Mom,
    Moment,
    Money,
    Monitor,
    Month,
    Mood,
    Moon,
    Moral,
    More,
    Moreover,
    Morning,
    Mortgage,
    Most,
    Mostly,
    Mother,
    Motion,
    Motivation,
    Motor,
    Mount,
    Mountain,
    Mouse,
    Mouth,
    Move,
    Movement,
    Movie,
    Mr,
    Mrs,
    Ms,
    Much,
    Multiple,
    Murder,
    Muscle,
    Museum,
    Music,
    Musical,
    Musician,
    Muslim,
    Must,
    Mutual,
    My,
    Myself,
    Mystery,
    Myth,
    Naked,
    Name,
    Narrative,
    Narrow,
    Nation,
    National,
    Native,
    Natural,
    Naturally,
    Nature,
    Near,
    Nearby,
    Nearly,
    Necessarily,
    Necessary,
    Neck,
    Need,
    Negative,
    Negotiate,
    Negotiation,
    Neighbor,
    Neighborhood,
    Neither,
    Nerve,
    Nervous,
    Net,
    Network,
    Nevertheless,
    New,
    Newly,
    News,
    Newspaper,
    Next,
    Nice,
    Night,
    Nine,
    No,
    Nobody,
    Nod,
    Noise,
    Nomination,
    None,
    Nonetheless,
    Normal,
    Normally,
    North,
    Northern,
    Nose,
    Not,
    Note,
    Nothing,
    Notice,
    Notion,
    Novel,
    Now,
    Nowhere,
    Nuclear,
    Number,
    Numerous,
    Nurse,
    Nut,
    Objective,
    Obligation,
    Observation,
    Observe,
    Observer,
    Obtain,
    Obvious,
    Obviously,
    Occasion,
    Occasionally,
    Occupation,
    Occupy,
    Occur,
    Ocean,
    Odd,
    Odds,
    Off,
    Offensive,
    Offer,
    Office,
    Officer,
    Official,
    Often,
    Oh,
    Oil,
    Ok,
    Okay,
    Old,
    Olympic,
    On,
    Once,
    One,
    Ongoing,
    Onion,
    Online,
    Only,
    Onto,
    Open,
    Opening,
    Operate,
    Operating,
    Operation,
    Operator,
    Opinion,
    Opponent,
    Opportunity,
    Oppose,
    Opposite,
    Opposition,
    Option,
    Orange,
    Order,
    Ordinary,
    Organic,
    Organization,
    Organize,
    Orientation,
    Origin,
    Original,
    Originally,
    Other,
    Others,
    Otherwise,
    Ought,
    Our,
    Ourselves,
    Out,
    Outcome,
    Outside,
    Oven,
    Over,
    Overall,
    Overcome,
    Overlook,
    Owe,
    Own,
    Owner,
    Pace,
    Pack,
    Package,
    Page,
    Pain,
    Painful,
    Paint,
    Painter,
    Painting,
    Pair,
    Pale,
    Palestinian,
    Palm,
    Pan,
    Panel,
    Pant,
    Paper,
    Parent,
    Park,
    Parking,
    Part,
    Participant,
    Participate,
    Participation,
    Particular,
    Particularly,
    Partly,
    Partner,
    Partnership,
    Party,
    Pass,
    Passage,
    Passenger,
    Passion,
    Past,
    Patch,
    Path,
    Patient,
    Pattern,
    Pause,
    Pay,
    Payment,
    Pc,
    Peace,
    Peak,
    Peer,
    Penalty,
    Pepper,
    Per,
    Perceive,
    Percentage,
    Perception,
    Perfect,
    Perfectly,
    Perform,
    Performance,
    Perhaps,
    Period,
    Permanent,
    Permission,
    Permit,
    Personal,
    Personality,
    Personally,
    Personnel,
    Perspective,
    Persuade,
    Pet,
    Phase,
    Phenomenon,
    Philosophy,
    Phone,
    Photo,
    Photograph,
    Photographer,
    Phrase,
    Physical,
    Physically,
    Physician,
    Piano,
    Pick,
    Picture,
    Pie,
    Piece,
    Pile,
    Pilot,
    Pine,
    Pink,
    Pipe,
    Pitch,
    Plan,
    Plane,
    Planet,
    Planning,
    Plant,
    Plastic,
    Plate,
    Platform,
    Play,
    Player,
    Please,
    Pleasure,
    Plenty,
    Plot,
    Plus,
    Pm,
    Pocket,
    Poem,
    Poet,
    Poetry,
    Point,
    Pole,
    Police,
    Policy,
    Political,
    Politically,
    Politician,
    Politics,
    Poll,
    Pollution,
    Pool,
    Poor,
    Pop,
    Popular,
    Population,
    Porch,
    Port,
    Portion,
    Portrait,
    Portray,
    Pose,
    Position,
    Positive,
    Possess,
    Possibility,
    Possible,
    Possibly,
    Post,
    Pot,
    Potato,
    Potential,
    Potentially,
    Pound,
    Pour,
    Poverty,
    Powder,
    Power,
    Powerful,
    Practical,
    Practice,
    Pray,
    Prayer,
    Precisely,
    Predict,
    Prefer,
    Preference,
    Pregnancy,
    Pregnant,
    Preparation,
    Prepare,
    Prescription,
    Presence,
    Present,
    Presentation,
    Preserve,
    President,
    Presidential,
    Press,
    Pressure,
    Pretend,
    Pretty,
    Prevent,
    Previous,
    Previously,
    Price,
    Pride,
    Priest,
    Primarily,
    Primary,
    Prime,
    Principal,
    Principle,
    Print,
    Prior,
    Priority,
    Prison,
    Prisoner,
    Privacy,
    Private,
    Probably,
    Problem,
    Procedure,
    Proceed,
    Process,
    Produce,
    Producer,
    Product,
    Production,
    Profession,
    Professional,
    Professor,
    Profile,
    Profit,
    Program,
    Progress,
    Prominent,
    Promise,
    Promote,
    Prompt,
    Proof,
    Proper,
    Properly,
    Property,
    Proportion,
    Proposal,
    Propose,
    Proposed,
    Prosecutor,
    Prospect,
    Protect,
    Protection,
    Protein,
    Protest,
    Proud,
    Prove,
    Provide,
    Provider,
    Province,
    Provision,
    Psychological,
    Psychologist,
    Psychology,
    Public,
    Publication,
    Publicly,
    Publish,
    Publisher,
    Pull,
    Punishment,
    Purchase,
    Pure,
    Purpose,
    Pursue,
    Push,
    Put,
    Qualify,
    Quality,
    Quarter,
    Quarterback,
    Quick,
    Quickly,
    Quiet,
    Quietly,
    Quit,
    Quite,
    Quote,
    Race,
    Racial,
    Radical,
    Radio,
    Rail,
    Rain,
    Raise,
    Range,
    Rank,
    Rapid,
    Rapidly,
    Rare,
    Rarely,
    Rate,
    Rather,
    Rating,
    Ratio,
    Raw,
    Reach,
    React,
    Reaction,
    Read,
    Reader,
    Reading,
    Ready,
    Real,
    Reality,
    Realize,
    Really,
    Reason,
    Reasonable,
    Recall,
    Receive,
    Recent,
    Recently,
    Recipe,
    Recognition,
    Recognize,
    Recommend,
    Recommendation,
    Record,
    Recording,
    Recover,
    Recovery,
    Recruit,
    Red,
    Reduce,
    Reduction,
    Reflect,
    Reflection,
    Reform,
    Refugee,
    Refuse,
    Regard,
    Regarding,
    Regardless,
    Regime,
    Region,
    Regional,
    Register,
    Regular,
    Regularly,
    Regulate,
    Regulation,
    Reinforce,
    Reject,
    Relate,
    Relation,
    Relationship,
    Relatively,
    Relax,
    Release,
    Relevant,
    Relief,
    Religion,
    Religious,
    Rely,
    Remain,
    Remaining,
    Remarkable,
    Remember,
    Remind,
    Remote,
    Remove,
    Repeat,
    Repeatedly,
    Replace,
    Reply,
    Report,
    Reporter,
    Represent,
    Representation,
    Representative,
    Republican,
    Reputation,
    Request,
    Require,
    Requirement,
    Research,
    Researcher,
    Resemble,
    Reservation,
    Resident,
    Resist,
    Resistance,
    Resolution,
    Resolve,
    Resort,
    Resource,
    Respect,
    Respond,
    Respondent,
    Response,
    Responsibility,
    Responsible,
    Rest,
    Restaurant,
    Restore,
    Restriction,
    Result,
    Retain,
    Retire,
    Retirement,
    Return,
    Reveal,
    Revenue,
    Review,
    Revolution,
    Rhythm,
    Rice,
    Rich,
    Rid,
    Ride,
    Rifle,
    Right,
    Ring,
    Rise,
    Risk,
    River,
    Road,
    Rock,
    Role,
    Roll,
    Romantic,
    Roof,
    Room,
    Root,
    Rope,
    Rose,
    Rough,
    Roughly,
    Round,
    Route,
    Routine,
    Row,
    Rub,
    Rule,
    Run,
    Running,
    Rural,
    Rush,
    Russian,
    Sacred,
    Sad,
    Safe,
    Safety,
    Sake,
    Salad,
    Salary,
    Sale,
    Sales,
    Salt,
    Same,
    Sample,
    Sanction,
    Sand,
    Satellite,
    Satisfaction,
    Satisfy,
    Sauce,
    Save,
    Saving,
    Say,
    Scale,
    Scandal,
    Scared,
    Scenario,
    Scene,
    Schedule,
    Scheme,
    Scholar,
    Scholarship,
    School,
    Science,
    Scientific,
    Scientist,
    Scope,
    Score,
    Scream,
    Screen,
    Script,
    Sea,
    Search,
    Season,
    Seat,
    Second,
    Secret,
    Secretary,
    Section,
    Sector,
    Secure,
    Security,
    See,
    Seed,
    Seek,
    Seem,
    Segment,
    Seize,
    Select,
    Selection,
    Sell,
    Senate,
    Senator,
    Send,
    Senior,
    Sense,
    Sensitive,
    Sentence,
    Separate,
    Sequence,
    Series,
    Serious,
    Seriously,
    Serve,
    Service,
    Session,
    Set,
    Setting,
    Settle,
    Settlement,
    Seven,
    Several,
    Severe,
    Sex,
    Sexual,
    Shade,
    Shadow,
    Shake,
    Shall,
    Shape,
    Share,
    Sharp,
    She,
    Sheet,
    Shelf,
    Shell,
    Shelter,
    Shift,
    Shine,
    Ship,
    Shirt,
    Shit,
    Shock,
    Shoe,
    Shoot,
    Shooting,
    Shop,
    Shopping,
    Shore,
    Short,
    Shortly,
    Shot,
    Should,
    Shoulder,
    Shout,
    Show,
    Shower,
    Shrug,
    Shut,
    Sick,
    Side,
    Sigh,
    Sight,
    Sign,
    Signal,
    Significance,
    Significant,
    Significantly,
    Silence,
    Silent,
    Silver,
    Similar,
    Similarly,
    Simple,
    Simply,
    Sin,
    Since,
    Sing,
    Singer,
    Single,
    Sink,
    Sir,
    Sister,
    Sit,
    Site,
    Situation,
    Six,
    Size,
    Ski,
    Skill,
    Skin,
    Sky,
    Slave,
    Sleep,
    Slice,
    Slide,
    Slight,
    Slightly,
    Slip,
    Slow,
    Slowly,
    Small,
    Smart,
    Smell,
    Smile,
    Smoke,
    Smooth,
    Snap,
    Snow,
    So,
    SoCalled,
    Soccer,
    Social,
    Society,
    Soft,
    Software,
    Soil,
    Solar,
    Soldier,
    Solid,
    Solution,
    Solve,
    Some,
    Somebody,
    Somehow,
    Something,
    Sometimes,
    Somewhat,
    Somewhere,
    Son,
    Song,
    Soon,
    Sophisticated,
    Sorry,
    Sort,
    Soul,
    Sound,
    Soup,
    Source,
    South,
    Southern,
    Soviet,
    Space,
    Spanish,
    Speak,
    Speaker,
    Special,
    Specialist,
    Species,
    Specific,
    Specifically,
    Speech,
    Speed,
    Spend,
    Spending,
    Spin,
    Spirit,
    Spiritual,
    Split,
    Spokesman,
    Sport,
    Spot,
    Spread,
    Spring,
    Square,
    Squeeze,
    Stability,
    Stable,
    Staff,
    Stage,
    Stair,
    Stake,
    Stand,
    Standard,
    Standing,
    Star,
    Stare,
    Start,
    State,
    Statement,
    Station,
    Statistics,
    Status,
    Stay,
    Steady,
    Steal,
    Steel,
    Step,
    Stick,
    Still,
    Stir,
    Stock,
    Stomach,
    Stone,
    Stop,
    Storage,
    Store,
    Storm,
    Story,
    Straight,
    Strange,
    Stranger,
    Strategic,
    Strategy,
    Stream,
    Street,
    Strength,
    Strengthen,
    Stress,
    Stretch,
    Strike,
    String,
    Strip,
    Stroke,
    Strong,
    Strongly,
    Structure,
    Struggle,
    Student,
    Studio,
    Study,
    Stuff,
    Stupid,
    Style,
    Submit,
    Subsequent,
    Substance,
    Substantial,
    Succeed,
    Success,
    Successful,
    Successfully,
    Such,
    Sudden,
    Suddenly,
    Sue,
    Suffer,
    Sufficient,
    Sugar,
    Suggest,
    Suggestion,
    Suicide,
    Suit,
    Summer,
    Summit,
    Sun,
    Super,
    Supply,
    Support,
    Supporter,
    Suppose,
    Supposed,
    Supreme,
    Sure,
    Surely,
    Surface,
    Surgery,
    Surprise,
    Surprised,
    Surprising,
    Surprisingly,
    Surround,
    Survey,
    Survival,
    Survive,
    Survivor,
    Suspect,
    Sustain,
    Swear,
    Sweep,
    Sweet,
    Swim,
    Swing,
    Switch,
    Symbol,
    Symptom,
    System,
    Table,
    Tablespoon,
    Tactic,
    Tail,
    Take,
    Tale,
    Talent,
    Talk,
    Tall,
    Tank,
    Tap,
    Tape,
    Target,
    Task,
    Taste,
    Tax,
    Taxpayer,
    Tea,
    Teach,
    Teacher,
    Teaching,
    Team,
    Tear,
    Teaspoon,
    Technical,
    Technique,
    Technology,
    Teen,
    Teenager,
    Telephone,
    Telescope,
    Television,
    Tell,
    Temperature,
    Temporary,
    Ten,
    Tend,
    Tendency,
    Tennis,
    Tension,
    Tent,
    Term,
    Terms,
    Terrible,
    Territory,
    Terror,
    Terrorism,
    Terrorist,
    Testify,
    Testimony,
    Testing,
    Text,
    Than,
    Thank,
    Thanks,
    That,
    Theater,
    Their,
    Them,
    Theme,
    Themselves,
    Then,
    Theory,
    Therapy,
    There,
    Therefore,
    These,
    They,
    Thick,
    Thin,
    Think,
    Thinking,
    Third,
    Thirty,
    This,
    Those,
    Though,
    Thought,
    Thousand,
    Threat,
    Threaten,
    Three,
    Throat,
    Through,
    Throughout,
    Throw,
    Thus,
    Ticket,
    Tie,
    Tight,
    Tiny,
    Tip,
    Tire,
    Tired,
    Tissue,
    Title,
    Tobacco,
    Today,
    Toe,
    Together,
    Tomato,
    Tomorrow,
    Tone,
    Tongue,
    Tonight,
    Too,
    Tool,
    Tooth,
    Top,
    Topic,
    Toss,
    Total,
    Totally,
    Touch,
    Tough,
    Tour,
    Tourist,
    Tournament,
    Toward,
    Towards,
    Tower,
    Town,
    Toy,
    Trace,
    Track,
    Trade,
    Tradition,
    Traditional,
    Traffic,
    Tragedy,
    Trail,
    Train,
    Training,
    Transfer,
    Transform,
    Transformation,
    Transition,
    Translate,
    Transportation,
    Travel,
    Treat,
    Treatment,
    Treaty,
    Tree,
    Tremendous,
    Trend,
    Trial,
    Tribe,
    Trick,
    Trip,
    Troop,
    Trouble,
    Truck,
    True,
    Truly,
    Trust,
    Truth,
    Try,
    Tube,
    Tunnel,
    Turn,
    Tv,
    Twelve,
    Twenty,
    Twice,
    Twin,
    Two,
    Type,
    Typical,
    Typically,
    Ugly,
    Ultimate,
    Ultimately,
    Unable,
    Uncle,
    Under,
    Undergo,
    Understand,
    Understanding,
    Unfortunately,
    Uniform,
    Union,
    Unique,
    Unit,
    United,
    Universal,
    Universe,
    University,
    Unless,
    Unlike,
    Unlikely,
    Until,
    Unusual,
    Up,
    Upon,
    Upper,
    Urban,
    Urge,
    Us,
    Useful,
    User,
    Usual,
    Usually,
    Utility,
    Vacation,
    Valley,
    Valuable,
    Value,
    Variable,
    Variation,
    Variety,
    Various,
    Vary,
    Vast,
    Vegetable,
    Vehicle,
    Venture,
    Version,
    Versus,
    Very,
    Vessel,
    Veteran,
    Via,
    Victim,
    Victory,
    Video,
    View,
    Viewer,
    Village,
    Violate,
    Violation,
    Violence,
    Violent,
    Virtually,
    Virtue,
    Virus,
    Visible,
    Vision,
    Visit,
    Visitor,
    Visual,
    Vital,
    Voice,
    Volume,
    Volunteer,
    Vote,
    Voter,
    Vs,
    Vulnerable,
    Wage,
    Wait,
    Wake,
    Walk,
    Wall,
    Wander,
    Want,
    War,
    Warm,
    Warn,
    Warning,
    Wash,
    Waste,
    Watch,
    Water,
    Wave,
    Way,
    We,
    Weak,
    Wealth,
    Wealthy,
    Weapon,
    Wear,
    Weather,
    Wedding,
    Week,
    Weekend,
    Weekly,
    Weigh,
    Weight,
    Welcome,
    Welfare,
    Well,
    West,
    Western,
    Wet,
    Whatever,
    Wheel,
    Whenever,
    Whereas,
    Whether,
    While,
    Whisper,
    White,
    Whole,
    Whom,
    Wide,
    Widely,
    Widespread,
    Wife,
    Wild,
    Will,
    Willing,
    Win,
    Wind,
    Window,
    Wine,
    Wing,
    Winner,
    Winter,
    Wipe,
    Wire,
    Wisdom,
    Wise,
    Wish,
    With,
    Withdraw,
    Within,
    Without,
    Witness,
    Woman,
    Wonder,
    Wonderful,
    Wood,
    Wooden,
    Word,
    Work,
    Worker,
    Working,
    Works,
    Workshop,
    World,
    Worried,
    Worry,
    Worth,
    Would,
    Wound,
    Wrap,
    Write,
    Writer,
    Writing,
    Wrong,
    Yard,
    Yeah,
    Year,
    Yell,
    Yellow,
    Yes,
    Yesterday,
    Yet,
    Yield,
    You,
    Young,
    Your,
    Yours,
    Yourself,
    Youth,
    Zone,
    Whoever,
    Stands,
    Nations,
    Understood,
    Referring,
    Initialism,
    An,
    Activities,
    Coordinates,
    Governments,
    Services,
    Persons,
    Peoples,
    Mortal,
    Personage,
    Inhabitant,
    Denizen,
    Whomever,
    Noone,
    Ended,
    Possibilities,
    Things,
    Disbelief,
    Abrupt,
    Unfriendly,
    Enquiry,
    Desires,
    Emphasises,
    Noteworthy,
    Addressed,
    Prepositional,
    Emphasise,
    Taken,
    Exclamations,
    Indicating,
    Emphasizes,
    Assertion,
    Made,
    Contradict,
    Evidently,
    Held,
    Approximation,
    Followed,
    Tag,
    Inviting,
    Asking,
    Questions,
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
            vec!
            [
                Entity::With,
                Entity::Antecedent,
                Entity::As,
                Entity::Subject,
            ],

            vec!
            [
                Entity::With,
                Entity::Antecedent,
                Entity::As,
                Entity::Object,
            ],

            vec!
            [
                Entity::Noun,
                Entity::A,
                Entity::Person,
                Entity::Under,
                Entity::Discussion,
            ],

            vec!
            [
                Entity::Noun,
                Entity::A,
                Entity::Question,
                Entity::Of,
                Entity::Which,
                Entity::Person,
            ],

            vec!
            [
                Entity::Whoever,
            ],

            vec!
            [
                Entity::FusedRelative,
                Entity::He,
                Entity::Who,
            ],

            vec!
            [
                Entity::FusedRelative,
                Entity::They,
                Entity::Who,
            ],

            vec!
            [
                Entity::Military,
                Entity::Stands,
                Entity::For,
                Entity::United,
                Entity::Nations,
                Entity::World,
                Entity::Health,
                Entity::Organization,
            ],

            vec!
            [
                Entity::Used,
                Entity::When,
                Entity::Referring,
                Entity::To,
                Entity::God,
                Entity::Or,
                Entity::Another,
                Entity::Important,
                Entity::Figure,
                Entity::Who,
                Entity::Is,
                Entity::Understood,
                Entity::From,
                Entity::Context,
            ],

            vec!
            [
                Entity::Noun,
                Entity::Initialism,
                Entity::Of,
                Entity::World,
                Entity::Health,
                Entity::Organization,
            ],

            vec!
            [
                Entity::Noun,
                Entity::An,
                Entity::Agency,
                Entity::Which,
                Entity::Coordinates,
                Entity::International,
                Entity::Health,
                Entity::Activities,
                Entity::And,
                Entity::To,
                Entity::Help,
                Entity::Governments,
                Entity::Improve,
                Entity::Health,
                Entity::Services
            ]
        ]
    }
    
    pub fn what() -> Definitions
    {
        vec!
        [
            vec!
            [
                Entity::Interrogative,
                Entity::Which,
            ],

            vec!
            [
                Entity::Interrogative,
                Entity::Which,
                Entity::Thing,
            ],

            vec!
            [
                Entity::Interrogative,
                Entity::Which,
                Entity::Event,
            ],

            vec!
            [
                Entity::Interrogative,
                Entity::Which,
                Entity::Circumstance,
            ],

            vec!
            [
                Entity::Interrogative,
                Entity::Which,
                Entity::Of,
                Entity::An,
                Entity::Open,
                Entity::Ended,
                Entity::Set,
                Entity::Of,
                Entity::Possibilities,
            ],

            vec!
            [
                Entity::Relative,
                Entity::Which,
                Entity::The,
            ],

            vec!
            [
                Entity::Relative,
                Entity::Which,
                Entity::That,
            ],

            vec!
            [
                Entity::FusedRelative,
                Entity::That,
                Entity::Which,
            ],

            vec!
            [
                Entity::FusedRelative,
                Entity::Those,
                Entity::That,
            ],

            vec!
            [
                Entity::FusedRelative,
                Entity::The,
                Entity::Thing,
                Entity::That,
            ],

            vec!
            [
                Entity::FusedRelative,
                Entity::The,
                Entity::Things,
                Entity::That,
            ],

            vec!
            [
                Entity::Relative,
                Entity::Any,
                Entity::That,
            ],

            vec!
            [
                Entity::Relative,
                Entity::All,
                Entity::That,
            ],

            vec!
            [
                Entity::Relative,
                Entity::Whatever,
            ],

            vec!
            [
                Entity::Relative,
                Entity::That,
            ],

            vec!
            [
                Entity::Relative,
                Entity::Which,
            ],

            vec!
            [
                Entity::Relative,
                Entity::Who,
            ],
            
            vec!
            [
                Entity::Interjection,
                Entity::An,
                Entity::Expression,
                Entity::Of,
                Entity::Surprise,
            ],

            vec!
            [
                Entity::Interjection,
                Entity::An,
                Entity::Expression,
                Entity::Of,
                Entity::Disbelief,
            ],
            
            vec!
            [
                Entity::FusedRelative,
                Entity::Anything,
                Entity::That,
            ],
            
            vec!
            [
                Entity::FusedRelative,
                Entity::All,
                Entity::That,
            ],
            
            vec!
            [
                Entity::FusedRelative,
                Entity::Whatever,
            ],
            
            vec!
            [
                Entity::Indexable,
                Entity::The,
                Entity::Identity,
                Entity::Of,
                Entity::A,
                Entity::Thing,
                Entity::As,
                Entity::An,
                Entity::Answer,
                Entity::To,
                Entity::A,
                Entity::Question,
                Entity::Of,
                Entity::What,
            ],

            vec!
            [
                Entity::Interjection,
                Entity::What,
                Entity::Do,
                Entity::You,
                Entity::Want,
            ],

            vec!
            [
                Entity::Interjection,
                Entity::An,
                Entity::Abrupt,
                Entity::Usually,
                Entity::Unfriendly,
                Entity::Enquiry,
                Entity::As,
                Entity::To,
                Entity::What,
                Entity::A,
                Entity::Person,
                Entity::Desires,
            ],

            vec!
            [
                Entity::Noteable,
                Entity::Emphasises,
                Entity::That,
                Entity::Something,
                Entity::Is,
                Entity::Noteworthy,
                Entity::Or,
                Entity::Remarkable,
                Entity::In,
                Entity::Quality,
                Entity::Or,
                Entity::Degree,
            ],

            vec!
            [
                Entity::Noteable,
                Entity::Used,
                Entity::In,
                Entity::Combination,
                Entity::With,
                Entity::Some,
                Entity::Other,
                Entity::Determiner,
            ],

            vec!
            [
                Entity::Indexable,
                Entity::Something,
                Entity::That,
                Entity::Is,
                Entity::Addressed,
                Entity::By,
                Entity::What,
            ],

            vec!
            [
                Entity::Adverb,
                Entity::Used,
                Entity::Before,
                Entity::A,
                Entity::Prepositional,
                Entity::Phrase,
                Entity::To,
                Entity::Emphasise,
                Entity::That,
                Entity::Something,
                Entity::Is,
                Entity::Taken,
                Entity::Into,
                Entity::Consideration,
                Entity::As,
                Entity::A,
                Entity::Cause,
                Entity::Or,
                Entity::Reason,
            ],
            
            vec!
            [
                Entity::Adverb,
                Entity::Used,
                Entity::In,
                Entity::The,
                Entity::Conjunction,
                Entity::With,
                Entity::With,
            ],

            vec!
            [
                Entity::Exclamative,
                Entity::Used,
                Entity::To,
                Entity::Form,
                Entity::Exclamations,
                Entity::Indicating,
                Entity::That,
                Entity::Something,
                Entity::Is,
                Entity::Remarkable,
            ],

            vec!
            [
                Entity::Adjective,
                Entity::Used,
                Entity::To,
                Entity::Form,
                Entity::Exclamations,
                Entity::Followed,
                Entity::By,
                Entity::A,
                Entity::Question,
            ],

            vec!
            [
                Entity::Noun,
                Entity::Emphasizes,
                Entity::The,
                Entity::Truth,
                Entity::Of,
                Entity::An,
                Entity::Assertion,
                Entity::Made,
                Entity::To,
                Entity::Contradict,
                Entity::An,
                Entity::Evidently,
                Entity::False,
                Entity::Assumption,
                Entity::Held,
                Entity::By,
                Entity::The,
                Entity::Person,
            ],

            vec!
            [
                Entity::Interjection,
                Entity::Indicating,
                Entity::A,
                Entity::Guess,
                Entity::To,
                Entity::Try,
                Entity::To,
                Entity::Recall,
                Entity::Information,
            ],

            vec!
            [
                Entity::Interjection,
                Entity::Indicating,
                Entity::An,
                Entity::Approximation,
                Entity::To,
                Entity::Try,
                Entity::To,
                Entity::Recall,
                Entity::Information,
            ],

            vec!
            [
                Entity::Interjection,
                Entity::Indicating,
                Entity::A,
                Entity::Pause,
                Entity::To,
                Entity::Try,
                Entity::To,
                Entity::Recall,
                Entity::Information,
            ],

            vec!
            [
                Entity::Adjective,
                Entity::As,
                Entity::A,
                Entity::Relative,
                Entity::Pronoun,
            ],

            vec!
            [
                Entity::Ambiguous,
                Entity::Something,
            ],

            vec!
            [
                Entity::Ambiguous,
                Entity::Thing,
            ],

            vec!
            [
                Entity::Ambiguous,
                Entity::Stuff,
            ],

            vec!
            [
                Entity::Truncation,
                Entity::What,
                Entity::Do,
                Entity::You,
                Entity::Say,
            ],

            vec!
            [
                Entity::Rhetorical,
                Entity::Used,
                Entity::As,
                Entity::A,
                Entity::Tag,
                Entity::Question,
                Entity::To,
                Entity::Emphasise,
                Entity::A,
                Entity::Statement,
                Entity::Inviting,
                Entity::Agreement,
            ],

            vec!
            [
                Entity::Pronoun,
                Entity::Used,
                Entity::In,
                Entity::Asking,
                Entity::Questions,
                Entity::Regarding,
                Entity::Either,
                Entity::Persons,
                Entity::Or,
                Entity::Things,
            ],

            vec!
            [
                Entity::Adjective,
                Entity::How,
            ],

            vec!
            [
                Entity::Adjective,
                Entity::Which,
            ],
        ]
    }
}

#[derive( Clone, Debug )]
pub enum Ident
{
    Defense( Definitions ),
    Offense( Definitions ),
    Perjorate( Definitions ),
    What( Definitions ),
    Who( Definitions ),
    Whose( Definitions ),
    Which( Definitions ),
    Where( Definitions ),
    Why( Definitions ),
    How( Definitions ),
    When( Definitions ),
    Mock( Definitions ),
    Inequal( Definitions ),
    Equal( Definitions ),
    Predicate( Definitions ),
    Compare( Definitions ),
    Indicate( Definitions ),
    Instance( Definitions ),
    Referral( Definitions ),
    Refer( Definitions ),
    Referrer( Definitions ),
    Reference( Definitions ),
    Transgender( Definitions ),
    Gender( Definitions ),
    Person( Definitions ),
    People( Definitions ),
    Animal( Definitions ),
    Animate( Definitions ),
    Inanimate( Definitions ),
    Project( Definitions ),
    Subject( Definitions ),
    Object( Definitions ),
    Place( Definitions ),
    Never( Definitions ),
    Sometime( Definitions ),
    Time( Definitions ),
    Abstract( Definitions ),
    Noun( Definitions ),    
    Verb( Definitions ),
    Adverb( Definitions ),
    Adjective( Definitions ),
    Pronoun( Definitions ),
    Interrogative( Definitions ),
    Relative( Definitions ),
    Interjection( Definitions ),
    Indexable( Definitions ),
    Noteable( Definitions ),
    Determiner( Definitions ),
    Conjunction( Definitions ),
    Exclamative( Definitions ),
    Ambiguous( Definitions ),    
    Truncation( Definitions ),
    Rhetorical( Definitions ),
    Unknown( Definitions ),
    Is( Definitions ),
    Be( Definitions ),
    It( Definitions ),
    To( Definitions ),
    Use( Definitions ),
    Nor( Definitions ),
    Or( Definitions ),
    Ask( Definitions ),
    For( Definitions ),
    The( Definitions ),
    Identity( Definitions ),
    Of( Definitions ),
    Someone( Definitions ),
    Used( Definitions ),
    In( Definitions ),
    A( Definitions ),
    Direct( Definitions ),
    Indirect( Definitions ),
    Question( Definitions ),
    Introduces( Definitions ),
    Clause( Definitions ),
    Having( Definitions ),
    Human( Definitions ),
    Antecedent( Definitions ),
    Test( Definitions ),
    Abandon( Definitions ),
    Ability( Definitions ),
    Able( Definitions ),
    Abortion( Definitions ),
    About( Definitions ),
    Above( Definitions ),
    Abroad( Definitions ),
    Absence( Definitions ),
    Absolute( Definitions ),
    Absolutely( Definitions ),
    Absorb( Definitions ),
    Abuse( Definitions ),
    Academic( Definitions ),
    Accept( Definitions ),
    Access( Definitions ),
    Accident( Definitions ),
    Accompany( Definitions ),
    Accomplish( Definitions ),
    According( Definitions ),
    Account( Definitions ),
    Accurate( Definitions ),
    Accuse( Definitions ),
    Achieve( Definitions ),
    Achievement( Definitions ),
    Acid( Definitions ),
    Acknowledge( Definitions ),
    Acquire( Definitions ),
    Across( Definitions ),
    Act( Definitions ),
    Action( Definitions ),
    Active( Definitions ),
    Activist( Definitions ),
    Activity( Definitions ),
    Actor( Definitions ),
    Actress( Definitions ),
    Actual( Definitions ),
    Actually( Definitions ),
    Ad( Definitions ),
    Adapt( Definitions ),
    Add( Definitions ),
    Addition( Definitions ),
    Additional( Definitions ),
    Address( Definitions ),
    Adequate( Definitions ),
    Adjust( Definitions ),
    Adjustment( Definitions ),
    Administration( Definitions ),
    Administrator( Definitions ),
    Admire( Definitions ),
    Admission( Definitions ),
    Admit( Definitions ),
    Adolescent( Definitions ),
    Adopt( Definitions ),
    Adult( Definitions ),
    Advance( Definitions ),
    Advanced( Definitions ),
    Advantage( Definitions ),
    Adventure( Definitions ),
    Advertising( Definitions ),
    Advice( Definitions ),
    Advise( Definitions ),
    Adviser( Definitions ),
    Advocate( Definitions ),
    Affair( Definitions ),
    Affect( Definitions ),
    Afford( Definitions ),
    Afraid( Definitions ),
    African( Definitions ),
    AfricanAmerican( Definitions ),
    After( Definitions ),
    Afternoon( Definitions ),
    Again( Definitions ),
    Against( Definitions ),
    Age( Definitions ),
    Agency( Definitions ),
    Agenda( Definitions ),
    Agent( Definitions ),
    Aggressive( Definitions ),
    Ago( Definitions ),
    Agree( Definitions ),
    Agreement( Definitions ),
    Agricultural( Definitions ),
    Ah( Definitions ),
    Ahead( Definitions ),
    Aid( Definitions ),
    Aide( Definitions ),
    Aids( Definitions ),
    Aim( Definitions ),
    Air( Definitions ),
    Aircraft( Definitions ),
    Airline( Definitions ),
    Airport( Definitions ),
    Album( Definitions ),
    Alcohol( Definitions ),
    Alive( Definitions ),
    All( Definitions ),
    Alliance( Definitions ),
    Allow( Definitions ),
    Ally( Definitions ),
    Almost( Definitions ),
    Alone( Definitions ),
    Along( Definitions ),
    Already( Definitions ),
    Also( Definitions ),
    Alter( Definitions ),
    Alternative( Definitions ),
    Although( Definitions ),
    Always( Definitions ),
    Am( Definitions ),
    Amazing( Definitions ),
    American( Definitions ),
    Among( Definitions ),
    Amount( Definitions ),
    Analysis( Definitions ),
    Analyst( Definitions ),
    Analyze( Definitions ),
    Ancient( Definitions ),
    And( Definitions ),
    Anger( Definitions ),
    Angle( Definitions ),
    Angry( Definitions ),
    Anniversary( Definitions ),
    Announce( Definitions ),
    Annual( Definitions ),
    Another( Definitions ),
    Answer( Definitions ),
    Anticipate( Definitions ),
    Anxiety( Definitions ),
    Any( Definitions ),
    Anybody( Definitions ),
    Anymore( Definitions ),
    Anyone( Definitions ),
    Anything( Definitions ),
    Anyway( Definitions ),
    Anywhere( Definitions ),
    Apart( Definitions ),
    Apartment( Definitions ),
    Apparent( Definitions ),
    Apparently( Definitions ),
    Appeal( Definitions ),
    Appear( Definitions ),
    Appearance( Definitions ),
    Apple( Definitions ),
    Application( Definitions ),
    Apply( Definitions ),
    Appoint( Definitions ),
    Appointment( Definitions ),
    Appreciate( Definitions ),
    Approach( Definitions ),
    Appropriate( Definitions ),
    Approval( Definitions ),
    Approve( Definitions ),
    Approximately( Definitions ),
    Arab( Definitions ),
    Architect( Definitions ),
    Area( Definitions ),
    Argue( Definitions ),
    Argument( Definitions ),
    Arise( Definitions ),
    Arm( Definitions ),
    Armed( Definitions ),
    Army( Definitions ),
    Around( Definitions ),
    Arrange( Definitions ),
    Arrangement( Definitions ),
    Arrest( Definitions ),
    Arrival( Definitions ),
    Arrive( Definitions ),
    Art( Definitions ),
    Article( Definitions ),
    Artist( Definitions ),
    Artistic( Definitions ),
    As( Definitions ),
    Asian( Definitions ),
    Aside( Definitions ),
    Asleep( Definitions ),
    Aspect( Definitions ),
    Assault( Definitions ),
    Assert( Definitions ),
    Assess( Definitions ),
    Assessment( Definitions ),
    Asset( Definitions ),
    Assign( Definitions ),
    Assignment( Definitions ),
    Assist( Definitions ),
    Assistance( Definitions ),
    Assistant( Definitions ),
    Associate( Definitions ),
    Association( Definitions ),
    Assume( Definitions ),
    Assumption( Definitions ),
    Assure( Definitions ),
    At( Definitions ),
    Athlete( Definitions ),
    Athletic( Definitions ),
    Atmosphere( Definitions ),
    Attach( Definitions ),
    Attack( Definitions ),
    Attempt( Definitions ),
    Attend( Definitions ),
    Attention( Definitions ),
    Attitude( Definitions ),
    Attorney( Definitions ),
    Attract( Definitions ),
    Attractive( Definitions ),
    Attribute( Definitions ),
    Audience( Definitions ),
    Author( Definitions ),
    Authority( Definitions ),
    Auto( Definitions ),
    Available( Definitions ),
    Average( Definitions ),
    Avoid( Definitions ),
    Award( Definitions ),
    Aware( Definitions ),
    Awareness( Definitions ),
    Away( Definitions ),
    Awful( Definitions ),
    Baby( Definitions ),
    Back( Definitions ),
    Background( Definitions ),
    Bad( Definitions ),
    Badly( Definitions ),
    Bag( Definitions ),
    Bake( Definitions ),
    Balance( Definitions ),
    Ball( Definitions ),
    Ban( Definitions ),
    Band( Definitions ),
    Bank( Definitions ),
    Bar( Definitions ),
    Barely( Definitions ),
    Barrel( Definitions ),
    Barrier( Definitions ),
    Base( Definitions ),
    Baseball( Definitions ),
    Basic( Definitions ),
    Basically( Definitions ),
    Basis( Definitions ),
    Basket( Definitions ),
    Basketball( Definitions ),
    Bathroom( Definitions ),
    Battery( Definitions ),
    Battle( Definitions ),
    Beach( Definitions ),
    Bean( Definitions ),
    Bear( Definitions ),
    Beat( Definitions ),
    Beautiful( Definitions ),
    Beauty( Definitions ),
    Because( Definitions ),
    Become( Definitions ),
    Bed( Definitions ),
    Bedroom( Definitions ),
    Beer( Definitions ),
    Before( Definitions ),
    Begin( Definitions ),
    Beginning( Definitions ),
    Behavior( Definitions ),
    Behind( Definitions ),
    Being( Definitions ),
    Belief( Definitions ),
    Believe( Definitions ),
    Bell( Definitions ),
    Belong( Definitions ),
    Below( Definitions ),
    Belt( Definitions ),
    Bench( Definitions ),
    Bend( Definitions ),
    Beneath( Definitions ),
    Benefit( Definitions ),
    Beside( Definitions ),
    Besides( Definitions ),
    Best( Definitions ),
    Bet( Definitions ),
    Better( Definitions ),
    Between( Definitions ),
    Beyond( Definitions ),
    Bible( Definitions ),
    Big( Definitions ),
    Bike( Definitions ),
    Bill( Definitions ),
    Billion( Definitions ),
    Bind( Definitions ),
    Biological( Definitions ),
    Bird( Definitions ),
    Birth( Definitions ),
    Birthday( Definitions ),
    Bit( Definitions ),
    Bite( Definitions ),
    Black( Definitions ),
    Blade( Definitions ),
    Blame( Definitions ),
    Blanket( Definitions ),
    Blind( Definitions ),
    Block( Definitions ),
    Blood( Definitions ),
    Blow( Definitions ),
    Blue( Definitions ),
    Board( Definitions ),
    Boat( Definitions ),
    Body( Definitions ),
    Bomb( Definitions ),
    Bombing( Definitions ),
    Bond( Definitions ),
    Bone( Definitions ),
    Book( Definitions ),
    Boom( Definitions ),
    Boot( Definitions ),
    Border( Definitions ),
    Born( Definitions ),
    Borrow( Definitions ),
    Boss( Definitions ),
    Both( Definitions ),
    Bother( Definitions ),
    Bottle( Definitions ),
    Bottom( Definitions ),
    Boundary( Definitions ),
    Bowl( Definitions ),
    Box( Definitions ),
    Boy( Definitions ),
    Boyfriend( Definitions ),
    Brain( Definitions ),
    Branch( Definitions ),
    Brand( Definitions ),
    Bread( Definitions ),
    Break( Definitions ),
    Breakfast( Definitions ),
    Breast( Definitions ),
    Breath( Definitions ),
    Breathe( Definitions ),
    Brick( Definitions ),
    Bridge( Definitions ),
    Brief( Definitions ),
    Briefly( Definitions ),
    Bright( Definitions ),
    Brilliant( Definitions ),
    Bring( Definitions ),
    British( Definitions ),
    Broad( Definitions ),
    Broken( Definitions ),
    Brother( Definitions ),
    Brown( Definitions ),
    Brush( Definitions ),
    Buck( Definitions ),
    Budget( Definitions ),
    Build( Definitions ),
    Building( Definitions ),
    Bullet( Definitions ),
    Bunch( Definitions ),
    Burden( Definitions ),
    Burn( Definitions ),
    Bury( Definitions ),
    Bus( Definitions ),
    Business( Definitions ),
    Busy( Definitions ),
    But( Definitions ),
    Butter( Definitions ),
    Button( Definitions ),
    Buy( Definitions ),
    Buyer( Definitions ),
    By( Definitions ),
    Cabin( Definitions ),
    Cabinet( Definitions ),
    Cable( Definitions ),
    Cake( Definitions ),
    Calculate( Definitions ),
    Call( Definitions ),
    Camera( Definitions ),
    Camp( Definitions ),
    Campaign( Definitions ),
    Campus( Definitions ),
    Can( Definitions ),
    Canadian( Definitions ),
    Cancer( Definitions ),
    Candidate( Definitions ),
    Cap( Definitions ),
    Capability( Definitions ),
    Capable( Definitions ),
    Capacity( Definitions ),
    Capital( Definitions ),
    Captain( Definitions ),
    Capture( Definitions ),
    Car( Definitions ),
    Carbon( Definitions ),
    Card( Definitions ),
    Care( Definitions ),
    Career( Definitions ),
    Careful( Definitions ),
    Carefully( Definitions ),
    Carrier( Definitions ),
    Carry( Definitions ),
    Case( Definitions ),
    Cash( Definitions ),
    Cast( Definitions ),
    Cat( Definitions ),
    Catch( Definitions ),
    Category( Definitions ),
    Catholic( Definitions ),
    Cause( Definitions ),
    Ceiling( Definitions ),
    Celebrate( Definitions ),
    Celebration( Definitions ),
    Celebrity( Definitions ),
    Cell( Definitions ),
    Center( Definitions ),
    Central( Definitions ),
    Century( Definitions ),
    Ceo( Definitions ),
    Ceremony( Definitions ),
    Certain( Definitions ),
    Certainly( Definitions ),
    Chain( Definitions ),
    Chair( Definitions ),
    Chairman( Definitions ),
    Challenge( Definitions ),
    Chamber( Definitions ),
    Champion( Definitions ),
    Championship( Definitions ),
    Chance( Definitions ),
    Change( Definitions ),
    Changing( Definitions ),
    Channel( Definitions ),
    Chapter( Definitions ),
    Character( Definitions ),
    Characteristic( Definitions ),
    Characterize( Definitions ),
    Charge( Definitions ),
    Charity( Definitions ),
    Chart( Definitions ),
    Chase( Definitions ),
    Cheap( Definitions ),
    Check( Definitions ),
    Cheek( Definitions ),
    Cheese( Definitions ),
    Chef( Definitions ),
    Chemical( Definitions ),
    Chest( Definitions ),
    Chicken( Definitions ),
    Chief( Definitions ),
    Child( Definitions ),
    Childhood( Definitions ),
    Chinese( Definitions ),
    Chip( Definitions ),
    Chocolate( Definitions ),
    Choice( Definitions ),
    Cholesterol( Definitions ),
    Choose( Definitions ),
    Christian( Definitions ),
    Christmas( Definitions ),
    Church( Definitions ),
    Cigarette( Definitions ),
    Circle( Definitions ),
    Circumstance( Definitions ),
    Cite( Definitions ),
    Citizen( Definitions ),
    City( Definitions ),
    Civil( Definitions ),
    Civilian( Definitions ),
    Claim( Definitions ),
    Class( Definitions ),
    Classic( Definitions ),
    Classroom( Definitions ),
    Clean( Definitions ),
    Clear( Definitions ),
    Clearly( Definitions ),
    Client( Definitions ),
    Climate( Definitions ),
    Climb( Definitions ),
    Clinic( Definitions ),
    Clinical( Definitions ),
    Clock( Definitions ),
    Close( Definitions ),
    Closely( Definitions ),
    Closer( Definitions ),
    Clothes( Definitions ),
    Clothing( Definitions ),
    Cloud( Definitions ),
    Club( Definitions ),
    Clue( Definitions ),
    Cluster( Definitions ),
    Coach( Definitions ),
    Coal( Definitions ),
    Coalition( Definitions ),
    Coast( Definitions ),
    Coat( Definitions ),
    Code( Definitions ),
    Coffee( Definitions ),
    Cognitive( Definitions ),
    Cold( Definitions ),
    Collapse( Definitions ),
    Colleague( Definitions ),
    Collect( Definitions ),
    Collection( Definitions ),
    Collective( Definitions ),
    College( Definitions ),
    Colonial( Definitions ),
    Color( Definitions ),
    Column( Definitions ),
    Combination( Definitions ),
    Combine( Definitions ),
    Come( Definitions ),
    Comedy( Definitions ),
    Comfort( Definitions ),
    Comfortable( Definitions ),
    Command( Definitions ),
    Commander( Definitions ),
    Comment( Definitions ),
    Commercial( Definitions ),
    Commission( Definitions ),
    Commit( Definitions ),
    Commitment( Definitions ),
    Committee( Definitions ),
    Common( Definitions ),
    Communicate( Definitions ),
    Communication( Definitions ),
    Community( Definitions ),
    Company( Definitions ),
    Comparison( Definitions ),
    Compete( Definitions ),
    Competition( Definitions ),
    Competitive( Definitions ),
    Competitor( Definitions ),
    Complain( Definitions ),
    Complaint( Definitions ),
    Complete( Definitions ),
    Completely( Definitions ),
    Complex( Definitions ),
    Complicated( Definitions ),
    Component( Definitions ),
    Compose( Definitions ),
    Composition( Definitions ),
    Comprehensive( Definitions ),
    Computer( Definitions ),
    Concentrate( Definitions ),
    Concentration( Definitions ),
    Concept( Definitions ),
    Concern( Definitions ),
    Concerned( Definitions ),
    Concert( Definitions ),
    Conclude( Definitions ),
    Conclusion( Definitions ),
    Concrete( Definitions ),
    Condition( Definitions ),
    Conduct( Definitions ),
    Conference( Definitions ),
    Confidence( Definitions ),
    Confident( Definitions ),
    Confirm( Definitions ),
    Conflict( Definitions ),
    Confront( Definitions ),
    Confusion( Definitions ),
    Congress( Definitions ),
    Congressional( Definitions ),
    Connect( Definitions ),
    Connection( Definitions ),
    Consciousness( Definitions ),
    Consensus( Definitions ),
    Consequence( Definitions ),
    Conservative( Definitions ),
    Consider( Definitions ),
    Considerable( Definitions ),
    Consideration( Definitions ),
    Consist( Definitions ),
    Consistent( Definitions ),
    Constant( Definitions ),
    Constantly( Definitions ),
    Constitute( Definitions ),
    Constitutional( Definitions ),
    Construct( Definitions ),
    Construction( Definitions ),
    Consultant( Definitions ),
    Consume( Definitions ),
    Consumer( Definitions ),
    Consumption( Definitions ),
    Contact( Definitions ),
    Contain( Definitions ),
    Container( Definitions ),
    Contemporary( Definitions ),
    Content( Definitions ),
    Contest( Definitions ),
    Context( Definitions ),
    Continue( Definitions ),
    Continued( Definitions ),
    Contract( Definitions ),
    Contrast( Definitions ),
    Contribute( Definitions ),
    Contribution( Definitions ),
    Control( Definitions ),
    Controversial( Definitions ),
    Controversy( Definitions ),
    Convention( Definitions ),
    Conventional( Definitions ),
    Conversation( Definitions ),
    Convert( Definitions ),
    Conviction( Definitions ),
    Convince( Definitions ),
    Cook( Definitions ),
    Cookie( Definitions ),
    Cooking( Definitions ),
    Cool( Definitions ),
    Cooperation( Definitions ),
    Cop( Definitions ),
    Cope( Definitions ),
    Copy( Definitions ),
    Core( Definitions ),
    Corn( Definitions ),
    Corner( Definitions ),
    Corporate( Definitions ),
    Corporation( Definitions ),
    Correct( Definitions ),
    Correspondent( Definitions ),
    Cost( Definitions ),
    Cotton( Definitions ),
    Couch( Definitions ),
    Could( Definitions ),
    Council( Definitions ),
    Counselor( Definitions ),
    Count( Definitions ),
    Counter( Definitions ),
    Country( Definitions ),
    County( Definitions ),
    Couple( Definitions ),
    Courage( Definitions ),
    Course( Definitions ),
    Court( Definitions ),
    Cousin( Definitions ),
    Cover( Definitions ),
    Coverage( Definitions ),
    Cow( Definitions ),
    Crack( Definitions ),
    Craft( Definitions ),
    Crash( Definitions ),
    Crazy( Definitions ),
    Cream( Definitions ),
    Create( Definitions ),
    Creation( Definitions ),
    Creative( Definitions ),
    Creature( Definitions ),
    Credit( Definitions ),
    Crew( Definitions ),
    Crime( Definitions ),
    Criminal( Definitions ),
    Crisis( Definitions ),
    Criteria( Definitions ),
    Critic( Definitions ),
    Critical( Definitions ),
    Criticism( Definitions ),
    Criticize( Definitions ),
    Crop( Definitions ),
    Cross( Definitions ),
    Crowd( Definitions ),
    Crucial( Definitions ),
    Cry( Definitions ),
    Cultural( Definitions ),
    Culture( Definitions ),
    Cup( Definitions ),
    Curious( Definitions ),
    Current( Definitions ),
    Currently( Definitions ),
    Curriculum( Definitions ),
    Custom( Definitions ),
    Customer( Definitions ),
    Cut( Definitions ),
    Cycle( Definitions ),
    Dad( Definitions ),
    Daily( Definitions ),
    Damage( Definitions ),
    Dance( Definitions ),
    Danger( Definitions ),
    Dangerous( Definitions ),
    Dare( Definitions ),
    Dark( Definitions ),
    Darkness( Definitions ),
    Data( Definitions ),
    Date( Definitions ),
    Daughter( Definitions ),
    Day( Definitions ),
    Dead( Definitions ),
    Deal( Definitions ),
    Dealer( Definitions ),
    Dear( Definitions ),
    Death( Definitions ),
    Debate( Definitions ),
    Debt( Definitions ),
    Decade( Definitions ),
    Decide( Definitions ),
    Decision( Definitions ),
    Deck( Definitions ),
    Declare( Definitions ),
    Decline( Definitions ),
    Decrease( Definitions ),
    Deep( Definitions ),
    Deeply( Definitions ),
    Deer( Definitions ),
    Defeat( Definitions ),
    Defend( Definitions ),
    Defendant( Definitions ),
    Defensive( Definitions ),
    Deficit( Definitions ),
    Define( Definitions ),
    Definitely( Definitions ),
    Definition( Definitions ),
    Degree( Definitions ),
    Delay( Definitions ),
    Deliver( Definitions ),
    Delivery( Definitions ),
    Demand( Definitions ),
    Democracy( Definitions ),
    Democrat( Definitions ),
    Democratic( Definitions ),
    Demonstrate( Definitions ),
    Demonstration( Definitions ),
    Deny( Definitions ),
    Department( Definitions ),
    Depend( Definitions ),
    Dependent( Definitions ),
    Depending( Definitions ),
    Depict( Definitions ),
    Depression( Definitions ),
    Depth( Definitions ),
    Deputy( Definitions ),
    Derive( Definitions ),
    Describe( Definitions ),
    Description( Definitions ),
    Desert( Definitions ),
    Deserve( Definitions ),
    Design( Definitions ),
    Designer( Definitions ),
    Desire( Definitions ),
    Desk( Definitions ),
    Desperate( Definitions ),
    Despite( Definitions ),
    Destroy( Definitions ),
    Destruction( Definitions ),
    Detail( Definitions ),
    Detailed( Definitions ),
    Detect( Definitions ),
    Determine( Definitions ),
    Develop( Definitions ),
    Developing( Definitions ),
    Development( Definitions ),
    Device( Definitions ),
    Devote( Definitions ),
    Dialogue( Definitions ),
    Die( Definitions ),
    Diet( Definitions ),
    Differ( Definitions ),
    Difference( Definitions ),
    Different( Definitions ),
    Differently( Definitions ),
    Difficult( Definitions ),
    Difficulty( Definitions ),
    Dig( Definitions ),
    Digital( Definitions ),
    Dimension( Definitions ),
    Dining( Definitions ),
    Dinner( Definitions ),
    Direction( Definitions ),
    Directly( Definitions ),
    Director( Definitions ),
    Dirt( Definitions ),
    Dirty( Definitions ),
    Disability( Definitions ),
    Disagree( Definitions ),
    Disappear( Definitions ),
    Disaster( Definitions ),
    Discipline( Definitions ),
    Discourse( Definitions ),
    Discover( Definitions ),
    Discovery( Definitions ),
    Discrimination( Definitions ),
    Discuss( Definitions ),
    Discussion( Definitions ),
    Disease( Definitions ),
    Dish( Definitions ),
    Dismiss( Definitions ),
    Disorder( Definitions ),
    Display( Definitions ),
    Dispute( Definitions ),
    Distance( Definitions ),
    Distant( Definitions ),
    Distinct( Definitions ),
    Distinction( Definitions ),
    Distinguish( Definitions ),
    Distribute( Definitions ),
    Distribution( Definitions ),
    District( Definitions ),
    Diverse( Definitions ),
    Diversity( Definitions ),
    Divide( Definitions ),
    Division( Definitions ),
    Divorce( Definitions ),
    Dna( Definitions ),
    Do( Definitions ),
    Doctor( Definitions ),
    Document( Definitions ),
    Dog( Definitions ),
    Domestic( Definitions ),
    Dominant( Definitions ),
    Dominate( Definitions ),
    Door( Definitions ),
    Double( Definitions ),
    Doubt( Definitions ),
    Down( Definitions ),
    Downtown( Definitions ),
    Dozen( Definitions ),
    Draft( Definitions ),
    Drag( Definitions ),
    Drama( Definitions ),
    Dramatic( Definitions ),
    Dramatically( Definitions ),
    Draw( Definitions ),
    Drawing( Definitions ),
    Dream( Definitions ),
    Dress( Definitions ),
    Drink( Definitions ),
    Drive( Definitions ),
    Driver( Definitions ),
    Drop( Definitions ),
    Drug( Definitions ),
    Dry( Definitions ),
    Due( Definitions ),
    During( Definitions ),
    Dust( Definitions ),
    Duty( Definitions ),
    Each( Definitions ),
    Eager( Definitions ),
    Ear( Definitions ),
    Early( Definitions ),
    Earn( Definitions ),
    Earnings( Definitions ),
    Earth( Definitions ),
    Ease( Definitions ),
    Easily( Definitions ),
    East( Definitions ),
    Eastern( Definitions ),
    Easy( Definitions ),
    Eat( Definitions ),
    Economic( Definitions ),
    Economics( Definitions ),
    Economist( Definitions ),
    Economy( Definitions ),
    Edge( Definitions ),
    Edition( Definitions ),
    Editor( Definitions ),
    Educate( Definitions ),
    Education( Definitions ),
    Educational( Definitions ),
    Educator( Definitions ),
    Effect( Definitions ),
    Effective( Definitions ),
    Effectively( Definitions ),
    Efficiency( Definitions ),
    Efficient( Definitions ),
    Effort( Definitions ),
    Egg( Definitions ),
    Eight( Definitions ),
    Either( Definitions ),
    Elderly( Definitions ),
    Elect( Definitions ),
    Election( Definitions ),
    Electric( Definitions ),
    Electricity( Definitions ),
    Electronic( Definitions ),
    Element( Definitions ),
    Elementary( Definitions ),
    Eliminate( Definitions ),
    Elite( Definitions ),
    Else( Definitions ),
    Elsewhere( Definitions ),
    Email( Definitions ),
    Embrace( Definitions ),
    Emerge( Definitions ),
    Emergency( Definitions ),
    Emission( Definitions ),
    Emotion( Definitions ),
    Emotional( Definitions ),
    Emphasis( Definitions ),
    Emphasize( Definitions ),
    Employ( Definitions ),
    Employee( Definitions ),
    Employer( Definitions ),
    Employment( Definitions ),
    Empty( Definitions ),
    Enable( Definitions ),
    Encounter( Definitions ),
    Encourage( Definitions ),
    End( Definitions ),
    Enemy( Definitions ),
    Energy( Definitions ),
    Enforcement( Definitions ),
    Engage( Definitions ),
    Engine( Definitions ),
    Engineer( Definitions ),
    Engineering( Definitions ),
    English( Definitions ),
    Enhance( Definitions ),
    Enjoy( Definitions ),
    Enormous( Definitions ),
    Enough( Definitions ),
    Ensure( Definitions ),
    Enter( Definitions ),
    Enterprise( Definitions ),
    Entertainment( Definitions ),
    Entire( Definitions ),
    Entirely( Definitions ),
    Entrance( Definitions ),
    Entry( Definitions ),
    Environment( Definitions ),
    Environmental( Definitions ),
    Episode( Definitions ),
    Equally( Definitions ),
    Equipment( Definitions ),
    Era( Definitions ),
    Error( Definitions ),
    Escape( Definitions ),
    Especially( Definitions ),
    Essay( Definitions ),
    Essential( Definitions ),
    Essentially( Definitions ),
    Establish( Definitions ),
    Establishment( Definitions ),
    Estate( Definitions ),
    Estimate( Definitions ),
    Etc( Definitions ),
    Ethics( Definitions ),
    Ethnic( Definitions ),
    European( Definitions ),
    Evaluate( Definitions ),
    Evaluation( Definitions ),
    Even( Definitions ),
    Evening( Definitions ),
    Event( Definitions ),
    Eventually( Definitions ),
    Ever( Definitions ),
    Every( Definitions ),
    Everybody( Definitions ),
    Everyday( Definitions ),
    Everyone( Definitions ),
    Everything( Definitions ),
    Everywhere( Definitions ),
    Evidence( Definitions ),
    Evolution( Definitions ),
    Evolve( Definitions ),
    Exact( Definitions ),
    Exactly( Definitions ),
    Examination( Definitions ),
    Examine( Definitions ),
    Example( Definitions ),
    Exceed( Definitions ),
    Excellent( Definitions ),
    Except( Definitions ),
    Exception( Definitions ),
    Exchange( Definitions ),
    Exciting( Definitions ),
    Executive( Definitions ),
    Exercise( Definitions ),
    Exhibit( Definitions ),
    Exhibition( Definitions ),
    Exist( Definitions ),
    Existence( Definitions ),
    Existing( Definitions ),
    Expand( Definitions ),
    Expansion( Definitions ),
    Expect( Definitions ),
    Expectation( Definitions ),
    Expense( Definitions ),
    Expensive( Definitions ),
    Experience( Definitions ),
    Experiment( Definitions ),
    Expert( Definitions ),
    Explain( Definitions ),
    Explanation( Definitions ),
    Explode( Definitions ),
    Explore( Definitions ),
    Explosion( Definitions ),
    Expose( Definitions ),
    Exposure( Definitions ),
    Express( Definitions ),
    Expression( Definitions ),
    Extend( Definitions ),
    Extension( Definitions ),
    Extensive( Definitions ),
    Extent( Definitions ),
    External( Definitions ),
    Extra( Definitions ),
    Extraordinary( Definitions ),
    Extreme( Definitions ),
    Extremely( Definitions ),
    Eye( Definitions ),
    Fabric( Definitions ),
    Face( Definitions ),
    Facility( Definitions ),
    Fact( Definitions ),
    Factor( Definitions ),
    Factory( Definitions ),
    Faculty( Definitions ),
    Fade( Definitions ),
    Fail( Definitions ),
    Failure( Definitions ),
    Fair( Definitions ),
    Fairly( Definitions ),
    Faith( Definitions ),
    Fall( Definitions ),
    False( Definitions ),
    Familiar( Definitions ),
    Family( Definitions ),
    Famous( Definitions ),
    Fan( Definitions ),
    Fantasy( Definitions ),
    Far( Definitions ),
    Farm( Definitions ),
    Farmer( Definitions ),
    Fashion( Definitions ),
    Fast( Definitions ),
    Fat( Definitions ),
    Fate( Definitions ),
    Father( Definitions ),
    Fault( Definitions ),
    Favor( Definitions ),
    Favorite( Definitions ),
    Fear( Definitions ),
    Feature( Definitions ),
    Federal( Definitions ),
    Fee( Definitions ),
    Feed( Definitions ),
    Feel( Definitions ),
    Feeling( Definitions ),
    Fellow( Definitions ),
    Female( Definitions ),
    Fence( Definitions ),
    Few( Definitions ),
    Fewer( Definitions ),
    Fiber( Definitions ),
    Fiction( Definitions ),
    Field( Definitions ),
    Fifteen( Definitions ),
    Fifth( Definitions ),
    Fifty( Definitions ),
    Fight( Definitions ),
    Fighter( Definitions ),
    Fighting( Definitions ),
    Figure( Definitions ),
    File( Definitions ),
    Fill( Definitions ),
    Film( Definitions ),
    Final( Definitions ),
    Finally( Definitions ),
    Finance( Definitions ),
    Financial( Definitions ),
    Find( Definitions ),
    Finding( Definitions ),
    Fine( Definitions ),
    Finger( Definitions ),
    Finish( Definitions ),
    Fire( Definitions ),
    Firm( Definitions ),
    First( Definitions ),
    Fish( Definitions ),
    Fishing( Definitions ),
    Fit( Definitions ),
    Fitness( Definitions ),
    Five( Definitions ),
    Fix( Definitions ),
    Flag( Definitions ),
    Flame( Definitions ),
    Flat( Definitions ),
    Flavor( Definitions ),
    Flee( Definitions ),
    Flesh( Definitions ),
    Flight( Definitions ),
    Float( Definitions ),
    Floor( Definitions ),
    Flow( Definitions ),
    Flower( Definitions ),
    Fly( Definitions ),
    Focus( Definitions ),
    Folk( Definitions ),
    Follow( Definitions ),
    Following( Definitions ),
    Food( Definitions ),
    Foot( Definitions ),
    Football( Definitions ),
    Force( Definitions ),
    Foreign( Definitions ),
    Forest( Definitions ),
    Forever( Definitions ),
    Forget( Definitions ),
    Form( Definitions ),
    Formal( Definitions ),
    Formation( Definitions ),
    Former( Definitions ),
    Formula( Definitions ),
    Forth( Definitions ),
    Fortune( Definitions ),
    Forward( Definitions ),
    Found( Definitions ),
    Foundation( Definitions ),
    Founder( Definitions ),
    Four( Definitions ),
    Fourth( Definitions ),
    Frame( Definitions ),
    Framework( Definitions ),
    Free( Definitions ),
    Freedom( Definitions ),
    Freeze( Definitions ),
    French( Definitions ),
    Frequency( Definitions ),
    Frequent( Definitions ),
    Frequently( Definitions ),
    Fresh( Definitions ),
    Friend( Definitions ),
    Friendly( Definitions ),
    Friendship( Definitions ),
    From( Definitions ),
    Front( Definitions ),
    Fruit( Definitions ),
    Frustration( Definitions ),
    Fuel( Definitions ),
    Full( Definitions ),
    Fully( Definitions ),
    Fun( Definitions ),
    Function( Definitions ),
    Fund( Definitions ),
    Fundamental( Definitions ),
    Funding( Definitions ),
    Funeral( Definitions ),
    Funny( Definitions ),
    Furniture( Definitions ),
    Furthermore( Definitions ),
    Future( Definitions ),
    Gain( Definitions ),
    Galaxy( Definitions ),
    Gallery( Definitions ),
    Game( Definitions ),
    Gang( Definitions ),
    Gap( Definitions ),
    Garage( Definitions ),
    Garden( Definitions ),
    Garlic( Definitions ),
    Gas( Definitions ),
    Gate( Definitions ),
    Gather( Definitions ),
    Gay( Definitions ),
    Gaze( Definitions ),
    Gear( Definitions ),
    Gene( Definitions ),
    General( Definitions ),
    Generally( Definitions ),
    Generate( Definitions ),
    Generation( Definitions ),
    Genetic( Definitions ),
    Gentleman( Definitions ),
    Gently( Definitions ),
    German( Definitions ),
    Gesture( Definitions ),
    Get( Definitions ),
    Ghost( Definitions ),
    Giant( Definitions ),
    Gift( Definitions ),
    Gifted( Definitions ),
    Girl( Definitions ),
    Girlfriend( Definitions ),
    Give( Definitions ),
    Given( Definitions ),
    Glad( Definitions ),
    Glance( Definitions ),
    Glass( Definitions ),
    Global( Definitions ),
    Glove( Definitions ),
    Go( Definitions ),
    Goal( Definitions ),
    God( Definitions ),
    Gold( Definitions ),
    Golden( Definitions ),
    Golf( Definitions ),
    Good( Definitions ),
    Government( Definitions ),
    Governor( Definitions ),
    Grab( Definitions ),
    Grade( Definitions ),
    Gradually( Definitions ),
    Graduate( Definitions ),
    Grain( Definitions ),
    Grand( Definitions ),
    Grandfather( Definitions ),
    Grandmother( Definitions ),
    Grant( Definitions ),
    Grass( Definitions ),
    Grave( Definitions ),
    Gray( Definitions ),
    Great( Definitions ),
    Greatest( Definitions ),
    Green( Definitions ),
    Grocery( Definitions ),
    Ground( Definitions ),
    Group( Definitions ),
    Grow( Definitions ),
    Growing( Definitions ),
    Growth( Definitions ),
    Guarantee( Definitions ),
    Guard( Definitions ),
    Guess( Definitions ),
    Guest( Definitions ),
    Guide( Definitions ),
    Guideline( Definitions ),
    Guilty( Definitions ),
    Gun( Definitions ),
    Guy( Definitions ),
    Habit( Definitions ),
    Habitat( Definitions ),
    Hair( Definitions ),
    Half( Definitions ),
    Hall( Definitions ),
    Hand( Definitions ),
    Handful( Definitions ),
    Handle( Definitions ),
    Hang( Definitions ),
    Happen( Definitions ),
    Happy( Definitions ),
    Hard( Definitions ),
    Hardly( Definitions ),
    Hat( Definitions ),
    Hate( Definitions ),
    Have( Definitions ),
    He( Definitions ),
    Head( Definitions ),
    Headline( Definitions ),
    Headquarters( Definitions ),
    Health( Definitions ),
    Healthy( Definitions ),
    Hear( Definitions ),
    Hearing( Definitions ),
    Heart( Definitions ),
    Heat( Definitions ),
    Heaven( Definitions ),
    Heavily( Definitions ),
    Heavy( Definitions ),
    Heel( Definitions ),
    Height( Definitions ),
    Helicopter( Definitions ),
    Hell( Definitions ),
    Hello( Definitions ),
    Help( Definitions ),
    Helpful( Definitions ),
    Her( Definitions ),
    Here( Definitions ),
    Heritage( Definitions ),
    Hero( Definitions ),
    Herself( Definitions ),
    Hey( Definitions ),
    Hi( Definitions ),
    Hide( Definitions ),
    High( Definitions ),
    Highlight( Definitions ),
    Highly( Definitions ),
    Highway( Definitions ),
    Hill( Definitions ),
    Him( Definitions ),
    Himself( Definitions ),
    Hip( Definitions ),
    Hire( Definitions ),
    His( Definitions ),
    Historian( Definitions ),
    Historic( Definitions ),
    Historical( Definitions ),
    History( Definitions ),
    Hit( Definitions ),
    Hold( Definitions ),
    Hole( Definitions ),
    Holiday( Definitions ),
    Holy( Definitions ),
    Home( Definitions ),
    Homeless( Definitions ),
    Honest( Definitions ),
    Honey( Definitions ),
    Honor( Definitions ),
    Hope( Definitions ),
    Horizon( Definitions ),
    Horror( Definitions ),
    Horse( Definitions ),
    Hospital( Definitions ),
    Host( Definitions ),
    Hot( Definitions ),
    Hotel( Definitions ),
    Hour( Definitions ),
    House( Definitions ),
    Household( Definitions ),
    Housing( Definitions ),
    However( Definitions ),
    Huge( Definitions ),
    Humor( Definitions ),
    Hundred( Definitions ),
    Hungry( Definitions ),
    Hunter( Definitions ),
    Hunting( Definitions ),
    Hurt( Definitions ),
    Husband( Definitions ),
    Hypothesis( Definitions ),
    I( Definitions ),
    Ice( Definitions ),
    Idea( Definitions ),
    Ideal( Definitions ),
    Identification( Definitions ),
    Identify( Definitions ),
    Ie( Definitions ),
    If( Definitions ),
    Ignore( Definitions ),
    Ill( Definitions ),
    Illegal( Definitions ),
    Illness( Definitions ),
    Illustrate( Definitions ),
    Image( Definitions ),
    Imagination( Definitions ),
    Imagine( Definitions ),
    Immediate( Definitions ),
    Immediately( Definitions ),
    Immigrant( Definitions ),
    Immigration( Definitions ),
    Impact( Definitions ),
    Implement( Definitions ),
    Implication( Definitions ),
    Imply( Definitions ),
    Importance( Definitions ),
    Important( Definitions ),
    Impose( Definitions ),
    Impossible( Definitions ),
    Impress( Definitions ),
    Impression( Definitions ),
    Impressive( Definitions ),
    Improve( Definitions ),
    Improvement( Definitions ),
    Incentive( Definitions ),
    Incident( Definitions ),
    Include( Definitions ),
    Including( Definitions ),
    Income( Definitions ),
    Incorporate( Definitions ),
    Increase( Definitions ),
    Increased( Definitions ),
    Increasing( Definitions ),
    Increasingly( Definitions ),
    Incredible( Definitions ),
    Indeed( Definitions ),
    Independence( Definitions ),
    Independent( Definitions ),
    Index( Definitions ),
    Indian( Definitions ),
    Indication( Definitions ),
    Individual( Definitions ),
    Industrial( Definitions ),
    Industry( Definitions ),
    Infant( Definitions ),
    Infection( Definitions ),
    Inflation( Definitions ),
    Influence( Definitions ),
    Inform( Definitions ),
    Information( Definitions ),
    Ingredient( Definitions ),
    Initial( Definitions ),
    Initially( Definitions ),
    Initiative( Definitions ),
    Injury( Definitions ),
    Inner( Definitions ),
    Innocent( Definitions ),
    Inquiry( Definitions ),
    Insight( Definitions ),
    Insist( Definitions ),
    Inspire( Definitions ),
    Install( Definitions ),
    Instead( Definitions ),
    Institution( Definitions ),
    Institutional( Definitions ),
    Instruction( Definitions ),
    Instructor( Definitions ),
    Instrument( Definitions ),
    Insurance( Definitions ),
    Intellectual( Definitions ),
    Intelligence( Definitions ),
    Intend( Definitions ),
    Intense( Definitions ),
    Intensity( Definitions ),
    Intention( Definitions ),
    Interaction( Definitions ),
    Interest( Definitions ),
    Interested( Definitions ),
    Interesting( Definitions ),
    Internal( Definitions ),
    International( Definitions ),
    Internet( Definitions ),
    Interpret( Definitions ),
    Interpretation( Definitions ),
    Intervention( Definitions ),
    Interview( Definitions ),
    Into( Definitions ),
    Introduce( Definitions ),
    Introduction( Definitions ),
    Invasion( Definitions ),
    Invest( Definitions ),
    Investigate( Definitions ),
    Investigation( Definitions ),
    Investigator( Definitions ),
    Investment( Definitions ),
    Investor( Definitions ),
    Invite( Definitions ),
    Involve( Definitions ),
    Involved( Definitions ),
    Involvement( Definitions ),
    Iraqi( Definitions ),
    Irish( Definitions ),
    Iron( Definitions ),
    Islamic( Definitions ),
    Island( Definitions ),
    Israeli( Definitions ),
    Issue( Definitions ),
    Italian( Definitions ),
    Item( Definitions ),
    Its( Definitions ),
    Itself( Definitions ),
    Jacket( Definitions ),
    Jail( Definitions ),
    Japanese( Definitions ),
    Jet( Definitions ),
    Jew( Definitions ),
    Jewish( Definitions ),
    Job( Definitions ),
    Join( Definitions ),
    Joint( Definitions ),
    Joke( Definitions ),
    Journal( Definitions ),
    Journalist( Definitions ),
    Journey( Definitions ),
    Joy( Definitions ),
    Judge( Definitions ),
    Judgment( Definitions ),
    Juice( Definitions ),
    Jump( Definitions ),
    Junior( Definitions ),
    Jury( Definitions ),
    Just( Definitions ),
    Justice( Definitions ),
    Justify( Definitions ),
    Keep( Definitions ),
    Key( Definitions ),
    Kick( Definitions ),
    Kid( Definitions ),
    Kill( Definitions ),
    Killer( Definitions ),
    Killing( Definitions ),
    Kind( Definitions ),
    King( Definitions ),
    Kiss( Definitions ),
    Kitchen( Definitions ),
    Knee( Definitions ),
    Knife( Definitions ),
    Knock( Definitions ),
    Know( Definitions ),
    Knowledge( Definitions ),
    Lab( Definitions ),
    Label( Definitions ),
    Labor( Definitions ),
    Laboratory( Definitions ),
    Lack( Definitions ),
    Lady( Definitions ),
    Lake( Definitions ),
    Land( Definitions ),
    Landscape( Definitions ),
    Language( Definitions ),
    Lap( Definitions ),
    Large( Definitions ),
    Largely( Definitions ),
    Last( Definitions ),
    Late( Definitions ),
    Later( Definitions ),
    Latin( Definitions ),
    Latter( Definitions ),
    Laugh( Definitions ),
    Launch( Definitions ),
    Law( Definitions ),
    Lawn( Definitions ),
    Lawsuit( Definitions ),
    Lawyer( Definitions ),
    Lay( Definitions ),
    Layer( Definitions ),
    Lead( Definitions ),
    Leader( Definitions ),
    Leadership( Definitions ),
    Leading( Definitions ),
    Leaf( Definitions ),
    League( Definitions ),
    Lean( Definitions ),
    Learn( Definitions ),
    Learning( Definitions ),
    Least( Definitions ),
    Leather( Definitions ),
    Leave( Definitions ),
    Left( Definitions ),
    Leg( Definitions ),
    Legacy( Definitions ),
    Legal( Definitions ),
    Legend( Definitions ),
    Legislation( Definitions ),
    Legitimate( Definitions ),
    Lemon( Definitions ),
    Length( Definitions ),
    Less( Definitions ),
    Lesson( Definitions ),
    Let( Definitions ),
    Letter( Definitions ),
    Level( Definitions ),
    Liberal( Definitions ),
    Library( Definitions ),
    License( Definitions ),
    Lie( Definitions ),
    Life( Definitions ),
    Lifestyle( Definitions ),
    Lifetime( Definitions ),
    Lift( Definitions ),
    Light( Definitions ),
    Like( Definitions ),
    Likely( Definitions ),
    Limit( Definitions ),
    Limitation( Definitions ),
    Limited( Definitions ),
    Line( Definitions ),
    Link( Definitions ),
    Lip( Definitions ),
    List( Definitions ),
    Listen( Definitions ),
    Literally( Definitions ),
    Literary( Definitions ),
    Literature( Definitions ),
    Little( Definitions ),
    Live( Definitions ),
    Living( Definitions ),
    Load( Definitions ),
    Loan( Definitions ),
    Local( Definitions ),
    Locate( Definitions ),
    Location( Definitions ),
    Lock( Definitions ),
    Long( Definitions ),
    LongTerm( Definitions ),
    Look( Definitions ),
    Loose( Definitions ),
    Lose( Definitions ),
    Loss( Definitions ),
    Lost( Definitions ),
    Lot( Definitions ),
    Lots( Definitions ),
    Loud( Definitions ),
    Love( Definitions ),
    Lovely( Definitions ),
    Lover( Definitions ),
    Low( Definitions ),
    Lower( Definitions ),
    Luck( Definitions ),
    Lucky( Definitions ),
    Lunch( Definitions ),
    Lung( Definitions ),
    Machine( Definitions ),
    Mad( Definitions ),
    Magazine( Definitions ),
    Mail( Definitions ),
    Main( Definitions ),
    Mainly( Definitions ),
    Maintain( Definitions ),
    Maintenance( Definitions ),
    Major( Definitions ),
    Majority( Definitions ),
    Make( Definitions ),
    Maker( Definitions ),
    Makeup( Definitions ),
    Male( Definitions ),
    Mall( Definitions ),
    Man( Definitions ),
    Manage( Definitions ),
    Management( Definitions ),
    Manager( Definitions ),
    Manner( Definitions ),
    Manufacturer( Definitions ),
    Manufacturing( Definitions ),
    Many( Definitions ),
    Map( Definitions ),
    Margin( Definitions ),
    Mark( Definitions ),
    Market( Definitions ),
    Marketing( Definitions ),
    Marriage( Definitions ),
    Married( Definitions ),
    Marry( Definitions ),
    Mask( Definitions ),
    Mass( Definitions ),
    Massive( Definitions ),
    Master( Definitions ),
    Match( Definitions ),
    Material( Definitions ),
    Math( Definitions ),
    Matter( Definitions ),
    May( Definitions ),
    Maybe( Definitions ),
    Mayor( Definitions ),
    Me( Definitions ),
    Meal( Definitions ),
    Mean( Definitions ),
    Meaning( Definitions ),
    Meanwhile( Definitions ),
    Measure( Definitions ),
    Measurement( Definitions ),
    Meat( Definitions ),
    Mechanism( Definitions ),
    Media( Definitions ),
    Medical( Definitions ),
    Medication( Definitions ),
    Medicine( Definitions ),
    Medium( Definitions ),
    Meet( Definitions ),
    Meeting( Definitions ),
    Member( Definitions ),
    Membership( Definitions ),
    Memory( Definitions ),
    Mental( Definitions ),
    Mention( Definitions ),
    Menu( Definitions ),
    Mere( Definitions ),
    Merely( Definitions ),
    Mess( Definitions ),
    Message( Definitions ),
    Metal( Definitions ),
    Meter( Definitions ),
    Method( Definitions ),
    Mexican( Definitions ),
    Middle( Definitions ),
    Might( Definitions ),
    Military( Definitions ),
    Milk( Definitions ),
    Million( Definitions ),
    Mind( Definitions ),
    Mine( Definitions ),
    Minister( Definitions ),
    Minor( Definitions ),
    Minority( Definitions ),
    Minute( Definitions ),
    Miracle( Definitions ),
    Mirror( Definitions ),
    Miss( Definitions ),
    Missile( Definitions ),
    Mission( Definitions ),
    Mistake( Definitions ),
    Mix( Definitions ),
    Mixture( Definitions ),
    MmHmm( Definitions ),
    Mode( Definitions ),
    Model( Definitions ),
    Moderate( Definitions ),
    Modern( Definitions ),
    Modest( Definitions ),
    Mom( Definitions ),
    Moment( Definitions ),
    Money( Definitions ),
    Monitor( Definitions ),
    Month( Definitions ),
    Mood( Definitions ),
    Moon( Definitions ),
    Moral( Definitions ),
    More( Definitions ),
    Moreover( Definitions ),
    Morning( Definitions ),
    Mortgage( Definitions ),
    Most( Definitions ),
    Mostly( Definitions ),
    Mother( Definitions ),
    Motion( Definitions ),
    Motivation( Definitions ),
    Motor( Definitions ),
    Mount( Definitions ),
    Mountain( Definitions ),
    Mouse( Definitions ),
    Mouth( Definitions ),
    Move( Definitions ),
    Movement( Definitions ),
    Movie( Definitions ),
    Mr( Definitions ),
    Mrs( Definitions ),
    Ms( Definitions ),
    Much( Definitions ),
    Multiple( Definitions ),
    Murder( Definitions ),
    Muscle( Definitions ),
    Museum( Definitions ),
    Music( Definitions ),
    Musical( Definitions ),
    Musician( Definitions ),
    Muslim( Definitions ),
    Must( Definitions ),
    Mutual( Definitions ),
    My( Definitions ),
    Myself( Definitions ),
    Mystery( Definitions ),
    Myth( Definitions ),
    Naked( Definitions ),
    Name( Definitions ),
    Narrative( Definitions ),
    Narrow( Definitions ),
    Nation( Definitions ),
    National( Definitions ),
    Native( Definitions ),
    Natural( Definitions ),
    Naturally( Definitions ),
    Nature( Definitions ),
    Near( Definitions ),
    Nearby( Definitions ),
    Nearly( Definitions ),
    Necessarily( Definitions ),
    Necessary( Definitions ),
    Neck( Definitions ),
    Need( Definitions ),
    Negative( Definitions ),
    Negotiate( Definitions ),
    Negotiation( Definitions ),
    Neighbor( Definitions ),
    Neighborhood( Definitions ),
    Neither( Definitions ),
    Nerve( Definitions ),
    Nervous( Definitions ),
    Net( Definitions ),
    Network( Definitions ),
    Nevertheless( Definitions ),
    New( Definitions ),
    Newly( Definitions ),
    News( Definitions ),
    Newspaper( Definitions ),
    Next( Definitions ),
    Nice( Definitions ),
    Night( Definitions ),
    Nine( Definitions ),
    No( Definitions ),
    Nobody( Definitions ),
    Nod( Definitions ),
    Noise( Definitions ),
    Nomination( Definitions ),
    None( Definitions ),
    Nonetheless( Definitions ),
    Normal( Definitions ),
    Normally( Definitions ),
    North( Definitions ),
    Northern( Definitions ),
    Nose( Definitions ),
    Not( Definitions ),
    Note( Definitions ),
    Nothing( Definitions ),
    Notice( Definitions ),
    Notion( Definitions ),
    Novel( Definitions ),
    Now( Definitions ),
    Nowhere( Definitions ),
    Nuclear( Definitions ),
    Number( Definitions ),
    Numerous( Definitions ),
    Nurse( Definitions ),
    Nut( Definitions ),
    Objective( Definitions ),
    Obligation( Definitions ),
    Observation( Definitions ),
    Observe( Definitions ),
    Observer( Definitions ),
    Obtain( Definitions ),
    Obvious( Definitions ),
    Obviously( Definitions ),
    Occasion( Definitions ),
    Occasionally( Definitions ),
    Occupation( Definitions ),
    Occupy( Definitions ),
    Occur( Definitions ),
    Ocean( Definitions ),
    Odd( Definitions ),
    Odds( Definitions ),
    Off( Definitions ),
    Offensive( Definitions ),
    Offer( Definitions ),
    Office( Definitions ),
    Officer( Definitions ),
    Official( Definitions ),
    Often( Definitions ),
    Oh( Definitions ),
    Oil( Definitions ),
    Ok( Definitions ),
    Okay( Definitions ),
    Old( Definitions ),
    Olympic( Definitions ),
    On( Definitions ),
    Once( Definitions ),
    One( Definitions ),
    Ongoing( Definitions ),
    Onion( Definitions ),
    Online( Definitions ),
    Only( Definitions ),
    Onto( Definitions ),
    Open( Definitions ),
    Opening( Definitions ),
    Operate( Definitions ),
    Operating( Definitions ),
    Operation( Definitions ),
    Operator( Definitions ),
    Opinion( Definitions ),
    Opponent( Definitions ),
    Opportunity( Definitions ),
    Oppose( Definitions ),
    Opposite( Definitions ),
    Opposition( Definitions ),
    Option( Definitions ),
    Orange( Definitions ),
    Order( Definitions ),
    Ordinary( Definitions ),
    Organic( Definitions ),
    Organization( Definitions ),
    Organize( Definitions ),
    Orientation( Definitions ),
    Origin( Definitions ),
    Original( Definitions ),
    Originally( Definitions ),
    Other( Definitions ),
    Others( Definitions ),
    Otherwise( Definitions ),
    Ought( Definitions ),
    Our( Definitions ),
    Ourselves( Definitions ),
    Out( Definitions ),
    Outcome( Definitions ),
    Outside( Definitions ),
    Oven( Definitions ),
    Over( Definitions ),
    Overall( Definitions ),
    Overcome( Definitions ),
    Overlook( Definitions ),
    Owe( Definitions ),
    Own( Definitions ),
    Owner( Definitions ),
    Pace( Definitions ),
    Pack( Definitions ),
    Package( Definitions ),
    Page( Definitions ),
    Pain( Definitions ),
    Painful( Definitions ),
    Paint( Definitions ),
    Painter( Definitions ),
    Painting( Definitions ),
    Pair( Definitions ),
    Pale( Definitions ),
    Palestinian( Definitions ),
    Palm( Definitions ),
    Pan( Definitions ),
    Panel( Definitions ),
    Pant( Definitions ),
    Paper( Definitions ),
    Parent( Definitions ),
    Park( Definitions ),
    Parking( Definitions ),
    Part( Definitions ),
    Participant( Definitions ),
    Participate( Definitions ),
    Participation( Definitions ),
    Particular( Definitions ),
    Particularly( Definitions ),
    Partly( Definitions ),
    Partner( Definitions ),
    Partnership( Definitions ),
    Party( Definitions ),
    Pass( Definitions ),
    Passage( Definitions ),
    Passenger( Definitions ),
    Passion( Definitions ),
    Past( Definitions ),
    Patch( Definitions ),
    Path( Definitions ),
    Patient( Definitions ),
    Pattern( Definitions ),
    Pause( Definitions ),
    Pay( Definitions ),
    Payment( Definitions ),
    Pc( Definitions ),
    Peace( Definitions ),
    Peak( Definitions ),
    Peer( Definitions ),
    Penalty( Definitions ),
    Pepper( Definitions ),
    Per( Definitions ),
    Perceive( Definitions ),
    Percentage( Definitions ),
    Perception( Definitions ),
    Perfect( Definitions ),
    Perfectly( Definitions ),
    Perform( Definitions ),
    Performance( Definitions ),
    Perhaps( Definitions ),
    Period( Definitions ),
    Permanent( Definitions ),
    Permission( Definitions ),
    Permit( Definitions ),
    Personal( Definitions ),
    Personality( Definitions ),
    Personally( Definitions ),
    Personnel( Definitions ),
    Perspective( Definitions ),
    Persuade( Definitions ),
    Pet( Definitions ),
    Phase( Definitions ),
    Phenomenon( Definitions ),
    Philosophy( Definitions ),
    Phone( Definitions ),
    Photo( Definitions ),
    Photograph( Definitions ),
    Photographer( Definitions ),
    Phrase( Definitions ),
    Physical( Definitions ),
    Physically( Definitions ),
    Physician( Definitions ),
    Piano( Definitions ),
    Pick( Definitions ),
    Picture( Definitions ),
    Pie( Definitions ),
    Piece( Definitions ),
    Pile( Definitions ),
    Pilot( Definitions ),
    Pine( Definitions ),
    Pink( Definitions ),
    Pipe( Definitions ),
    Pitch( Definitions ),
    Plan( Definitions ),
    Plane( Definitions ),
    Planet( Definitions ),
    Planning( Definitions ),
    Plant( Definitions ),
    Plastic( Definitions ),
    Plate( Definitions ),
    Platform( Definitions ),
    Play( Definitions ),
    Player( Definitions ),
    Please( Definitions ),
    Pleasure( Definitions ),
    Plenty( Definitions ),
    Plot( Definitions ),
    Plus( Definitions ),
    Pm( Definitions ),
    Pocket( Definitions ),
    Poem( Definitions ),
    Poet( Definitions ),
    Poetry( Definitions ),
    Point( Definitions ),
    Pole( Definitions ),
    Police( Definitions ),
    Policy( Definitions ),
    Political( Definitions ),
    Politically( Definitions ),
    Politician( Definitions ),
    Politics( Definitions ),
    Poll( Definitions ),
    Pollution( Definitions ),
    Pool( Definitions ),
    Poor( Definitions ),
    Pop( Definitions ),
    Popular( Definitions ),
    Population( Definitions ),
    Porch( Definitions ),
    Port( Definitions ),
    Portion( Definitions ),
    Portrait( Definitions ),
    Portray( Definitions ),
    Pose( Definitions ),
    Position( Definitions ),
    Positive( Definitions ),
    Possess( Definitions ),
    Possibility( Definitions ),
    Possible( Definitions ),
    Possibly( Definitions ),
    Post( Definitions ),
    Pot( Definitions ),
    Potato( Definitions ),
    Potential( Definitions ),
    Potentially( Definitions ),
    Pound( Definitions ),
    Pour( Definitions ),
    Poverty( Definitions ),
    Powder( Definitions ),
    Power( Definitions ),
    Powerful( Definitions ),
    Practical( Definitions ),
    Practice( Definitions ),
    Pray( Definitions ),
    Prayer( Definitions ),
    Precisely( Definitions ),
    Predict( Definitions ),
    Prefer( Definitions ),
    Preference( Definitions ),
    Pregnancy( Definitions ),
    Pregnant( Definitions ),
    Preparation( Definitions ),
    Prepare( Definitions ),
    Prescription( Definitions ),
    Presence( Definitions ),
    Present( Definitions ),
    Presentation( Definitions ),
    Preserve( Definitions ),
    President( Definitions ),
    Presidential( Definitions ),
    Press( Definitions ),
    Pressure( Definitions ),
    Pretend( Definitions ),
    Pretty( Definitions ),
    Prevent( Definitions ),
    Previous( Definitions ),
    Previously( Definitions ),
    Price( Definitions ),
    Pride( Definitions ),
    Priest( Definitions ),
    Primarily( Definitions ),
    Primary( Definitions ),
    Prime( Definitions ),
    Principal( Definitions ),
    Principle( Definitions ),
    Print( Definitions ),
    Prior( Definitions ),
    Priority( Definitions ),
    Prison( Definitions ),
    Prisoner( Definitions ),
    Privacy( Definitions ),
    Private( Definitions ),
    Probably( Definitions ),
    Problem( Definitions ),
    Procedure( Definitions ),
    Proceed( Definitions ),
    Process( Definitions ),
    Produce( Definitions ),
    Producer( Definitions ),
    Product( Definitions ),
    Production( Definitions ),
    Profession( Definitions ),
    Professional( Definitions ),
    Professor( Definitions ),
    Profile( Definitions ),
    Profit( Definitions ),
    Program( Definitions ),
    Progress( Definitions ),
    Prominent( Definitions ),
    Promise( Definitions ),
    Promote( Definitions ),
    Prompt( Definitions ),
    Proof( Definitions ),
    Proper( Definitions ),
    Properly( Definitions ),
    Property( Definitions ),
    Proportion( Definitions ),
    Proposal( Definitions ),
    Propose( Definitions ),
    Proposed( Definitions ),
    Prosecutor( Definitions ),
    Prospect( Definitions ),
    Protect( Definitions ),
    Protection( Definitions ),
    Protein( Definitions ),
    Protest( Definitions ),
    Proud( Definitions ),
    Prove( Definitions ),
    Provide( Definitions ),
    Provider( Definitions ),
    Province( Definitions ),
    Provision( Definitions ),
    Psychological( Definitions ),
    Psychologist( Definitions ),
    Psychology( Definitions ),
    Public( Definitions ),
    Publication( Definitions ),
    Publicly( Definitions ),
    Publish( Definitions ),
    Publisher( Definitions ),
    Pull( Definitions ),
    Punishment( Definitions ),
    Purchase( Definitions ),
    Pure( Definitions ),
    Purpose( Definitions ),
    Pursue( Definitions ),
    Push( Definitions ),
    Put( Definitions ),
    Qualify( Definitions ),
    Quality( Definitions ),
    Quarter( Definitions ),
    Quarterback( Definitions ),
    Quick( Definitions ),
    Quickly( Definitions ),
    Quiet( Definitions ),
    Quietly( Definitions ),
    Quit( Definitions ),
    Quite( Definitions ),
    Quote( Definitions ),
    Race( Definitions ),
    Racial( Definitions ),
    Radical( Definitions ),
    Radio( Definitions ),
    Rail( Definitions ),
    Rain( Definitions ),
    Raise( Definitions ),
    Range( Definitions ),
    Rank( Definitions ),
    Rapid( Definitions ),
    Rapidly( Definitions ),
    Rare( Definitions ),
    Rarely( Definitions ),
    Rate( Definitions ),
    Rather( Definitions ),
    Rating( Definitions ),
    Ratio( Definitions ),
    Raw( Definitions ),
    Reach( Definitions ),
    React( Definitions ),
    Reaction( Definitions ),
    Read( Definitions ),
    Reader( Definitions ),
    Reading( Definitions ),
    Ready( Definitions ),
    Real( Definitions ),
    Reality( Definitions ),
    Realize( Definitions ),
    Really( Definitions ),
    Reason( Definitions ),
    Reasonable( Definitions ),
    Recall( Definitions ),
    Receive( Definitions ),
    Recent( Definitions ),
    Recently( Definitions ),
    Recipe( Definitions ),
    Recognition( Definitions ),
    Recognize( Definitions ),
    Recommend( Definitions ),
    Recommendation( Definitions ),
    Record( Definitions ),
    Recording( Definitions ),
    Recover( Definitions ),
    Recovery( Definitions ),
    Recruit( Definitions ),
    Red( Definitions ),
    Reduce( Definitions ),
    Reduction( Definitions ),
    Reflect( Definitions ),
    Reflection( Definitions ),
    Reform( Definitions ),
    Refugee( Definitions ),
    Refuse( Definitions ),
    Regard( Definitions ),
    Regarding( Definitions ),
    Regardless( Definitions ),
    Regime( Definitions ),
    Region( Definitions ),
    Regional( Definitions ),
    Register( Definitions ),
    Regular( Definitions ),
    Regularly( Definitions ),
    Regulate( Definitions ),
    Regulation( Definitions ),
    Reinforce( Definitions ),
    Reject( Definitions ),
    Relate( Definitions ),
    Relation( Definitions ),
    Relationship( Definitions ),
    Relatively( Definitions ),
    Relax( Definitions ),
    Release( Definitions ),
    Relevant( Definitions ),
    Relief( Definitions ),
    Religion( Definitions ),
    Religious( Definitions ),
    Rely( Definitions ),
    Remain( Definitions ),
    Remaining( Definitions ),
    Remarkable( Definitions ),
    Remember( Definitions ),
    Remind( Definitions ),
    Remote( Definitions ),
    Remove( Definitions ),
    Repeat( Definitions ),
    Repeatedly( Definitions ),
    Replace( Definitions ),
    Reply( Definitions ),
    Report( Definitions ),
    Reporter( Definitions ),
    Represent( Definitions ),
    Representation( Definitions ),
    Representative( Definitions ),
    Republican( Definitions ),
    Reputation( Definitions ),
    Request( Definitions ),
    Require( Definitions ),
    Requirement( Definitions ),
    Research( Definitions ),
    Researcher( Definitions ),
    Resemble( Definitions ),
    Reservation( Definitions ),
    Resident( Definitions ),
    Resist( Definitions ),
    Resistance( Definitions ),
    Resolution( Definitions ),
    Resolve( Definitions ),
    Resort( Definitions ),
    Resource( Definitions ),
    Respect( Definitions ),
    Respond( Definitions ),
    Respondent( Definitions ),
    Response( Definitions ),
    Responsibility( Definitions ),
    Responsible( Definitions ),
    Rest( Definitions ),
    Restaurant( Definitions ),
    Restore( Definitions ),
    Restriction( Definitions ),
    Result( Definitions ),
    Retain( Definitions ),
    Retire( Definitions ),
    Retirement( Definitions ),
    Return( Definitions ),
    Reveal( Definitions ),
    Revenue( Definitions ),
    Review( Definitions ),
    Revolution( Definitions ),
    Rhythm( Definitions ),
    Rice( Definitions ),
    Rich( Definitions ),
    Rid( Definitions ),
    Ride( Definitions ),
    Rifle( Definitions ),
    Right( Definitions ),
    Ring( Definitions ),
    Rise( Definitions ),
    Risk( Definitions ),
    River( Definitions ),
    Road( Definitions ),
    Rock( Definitions ),
    Role( Definitions ),
    Roll( Definitions ),
    Romantic( Definitions ),
    Roof( Definitions ),
    Room( Definitions ),
    Root( Definitions ),
    Rope( Definitions ),
    Rose( Definitions ),
    Rough( Definitions ),
    Roughly( Definitions ),
    Round( Definitions ),
    Route( Definitions ),
    Routine( Definitions ),
    Row( Definitions ),
    Rub( Definitions ),
    Rule( Definitions ),
    Run( Definitions ),
    Running( Definitions ),
    Rural( Definitions ),
    Rush( Definitions ),
    Russian( Definitions ),
    Sacred( Definitions ),
    Sad( Definitions ),
    Safe( Definitions ),
    Safety( Definitions ),
    Sake( Definitions ),
    Salad( Definitions ),
    Salary( Definitions ),
    Sale( Definitions ),
    Sales( Definitions ),
    Salt( Definitions ),
    Same( Definitions ),
    Sample( Definitions ),
    Sanction( Definitions ),
    Sand( Definitions ),
    Satellite( Definitions ),
    Satisfaction( Definitions ),
    Satisfy( Definitions ),
    Sauce( Definitions ),
    Save( Definitions ),
    Saving( Definitions ),
    Say( Definitions ),
    Scale( Definitions ),
    Scandal( Definitions ),
    Scared( Definitions ),
    Scenario( Definitions ),
    Scene( Definitions ),
    Schedule( Definitions ),
    Scheme( Definitions ),
    Scholar( Definitions ),
    Scholarship( Definitions ),
    School( Definitions ),
    Science( Definitions ),
    Scientific( Definitions ),
    Scientist( Definitions ),
    Scope( Definitions ),
    Score( Definitions ),
    Scream( Definitions ),
    Screen( Definitions ),
    Script( Definitions ),
    Sea( Definitions ),
    Search( Definitions ),
    Season( Definitions ),
    Seat( Definitions ),
    Second( Definitions ),
    Secret( Definitions ),
    Secretary( Definitions ),
    Section( Definitions ),
    Sector( Definitions ),
    Secure( Definitions ),
    Security( Definitions ),
    See( Definitions ),
    Seed( Definitions ),
    Seek( Definitions ),
    Seem( Definitions ),
    Segment( Definitions ),
    Seize( Definitions ),
    Select( Definitions ),
    Selection( Definitions ),
    Sell( Definitions ),
    Senate( Definitions ),
    Senator( Definitions ),
    Send( Definitions ),
    Senior( Definitions ),
    Sense( Definitions ),
    Sensitive( Definitions ),
    Sentence( Definitions ),
    Separate( Definitions ),
    Sequence( Definitions ),
    Series( Definitions ),
    Serious( Definitions ),
    Seriously( Definitions ),
    Serve( Definitions ),
    Service( Definitions ),
    Session( Definitions ),
    Set( Definitions ),
    Setting( Definitions ),
    Settle( Definitions ),
    Settlement( Definitions ),
    Seven( Definitions ),
    Several( Definitions ),
    Severe( Definitions ),
    Sex( Definitions ),
    Sexual( Definitions ),
    Shade( Definitions ),
    Shadow( Definitions ),
    Shake( Definitions ),
    Shall( Definitions ),
    Shape( Definitions ),
    Share( Definitions ),
    Sharp( Definitions ),
    She( Definitions ),
    Sheet( Definitions ),
    Shelf( Definitions ),
    Shell( Definitions ),
    Shelter( Definitions ),
    Shift( Definitions ),
    Shine( Definitions ),
    Ship( Definitions ),
    Shirt( Definitions ),
    Shit( Definitions ),
    Shock( Definitions ),
    Shoe( Definitions ),
    Shoot( Definitions ),
    Shooting( Definitions ),
    Shop( Definitions ),
    Shopping( Definitions ),
    Shore( Definitions ),
    Short( Definitions ),
    Shortly( Definitions ),
    Shot( Definitions ),
    Should( Definitions ),
    Shoulder( Definitions ),
    Shout( Definitions ),
    Show( Definitions ),
    Shower( Definitions ),
    Shrug( Definitions ),
    Shut( Definitions ),
    Sick( Definitions ),
    Side( Definitions ),
    Sigh( Definitions ),
    Sight( Definitions ),
    Sign( Definitions ),
    Signal( Definitions ),
    Significance( Definitions ),
    Significant( Definitions ),
    Significantly( Definitions ),
    Silence( Definitions ),
    Silent( Definitions ),
    Silver( Definitions ),
    Similar( Definitions ),
    Similarly( Definitions ),
    Simple( Definitions ),
    Simply( Definitions ),
    Sin( Definitions ),
    Since( Definitions ),
    Sing( Definitions ),
    Singer( Definitions ),
    Single( Definitions ),
    Sink( Definitions ),
    Sir( Definitions ),
    Sister( Definitions ),
    Sit( Definitions ),
    Site( Definitions ),
    Situation( Definitions ),
    Six( Definitions ),
    Size( Definitions ),
    Ski( Definitions ),
    Skill( Definitions ),
    Skin( Definitions ),
    Sky( Definitions ),
    Slave( Definitions ),
    Sleep( Definitions ),
    Slice( Definitions ),
    Slide( Definitions ),
    Slight( Definitions ),
    Slightly( Definitions ),
    Slip( Definitions ),
    Slow( Definitions ),
    Slowly( Definitions ),
    Small( Definitions ),
    Smart( Definitions ),
    Smell( Definitions ),
    Smile( Definitions ),
    Smoke( Definitions ),
    Smooth( Definitions ),
    Snap( Definitions ),
    Snow( Definitions ),
    So( Definitions ),
    SoCalled( Definitions ),
    Soccer( Definitions ),
    Social( Definitions ),
    Society( Definitions ),
    Soft( Definitions ),
    Software( Definitions ),
    Soil( Definitions ),
    Solar( Definitions ),
    Soldier( Definitions ),
    Solid( Definitions ),
    Solution( Definitions ),
    Solve( Definitions ),
    Some( Definitions ),
    Somebody( Definitions ),
    Somehow( Definitions ),
    Something( Definitions ),
    Sometimes( Definitions ),
    Somewhat( Definitions ),
    Somewhere( Definitions ),
    Son( Definitions ),
    Song( Definitions ),
    Soon( Definitions ),
    Sophisticated( Definitions ),
    Sorry( Definitions ),
    Sort( Definitions ),
    Soul( Definitions ),
    Sound( Definitions ),
    Soup( Definitions ),
    Source( Definitions ),
    South( Definitions ),
    Southern( Definitions ),
    Soviet( Definitions ),
    Space( Definitions ),
    Spanish( Definitions ),
    Speak( Definitions ),
    Speaker( Definitions ),
    Special( Definitions ),
    Specialist( Definitions ),
    Species( Definitions ),
    Specific( Definitions ),
    Specifically( Definitions ),
    Speech( Definitions ),
    Speed( Definitions ),
    Spend( Definitions ),
    Spending( Definitions ),
    Spin( Definitions ),
    Spirit( Definitions ),
    Spiritual( Definitions ),
    Split( Definitions ),
    Spokesman( Definitions ),
    Sport( Definitions ),
    Spot( Definitions ),
    Spread( Definitions ),
    Spring( Definitions ),
    Square( Definitions ),
    Squeeze( Definitions ),
    Stability( Definitions ),
    Stable( Definitions ),
    Staff( Definitions ),
    Stage( Definitions ),
    Stair( Definitions ),
    Stake( Definitions ),
    Stand( Definitions ),
    Standard( Definitions ),
    Standing( Definitions ),
    Star( Definitions ),
    Stare( Definitions ),
    Start( Definitions ),
    State( Definitions ),
    Statement( Definitions ),
    Station( Definitions ),
    Statistics( Definitions ),
    Status( Definitions ),
    Stay( Definitions ),
    Steady( Definitions ),
    Steal( Definitions ),
    Steel( Definitions ),
    Step( Definitions ),
    Stick( Definitions ),
    Still( Definitions ),
    Stir( Definitions ),
    Stock( Definitions ),
    Stomach( Definitions ),
    Stone( Definitions ),
    Stop( Definitions ),
    Storage( Definitions ),
    Store( Definitions ),
    Storm( Definitions ),
    Story( Definitions ),
    Straight( Definitions ),
    Strange( Definitions ),
    Stranger( Definitions ),
    Strategic( Definitions ),
    Strategy( Definitions ),
    Stream( Definitions ),
    Street( Definitions ),
    Strength( Definitions ),
    Strengthen( Definitions ),
    Stress( Definitions ),
    Stretch( Definitions ),
    Strike( Definitions ),
    String( Definitions ),
    Strip( Definitions ),
    Stroke( Definitions ),
    Strong( Definitions ),
    Strongly( Definitions ),
    Structure( Definitions ),
    Struggle( Definitions ),
    Student( Definitions ),
    Studio( Definitions ),
    Study( Definitions ),
    Stuff( Definitions ),
    Stupid( Definitions ),
    Style( Definitions ),
    Submit( Definitions ),
    Subsequent( Definitions ),
    Substance( Definitions ),
    Substantial( Definitions ),
    Succeed( Definitions ),
    Success( Definitions ),
    Successful( Definitions ),
    Successfully( Definitions ),
    Such( Definitions ),
    Sudden( Definitions ),
    Suddenly( Definitions ),
    Sue( Definitions ),
    Suffer( Definitions ),
    Sufficient( Definitions ),
    Sugar( Definitions ),
    Suggest( Definitions ),
    Suggestion( Definitions ),
    Suicide( Definitions ),
    Suit( Definitions ),
    Summer( Definitions ),
    Summit( Definitions ),
    Sun( Definitions ),
    Super( Definitions ),
    Supply( Definitions ),
    Support( Definitions ),
    Supporter( Definitions ),
    Suppose( Definitions ),
    Supposed( Definitions ),
    Supreme( Definitions ),
    Sure( Definitions ),
    Surely( Definitions ),
    Surface( Definitions ),
    Surgery( Definitions ),
    Surprise( Definitions ),
    Surprised( Definitions ),
    Surprising( Definitions ),
    Surprisingly( Definitions ),
    Surround( Definitions ),
    Survey( Definitions ),
    Survival( Definitions ),
    Survive( Definitions ),
    Survivor( Definitions ),
    Suspect( Definitions ),
    Sustain( Definitions ),
    Swear( Definitions ),
    Sweep( Definitions ),
    Sweet( Definitions ),
    Swim( Definitions ),
    Swing( Definitions ),
    Switch( Definitions ),
    Symbol( Definitions ),
    Symptom( Definitions ),
    System( Definitions ),
    Table( Definitions ),
    Tablespoon( Definitions ),
    Tactic( Definitions ),
    Tail( Definitions ),
    Take( Definitions ),
    Tale( Definitions ),
    Talent( Definitions ),
    Talk( Definitions ),
    Tall( Definitions ),
    Tank( Definitions ),
    Tap( Definitions ),
    Tape( Definitions ),
    Target( Definitions ),
    Task( Definitions ),
    Taste( Definitions ),
    Tax( Definitions ),
    Taxpayer( Definitions ),
    Tea( Definitions ),
    Teach( Definitions ),
    Teacher( Definitions ),
    Teaching( Definitions ),
    Team( Definitions ),
    Tear( Definitions ),
    Teaspoon( Definitions ),
    Technical( Definitions ),
    Technique( Definitions ),
    Technology( Definitions ),
    Teen( Definitions ),
    Teenager( Definitions ),
    Telephone( Definitions ),
    Telescope( Definitions ),
    Television( Definitions ),
    Tell( Definitions ),
    Temperature( Definitions ),
    Temporary( Definitions ),
    Ten( Definitions ),
    Tend( Definitions ),
    Tendency( Definitions ),
    Tennis( Definitions ),
    Tension( Definitions ),
    Tent( Definitions ),
    Term( Definitions ),
    Terms( Definitions ),
    Terrible( Definitions ),
    Territory( Definitions ),
    Terror( Definitions ),
    Terrorism( Definitions ),
    Terrorist( Definitions ),
    Testify( Definitions ),
    Testimony( Definitions ),
    Testing( Definitions ),
    Text( Definitions ),
    Than( Definitions ),
    Thank( Definitions ),
    Thanks( Definitions ),
    That( Definitions ),
    Theater( Definitions ),
    Their( Definitions ),
    Them( Definitions ),
    Theme( Definitions ),
    Themselves( Definitions ),
    Then( Definitions ),
    Theory( Definitions ),
    Therapy( Definitions ),
    There( Definitions ),
    Therefore( Definitions ),
    These( Definitions ),
    They( Definitions ),
    Thick( Definitions ),
    Thin( Definitions ),
    Thing( Definitions ),
    Think( Definitions ),
    Thinking( Definitions ),
    Third( Definitions ),
    Thirty( Definitions ),
    This( Definitions ),
    Those( Definitions ),
    Though( Definitions ),
    Thought( Definitions ),
    Thousand( Definitions ),
    Threat( Definitions ),
    Threaten( Definitions ),
    Three( Definitions ),
    Throat( Definitions ),
    Through( Definitions ),
    Throughout( Definitions ),
    Throw( Definitions ),
    Thus( Definitions ),
    Ticket( Definitions ),
    Tie( Definitions ),
    Tight( Definitions ),
    Tiny( Definitions ),
    Tip( Definitions ),
    Tire( Definitions ),
    Tired( Definitions ),
    Tissue( Definitions ),
    Title( Definitions ),
    Tobacco( Definitions ),
    Today( Definitions ),
    Toe( Definitions ),
    Together( Definitions ),
    Tomato( Definitions ),
    Tomorrow( Definitions ),
    Tone( Definitions ),
    Tongue( Definitions ),
    Tonight( Definitions ),
    Too( Definitions ),
    Tool( Definitions ),
    Tooth( Definitions ),
    Top( Definitions ),
    Topic( Definitions ),
    Toss( Definitions ),
    Total( Definitions ),
    Totally( Definitions ),
    Touch( Definitions ),
    Tough( Definitions ),
    Tour( Definitions ),
    Tourist( Definitions ),
    Tournament( Definitions ),
    Toward( Definitions ),
    Towards( Definitions ),
    Tower( Definitions ),
    Town( Definitions ),
    Toy( Definitions ),
    Trace( Definitions ),
    Track( Definitions ),
    Trade( Definitions ),
    Tradition( Definitions ),
    Traditional( Definitions ),
    Traffic( Definitions ),
    Tragedy( Definitions ),
    Trail( Definitions ),
    Train( Definitions ),
    Training( Definitions ),
    Transfer( Definitions ),
    Transform( Definitions ),
    Transformation( Definitions ),
    Transition( Definitions ),
    Translate( Definitions ),
    Transportation( Definitions ),
    Travel( Definitions ),
    Treat( Definitions ),
    Treatment( Definitions ),
    Treaty( Definitions ),
    Tree( Definitions ),
    Tremendous( Definitions ),
    Trend( Definitions ),
    Trial( Definitions ),
    Tribe( Definitions ),
    Trick( Definitions ),
    Trip( Definitions ),
    Troop( Definitions ),
    Trouble( Definitions ),
    Truck( Definitions ),
    True( Definitions ),
    Truly( Definitions ),
    Trust( Definitions ),
    Truth( Definitions ),
    Try( Definitions ),
    Tube( Definitions ),
    Tunnel( Definitions ),
    Turn( Definitions ),
    TV( Definitions ),
    Twelve( Definitions ),
    Twenty( Definitions ),
    Twice( Definitions ),
    Twin( Definitions ),
    Two( Definitions ),
    Type( Definitions ),
    Typical( Definitions ),
    Typically( Definitions ),
    Ugly( Definitions ),
    Ultimate( Definitions ),
    Ultimately( Definitions ),
    Unable( Definitions ),
    Uncle( Definitions ),
    Under( Definitions ),
    Undergo( Definitions ),
    Understand( Definitions ),
    Understanding( Definitions ),
    Unfortunately( Definitions ),
    Uniform( Definitions ),
    Union( Definitions ),
    Unique( Definitions ),
    Unit( Definitions ),
    United( Definitions ),
    Universal( Definitions ),
    Universe( Definitions ),
    University( Definitions ),
    Unless( Definitions ),
    Unlike( Definitions ),
    Unlikely( Definitions ),
    Until( Definitions ),
    Unusual( Definitions ),
    Up( Definitions ),
    Upon( Definitions ),
    Upper( Definitions ),
    Urban( Definitions ),
    Urge( Definitions ),
    Us( Definitions ),
    Useful( Definitions ),
    User( Definitions ),
    Usual( Definitions ),
    Usually( Definitions ),
    Utility( Definitions ),
    Vacation( Definitions ),
    Valley( Definitions ),
    Valuable( Definitions ),
    Value( Definitions ),
    Variable( Definitions ),
    Variation( Definitions ),
    Variety( Definitions ),
    Various( Definitions ),
    Vary( Definitions ),
    Vast( Definitions ),
    Vegetable( Definitions ),
    Vehicle( Definitions ),
    Venture( Definitions ),
    Version( Definitions ),
    Versus( Definitions ),
    Very( Definitions ),
    Vessel( Definitions ),
    Veteran( Definitions ),
    Via( Definitions ),
    Victim( Definitions ),
    Victory( Definitions ),
    Video( Definitions ),
    View( Definitions ),
    Viewer( Definitions ),
    Village( Definitions ),
    Violate( Definitions ),
    Violation( Definitions ),
    Violence( Definitions ),
    Violent( Definitions ),
    Virtually( Definitions ),
    Virtue( Definitions ),
    Virus( Definitions ),
    Visible( Definitions ),
    Vision( Definitions ),
    Visit( Definitions ),
    Visitor( Definitions ),
    Visual( Definitions ),
    Vital( Definitions ),
    Voice( Definitions ),
    Volume( Definitions ),
    Volunteer( Definitions ),
    Vote( Definitions ),
    Voter( Definitions ),
    Vs( Definitions ),
    Vulnerable( Definitions ),
    Wage( Definitions ),
    Wait( Definitions ),
    Wake( Definitions ),
    Walk( Definitions ),
    Wall( Definitions ),
    Wander( Definitions ),
    Want( Definitions ),
    War( Definitions ),
    Warm( Definitions ),
    Warn( Definitions ),
    Warning( Definitions ),
    Wash( Definitions ),
    Waste( Definitions ),
    Watch( Definitions ),
    Water( Definitions ),
    Wave( Definitions ),
    Way( Definitions ),
    We( Definitions ),
    Weak( Definitions ),
    Wealth( Definitions ),
    Wealthy( Definitions ),
    Weapon( Definitions ),
    Wear( Definitions ),
    Weather( Definitions ),
    Wedding( Definitions ),
    Week( Definitions ),
    Weekend( Definitions ),
    Weekly( Definitions ),
    Weigh( Definitions ),
    Weight( Definitions ),
    Welcome( Definitions ),
    Welfare( Definitions ),
    Well( Definitions ),
    West( Definitions ),
    Western( Definitions ),
    Wet( Definitions ),
    Whatever( Definitions ),
    Wheel( Definitions ),
    Whenever( Definitions ),
    Whereas( Definitions ),
    Whether( Definitions ),
    While( Definitions ),
    Whisper( Definitions ),
    White( Definitions ),
    Whole( Definitions ),
    Whom( Definitions ),
    Wide( Definitions ),
    Widely( Definitions ),
    Widespread( Definitions ),
    Wife( Definitions ),
    Wild( Definitions ),
    Will( Definitions ),
    Willing( Definitions ),
    Win( Definitions ),
    Wind( Definitions ),
    Window( Definitions ),
    Wine( Definitions ),
    Wing( Definitions ),
    Winner( Definitions ),
    Winter( Definitions ),
    Wipe( Definitions ),
    Wire( Definitions ),
    Wisdom( Definitions ),
    Wise( Definitions ),
    Wish( Definitions ),
    With( Definitions ),
    Withdraw( Definitions ),
    Within( Definitions ),
    Without( Definitions ),
    Witness( Definitions ),
    Woman( Definitions ),
    Wonder( Definitions ),
    Wonderful( Definitions ),
    Wood( Definitions ),
    Wooden( Definitions ),
    Word( Definitions ),
    Work( Definitions ),
    Worker( Definitions ),
    Working( Definitions ),
    Works( Definitions ),
    Workshop( Definitions ),
    World( Definitions ),
    Worried( Definitions ),
    Worry( Definitions ),
    Worth( Definitions ),
    Would( Definitions ),
    Wound( Definitions ),
    Wrap( Definitions ),
    Write( Definitions ),
    Writer( Definitions ),
    Writing( Definitions ),
    Wrong( Definitions ),
    Yard( Definitions ),
    Yeah( Definitions ),
    Year( Definitions ),
    Yell( Definitions ),
    Yellow( Definitions ),
    Yes( Definitions ),
    Yesterday( Definitions ),
    Yet( Definitions ),
    Yield( Definitions ),
    You( Definitions ),
    Young( Definitions ),
    Your( Definitions ),
    Yours( Definitions ),
    Yourself( Definitions ),
    Youth( Definitions ),
    Zone( Definitions ),
    Whoever( Definitions ),
    Stands( Definitions ),
    Nations( Definitions ),
    Understood( Definitions ),
    Referring( Definitions ),
    Initialism( Definitions ),
    An( Definitions ),
    Activities( Definitions ),
    Coordinates( Definitions ),
    Governments( Definitions ),
    Services( Definitions ),
    Persons( Definitions ),
    Peoples( Definitions ),
    Mortal( Definitions ),
    Personage( Definitions ),
    Inhabitant( Definitions ),
    Denizen( Definitions ),
    Whomever( Definitions ),
    Noone( Definitions ),    
    Ended( Definitions ),
    Possibilities( Definitions ),
    Things( Definitions ),
    Disbelief( Definitions ),
    Abrupt( Definitions ),
    Unfriendly( Definitions ),
    Enquiry( Definitions ),
    Desires( Definitions ),
    Emphasises( Definitions ),
    Noteworthy( Definitions ),
    Addressed( Definitions ),
    Prepositional( Definitions ),
    Emphasise( Definitions ),
    Taken( Definitions ),
    Exclamations( Definitions ),
    Indicating( Definitions ),
    Emphasizes( Definitions ),
    Assertion( Definitions ),
    Made( Definitions ),
    Contradict( Definitions ),
    Evidently( Definitions ),
    Held( Definitions ),
    Approximation( Definitions ),
    Followed( Definitions ),
    Tag( Definitions ),
    Inviting( Definitions ),    
    Asking( Definitions ),
    Questions( Definitions ),
}

impl Ident
{
    pub fn who() -> Self
    {
        Self::Who
        (
            Entity::who()       
        )
    }

    pub fn what() -> Self
    {
        Self::What
        (
            Entity::what()       
        )
    }

    pub fn synonyms( &self ) -> Definition
    {
        use Ident::{*};
        match self
        {
            Who( _ ) =>
            {
                vec!
                [
                    Entity::Whom,
                    Entity::Which,
                    Entity::Whoever,
                    Entity::Persons,
                    Entity::Peoples,
                    Entity::One,
                    Entity::Individual,
                    Entity::Being,
                    Entity::Mortal,
                    Entity::Soul,
                    Entity::Party,
                    Entity::Personage,
                    Entity::Human,
                    Entity::Man,
                    Entity::Woman,
                    Entity::Character,
                    Entity::Customer,
                    Entity::Inhabitant,
                    Entity::Denizen,
                ]
            }
            _ =>
            {
                vec![]
            }
        }
    }

    pub fn antonyms( &self ) -> Definition
    {
        use Ident::{*};
        match self
        {
            Who( _ ) =>
            {
                vec!
                [
                    Entity::What,
                    Entity::Whoever,
                    Entity::Whomever,
                    Entity::Nobody,
                    Entity::Anyone,
                    Entity::Noone,
                    Entity::Everybody,
                    Entity::As,
                    Entity::How,
                ]
            }
            _ =>
            {
                vec![]
            }
        }
    }
}

#[macro_use] pub mod macros
{
    use ::
    {
        *,
    };

    #[macro_export] macro_rules! tri
    {
        ($expr:expr) =>
        {
            match $expr
            {
                Ok(val) => val,
                Err(err) => return Err(err),
            }
        };
    }

    #[macro_export(local_inner_macros)] macro_rules! forward_to_deserialize_any
    {
        (<$visitor:ident: Visitor<$lifetime:tt>> $($func:ident)*) =>
        {
            $(forward_to_deserialize_any_helper!{$func<$lifetime, $visitor>})*
        };
        
        ($($func:ident)*) =>
        {
            $(forward_to_deserialize_any_helper!{$func<'de, V>})*
        };
    }

    #[macro_export] macro_rules! forward_to_deserialize_any_method
    {
        ($func:ident<$l:tt, $v:ident>($($arg:ident : $ty:ty),*)) =>
        {
            #[inline] fn $func<$v>(self, $($arg: $ty,)* visitor: $v) -> ::Result<$v::Value, <Self as ::core::serde::de::Deserializer<$l>>::Error> where
            $v: ::core::serde::de::Visitor<$l>,
            {
                $( let _ = $arg; )*
                self.deserialize_any(visitor)
            }
        };
    }
    
    #[macro_export(local_inner_macros)] macro_rules! forward_to_deserialize_any_helper 
    {
        (bool<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_bool<$l, $v>()}
        };
        
        (i8<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_i8<$l, $v>()}
        };
        
        (i16<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_i16<$l, $v>()}
        };
        
        (i32<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_i32<$l, $v>()}
        };
        
        (i64<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_i64<$l, $v>()}
        };
        
        (i128<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_i128<$l, $v>()}
        };
        
        (u8<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_u8<$l, $v>()}
        };
        
        (u16<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_u16<$l, $v>()}
        };
        
        (u32<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_u32<$l, $v>()}
        };
        
        (u64<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_u64<$l, $v>()}
        };
        
        (u128<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_u128<$l, $v>()}
        };
        
        (f32<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_f32<$l, $v>()}
        };
        
        (f64<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_f64<$l, $v>()}
        };
        
        (char<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_char<$l, $v>()}
        };
        
        (str<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_str<$l, $v>()}
        };
        
        (string<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_string<$l, $v>()}
        };
        
        (bytes<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_bytes<$l, $v>()}
        };
        
        (byte_buf<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_byte_buf<$l, $v>()}
        };
        
        (option<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_option<$l, $v>()}
        };
        
        (unit<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_unit<$l, $v>()}
        };
        
        (unit_struct<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_unit_struct<$l, $v>(name: &'static str)}
        };
        
        (newtype_struct<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_newtype_struct<$l, $v>(name: &'static str)}
        };
        
        (seq<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_seq<$l, $v>()}
        };
        
        (tuple<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_tuple<$l, $v>(len: usize)}
        };
        
        (tuple_struct<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_tuple_struct<$l, $v>(name: &'static str, len: usize)}
        };
        
        (map<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_map<$l, $v>()}
        };
        
        (struct<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_struct<$l, $v>(name: &'static str, fields: &'static [&'static str])}
        };
        
        (enum<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_enum<$l, $v>(name: &'static str, variants: &'static [&'static str])}
        };
        
        (identifier<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_identifier<$l, $v>()}
        };
        
        (ignored_any<$l:tt, $v:ident>) =>
        {
            forward_to_deserialize_any_method!{deserialize_ignored_any<$l, $v>()}
        };
    }

    #[macro_export] macro_rules! __require_serde_not_serde_core
    {
        () =>
        {
            compile_error!
            (
                "Serde derive requires a dependency on the serde crate, not serde_core"
            );
        };
    }
}

pub mod alloc
{
    pub use std::alloc::{ * };
}

pub mod borrow
{
    pub use std::borrow::{ * };
}

pub mod boxed
{
    pub use std::boxed::{ * };
}

pub mod cell
{
    pub use std::cell::{ * };
}

pub mod cmp
{
    pub use std::cmp::{ * };
}

pub mod collections
{
    pub use std::collections::{ * };
}

pub mod core
{
    use ::
    {
        *,
    };

    pub mod csv
    {
        use ::
        {
            *,
        };
    }

    pub mod serde
    {
        /*! Serde is a framework for ***ser***ializing and ***de***serializing Rust data structures efficiently and generically. */
        use ::
        {
            *,
        };
        /*
            [package]
            name = "serde_core"
            version = "1.0.221"
            authors = ["Erick Tryzelaar <erick.tryzelaar@gmail.com>", "David Tolnay <dtolnay@gmail.com>"]
            build = "build.rs"
            categories = ["encoding", "no-std", "no-std::no-alloc"]
            description = "Serde traits only, with no support for derive -- use the `serde` crate instead"
            documentation = "https://docs.rs/serde_core"
            edition = "2021"
            homepage = "https://serde.rs"
            keywords = ["serde", "serialization", "no_std"]
            license = "MIT OR Apache-2.0"
            repository = "https://github.com/serde-rs/serde"
            rust-version = "1.56"

            [dev-dependencies]
            serde = { version = "1", path = "../serde" }
            serde_derive = { version = "1", path = "../serde_derive" }

            [package.metadata.playground]
            features = ["rc", "result"]

            [package.metadata.docs.rs]
            features = ["rc", "result", "unstable"]
            targets = ["x86_64-unknown-linux-gnu"]
            rustdoc-args = [
                "--generate-link-to-definition",
                "--extern-html-root-url=core=https://doc.rust-lang.org",
                "--extern-html-root-url=alloc=https://doc.rust-lang.org",
                "--extern-html-root-url=std=https://doc.rust-lang.org",
            ]

            # This cfg cannot be enabled, but it still forces Cargo to keep serde_derive's
            # version in lockstep with serde's, even if someone depends on the two crates
            # separately with serde's "derive" feature disabled. Every serde_derive release
            # is compatible with exactly one serde release because the generated code
            # involves nonpublic APIs which are not bound by semver.
            [target.'cfg(any())'.dependencies]
            serde_derive = { version = "=1.0.221", path = "../serde_derive" }


            ### FEATURES #################################################################

            [features]
            default = ["std", "result"]

            # Provide impls for common standard library types like Vec<T> and HashMap<K, V>.
            # Requires a dependency on the Rust standard library.
            std = []

            # Provide impls for types that require unstable functionality. For tracking and
            # discussion of unstable functionality please refer to this issue:
            #
            #    https://github.com/serde-rs/serde/issues/812
            unstable = []

            # Provide impls for types in the Rust core allocation and collections library
            # including String, Box<T>, Vec<T>, and Cow<T>. This is a subset of std but may
            # be enabled without depending on all of std.
            alloc = []

            # Opt into impls for Rc<T> and Arc<T>. Serializing and deserializing these types
            # does not preserve identity and may result in multiple copies of the same data.
            # Be sure that this is what you want before enabling this feature.
            rc = []

            # Provide impls for Result<T, E>. Convenient in some contexts but can lead to
            # confusion if ? or unwrap are used incautiously.
            result = []
        */
        pub mod lib
        {
            /*! A facade around all the types we need from the `std`, `core`, and `alloc` crates.*/
            pub use ::
            {
                borrow::{ Cow, ToOwned },
                boxed::{ Box },
                cell::{ Cell, RefCell },
                cmp::{ Reverse },
                collections::{ BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque },
                ffi::{ CStr, CString, OsStr, OsString },
                fmt::{ self, Debug, Display, Write as FmtWrite },
                hash::{ BuildHasher, Hash },
                io::{ Write },
                marker::{ PhantomData },
                num::{ Saturating, Wrapping },
                ops::{ Bound, Range, RangeFrom, RangeInclusive, RangeTo },
                path::{ Path, PathBuf },
                rc::{ Rc, Weak as RcWeak },
                string::{ String, ToString },
                sync::{ Arc, Weak as ArcWeak, Mutex, RwLock },
                time::{ Duration, SystemTime, UNIX_EPOCH },
                vec::{ Vec },
                *,
            };
            /*
            */
        }
        
        pub mod de
        {
            //! Generic data structure deserialization framework.
            use ::
            {
                core::
                {
                    fmt::{ Display, Write },
                    marker::{ PhantomData },
                    serde::
                    {
                        InPlaceSeed
                    },
                },
                error::{ Error as StdError },
                *,
            };

            macro_rules! declare_error_trait
            {
                (Error: Sized $(+ $($supertrait:ident)::+)*) =>
                {
                    pub trait Error: Sized $(+ $($supertrait)::+)* 
                    {
                        fn custom<T>(msg: T) -> Self where T:Display;                
                        #[cold] fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self { Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp)) }
                        #[cold] fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self { Error::custom(format_args!("invalid value: {}, expected {}", unexp, exp)) }
                        #[cold] fn invalid_length(len: usize, exp: &dyn Expected) -> Self { Error::custom(format_args!("invalid length {}, expected {}", len, exp)) }
                        
                        #[cold] fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self
                        {
                            if expected.is_empty()
                            {
                                Error::custom( format_args!( "unknown variant `{}`, there are no variants", variant ) )
                            }
                            
                            else
                            {
                                Error::custom( format_args!( "unknown variant `{}`, expected {}", variant, OneOf { names: expected } ) )
                            }
                        }
                        
                        #[cold] fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self
                        {
                            if expected.is_empty()
                            {
                                Error::custom( format_args!( "unknown field `{}`, there are no fields", field ) )
                            }
                            
                            else
                            {
                                Error::custom( format_args!( "unknown field `{}`, expected {}", field, OneOf { names: expected } ) )
                            }
                        }
                        
                        #[cold] fn missing_field(field: &'static str) -> Self { Error::custom(format_args!("missing field `{}`", field)) }
                        #[cold] fn duplicate_field(field: &'static str) -> Self { Error::custom(format_args!("duplicate field `{}`", field)) }
                    }
                }
            }

            pub mod value
            {
                //! Building blocks for deserializing basic values using the `IntoDeserializer` trait.
                use ::
                {
                    core::serde::
                    {
                        de::{ self, Deserializer, Expected, IntoDeserializer, SeqAccess, Visitor },
                        ser, First, Second, size_hint
                    },
                    *,
                };

                macro_rules! impl_copy_clone 
                {
                    ($ty:ident $(<$lifetime:tt>)*) => 
                    {
                        impl<$($lifetime,)* E> Copy for $ty<$($lifetime,)* E> {}

                        impl<$($lifetime,)* E> Clone for $ty<$($lifetime,)* E> 
                        {
                            fn clone(&self) -> Self { *self }
                        }
                    };
                }
                
                #[derive(Clone, PartialEq)]
                pub struct Error
                {
                    err: ErrorImpl,
                }

                type ErrorImpl = Box<str>;

                impl de::Error for Error
                {
                    #[cold] fn custom<T>(msg: T) -> Self where
                    T: Display
                    {
                        Error { err: msg.to_string().into_boxed_str() }
                    }
                }

                impl ser::Error for Error
                {
                    #[cold] fn custom<T>(msg: T) -> Self where
                    T: Display
                    { de::Error::custom(msg) }
                }

                impl Display for Error 
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result { formatter.write_str(&self.err) }
                }

                impl Debug for Error
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
                    {
                        let mut debug = formatter.debug_tuple("Error");
                        debug.field(&self.err);
                        debug.finish()
                    }
                }
                
                impl error::Error for Error
                {
                    fn description(&self) -> &str { &self.err }
                }

                impl<'de, E> IntoDeserializer<'de, E> for () where
                E: de::Error
                {
                    type Deserializer = UnitDeserializer<E>;
                    fn into_deserializer(self) -> UnitDeserializer<E> { UnitDeserializer::new() }
                }
                
                pub struct UnitDeserializer<E>
                {
                    marker: PhantomData<E>,
                }

                impl_copy_clone!(UnitDeserializer);

                impl<E> UnitDeserializer<E>
                {
                    pub fn new() -> Self
                    {
                        UnitDeserializer
                        {
                            marker: PhantomData,
                        }
                    }
                }

                impl<'de, E> de::Deserializer<'de> for UnitDeserializer<E> where
                E: de::Error
                {
                    type Error = E;

                    forward_to_deserialize_any! 
                    {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
                        map struct enum identifier ignored_any
                    }

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    {
                        visitor.visit_unit()
                    }

                    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    {
                        visitor.visit_none()
                    }
                }

                impl<'de, E> IntoDeserializer<'de, E> for UnitDeserializer<E> where
                E: de::Error
                {
                    type Deserializer = Self;
                    fn into_deserializer(self) -> Self { self }
                }

                impl<E> Debug for UnitDeserializer<E> 
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result { formatter.debug_struct("UnitDeserializer").finish() }
                }
                
                macro_rules! primitive_deserializer
                {
                    ($ty:ty, $doc:tt, $name:ident, $method:ident $($cast:tt)*) =>
                    {
                        #[doc = "A deserializer holding"]
                        #[doc = $doc]
                        pub struct $name<E> {
                            value: $ty,
                            marker: PhantomData<E>
                        }

                        impl_copy_clone!($name);

                        impl<'de, E> IntoDeserializer<'de, E> for $ty
                        where
                            E: de::Error,
                        {
                            type Deserializer = $name<E>;

                            fn into_deserializer(self) -> $name<E> {
                                $name::new(self)
                            }
                        }

                        impl<E> $name<E> {
                            #[allow(missing_docs)]
                            pub fn new(value: $ty) -> Self {
                                $name {
                                    value,
                                    marker: PhantomData,
                                }
                            }
                        }

                        impl<'de, E> de::Deserializer<'de> for $name<E>
                        where
                            E: de::Error,
                        {
                            type Error = E;

                            forward_to_deserialize_any! {
                                bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str
                                string bytes byte_buf option unit unit_struct newtype_struct seq
                                tuple tuple_struct map struct enum identifier ignored_any
                            }

                            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                            where
                                V: de::Visitor<'de>,
                            {
                                visitor.$method(self.value $($cast)*)
                            }
                        }

                        impl<'de, E> IntoDeserializer<'de, E> for $name<E>
                        where
                            E: de::Error,
                        {
                            type Deserializer = Self;

                            fn into_deserializer(self) -> Self {
                                self
                            }
                        }

                        impl<E> Debug for $name<E> {
                            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                formatter
                                    .debug_struct(stringify!($name))
                                    .field("value", &self.value)
                                    .finish()
                            }
                        }
                    }
                }

                primitive_deserializer!(bool, "a `bool`.", BoolDeserializer, visit_bool);
                primitive_deserializer!(i8, "an `i8`.", I8Deserializer, visit_i8);
                primitive_deserializer!(i16, "an `i16`.", I16Deserializer, visit_i16);
                primitive_deserializer!(i32, "an `i32`.", I32Deserializer, visit_i32);
                primitive_deserializer!(i64, "an `i64`.", I64Deserializer, visit_i64);
                primitive_deserializer!(i128, "an `i128`.", I128Deserializer, visit_i128);
                primitive_deserializer!(isize, "an `isize`.", IsizeDeserializer, visit_i64 as i64);
                primitive_deserializer!(u8, "a `u8`.", U8Deserializer, visit_u8);
                primitive_deserializer!(u16, "a `u16`.", U16Deserializer, visit_u16);
                primitive_deserializer!(u64, "a `u64`.", U64Deserializer, visit_u64);
                primitive_deserializer!(u128, "a `u128`.", U128Deserializer, visit_u128);
                primitive_deserializer!(usize, "a `usize`.", UsizeDeserializer, visit_u64 as u64);
                primitive_deserializer!(f32, "an `f32`.", F32Deserializer, visit_f32);
                primitive_deserializer!(f64, "an `f64`.", F64Deserializer, visit_f64);
                primitive_deserializer!(char, "a `char`.", CharDeserializer, visit_char);
                
                pub struct U32Deserializer<E> 
                {
                    value: u32,
                    marker: PhantomData<E>,
                }

                impl_copy_clone!(U32Deserializer);

                impl<'de, E> IntoDeserializer<'de, E> for u32 where
                E: de::Error
                {
                    type Deserializer = U32Deserializer<E>;
                    fn into_deserializer(self) -> U32Deserializer<E> { U32Deserializer::new(self) }
                }

                impl<E> U32Deserializer<E>
                {
                    pub fn new(value: u32) -> Self
                    {
                        U32Deserializer
                        {
                            value,
                            marker: PhantomData,
                        }
                    }
                }

                impl<'de, E> de::Deserializer<'de> for U32Deserializer<E> where
                E: de::Error
                {
                    type Error = E;

                    forward_to_deserialize_any! 
                    {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct identifier ignored_any
                    }

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>,
                    {
                        visitor.visit_u32(self.value)
                    }

                    fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V ) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    {
                        let _ = name;
                        let _ = variants;
                        visitor.visit_enum(self)
                    }
                }

                impl<'de, E> IntoDeserializer<'de, E> for U32Deserializer<E> where
                E: de::Error
                {
                    type Deserializer = Self;
                    fn into_deserializer(self) -> Self { self }
                }

                impl<'de, E> de::EnumAccess<'de> for U32Deserializer<E> where
                E: de::Error
                {
                    type Error = E;
                    type Variant = private::UnitOnly<E>;

                    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error> where
                    T: de::DeserializeSeed<'de>
                    { seed.deserialize(self).map(private::unit_only) }
                }

                impl<E> Debug for U32Deserializer<E> 
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result 
                    {
                        formatter
                        .debug_struct("U32Deserializer")
                        .field("value", &self.value)
                        .finish()
                    }
                }
                
                pub struct StrDeserializer<'a, E> 
                {
                    value: &'a str,
                    marker: PhantomData<E>,
                }

                impl_copy_clone!(StrDeserializer<'de>);

                impl<'de, 'a, E> IntoDeserializer<'de, E> for &'a str where
                E: de::Error,
                {
                    type Deserializer = StrDeserializer<'a, E>;
                    fn into_deserializer(self) -> StrDeserializer<'a, E> { StrDeserializer::new(self) }
                }

                impl<'a, E> StrDeserializer<'a, E>
                {
                    pub fn new(value: &'a str) -> Self
                    {
                        StrDeserializer
                        {
                            value,
                            marker: PhantomData,
                        }
                    }
                }

                impl<'de, 'a, E> de::Deserializer<'de> for StrDeserializer<'a, E> where
                E: de::Error,
                {
                    type Error = E;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    { visitor.visit_str(self.value) }

                    fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V ) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    {
                        let _ = name;
                        let _ = variants;
                        visitor.visit_enum(self)
                    }

                    forward_to_deserialize_any!
                    {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct identifier ignored_any
                    }
                }

                impl<'de, 'a, E> IntoDeserializer<'de, E> for StrDeserializer<'a, E> where
                E: de::Error
                {
                    type Deserializer = Self;
                    fn into_deserializer(self) -> Self { self }
                }

                impl<'de, 'a, E> de::EnumAccess<'de> for StrDeserializer<'a, E> where
                E: de::Error
                {
                    type Error = E;
                    type Variant = private::UnitOnly<E>;

                    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error> where
                    T: de::DeserializeSeed<'de>
                    { seed.deserialize(self).map(private::unit_only) }
                }

                impl<'a, E> Debug for StrDeserializer<'a, E>
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
                    {
                        formatter
                        .debug_struct("StrDeserializer")
                        .field("value", &self.value)
                        .finish()
                    }
                }
                
                pub struct BorrowedStrDeserializer<'de, E>
                {
                    value: &'de str,
                    marker: PhantomData<E>,
                }

                impl_copy_clone!(BorrowedStrDeserializer<'de>);

                impl<'de, E> BorrowedStrDeserializer<'de, E>
                {
                    pub fn new(value: &'de str) -> BorrowedStrDeserializer<'de, E>
                    {
                        BorrowedStrDeserializer
                        {
                            value,
                            marker: PhantomData,
                        }
                    }
                }

                impl<'de, E> de::Deserializer<'de> for BorrowedStrDeserializer<'de, E> where
                E: de::Error
                {
                    type Error = E;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    { visitor.visit_borrowed_str(self.value) }

                    fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V ) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    {
                        let _ = name;
                        let _ = variants;
                        visitor.visit_enum(self)
                    }

                    forward_to_deserialize_any! 
                    {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct identifier ignored_any
                    }
                }

                impl<'de, E> IntoDeserializer<'de, E> for BorrowedStrDeserializer<'de, E> where
                E: de::Error
                {
                    type Deserializer = Self;
                    fn into_deserializer(self) -> Self { self }
                }

                impl<'de, E> de::EnumAccess<'de> for BorrowedStrDeserializer<'de, E> where
                E: de::Error
                {
                    type Error = E;
                    type Variant = private::UnitOnly<E>;

                    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error> where
                    T: de::DeserializeSeed<'de>
                    { seed.deserialize(self).map(private::unit_only) }
                }

                impl<'de, E> Debug for BorrowedStrDeserializer<'de, E>
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
                    {
                        formatter
                        .debug_struct("BorrowedStrDeserializer")
                        .field("value", &self.value)
                        .finish()
                    }
                }
                
                pub struct StringDeserializer<E>
                {
                    value: String,
                    marker: PhantomData<E>,
                }
                
                impl<E> Clone for StringDeserializer<E>
                {
                    fn clone(&self) -> Self
                    {
                        StringDeserializer
                        {
                            value: self.value.clone(),
                            marker: PhantomData,
                        }
                    }
                }
                
                impl<'de, E> IntoDeserializer<'de, E> for String where
                E: de::Error
                {
                    type Deserializer = StringDeserializer<E>;
                    fn into_deserializer(self) -> StringDeserializer<E> { StringDeserializer::new(self) }
                }
                
                impl<E> StringDeserializer<E> 
                {
                    pub fn new(value: String) -> Self
                    {
                        StringDeserializer
                        {
                            value,
                            marker: PhantomData,
                        }
                    }
                }
                
                impl<'de, E> de::Deserializer<'de> for StringDeserializer<E> where
                E: de::Error
                {
                    type Error = E;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>,
                    { visitor.visit_string(self.value) }

                    fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V ) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    {
                        let _ = name;
                        let _ = variants;
                        visitor.visit_enum(self)
                    }

                    forward_to_deserialize_any!
                    {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct identifier ignored_any
                    }
                }
                
                impl<'de, E> IntoDeserializer<'de, E> for StringDeserializer<E> where
                E: de::Error
                {
                    type Deserializer = Self;
                    fn into_deserializer(self) -> Self { self }
                }
                
                impl<'de, E> de::EnumAccess<'de> for StringDeserializer<E> where
                E: de::Error
                {
                    type Error = E;
                    type Variant = private::UnitOnly<E>;

                    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error> where
                    T: de::DeserializeSeed<'de>
                    { seed.deserialize(self).map(private::unit_only) }
                }
                
                impl<E> Debug for StringDeserializer<E>
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
                    {
                        formatter
                        .debug_struct("StringDeserializer")
                        .field("value", &self.value)
                        .finish()
                    }
                }
                
                pub struct CowStrDeserializer<'a, E> 
                {
                    value: Cow<'a, str>,
                    marker: PhantomData<E>,
                }
                
                impl<'a, E> Clone for CowStrDeserializer<'a, E> 
                {
                    fn clone(&self) -> Self 
                    {
                        CowStrDeserializer 
                        {
                            value: self.value.clone(),
                            marker: PhantomData,
                        }
                    }
                }
                
                impl<'de, 'a, E> IntoDeserializer<'de, E> for Cow<'a, str> where
                E: de::Error
                {
                    type Deserializer = CowStrDeserializer<'a, E>;
                    fn into_deserializer(self) -> CowStrDeserializer<'a, E> { CowStrDeserializer::new(self) }
                }
                
                impl<'a, E> CowStrDeserializer<'a, E>
                {
                    pub fn new(value: Cow<'a, str>) -> Self
                    {
                        CowStrDeserializer
                        {
                            value,
                            marker: PhantomData
                        }
                    }
                }
                
                impl<'de, 'a, E> de::Deserializer<'de> for CowStrDeserializer<'a, E> where
                E: de::Error
                {
                    type Error = E;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    {
                        match self.value
                        {
                            Cow::Borrowed(string) => visitor.visit_str(string),
                            Cow::Owned(string) => visitor.visit_string(string),
                        }
                    }

                    fn deserialize_enum<V>( self, name: &str, variants: &'static [&'static str], visitor: V ) -> Result<V::Value, Self::Error> where
                    V: de::Visitor<'de>
                    {
                        let _ = name;
                        let _ = variants;
                        visitor.visit_enum(self)
                    }

                    forward_to_deserialize_any!
                    {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct identifier ignored_any
                    }
                }
                
                impl<'de, 'a, E> IntoDeserializer<'de, E> for CowStrDeserializer<'a, E> where
                E: de::Error
                {
                    type Deserializer = Self;
                    fn into_deserializer(self) -> Self { self }
                }
                
                impl<'de, 'a, E> de::EnumAccess<'de> for CowStrDeserializer<'a, E> where
                E: de::Error
                {
                    type Error = E;
                    type Variant = private::UnitOnly<E>;

                    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error> where
                    T: de::DeserializeSeed<'de>
                    { seed.deserialize(self).map(private::unit_only) }
                }
                
                impl<'a, E> Debug for CowStrDeserializer<'a, E>
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
                    {
                        formatter
                        .debug_struct("CowStrDeserializer")
                        .field("value", &self.value)
                        .finish()
                    }
                }
                
                pub struct BytesDeserializer<'a, E>
                {
                    value: &'a [u8],
                    marker: PhantomData<E>,
                }

                impl<'a, E> BytesDeserializer<'a, E>
                {
                    pub fn new(value: &'a [u8]) -> Self
                    {
                        BytesDeserializer
                        {
                            value,
                            marker: PhantomData,
                        }
                    }
                }

                impl_copy_clone!(BytesDeserializer<'a>);

                impl<'de, 'a, E> IntoDeserializer<'de, E> for &'a [u8] where
                E: de::Error
                {
                    type Deserializer = BytesDeserializer<'a, E>;
                    fn into_deserializer(self) -> BytesDeserializer<'a, E> { BytesDeserializer::new(self) }
                }

                impl<'de, 'a, E> Deserializer<'de> for BytesDeserializer<'a, E> where
                E: de::Error
                {
                    type Error = E;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                    V: Visitor<'de>
                    { visitor.visit_bytes(self.value) }

                    forward_to_deserialize_any!
                    {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct enum identifier ignored_any
                    }
                }

                impl<'de, 'a, E> IntoDeserializer<'de, E> for BytesDeserializer<'a, E> where
                E: de::Error
                {
                    type Deserializer = Self;

                    fn into_deserializer(self) -> Self { self }
                }

                impl<'a, E> Debug for BytesDeserializer<'a, E>
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
                    {
                        formatter
                        .debug_struct("BytesDeserializer")
                        .field("value", &self.value)
                        .finish()
                    }
                }

                /// A deserializer holding a `&[u8]` with a lifetime tied to another
                /// deserializer. Always calls [`Visitor::visit_borrowed_bytes`].
                pub struct BorrowedBytesDeserializer<'de, E> {
                    value: &'de [u8],
                    marker: PhantomData<E>,
                }

                impl<'de, E> BorrowedBytesDeserializer<'de, E> {
                    /// Create a new borrowed deserializer from the given borrowed bytes.
                    pub fn new(value: &'de [u8]) -> Self {
                        BorrowedBytesDeserializer {
                            value,
                            marker: PhantomData,
                        }
                    }
                }

                impl_copy_clone!(BorrowedBytesDeserializer<'de>);

                impl<'de, E> Deserializer<'de> for BorrowedBytesDeserializer<'de, E> where
                    E: de::Error,
                {
                    type Error = E;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: Visitor<'de>,
                    {
                        visitor.visit_borrowed_bytes(self.value)
                    }

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct enum identifier ignored_any
                    }
                }

                impl<'de, E> IntoDeserializer<'de, E> for BorrowedBytesDeserializer<'de, E> where
                    E: de::Error,
                {
                    type Deserializer = Self;

                    fn into_deserializer(self) -> Self {
                        self
                    }
                }

                impl<'de, E> Debug for BorrowedBytesDeserializer<'de, E> {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter
                            .debug_struct("BorrowedBytesDeserializer")
                            .field("value", &self.value)
                            .finish()
                    }
                }

                ////////////////////////////////////////////////////////////////////////////////

                /// A deserializer that iterates over a sequence.
                #[derive(Clone)]
                pub struct SeqDeserializer<I, E> {
                    iter: iter::Fuse<I>,
                    count: usize,
                    marker: PhantomData<E>,
                }

                impl<I, E> SeqDeserializer<I, E> where
                    I: Iterator,
                {
                    /// Construct a new `SeqDeserializer<I, E>`.
                    pub fn new(iter: I) -> Self {
                        SeqDeserializer {
                            iter: iter.fuse(),
                            count: 0,
                            marker: PhantomData,
                        }
                    }
                }

                impl<I, E> SeqDeserializer<I, E> where
                    I: Iterator,
                    E: de::Error,
                {
                    /// Check for remaining elements after passing a `SeqDeserializer` to
                    /// `Visitor::visit_seq`.
                    pub fn end(self) -> Result<(), E> {
                        let remaining = self.iter.count();
                        if remaining == 0 {
                            Ok(())
                        } else {
                            // First argument is the number of elements in the data, second
                            // argument is the number of elements expected by the Deserialize.
                            Err(de::Error::invalid_length(
                                self.count + remaining,
                                &ExpectedInSeq(self.count),
                            ))
                        }
                    }
                }

                impl<'de, I, T, E> de::Deserializer<'de> for SeqDeserializer<I, E> where
                    I: Iterator<Item = T>,
                    T: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Error = E;

                    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        let v = tri!(visitor.visit_seq(&mut self));
                        tri!(self.end());
                        Ok(v)
                    }

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct enum identifier ignored_any
                    }
                }

                impl<'de, I, T, E> IntoDeserializer<'de, E> for SeqDeserializer<I, E> where
                    I: Iterator<Item = T>,
                    T: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Deserializer = Self;

                    fn into_deserializer(self) -> Self {
                        self
                    }
                }

                impl<'de, I, T, E> de::SeqAccess<'de> for SeqDeserializer<I, E> where
                    I: Iterator<Item = T>,
                    T: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Error = E;

                    fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Self::Error> where
                        V: de::DeserializeSeed<'de>,
                    {
                        match self.iter.next() {
                            Some(value) => {
                                self.count += 1;
                                seed.deserialize(value.into_deserializer()).map(Some)
                            }
                            None => Ok(None),
                        }
                    }

                    fn size_hint(&self) -> Option<usize> {
                        size_hint::from_bounds(&self.iter)
                    }
                }

                struct ExpectedInSeq(usize);

                impl Expected for ExpectedInSeq {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        if self.0 == 1 {
                            formatter.write_str("1 element in sequence")
                        } else {
                            write!(formatter, "{} elements in sequence", self.0)
                        }
                    }
                }

                impl<I, E> Debug for SeqDeserializer<I, E> where
                    I: Debug,
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter
                            .debug_struct("SeqDeserializer")
                            .field("iter", &self.iter)
                            .field("count", &self.count)
                            .finish()
                    }
                }

                ////////////////////////////////////////////////////////////////////////////////

                #[cfg(any(feature = "std", feature = "alloc"))]
                #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
                impl<'de, T, E> IntoDeserializer<'de, E> for Vec<T> where
                    T: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Deserializer = SeqDeserializer<<Self as IntoIterator>::IntoIter, E>;

                    fn into_deserializer(self) -> Self::Deserializer {
                        SeqDeserializer::new(self.into_iter())
                    }
                }

                #[cfg(any(feature = "std", feature = "alloc"))]
                #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
                impl<'de, T, E> IntoDeserializer<'de, E> for BTreeSet<T> where
                    T: IntoDeserializer<'de, E> + Eq + Ord,
                    E: de::Error,
                {
                    type Deserializer = SeqDeserializer<<Self as IntoIterator>::IntoIter, E>;

                    fn into_deserializer(self) -> Self::Deserializer {
                        SeqDeserializer::new(self.into_iter())
                    }
                }

                #[cfg(feature = "std")]
                #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
                impl<'de, T, S, E> IntoDeserializer<'de, E> for HashSet<T, S> where
                    T: IntoDeserializer<'de, E> + Eq + Hash,
                    S: BuildHasher,
                    E: de::Error,
                {
                    type Deserializer = SeqDeserializer<<Self as IntoIterator>::IntoIter, E>;

                    fn into_deserializer(self) -> Self::Deserializer {
                        SeqDeserializer::new(self.into_iter())
                    }
                }

                ////////////////////////////////////////////////////////////////////////////////

                /// A deserializer holding a `SeqAccess`.
                #[derive(Clone, Debug)]
                pub struct SeqAccessDeserializer<A> {
                    seq: A,
                }

                impl<A> SeqAccessDeserializer<A> {
                    /// Construct a new `SeqAccessDeserializer<A>`.
                    pub fn new(seq: A) -> Self {
                        SeqAccessDeserializer { seq }
                    }
                }

                impl<'de, A> de::Deserializer<'de> for SeqAccessDeserializer<A> where
                    A: de::SeqAccess<'de>,
                {
                    type Error = A::Error;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        visitor.visit_seq(self.seq)
                    }

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct enum identifier ignored_any
                    }
                }

                impl<'de, A> IntoDeserializer<'de, A::Error> for SeqAccessDeserializer<A> where
                    A: de::SeqAccess<'de>,
                {
                    type Deserializer = Self;

                    fn into_deserializer(self) -> Self {
                        self
                    }
                }

                ////////////////////////////////////////////////////////////////////////////////

                /// A deserializer that iterates over a map.
                pub struct MapDeserializer<'de, I, E> where
                    I: Iterator,
                    I::Item: private::Pair,
                {
                    iter: iter::Fuse<I>,
                    value: Option<Second<I::Item>>,
                    count: usize,
                    lifetime: PhantomData<&'de ()>,
                    error: PhantomData<E>,
                }

                impl<'de, I, E> MapDeserializer<'de, I, E> where
                    I: Iterator,
                    I::Item: private::Pair,
                {
                    /// Construct a new `MapDeserializer<I, E>`.
                    pub fn new(iter: I) -> Self {
                        MapDeserializer {
                            iter: iter.fuse(),
                            value: None,
                            count: 0,
                            lifetime: PhantomData,
                            error: PhantomData,
                        }
                    }
                }

                impl<'de, I, E> MapDeserializer<'de, I, E> where
                    I: Iterator,
                    I::Item: private::Pair,
                    E: de::Error,
                {
                    /// Check for remaining elements after passing a `MapDeserializer` to
                    /// `Visitor::visit_map`.
                    pub fn end(self) -> Result<(), E> {
                        let remaining = self.iter.count();
                        if remaining == 0 {
                            Ok(())
                        } else {
                            // First argument is the number of elements in the data, second
                            // argument is the number of elements expected by the Deserialize.
                            Err(de::Error::invalid_length(
                                self.count + remaining,
                                &ExpectedInMap(self.count),
                            ))
                        }
                    }
                }

                impl<'de, I, E> MapDeserializer<'de, I, E> where
                    I: Iterator,
                    I::Item: private::Pair,
                {
                    fn next_pair(&mut self) -> Option<(First<I::Item>, Second<I::Item>)> {
                        match self.iter.next() {
                            Some(kv) => {
                                self.count += 1;
                                Some(private::Pair::split(kv))
                            }
                            None => None,
                        }
                    }
                }

                impl<'de, I, E> de::Deserializer<'de> for MapDeserializer<'de, I, E> where
                    I: Iterator,
                    I::Item: private::Pair,
                    First<I::Item>: IntoDeserializer<'de, E>,
                    Second<I::Item>: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Error = E;

                    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        let value = tri!(visitor.visit_map(&mut self));
                        tri!(self.end());
                        Ok(value)
                    }

                    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        let value = tri!(visitor.visit_seq(&mut self));
                        tri!(self.end());
                        Ok(value)
                    }

                    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        let _ = len;
                        self.deserialize_seq(visitor)
                    }

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct tuple_struct map
                        struct enum identifier ignored_any
                    }
                }

                impl<'de, I, E> IntoDeserializer<'de, E> for MapDeserializer<'de, I, E> where
                    I: Iterator,
                    I::Item: private::Pair,
                    First<I::Item>: IntoDeserializer<'de, E>,
                    Second<I::Item>: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Deserializer = Self;

                    fn into_deserializer(self) -> Self {
                        self
                    }
                }

                impl<'de, I, E> de::MapAccess<'de> for MapDeserializer<'de, I, E> where
                    I: Iterator,
                    I::Item: private::Pair,
                    First<I::Item>: IntoDeserializer<'de, E>,
                    Second<I::Item>: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Error = E;

                    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where
                        T: de::DeserializeSeed<'de>,
                    {
                        match self.next_pair() {
                            Some((key, value)) => {
                                self.value = Some(value);
                                seed.deserialize(key.into_deserializer()).map(Some)
                            }
                            None => Ok(None),
                        }
                    }

                    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error> where
                        T: de::DeserializeSeed<'de>,
                    {
                        let value = self.value.take();
                        // Panic because this indicates a bug in the program rather than an
                        // expected failure.
                        let value = value.expect("MapAccess::next_value called before next_key");
                        seed.deserialize(value.into_deserializer())
                    }

                    fn next_entry_seed<TK, TV>(
                        &mut self,
                        kseed: TK,
                        vseed: TV,
                    ) -> Result<Option<(TK::Value, TV::Value)>, Self::Error> where
                        TK: de::DeserializeSeed<'de>,
                        TV: de::DeserializeSeed<'de>,
                    {
                        match self.next_pair() {
                            Some((key, value)) => {
                                let key = tri!(kseed.deserialize(key.into_deserializer()));
                                let value = tri!(vseed.deserialize(value.into_deserializer()));
                                Ok(Some((key, value)))
                            }
                            None => Ok(None),
                        }
                    }

                    fn size_hint(&self) -> Option<usize> {
                        size_hint::from_bounds(&self.iter)
                    }
                }

                impl<'de, I, E> de::SeqAccess<'de> for MapDeserializer<'de, I, E> where
                    I: Iterator,
                    I::Item: private::Pair,
                    First<I::Item>: IntoDeserializer<'de, E>,
                    Second<I::Item>: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Error = E;

                    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where
                        T: de::DeserializeSeed<'de>,
                    {
                        match self.next_pair() {
                            Some((k, v)) => {
                                let de = PairDeserializer(k, v, PhantomData);
                                seed.deserialize(de).map(Some)
                            }
                            None => Ok(None),
                        }
                    }

                    fn size_hint(&self) -> Option<usize> {
                        size_hint::from_bounds(&self.iter)
                    }
                }

                // Cannot #[derive(Clone)] because of the bound `Second<I::Item>: Clone`.
                impl<'de, I, E> Clone for MapDeserializer<'de, I, E> where
                    I: Iterator + Clone,
                    I::Item: private::Pair,
                    Second<I::Item>: Clone,
                {
                    fn clone(&self) -> Self {
                        MapDeserializer {
                            iter: self.iter.clone(),
                            value: self.value.clone(),
                            count: self.count,
                            lifetime: self.lifetime,
                            error: self.error,
                        }
                    }
                }

                impl<'de, I, E> Debug for MapDeserializer<'de, I, E> where
                    I: Iterator + Debug,
                    I::Item: private::Pair,
                    Second<I::Item>: Debug,
                {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter
                            .debug_struct("MapDeserializer")
                            .field("iter", &self.iter)
                            .field("value", &self.value)
                            .field("count", &self.count)
                            .finish()
                    }
                }

                // Used in the `impl SeqAccess for MapDeserializer` to visit the map as a
                // sequence of pairs.
                struct PairDeserializer<A, B, E>(A, B, PhantomData<E>);

                impl<'de, A, B, E> de::Deserializer<'de> for PairDeserializer<A, B, E> where
                    A: IntoDeserializer<'de, E>,
                    B: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Error = E;

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct tuple_struct map
                        struct enum identifier ignored_any
                    }

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        self.deserialize_seq(visitor)
                    }

                    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        let mut pair_visitor = PairVisitor(Some(self.0), Some(self.1), PhantomData);
                        let pair = tri!(visitor.visit_seq(&mut pair_visitor));
                        if pair_visitor.1.is_none() {
                            Ok(pair)
                        } else {
                            let remaining = pair_visitor.size_hint().unwrap();
                            // First argument is the number of elements in the data, second
                            // argument is the number of elements expected by the Deserialize.
                            Err(de::Error::invalid_length(2, &ExpectedInSeq(2 - remaining)))
                        }
                    }

                    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        if len == 2 {
                            self.deserialize_seq(visitor)
                        } else {
                            // First argument is the number of elements in the data, second
                            // argument is the number of elements expected by the Deserialize.
                            Err(de::Error::invalid_length(2, &ExpectedInSeq(len)))
                        }
                    }
                }

                struct PairVisitor<A, B, E>(Option<A>, Option<B>, PhantomData<E>);

                impl<'de, A, B, E> de::SeqAccess<'de> for PairVisitor<A, B, E> where
                    A: IntoDeserializer<'de, E>,
                    B: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Error = E;

                    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where
                        T: de::DeserializeSeed<'de>,
                    {
                        if let Some(k) = self.0.take() {
                            seed.deserialize(k.into_deserializer()).map(Some)
                        } else if let Some(v) = self.1.take() {
                            seed.deserialize(v.into_deserializer()).map(Some)
                        } else {
                            Ok(None)
                        }
                    }

                    fn size_hint(&self) -> Option<usize> {
                        if self.0.is_some() {
                            Some(2)
                        } else if self.1.is_some() {
                            Some(1)
                        } else {
                            Some(0)
                        }
                    }
                }

                struct ExpectedInMap(usize);

                impl Expected for ExpectedInMap {
                    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        if self.0 == 1 {
                            formatter.write_str("1 element in map")
                        } else {
                            write!(formatter, "{} elements in map", self.0)
                        }
                    }
                }

                ////////////////////////////////////////////////////////////////////////////////

                #[cfg(any(feature = "std", feature = "alloc"))]
                #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
                impl<'de, K, V, E> IntoDeserializer<'de, E> for BTreeMap<K, V> where
                    K: IntoDeserializer<'de, E> + Eq + Ord,
                    V: IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Deserializer = MapDeserializer<'de, <Self as IntoIterator>::IntoIter, E>;

                    fn into_deserializer(self) -> Self::Deserializer {
                        MapDeserializer::new(self.into_iter())
                    }
                }

                #[cfg(feature = "std")]
                #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
                impl<'de, K, V, S, E> IntoDeserializer<'de, E> for HashMap<K, V, S> where
                    K: IntoDeserializer<'de, E> + Eq + Hash,
                    V: IntoDeserializer<'de, E>,
                    S: BuildHasher,
                    E: de::Error,
                {
                    type Deserializer = MapDeserializer<'de, <Self as IntoIterator>::IntoIter, E>;

                    fn into_deserializer(self) -> Self::Deserializer {
                        MapDeserializer::new(self.into_iter())
                    }
                }

                ////////////////////////////////////////////////////////////////////////////////

                /// A deserializer holding a `MapAccess`.
                #[derive(Clone, Debug)]
                pub struct MapAccessDeserializer<A> {
                    map: A,
                }

                impl<A> MapAccessDeserializer<A> {
                    /// Construct a new `MapAccessDeserializer<A>`.
                    pub fn new(map: A) -> Self {
                        MapAccessDeserializer { map }
                    }
                }

                impl<'de, A> de::Deserializer<'de> for MapAccessDeserializer<A> where
                    A: de::MapAccess<'de>,
                {
                    type Error = A::Error;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        visitor.visit_map(self.map)
                    }

                    fn deserialize_enum<V>(
                        self,
                        _name: &str,
                        _variants: &'static [&'static str],
                        visitor: V,
                    ) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        visitor.visit_enum(self)
                    }

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct identifier ignored_any
                    }
                }

                impl<'de, A> IntoDeserializer<'de, A::Error> for MapAccessDeserializer<A> where
                    A: de::MapAccess<'de>,
                {
                    type Deserializer = Self;

                    fn into_deserializer(self) -> Self {
                        self
                    }
                }

                impl<'de, A> de::EnumAccess<'de> for MapAccessDeserializer<A> where
                    A: de::MapAccess<'de>,
                {
                    type Error = A::Error;
                    type Variant = private::MapAsEnum<A>;

                    fn variant_seed<T>(mut self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error> where
                        T: de::DeserializeSeed<'de>,
                    {
                        match tri!(self.map.next_key_seed(seed)) {
                            Some(key) => Ok((key, private::map_as_enum(self.map))),
                            None => Err(de::Error::invalid_type(de::Unexpected::Map, &"enum")),
                        }
                    }
                }

                ////////////////////////////////////////////////////////////////////////////////

                /// A deserializer holding an `EnumAccess`.
                #[derive(Clone, Debug)]
                pub struct EnumAccessDeserializer<A> {
                    access: A,
                }

                impl<A> EnumAccessDeserializer<A> {
                    /// Construct a new `EnumAccessDeserializer<A>`.
                    pub fn new(access: A) -> Self {
                        EnumAccessDeserializer { access }
                    }
                }

                impl<'de, A> de::Deserializer<'de> for EnumAccessDeserializer<A> where
                    A: de::EnumAccess<'de>,
                {
                    type Error = A::Error;

                    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                        V: de::Visitor<'de>,
                    {
                        visitor.visit_enum(self.access)
                    }

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                        bytes byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map struct enum identifier ignored_any
                    }
                }

                impl<'de, A> IntoDeserializer<'de, A::Error> for EnumAccessDeserializer<A> where
                    A: de::EnumAccess<'de>,
                {
                    type Deserializer = Self;

                    fn into_deserializer(self) -> Self {
                        self
                    }
                }

                ////////////////////////////////////////////////////////////////////////////////

                mod private {
                    use crate::lib::*;

                    use crate::de::{
                        self, DeserializeSeed, Deserializer, MapAccess, Unexpected, VariantAccess, Visitor,
                    };

                    pub struct UnitOnly<E> {
                        marker: PhantomData<E>,
                    }

                    pub fn unit_only<T, E>(t: T) -> (T, UnitOnly<E>) {
                        (
                            t,
                            UnitOnly {
                                marker: PhantomData,
                            },
                        )
                    }

                    impl<'de, E> de::VariantAccess<'de> for UnitOnly<E> where
                        E: de::Error,
                    {
                        type Error = E;

                        fn unit_variant(self) -> Result<(), Self::Error> {
                            Ok(())
                        }

                        fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
                        where
                            T: de::DeserializeSeed<'de>,
                        {
                            Err(de::Error::invalid_type(
                                Unexpected::UnitVariant,
                                &"newtype variant",
                            ))
                        }

                        fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
                        where
                            V: de::Visitor<'de>,
                        {
                            Err(de::Error::invalid_type(
                                Unexpected::UnitVariant,
                                &"tuple variant",
                            ))
                        }

                        fn struct_variant<V>(
                            self,
                            _fields: &'static [&'static str],
                            _visitor: V,
                        ) -> Result<V::Value, Self::Error>
                        where
                            V: de::Visitor<'de>,
                        {
                            Err(de::Error::invalid_type(
                                Unexpected::UnitVariant,
                                &"struct variant",
                            ))
                        }
                    }

                    pub struct MapAsEnum<A> {
                        map: A,
                    }

                    pub fn map_as_enum<A>(map: A) -> MapAsEnum<A> {
                        MapAsEnum { map }
                    }

                    impl<'de, A> VariantAccess<'de> for MapAsEnum<A> where
                        A: MapAccess<'de>,
                    {
                        type Error = A::Error;

                        fn unit_variant(mut self) -> Result<(), Self::Error> {
                            self.map.next_value()
                        }

                        fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
                        where
                            T: DeserializeSeed<'de>,
                        {
                            self.map.next_value_seed(seed)
                        }

                        fn tuple_variant<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
                        where
                            V: Visitor<'de>,
                        {
                            self.map.next_value_seed(SeedTupleVariant { len, visitor })
                        }

                        fn struct_variant<V>(
                            mut self,
                            _fields: &'static [&'static str],
                            visitor: V,
                        ) -> Result<V::Value, Self::Error>
                        where
                            V: Visitor<'de>,
                        {
                            self.map.next_value_seed(SeedStructVariant { visitor })
                        }
                    }

                    struct SeedTupleVariant<V> {
                        len: usize,
                        visitor: V,
                    }

                    impl<'de, V> DeserializeSeed<'de> for SeedTupleVariant<V> where
                        V: Visitor<'de>,
                    {
                        type Value = V::Value;

                        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                        where
                            D: Deserializer<'de>,
                        {
                            deserializer.deserialize_tuple(self.len, self.visitor)
                        }
                    }

                    struct SeedStructVariant<V> {
                        visitor: V,
                    }

                    impl<'de, V> DeserializeSeed<'de> for SeedStructVariant<V> where
                        V: Visitor<'de>,
                    {
                        type Value = V::Value;

                        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                        where
                            D: Deserializer<'de>,
                        {
                            deserializer.deserialize_map(self.visitor)
                        }
                    }

                    /// Avoid having to restate the generic types on `MapDeserializer`. The
                    /// `Iterator::Item` contains enough information to figure out K and V.
                    pub trait Pair {
                        type First;
                        type Second;
                        fn split(self) -> (Self::First, Self::Second);
                    }

                    impl<A, B> Pair for (A, B) {
                        type First = A;
                        type Second = B;
                        fn split(self) -> (A, B) {
                            self
                        }
                    }

                    pub type First<T> = <T as Pair>::First;
                    pub type Second<T> = <T as Pair>::Second;
                }
            }

            mod ignored_any
            {
                use ::
                {
                    *,
                };
            }

            pub use self::ignored_any::IgnoredAny;

            mod impls
            {
                use ::
                {
                    *,
                };
            }
            
            declare_error_trait!(Error: Sized + StdError);
            
            #[derive(Copy, Clone, PartialEq, Debug)]
            pub enum Unexpected<'a> 
            {
                Bool(bool),
                Unsigned(u64),
                Signed(i64),
                Float(f64),
                Char(char),
                Str(&'a str),
                Bytes(&'a [u8]),
                Unit,
                Option,
                NewtypeStruct,
                Seq,
                Map,
                Enum,
                UnitVariant,
                NewtypeVariant,
                TupleVariant,
                StructVariant,
                Other(&'a str),
            }

            impl<'a> fmt::Display for Unexpected<'a> 
            {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result 
                {
                    use self::Unexpected::*;
                    match *self 
                    {
                        Bool(b) => write!(formatter, "boolean `{}`", b),
                        Unsigned(i) => write!(formatter, "integer `{}`", i),
                        Signed(i) => write!(formatter, "integer `{}`", i),
                        Float(f) => write!(formatter, "floating point `{}`", WithDecimalPoint(f)),
                        Char(c) => write!(formatter, "character `{}`", c),
                        Str(s) => write!(formatter, "string {:?}", s),
                        Bytes(_) => formatter.write_str("byte array"),
                        Unit => formatter.write_str("unit value"),
                        Option => formatter.write_str("Option value"),
                        NewtypeStruct => formatter.write_str("newtype struct"),
                        Seq => formatter.write_str("sequence"),
                        Map => formatter.write_str("map"),
                        Enum => formatter.write_str("enum"),
                        UnitVariant => formatter.write_str("unit variant"),
                        NewtypeVariant => formatter.write_str("newtype variant"),
                        TupleVariant => formatter.write_str("tuple variant"),
                        StructVariant => formatter.write_str("struct variant"),
                        Other(other) => formatter.write_str(other),
                    }
                }
            }
            
            pub trait Expected 
            {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result;
            }

            impl<'de, T> Expected for T where
            T: Visitor<'de>,
            {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result { self.expecting(formatter) }
            }

            impl Expected for &str
            {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result { formatter.write_str(self) }
            }

            impl Display for dyn Expected + '_
            {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result { Expected::fmt(self, formatter) }
            }
            
            pub trait Deserialize<'de>: Sized 
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>;
                
                fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where
                D: Deserializer<'de>,
                {
                    *place = tri!(Deserialize::deserialize(deserializer));
                    Ok(())
                }
            }
            
            pub trait DeserializeOwned: for<'de> Deserialize<'de> {}

            impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de> {}
            
            pub trait DeserializeSeed<'de>: Sized 
            {
                type Value;
                fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de>;
            }

            impl<'de, T> DeserializeSeed<'de> for PhantomData<T> where
            T: Deserialize<'de>,
            {
                type Value = T;

                #[inline] fn deserialize<D>(self, deserializer: D) -> Result<T, D::Error> where
                D: Deserializer<'de>,
                { T::deserialize(deserializer) }
            }
            
            pub trait Deserializer<'de>: Sized
            {
                type Error: Error;
                fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                
                fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                V: Visitor<'de>,
                {
                    let _ = visitor;
                    Err(Error::custom("i128 is not supported"))
                }
                
                fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;                
                fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                
                fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                V: Visitor<'de>,
                {
                    let _ = visitor;
                    Err(Error::custom("u128 is not supported"))
                }

                fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;                
                fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_unit_struct<V>( self, name: &'static str, visitor: V ) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_newtype_struct<V>( self, name: &'static str, visitor: V ) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_tuple_struct<V>( self, name: &'static str, len: usize, visitor: V ) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_struct<V>( self, name: &'static str, fields: &'static [&'static str], visitor: V ) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_enum<V>( self, name: &'static str, variants: &'static [&'static str], visitor: V ) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;

                #[inline] fn is_human_readable(&self) -> bool { true }
                fn __deserialize_content_v1<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
                V: Visitor<'de, Value = ::core::serde::Content<'de>>
                { self.deserialize_any(visitor) }
            }
            
            pub trait Visitor<'de>: Sized
            {
                type Value;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;
                
                fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where
                E: Error
                { Err(Error::invalid_type(Unexpected::Bool(v), &self)) }
                
                fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where
                E: Error
                { self.visit_i64(v as i64) }
                
                fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where
                E: Error
                { self.visit_i64(v as i64) }
                
                fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where
                E: Error
                { self.visit_i64(v as i64) }
                
                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where
                E: Error
                { Err(Error::invalid_type(Unexpected::Signed(v), &self)) }
                
                fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where
                E: Error
                {
                    let mut buf = [0u8; 58];
                    let mut writer = ::core::serde::format::Buf::new(&mut buf);
                    fmt::Write::write_fmt(&mut writer, format_args!("integer `{}` as i128", v)).unwrap();
                    Err( Error::invalid_type( Unexpected::Other(writer.as_str()), &self ) )
                }
                
                fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where
                E: Error
                { self.visit_u64(v as u64) }
                
                fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where
                E: Error
                { self.visit_u64(v as u64) }
                
                fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where
                E: Error
                { self.visit_u64(v as u64) }
                
                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where
                E: Error
                { Err(Error::invalid_type(Unexpected::Unsigned(v), &self)) }
                
                fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where
                E: Error
                {
                    let mut buf = [0u8; 57];
                    let mut writer = ::core::serde::format::Buf::new(&mut buf);
                    fmt::Write::write_fmt(&mut writer, format_args!("integer `{}` as u128", v)).unwrap();
                    Err( Error::invalid_type( Unexpected::Other(writer.as_str()), &self ) )
                }
                
                fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where
                E: Error
                { self.visit_f64(v as f64) }
                
                fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where
                E: Error
                { Err(Error::invalid_type(Unexpected::Float(v), &self)) }
                
                #[inline] fn visit_char<E>(self, v: char) -> Result<Self::Value, E> where
                E: Error
                { self.visit_str(v.encode_utf8(&mut [0u8; 4])) }
                
                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                E: Error
                { Err(Error::invalid_type(Unexpected::Str(v), &self)) }
                
                #[inline] fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where
                E: Error
                { self.visit_str(v) }
                
                #[inline] fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where
                E: Error
                { self.visit_str(&v) }
                
                fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where
                E: Error
                { Err(Error::invalid_type(Unexpected::Bytes(v), &self)) }
                
                #[inline] fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> where
                E: Error
                { self.visit_bytes(v) }
                
                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where
                E: Error
                { self.visit_bytes(&v) }
                
                fn visit_none<E>(self) -> Result<Self::Value, E> where
                E: Error
                { Err(Error::invalid_type(Unexpected::Option, &self)) }
                
                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where
                D: Deserializer<'de>
                {
                    let _ = deserializer;
                    Err(Error::invalid_type(Unexpected::Option, &self))
                }
                
                fn visit_unit<E>(self) -> Result<Self::Value, E> where
                E: Error
                { Err(Error::invalid_type(Unexpected::Unit, &self)) }
                
                fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where
                D: Deserializer<'de>
                {
                    let _ = deserializer;
                    Err(Error::invalid_type(Unexpected::NewtypeStruct, &self))
                }
                
                fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where
                A: SeqAccess<'de>
                {
                    let _ = seq;
                    Err(Error::invalid_type(Unexpected::Seq, &self))
                }
                
                fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where
                A: MapAccess<'de>
                {
                    let _ = map;
                    Err(Error::invalid_type(Unexpected::Map, &self))
                }
                
                fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error> where
                A: EnumAccess<'de>,
                {
                    let _ = data;
                    Err(Error::invalid_type(Unexpected::Enum, &self))
                }
                
                fn __private_visit_untagged_option<D>(self, _: D) -> Result<Self::Value, ()> where
                D: Deserializer<'de>,
                { Err(()) }
            }
            
            pub trait SeqAccess<'de> 
            {
                type Error: Error;
                fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: DeserializeSeed<'de>;
                
                #[inline] fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error> where
                T: Deserialize<'de>
                { self.next_element_seed(PhantomData) }
                
                #[inline] fn size_hint(&self) -> Option<usize> { None }
            }

            impl<'de, A> SeqAccess<'de> for &mut A where
            A: ?Sized + SeqAccess<'de>
            {
                type Error = A::Error;

                #[inline] fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where
                T: DeserializeSeed<'de>,
                { (**self).next_element_seed(seed) }

                #[inline] fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error> where
                T: Deserialize<'de>
                { (**self).next_element() }

                #[inline] fn size_hint(&self) -> Option<usize> { (**self).size_hint() }
            }
            
            pub trait MapAccess<'de> 
            {
                type Error: Error;
                fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> where K: DeserializeSeed<'de>;
                fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error> where V: DeserializeSeed<'de>;
                
                #[inline] fn next_entry_seed<K, V>( &mut self, kseed: K, vseed: V ) -> Result<Option<(K::Value, V::Value)>, Self::Error> where
                K: DeserializeSeed<'de>,
                V: DeserializeSeed<'de>
                {
                    match tri!(self.next_key_seed(kseed))
                    {
                        Some(key) =>
                        {
                            let value = tri!(self.next_value_seed(vseed));
                            Ok(Some((key, value)))
                        }

                        None => Ok(None),
                    }
                }
                
                #[inline] fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error> where
                K: Deserialize<'de>
                { self.next_key_seed(PhantomData) }
                
                #[inline] fn next_value<V>(&mut self) -> Result<V, Self::Error> where
                V: Deserialize<'de>
                { self.next_value_seed(PhantomData) }
                
                #[inline] fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error> where
                K: Deserialize<'de>,
                V: Deserialize<'de>
                { self.next_entry_seed(PhantomData, PhantomData) }
                
                #[inline] fn size_hint(&self) -> Option<usize> { None }
            }

            impl<'de, A> MapAccess<'de> for &mut A where
            A: ?Sized + MapAccess<'de>
            {
                type Error = A::Error;

                #[inline] fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> where
                K: DeserializeSeed<'de>
                { (**self).next_key_seed(seed) }

                #[inline] fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error> where
                V: DeserializeSeed<'de>
                { (**self).next_value_seed(seed) }

                #[inline] fn next_entry_seed<K, V>( &mut self, kseed: K, vseed: V ) -> Result<Option<(K::Value, V::Value)>, Self::Error> where
                K: DeserializeSeed<'de>,
                V: DeserializeSeed<'de>
                { (**self).next_entry_seed(kseed, vseed) }

                #[inline] fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error> where
                K: Deserialize<'de>,
                V: Deserialize<'de>
                { (**self).next_entry() }

                #[inline] fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error> where
                K: Deserialize<'de>
                { (**self).next_key() }

                #[inline] fn next_value<V>(&mut self) -> Result<V, Self::Error> where
                V: Deserialize<'de>
                { (**self).next_value() }

                #[inline] fn size_hint(&self) -> Option<usize> { (**self).size_hint() }
            }
            
            pub trait EnumAccess<'de>: Sized 
            {
                type Error: Error;
                type Variant: VariantAccess<'de, Error = Self::Error>;
                fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> where V: DeserializeSeed<'de>;
                
                #[inline] fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error> where
                V: Deserialize<'de>
                { self.variant_seed(PhantomData) }
            }
            
            pub trait VariantAccess<'de>: Sized 
            {
                type Error: Error;
                fn unit_variant(self) -> Result<(), Self::Error>;
                fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error> where T: DeserializeSeed<'de>;
                
                #[inline] fn newtype_variant<T>(self) -> Result<T, Self::Error> where
                T: Deserialize<'de>
                { self.newtype_variant_seed(PhantomData) }
                
                fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
                fn struct_variant<V>( self, fields: &'static [&'static str], visitor: V ) -> Result<V::Value, Self::Error> where V: Visitor<'de>;
            }
            
            pub trait IntoDeserializer<'de, E: Error = value::Error> 
            {
                type Deserializer: Deserializer<'de, Error = E>;
                fn into_deserializer(self) -> Self::Deserializer;
            }
            
            struct OneOf 
            {
                names: &'static [&'static str],
            }

            impl Display for OneOf 
            {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result 
                {
                    match self.names.len() 
                    {
                        0 => panic!(),
                        1 => write!(formatter, "`{}`", self.names[0]),
                        2 => write!(formatter, "`{}` or `{}`", self.names[0], self.names[1]),
                        _ => 
                        {
                            tri!(formatter.write_str("one of "));
                            for (i, alt) in self.names.iter().enumerate()
                            {
                                if i > 0 { tri!(formatter.write_str(", ")); }
                                tri!(write!(formatter, "`{}`", alt));
                            }

                            Ok(())
                        }
                    }
                }
            }

            struct WithDecimalPoint(f64);

            impl Display for WithDecimalPoint 
            {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result 
                {
                    struct LookForDecimalPoint<'f, 'a> 
                    {
                        formatter: &'f mut fmt::Formatter<'a>,
                        has_decimal_point: bool,
                    }

                    impl<'f, 'a> fmt::Write for LookForDecimalPoint<'f, 'a> 
                    {
                        fn write_str(&mut self, fragment: &str) -> fmt::Result 
                        {
                            self.has_decimal_point |= fragment.contains('.');
                            self.formatter.write_str(fragment)
                        }

                        fn write_char(&mut self, ch: char) -> fmt::Result 
                        {
                            self.has_decimal_point |= ch == '.';
                            self.formatter.write_char(ch)
                        }
                    }

                    if self.0.is_finite() 
                    {
                        let mut writer = LookForDecimalPoint 
                        {
                            formatter,
                            has_decimal_point: false,
                        };

                        tri!(write!(writer, "{}", self.0));
                        
                        if !writer.has_decimal_point { tri!(formatter.write_str(".0")); }
                    }

                    else { tri!(write!(formatter, "{}", self.0)); }

                    Ok(())
                }
            }
        }

        pub mod ser
        {
            use ::
            {
                *,
            };
        }
    }
}

pub mod csv
{
    use ::
    {
        *,
    };
}

pub mod error
{
    pub use std::error::{ * };
}

pub mod ffi
{
    pub use std::ffi::{ * };
}

pub mod fmt
{
    pub use std::fmt::{ * };
}

pub mod hash
{
    pub use std::hash::{ * };
}

pub mod io
{
    pub use std::io::{ * };
}

pub mod iter
{
    pub use std::iter::{ * };
}

pub mod marker
{
    pub use std::marker::{ * };
}

pub mod mem
{
    pub use std::mem::{ * };
}

pub mod net
{
    pub use std::net::{ * };
}

pub mod num
{
    pub use std::num::{ * };
}

pub mod ops
{
    pub use std::ops::{ * };
}

pub mod path
{
    pub use std::path::{ * };
}

pub mod rc
{
    pub use std::rc::{ * };
}

pub mod result
{
    pub use std::result::{ * };
}

pub mod str
{
    pub use std::str::{ * };
}

pub mod string
{
    pub use std::string::{ * };
}

pub mod sync
{
    pub use std::sync::{ * };
}

pub mod time
{
    pub use std::time::{ * };
}

pub mod vec
{
    pub use std::vec::{ * };
}

pub unsafe fn domain()
{
    unsafe
    {
        let who = Ident::who();
        println!( r#"{:?}"#, who.antonyms() );
    }
}

pub fn main()
{
    unsafe
    {
        domain();        
    }
}
