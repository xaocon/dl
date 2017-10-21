// #![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate tokio_core;
#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use futures::Future;
use futures::stream::Stream;
use hyper::Client;

fn main() {
    let matches = clap_app!(myapp =>
        (version: crate_version!())
        (author: "Evan Pitstick <emp@seclab.in>")
        (about: "Does awesome things")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg debug: -d ... "Sets the level of debugging information")
        // (@subcommand test =>
        //     (about: "controls testing features")
        //     (version: "1.3")
        //     (author: "Someone E. <someone_else@other.com>")
        //     (@arg verbose: -v --verbose "Print test information verbosely")
        // )
    ).get_matches();

    println!("{}", crate_version!());
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return;
        }
    };

    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return;
    }

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    let work = client.get(url).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: \n{}", res.headers());

        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_| {
        println!("\n\nDone.");
    });

    core.run(work).unwrap();
}
