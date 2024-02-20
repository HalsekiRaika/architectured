use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Stock(i32);

impl Stock {
    pub fn new(stock: impl Into<i32>) -> Stock {
        Self(stock.into())
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