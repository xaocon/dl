#![allow(warnings)]
use ureq;
use simple_logger::SimpleLogger;
use log::info;

fn main() {
	SimpleLogger::new()
		.with_level(log::LevelFilter::Debug)
		.init()
		.unwrap();

	info!("This is what it will look like");
    let agent = make_agent();
	let res = agent.get("https://httpbin.org/headers")
		.call()
		.unwrap();

	println!("{}", res.into_string().unwrap());
}

fn make_agent() -> ureq::Agent {
	ureq::AgentBuilder::new()
		.build()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one() {
		// let agent = make_agent();
		// agent.get("https://httpbin.org/")
		assert!(true)
	}
}
