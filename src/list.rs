use package::Package;
use repository::Repository;

/// Caches all the repositories into a single list, so they can be accessed
/// everytime without downloading all of them again.
#[derive(Serialize, Deserialize)]
pub struct RepositoryList(pub Vec<Repository>);