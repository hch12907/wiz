use utils::Hash;
use installation::PackageState;

struct PackageCache(Vec<(Hash, PackageState)>);