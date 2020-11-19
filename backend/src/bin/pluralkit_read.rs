use color_eyre::eyre::Result;
use mi::web::PluralKit;
use std::env;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let system_id = env::args()
        .skip(1)
        .next()
        .expect("usage: PLURALKIT_TOKEN=<token> pluralkit_read <system_id>");
    let token = env::var("PLURALKIT_TOKEN").expect("need PLURALKIT_TOKEN");
    let pk = PluralKit::new(token);
    let status = pk.status(system_id)?;

    println!("{:#?}", status);

    Ok(())
}
