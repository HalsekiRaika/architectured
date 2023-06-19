use kernel::interfaces::repository::DependOnPersonRepository;
use crate::services::{CreatePersonService, DeletePersonService, UpdatePersonService};

// Default Impl
impl<T> CreatePersonService for T
    where T: DependOnPersonRepository {}

// Default Impl
impl<T> UpdatePersonService for T
    where T: DependOnPersonRepository {}

// Default Impl
impl<T> DeletePersonService for T
    where T: DependOnPersonRepository {}
