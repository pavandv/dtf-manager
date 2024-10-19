use crate::{CommandContext, SgResult};

pub(crate) trait Executor {
    async fn execute(&self, context: &CommandContext) -> SgResult<()>;
}
