use color_eyre::eyre::Result;
use mi::web::SwitchCounter;
use std::env;

fn main() -> Result<()> {
    let whurl = env::args()
        .skip(1)
        .next()
        .expect("usage: switchcounter_read <webhook-url>");
    let cli = SwitchCounter::new(whurl);
    let status = cli.status()?;
    println!("{:?}", status);

    Ok(())
}
