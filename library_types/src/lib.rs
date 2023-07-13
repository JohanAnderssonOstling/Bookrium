pub mod home_types;
mod epub_type;
use std::path::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::Date;
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


pub type Books 	 = Vec<Book>; 		pub type Contents = Vec<Nav>;
pub type IDs 	 = Vec<Identifier>; 	pub type Dirs 	  = Vec<Dir>;
pub type Subjects= Vec<Subject>;	pub type Creators = Vec<Creator>;

stru!(Book {
  uuid:String,	ids:  IDs,	publisher:Publisher,
  desc:String,	len:  String,	subjects: Subjects,
  path:PathBuf,	publ: u32,	creators: Creators,
  pos: String,	title:String,	contents: Contents,
  dir: String, });

stru!(Publisher{uuid:String,	name:String, 	});
stru!(Subject{	uuid:String,	name:String, 	});
stru!(Creator{	uuid:String,	name:String,	role:CreatorRole,});
stru!(Nav{	name:String,	href:String,   	childs:Vec<Nav>,});

stru!(Dir{	uuid:String,	path:String,   	prev:String,	});

en!( CreatorRole {Author, Translator, Contributor, Narrator, });
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub enum Identifier { #[default] None, ISBN (String), Asin (String), GOOG (String), }

impl Book {
  pub fn new(path: &Path, dir: &str) -> Self {
    Self {
      uuid: uuid(), path: path.into(), dir: dir.into(),
      ..Default::default() } } }

impl Nav {
  pub fn new(name: &str, href: &str, childs: Vec<Nav>) -> Self {
    Self {name: name.into(), href: href.into(), childs} } }

impl Subject {
  pub fn new(name: &str) -> Self {
    Self {uuid:uuid(), name:name.to_string(), } } }



fn uuid() -> String {
  Uuid::new_v4().to_string() }



