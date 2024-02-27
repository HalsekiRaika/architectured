use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::entities::BookId;
use crate::error::KernelError;
use crate::event::{Applier, PersonEvent};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rental(HashSet<BookId>);

impl Rental {
    pub fn new(rental: impl Into<HashSet<BookId>>) -> Rental {
        Self(rental.into())
    }
    
    pub fn add(&mut self, id: BookId) -> Result<(), KernelError> {
        if !self.0.insert(id) { 
            return Err(KernelError::Validate {
                entity: "kernel::entities::person::Rental",
                source: "duplicate Books cannot be borrowed".to_string(),
            })
        }
        Ok(())
    }
    
    pub fn remove(&mut self, id: &BookId) -> Result<(), KernelError> {
        if !self.0.remove(id) { 
            return Err(KernelError::Validate {
                entity: "kernel::entities::person::Rental",
                source: "key does not exist, at least one must exist.".to_string(),
            })
        }
        Ok(())
    }
}

impl From<Rental> for HashSet<BookId> {
    fn from(value: Rental) -> Self {
        value.0
    }
}

impl IntoIterator for Rental {
    type Item = BookId;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl AsRef<HashSet<BookId>> for Rental {
    fn as_ref(&self) -> &HashSet<BookId> {
        &self.0
    }
}

#[allow(unreachable_patterns)]
impl Applier<PersonEvent> for Rental {
    fn apply(&mut self, event: PersonEvent) {
        match event {
            PersonEvent::Rented { id, .. } => { self.0.insert(id); },
            PersonEvent::Returned { ref id, .. } => { self.0.remove(id); }
            _ => {}
        }
    }
}



#[cfg(test)]
mod test {
    use error_stack::Report;
    use crate::entities::{BookId, Rental};
    use crate::error::test::AnyKernelError;
    
    #[test]
    fn add() -> Result<(), Report<AnyKernelError>> {
        let mut rental = Rental::default();
        let book = BookId::default();
        
        rental.add(book)?;
        
        Ok(())
    }
    
    #[test]
    #[should_panic]
    fn add_panic() {
        let mut rental = Rental::default();
        let book = BookId::default();
        
        rental.add(book).unwrap();
        rental.add(book).unwrap();
    }
    
    #[test]
    fn remove() -> Result<(), Report<AnyKernelError>> {
        let mut rental = Rental::default();
        let book = BookId::default();
        
        rental.add(book)?;
        rental.remove(&book)?;
        
        Ok(())
    }
    
    
    #[test]
    #[should_panic]
    fn remove_panic() {
        let mut rental = Rental::default();
        let book = BookId::default();
        
        rental.remove(&book).unwrap();
    }
}