use std::fmt;

pub trait EntityTrait {
    fn is_valid(&mut self) -> bool;
    fn fmt_insert(&mut self) -> &Self;
}

#[derive(Debug)]
pub struct EntityError;

impl fmt::Display for EntityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

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
}
