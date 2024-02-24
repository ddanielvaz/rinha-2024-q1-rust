use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BankTransaction {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
}

impl BankTransaction {
    pub fn is_valid(&self) -> bool {
        vec!["c", "d"].contains(&self.tipo.as_str())
            && !self.descricao.is_empty()
            && self.descricao.len() <= 10
            && self.valor > 0
    }
}
