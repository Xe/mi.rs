use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};
use structopt::StructOpt;
use tabular::{Row, Table};
use xdg::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    token: String,
}

#[derive(StructOpt)]
#[structopt(about = "hi Minty")]
enum Cmd {
    /// Grant good pone points to someone with a given message.
    Grant {
        to: String,
        amount: i32,
        message: String,
    },
    /// Prints information about yourself.
    Whoami,
    /// List all teh pones.
    List,
    /// Info
    Info { slug: String },
}

fn main() -> anyhow::Result<()> {
    let xdg_dirs =
        BaseDirectories::with_prefix("within-gpp").expect("able to find basedirs for within-gpp");
    let config_path = xdg_dirs.place_config_file("gaftercuha.toml")?;
    let mut fin = File::open(&config_path)?;
    let mut datni = String::new();
    fin.read_to_string(&mut datni)?;
    let cfg: Config = toml::from_str(datni.as_str())?;
    let cli = ponepoints::Client::new(cfg.token, "".to_string());

    let cmd = Cmd::from_args();

    match cmd {
        Cmd::Whoami => {
            let pone = cli.get_self()?.pone;
            println!("Name:            {}", pone.name);
            println!("Slug:            {}", pone.slug);
            println!("Joined at:       {}", pone.joined_at);
            println!("Giftable Points: {}", pone.giftable_points_count.unwrap());
            println!("Points:          {}", pone.points_count);
        }
        #[allow(dead_code)]
        Cmd::Grant {
            to,
            amount,
            message,
        } => {
            cli.give_points(to, amount, message)?;
        }
        Cmd::List => {
            let mut table = Table::new("{:<}  {:<}");
            let pones = cli.list_pones()?;
            table.add_row(Row::new().with_cell("Slug").with_cell("Points"));

            for pone in pones.into_iter() {
                table.add_row(Row::new().with_cell(pone.slug).with_cell(pone.points_count));
            }

            print!("{}", table);
        }
        Cmd::Info { slug } => {
            let pone = cli.get_pone(slug)?.pone;
            println!("Name:      {}", pone.name);
            println!("Slug:      {}", pone.slug);
            println!("Joined at: {}", pone.joined_at);
            println!("Points:    {}", pone.points_count);
        }
    }

    Ok(())
}
