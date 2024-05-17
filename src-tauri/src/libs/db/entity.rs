use std::fmt;

pub trait EntityTrait {
    fn is_valid(&mut self) -> bool;
    fn fmt_insert(&mut self) -> Self;
}

pub trait Tess {
    fn lol() -> bool;
}

#[derive(Debug)]
struct EntityError;

impl fmt::Display for EntityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// ==================================
// ==================================
// ==================================
// ==================================

pub enum Entity {
    Account,
    Domain,
    Metadata,
    Proxy,
    Records,
    Record,
}

impl Entity {
    pub fn validate<T: EntityTrait>(&self, mut arg: T) -> anyhow::Result<T, EntityError> {
        if arg.is_valid() == false {
            return Err(EntityError);
        }

        arg.fmt_insert();

        Ok(arg)
    }

    pub fn name(self) -> &'static str {
        match self {
            Entity::Account => "account",
            Entity::Domain => "domain",
            Entity::Metadata => "metadata",
            Entity::Proxy => "proxy",
            Entity::Records => "records",
            Entity::Record => "record",
        }
    }

    // pub fn doc_to_entity<T: Tess>(&self, doc: Document) -> anyhow::Result<T> {
    //     match self {
    //         Entity::Account => from_document::<Account>(doc).context("gfdghj"),
    //         Entity::Domain => from_document::<Account>(doc),
    //         Entity::Metadata => from_document::<Account>(doc),
    //         Entity::Proxy => from_document::<Account>(doc),
    //         Entity::Records => from_document::<Account>(doc),
    //         Entity::Record => from_document::<Account>(doc),
    //     }
    // }
}
