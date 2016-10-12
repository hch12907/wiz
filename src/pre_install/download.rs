/*
    TODO:
    Write code for downloading the package, into a
    specific path.
*/
crate hyper;
use self::hyper::client;

macro_rules! custom_try {
    ($x:expr) => (match $x {
        Ok(x) => x,
        Err(why) => panic!("An error occured during package downloading.", why),
    });
}

fn download_package(url: &str) {
    let client = Client::new();
    let response = custom_try!(client.get(url).send());
    let content = Vec::new(); 
    custom_try!(response.read_to_end(&mut content));
    return content;
}