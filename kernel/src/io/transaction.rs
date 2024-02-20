pub trait Transaction: Send {
    type Error: error_stack::Context;
    
    fn commit(self) -> impl std::future::Future<Output=Result<(), Self::Error>> + Send;
    fn rollback(self) -> impl std::future::Future<Output=Result<(), Self::Error>> + Send;
}


#[orbital::export_service]
pub trait AcquireTransaction: 'static + Sync + Send {
    type Transaction: Transaction;
    type Error: error_stack::Context;
    fn acquire(&self) -> impl std::future::Future<Output=Result<Self::Transaction, Self::Error>> + Send;
}

