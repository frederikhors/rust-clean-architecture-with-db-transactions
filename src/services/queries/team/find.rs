use crate::entities::team::Team;
use crate::{services::queries::RepoTrait, Deps};
use std::sync::Arc;

struct ExecutorImpl {
    deps: Arc<Deps>,
}

pub fn new_executor(deps: Arc<Deps>) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, id: &str) -> Result<Option<Team>, String>;
}

#[async_trait::async_trait]
impl Executor for ExecutorImpl {
    async fn execute(&self, id: &str) -> Result<Option<Team>, String> {
        let res = self.deps.queries_repo.team_by_id(id).await?;

        Ok(res)
    }
}
