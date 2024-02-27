use serde::{Deserialize, Serialize};
use crate::error::KernelError;
use crate::event::{Applier, BookEvent};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct Stock(i32);

impl Stock {
    pub fn new(stock: impl Into<i32>) -> Result<Stock, KernelError> {
        let stock = stock.into();
        
        if stock < 0 { 
            return Err(KernelError::Validate {
                entity: "Stock",
                source: "stock should be more than 1.".to_string(),
            })
        }
        
        Ok(Self(stock))
    }
    
    pub fn ret(&mut self) {
        self.0 += 1;
    }
    
    pub fn rent(&mut self) -> Result<(), KernelError> {
        if (self.0 - 1) < 0 {
            return Err(KernelError::Validate {
                entity: "kernel::entities::book::Stock",
                source: "stock of book will not be less than 0.".to_string(),
            })
        }
        
        self.0 -= 1;
        
        Ok(())
    }
}

impl From<Stock> for i32 {
    fn from(value: Stock) -> Self {
        value.0
    }
}

impl AsRef<i32> for Stock {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl Default for Stock {
    fn default() -> Self {
        Self(10)
    }
}

#[allow(unreachable_patterns)]
impl Applier<BookEvent> for Stock {
    fn apply(&mut self, event: BookEvent) {
        match event {
            BookEvent::Discarded { stock, .. } => self.0 -= stock.0,
            BookEvent::Rental { .. } => self.0 -= 1,
            BookEvent::Return { .. } => self.0 += 1,
            _ => {}
        }
    }
}


#[cfg(test)]
mod test {
    use error_stack::Report;
    use crate::entities::Stock;
    use crate::error::test::AnyKernelError;
    
    #[test]
    #[should_panic]
    fn init_panic() {
        let _wtf = Stock::new(-1).unwrap();
    }
    
    #[test]
    fn rental() -> Result<(), Report<AnyKernelError>> {
        let mut stock = Stock::new(5)?;
        
        for _ in 0..5 {
            stock.rent()?;
        }
        
        Ok(())
    }
    
    #[test]
    #[should_panic]
    fn rental_panic() {
        let mut stock = Stock::new(1).unwrap();
        stock.rent().unwrap();
        stock.rent().unwrap();
    }
}