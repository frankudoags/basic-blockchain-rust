#[allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

