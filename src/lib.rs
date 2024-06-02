// define modules and import run functions for each binary
pub mod rs_grep;
pub mod rs_echo;

pub use rs_grep::run as run_grep;
pub use rs_echo::run as run_echo;

