#[derive(Debug, Clone, PartialEq, Eq)] pub struct Record { pub id: String, pub kind: String }
impl Record { pub fn new(id: &str, kind: &str) -> Result<Self, &'static str> { if id.is_empty() || kind.is_empty() { Err("fields are required") } else { Ok(Self { id: id.into(), kind: kind.into() }) } } }
#[cfg(test)] mod tests { use super::*; #[test] fn validates() { assert!(Record::new("1", "domain").is_ok()); assert!(Record::new("", "domain").is_err()); } }
