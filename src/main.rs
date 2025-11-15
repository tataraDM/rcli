use clap::Parser;
use rcli::TextSubCommand;
use rcli::{Base64SubCommand, Opts, SubCommand, process_csv, process_genpass};
use rcli::{process_decode, process_encode};
use rcli::{process_text_generate, process_text_sign, process_text_verify};
use zxcvbn::zxcvbn;
fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.lowercase,
                opts.uppercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);
            let result = zxcvbn(&password, &[]);
            println!("Strength: {}", result.score());
        }
        SubCommand::Base64(sub) => match sub {
            Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded_str = String::from_utf8(decoded)?;
                println!("{}", decoded_str);
            }
        },
        SubCommand::Text(sub) => match sub {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified =
                    process_text_verify(&opts.input, &opts.key, opts.format, &opts.signature)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let keys = process_text_generate(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let private_key_path = opts.output.join("blake3.txt");
                        std::fs::write(&private_key_path, &keys[0])?;
                        println!("Private key saved to {}", private_key_path.display());
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        let private_key_path = opts.output.join("ed25519_private.key");
                        let public_key_path = opts.output.join("ed25519_public.key");
                        std::fs::write(&private_key_path, &keys[0])?;
                        std::fs::write(&public_key_path, &keys[1])?;
                        println!("Private key saved to {}", private_key_path.display());
                        println!("Public key saved to {}", public_key_path.display());
                    }
                }
            }
        },
    }
    Ok(())
}
