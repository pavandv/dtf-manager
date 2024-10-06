use crate::SgResult;

pub(crate) trait Executor {
    async fn execute(&self) -> SgResult<()>;
}
