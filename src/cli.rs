use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Build {
        file: String,

        #[arg(short = 'o', long = "out-file")]
        out_file: Option<String>,

        #[arg(short = 'l')]
        libraries: Vec<String>,

        /// {INP} = input files
        /// {DST} = output file
        /// {LIB} = library files (included & -l)
        #[arg(
            long = "link-with",
            default_value = "gcc -Wl,-O3,-pie -o {DST} {INP} {LIB}"
        )]
        link_command: String,

        #[arg(long = "shell-path", default_value = "/bin/sh")]
        shell_path: String,
    },
}

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}
