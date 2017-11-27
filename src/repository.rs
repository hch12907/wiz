use package::Package;

/// The struct which represents the repos.
pub struct Repository {
    url: String,
    packages: Vec<Package>,
}

impl Repository {
    /// Returns the URL of the Repository.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the packages which are provided by the Repository.
    pub fn packages(&self) -> &[Package] {
        self.packages.as_slice()
    }

    // Attempts to find the package which satifies the predicate.
    // If there are no packages that match the predicate, a `None` is returned.
    pub fn find_by_predicate<F>(&self, predicate: F) -> Option<&Package> 
        where F: Fn(&Package) -> bool {
        for pkg in &self.packages {
            if predicate(&pkg) {
                return Some(&pkg)
            }
        };

        None
    }
}