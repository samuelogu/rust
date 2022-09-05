// mod print;
// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointers_refs;
// mod structs;
// mod enums;
// mod cli;
mod async_await;

#[tokio::main]
async fn main() {
    // print::run();
    // vars::run();
    // types::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // conditionals::run();
    // loops::run();
    // functions::run();
    // pointers_refs::run();
    // structs::run();
    // enums::run();
    async_await::run().await;

}
