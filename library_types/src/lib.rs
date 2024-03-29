pub mod home_types;
pub mod epub_type;

use std::path::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
macro_rules! stru {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
        pub struct $name {
            $(pub $field: $t),*}}}

macro_rules! en {
    ($name:ident {$($variant:ident,)*}) => {
        #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
        pub enum $name {
            #[default] $($variant),*} } }

pub type Books = Vec<LibBook>;
pub type Contents = Vec<Nav>;
pub type IDs = Vec<Identifier>;
pub type Dirs = Vec<Dir>;
pub type Subjects = Vec<Subject>;
pub type Creators = Vec<Creator>;
pub type Authors = Vec<Author>;
pub type Strings = Vec<String>;

stru!(Book {
  uuid:String,	ids:  IDs,		publisher:Publisher,
  desc:String,	len:  String,	subjects: Subjects,
  path:PathBuf,	publ: String,	creators: Creators,
  pos: String,	title:String,	contents: Contents,
  dir: String,
});

stru!(LibBook   {uuid:String,		title:String,		progress:u8,	});
stru!(BookMData {desc:String,		pos:String,			contents:Contents,publ:u32,		ids:IDs,});
stru!(BookRel   {authors:Authors,	subjects:Subjects,	publisher:Publisher,});

stru!(ParseBook{
	book:LibBook,		mdata:BookMData,authors:Vec<String>,
	name:String,		subjects:Strings,
	publisher:Strings,	language:String,
});

stru!( Publisher {uuid:String,	name:String, 	});
stru!( Subject   {uuid:String,	name:String, 	});
stru!( Author    {uuid:String,	name:String,    });
stru!( Creator   {uuid:String,	name:String,	role:CreatorRole,});
stru!( Nav       {name:String,	href:String,   	childs:Vec<Nav>,});

stru!(	Dir{	uuid:String,	name:String,   	parent:String,		});

en!( CreatorRole {Author, Translator, Contributor, Narrator,	});
#[derive(Copy, Clone)]
pub enum Container {
	Creator = 0,	Subject = 1,	Publisher = 2,	Language = 3
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub enum Identifier {
	ISBN(String), Asin(String), GOOG(String),
	UUID(String), MISC(String), #[default] None,
}

impl Book {
    pub fn new(path: &Path, dir: &str) -> Self {
		Self {
	    	uuid: 	uuid(),
	    	path: 	path.into(),
	    	dir: 	dir.into(),
	    	..Default::default()
		}
    }
}

impl Nav {
    pub fn new(name: &str, href: &str, childs: Vec<Nav>) -> Self {
	Self { name: name.into(), href: href.into(), childs }
    }
}

impl Subject {
    pub fn new(name: &str) -> Self {
	Self { uuid: uuid(), name: name.to_string() }
    }
}


fn uuid() -> String {
    Uuid::new_v4().to_string()
}



