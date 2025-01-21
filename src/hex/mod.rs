use std::fmt::Display;
use std::fmt::Write;

use rand::Rng;
use colored::Colorize;

use crate::number::*;
use crate::terrain::*;

pub struct Hex {
    pub terrain: Terrain,
    pub number: Number,
}

impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        writeln!(&mut s, "  x----x  ")?;
        writeln!(&mut s, " /......\\ ")?;
        writeln!(&mut s, "x...{}...x", self.number)?;
        writeln!(&mut s, " \\....../ ")?;
        writeln!(&mut s, "  x----x  ")?;

        write!(f, "{}", s
            .chars()
            .map(|c| if c == '.' {
                let (option_a, option_b) = match self.terrain {
                    Terrain::Desert => (".".white(), ".".white()),
                    Terrain::Fields => ("󱔐".yellow(), "󱪲".bright_yellow()),
                    Terrain::Forest => ("".green(), "󰔱".bright_green()),
                    Terrain::Hills => ("󰟾".bright_red(), "".red()),
                    Terrain::Mountains => ("".bright_black(), "󱉏".white()),
                    Terrain::Pasture => ("󱔐".bright_green(), "󰳆".bright_white()),
                };

                let num = rand::thread_rng().gen_range(0..7);
                if num > 5 {
                    option_a
                } else if num > 4 {
                    option_b
                } else {
                    ".".white()
                }.to_string()
            } else { c.to_string() } )
            .collect::<String>()
        )
    }
}
