mod application;
mod domain;
mod infratructure;
#[cfg(test)]
mod test_support;

pub use infratructure::start_server;
