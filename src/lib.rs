// define modules and import run functions for each binary
pub mod rs_grep;
pub mod rs_echo;
pub mod rs_ls;

pub use rs_grep::run as run_grep;
pub use rs_echo::run as run_echo;
pub use rs_ls::run as run_ls;
