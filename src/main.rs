#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate tokio_core;
#[macro_use]
extern crate clap;

// use std::env;
use std::io::{self, Write};
use futures::Future;
use futures::stream::Stream;
use hyper::Client;
use clap::{Arg, App};

fn main() {
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about("A handy HTTP client")
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.value_name("FILE")
			.help("Sets a custom config file")
			.takes_value(true))
		.arg(Arg::with_name("URL")
			.help("The URL to reach")
			.required(true)
			.index(1))
		.arg(Arg::with_name("v")
			.short("v")
			.multiple(true)
			.help("Sets the level of verbosity"))
		.get_matches();

	let url = matches.value_of("URL").unwrap().parse::<hyper::Uri>().unwrap();
	if url.scheme() != Some("http") {
		println!("This example only works with 'http' URLs.");
		return;
	}

	let mut core = tokio_core::reactor::Core::new().unwrap();
	let handle = core.handle();
	let client = Client::new(&handle);

	let work = client.get(url).and_then(|res| {
		if matches.occurrences_of("v") > 0 {
			// 1.1 is hard coded for now
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
