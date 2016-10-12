/*
    TODO:
    1. Copy downloaded files to other places.
    2. Verify the downloaded package.
    3. Make it support repos someday.
*/
crate hyper;
use self::hyper::client;

macro_rules! custom_try {
    ($x:expr) => (match $x {
        Ok(x) => x,
        Err(why) => panic!("An error occured during package downloading.", why),
    });
}

fn download_package(url: &str) -> Vec<u8> {
    let client = Client::new();
    let response = custom_try!(client.get(url).send());
    let content = Vec::new(); 
    custom_try!(response.read_to_end(&mut content));
    return content;
}