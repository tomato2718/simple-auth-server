mod application;
mod domain;
mod infratructure;
#[cfg(test)]
#[allow(unused_variables)]
mod test_support;

pub use infratructure::start_server;
