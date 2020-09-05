use crate::parse::Chunk;
use crate::dtypes::Person;

enum Pagination {
    Page,
    Column,
    Line,
    Verse,
    Section,
    Parapgraph,
}

enum EditorType {
    Editor,
    Compiler,
    Founder,
    Continuator,
    Redactor,
    Reviser,
    Collaborator,
    Organizer,
}

enum Gender {
    SingularFemale,
    SingularMale,
    SingularNeuter,
    PluralFemale,
    PluralMale,
    PluralNeuter,
}

struct IntOrChunks {
    chunks: Option<Vec<Chunk>>,
    int: i64,
}

enum BiblatexFields {
    Abstract(Vec<Chunk>),
    Addendum(Vec<Chunk>),
    Afterword(Vec<Person>),
    Annotation(Vec<Chunk>),
    Annotator(Vec<Person>),
    Author(Vec<Person>),
    AuthorType(String),
    BookAuthor(Vec<Person>),
    BookPagination(Pagination),
    BookSubtitle(Vec<Chunk>),
    BookTitle(Vec<Chunk>),
    BookTitleAddon(Vec<Chunk>),
    Chapter(Vec<Chunk>),
    Commentator(Vec<Person>),
    // Date(Date),
    Doi(String),
    Edition(IntOrChunks),
    Editor(Vec<Person>),
    EditorA(Vec<Person>),
    EditorB(Vec<Person>),
    EditorC(Vec<Person>),
    EditorType(EditorType),
    EditorAType(EditorType),
    EditorBType(EditorType),
    EditorCType(EditorType),
    Eid(Vec<Chunk>),
    EntrySubtype(Vec<Chunk>),
    EPrint(String),
    EPrintClass(Vec<Chunk>),
    EPrintType(Vec<Chunk>),
    // EventDate(Date),
    EventTitle(Vec<Chunk>),
    EventTitleAddon(Vec<Chunk>),
    File(String),
    Foreword(Vec<Person>),
    Holder(Vec<Person>),
    HowPublished(Vec<Chunk>),
    IndexTitle(Vec<Chunk>),
    Institution(Vec<Chunk>),
    Introduction(Vec<Person>),
    ISAN(Vec<Chunk>),
    ISBN(Vec<Chunk>),
    ISMN(Vec<Chunk>),
    ISRN(Vec<Chunk>),
    ISSN(Vec<Chunk>),
    Issue(Vec<Chunk>),
    IssueSubtitle(Vec<Chunk>),
    IssueTitle(Vec<Chunk>),
    IssueTitleAddon(Vec<Chunk>),
    ISWC(Vec<Chunk>),
    JournalSubtitle(Vec<Chunk>),
    JournalTitle(Vec<Chunk>),
    JournalTitleAddon(Vec<Chunk>),
    Label(Vec<Chunk>),
    Language(String),
    Library(Vec<Chunk>),
    Location(Vec<Chunk>),
    MainSubTitle(Vec<Chunk>),
    MainTitle(Vec<Chunk>),
    MainTitleAddon(Vec<Chunk>),
    NameAddon(Vec<Chunk>),
    Note(Vec<Chunk>),
    Number(Vec<Chunk>),
    Organization(Vec<Vec<Chunk>>),
    // OrigDate(Date),
    OrigLanguage(String),
    OrigLocation(Vec<Chunk>),
    Pages(Vec<std::ops::Range<u32>>),
    PageTotal(Vec<Chunk>),
    Pagination(Pagination),
    Part(Vec<Chunk>),
    Publisher(Vec<Vec<Chunk>>),
    PubState(Vec<Chunk>),
    ReprintTitle(Vec<Chunk>),
    Series(Vec<Chunk>),
    ShortAuthor(Vec<Person>),
    ShortEditor(Vec<Person>),
    Shorthand(Vec<Chunk>),
    ShortSeries(Vec<Chunk>),
    ShortTitle(Vec<Chunk>),
    SubTitle(Vec<Chunk>),
    Title(Vec<Chunk>),
    TitleAddon(Vec<Chunk>),
    Translator(Vec<Person>),
    Type(Vec<Chunk>),
    Url(String),
    // UrlDate(Date),
    Venue(Vec<Chunk>),
    Version(Vec<Chunk>),
    Volume(Vec<Chunk>),
    Volumes(Vec<Chunk>),
    Gender(Vec<Gender>),
    Unknown(String, Vec<Chunk>),
}

enum BibtexFields {
    Address(Vec<Chunk>),
    Annote(Vec<Chunk>),
    Author(Vec<Person>),
    Booktitle(Vec<Chunk>),
    Chapter(Vec<Chunk>),
    // Date(Date),
    Edition(IntOrChunks),
    Editor(Vec<Person>),
    HowPublished(Vec<Chunk>),
    Institution(Vec<Chunk>),
    Journal(Vec<Chunk>),
    Note(Vec<Chunk>),
    Number(i64),
    Organization(Vec<Chunk>),
    Pages(Vec<std::ops::Range<u32>>),
    Publisher(Vec<Vec<Chunk>>),
    School(Vec<Chunk>),
    Series(Vec<Chunk>),
    Title(Vec<Chunk>),
    Type(Vec<Chunk>),
    Volume(Vec<Chunk>),

    Unknown(String, Vec<Chunk>),
}

