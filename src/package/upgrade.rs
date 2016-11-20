pub fn upgrade_package(name: &str, path: &Path) {
    let package = find_package::select_package(name, path);
    //Oops, looks like I need to record the version of downloaded package
}