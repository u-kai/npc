use clap::Parser;
use convertor::{IgnoreWordsConverter, PascalCaseReservedIdentifiersConverter};
use npc::*;

fn main() {
    let cli = Cli::parse();
    cli.run();
}

#[derive(Parser)]
struct Cli {
    #[clap(short = 's', long = "snake")]
    snake: bool,
    #[clap(short = 'c', long = "camel")]
    camel: bool,
    #[clap(short = 'p', long = "pascal")]
    pascal: bool,
    #[clap(short = 'k', long = "chain")]
    chain: bool,
    #[clap(short = 'o', long = "constant")]
    constant: bool,
    #[clap(short = 'i', long = "ignores")]
    ignores: Vec<String>,
    #[clap(long = "consider-well-known")]
    consider_well_known: bool,
    #[clap(long = "consider-words")]
    consider_words: Vec<String>,
    #[clap(help = "The sentence to convert")]
    sentence: String,
}

impl Cli {
    fn run(&self) {
        let principal = if self.snake {
            Principal::Snake
        } else if self.camel {
            Principal::Camel
        } else if self.pascal {
            Principal::Pascal
        } else if self.chain {
            Principal::Chain
        } else if self.constant {
            Principal::Constant
        } else {
            Principal::Snake
        };
        let param = Parameter::new(&self.sentence, principal);
        let mut convertors: Vec<Box<dyn PostConvert>> = Vec::new();
        if self.consider_well_known {
            let mut well_known = PascalCaseReservedIdentifiersConverter::wellknown();
            for word in &self.consider_words {
                well_known = well_known.add(word);
            }
            convertors.push(well_known.to_convertor());
        }
        if !self.ignores.is_empty() {
            let args = self
                .ignores
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>();
            let ignores = IgnoreWordsConverter::new(&args);
            convertors.push(ignores.to_convertor());
        }
        let param = convertors
            .into_iter()
            .fold(param, |acc, c| acc.add_post_convert(c));
        println!("{}", convert(&param));
    }
}
