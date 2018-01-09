// #![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
#[macro_use]
extern crate clap;

use clap::{Arg, App};
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;

fn main() {
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.arg(Arg::with_name("URL")
			.help("The URL to reach")
			.required(true)
			.index(1))
		.arg(Arg::with_name("v")
			.short("v")
			.multiple(true)
			.help("Sets the level of verbosity"))
		.get_matches();

	let mut url = matches.value_of("URL").unwrap().parse::<hyper::Uri>().unwrap();
	// TODO: this is really sloppy, need a better way to make uri. should i assume https?
	if url.scheme() == None {
		url = ("https://".to_string() + matches.value_of("URL").unwrap()).parse::<hyper::Uri>().unwrap();
	}
	if ! ( url.scheme() == Some("http") || url.scheme() == Some("https") ) {
		println!("This example only works with 'http' URLs.");
		return;
	}

	let mut core = Core::new().unwrap();
	let handle = core.handle();
	let client = hyper::Client::configure()
		.connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
		.build(&handle);

	let work = client.get(url).and_then(|res| {
		if matches.occurrences_of("v") > 0 {
			// TODO: 1.1 is hard coded for now
			println!("> HTTP/1.1 {}", res.status());
			// TODO: Should consider changing Display for hyper::Headers or using regex
			println!("> {}", res.headers().to_string().replace("\n", "\n> "));
		}

		res.body().for_each(|chunk| {
			io::stdout().write_all(&chunk).map_err(From::from)
		})
	}).map(|_| {
		if matches.occurrences_of("v") > 0 {
			println!("\n\nDone.");
		}
	});

	core.run(work).unwrap();
}
