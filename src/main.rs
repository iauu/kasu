use tracing::Level;

mod lib;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_target(true) // Shows the module path/logger name
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE) // Tracks span life
        .init();
    
    
    println!("Hello, world!");
}
