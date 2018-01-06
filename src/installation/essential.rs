use config::Config;
use cache::Cache;
use repository::RepositoryList;

pub struct Essentials {
    pub config: Config,
    pub cache: Cache,
    pub repositories: RepositoryList,
}