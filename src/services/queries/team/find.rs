use crate::entities::team::Team;
use crate::queries::RepoTeam;
use crate::Deps;
use std::sync::Arc;

struct ExecutorImpl<C, Q> {
    deps: Arc<Deps<C, Q>>,
}

pub fn new_executor<C: Send + Sync + 'static, Q: Send + Sync + RepoTeam + 'static>(
    deps: Arc<Deps<C, Q>>,
) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, id: &str) -> Result<Option<Team>, String>;
}

#[async_trait::async_trait]
impl<C: Send + Sync, Q: Send + Sync + RepoTeam> Executor for ExecutorImpl<C, Q> {
    async fn execute(&self, id: &str) -> Result<Option<Team>, String> {
        let res = self.deps.queries_repo.team_by_id(id).await?;

        Ok(res)
    }
}
