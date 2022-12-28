use ethereum::TransactionV2;

pub struct BasicPool {
    
}

impl BasicPool {
    pub fn new() -> Self {
        Self{}
    }
}

impl crate::TransactionPool for BasicPool {
    fn submit_one(&self, tx: TransactionV2) -> anyhow::Result<()> {
        todo!()
    }

    fn ready(&self) -> Vec<TransactionV2> {
        todo!()
    }
}
