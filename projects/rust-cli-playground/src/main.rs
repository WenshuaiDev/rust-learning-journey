// 为什么这里不用 use crate::cli;?
// 仍然是main与lib所属crate不同导致的
use rust_cli_playground::cli;

fn main() {
    cli::run(std::env::args().collect());
}