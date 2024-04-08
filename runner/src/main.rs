use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::{prelude::*, ThreadPoolBuilder};

use anyhow::{ Result, Context };
use clap::Parser;
use walkdir::WalkDir;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Parser, Debug)]
struct Args {
    in_folder: PathBuf,
    out_folder: PathBuf,
    program: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    eprintln!("{:?}", args);

    let Args { in_folder, out_folder, program } = args;
    let program = program.join(" ");

    let mut in_files = WalkDir::new(in_folder)
        .into_iter().into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|e| e.is_file())
        .collect::<Vec<_>>();

    in_files.sort_by(|a, b| a.cmp(b));

    let th_pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .with_context(|| format!("could not build thread pool"))?;

    let results = th_pool.install(|| {
        let pb = ProgressBar::new(in_files.len() as u64);
        pb.enable_steady_tick(Duration::new(1, 0));
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} cases ({eta})  ",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        let ps = in_files.par_iter().progress_with(pb).map(|entry| {

            run_test_single_case(entry.clone(), &out_folder, &program)
        });
        ps.collect::<Vec<Result<(PathBuf, String)>>>()
    });

    let results = results
        .into_iter()
        .collect::<Result<Vec<(PathBuf, String)>, _>>()
        .with_context(|| format!("failed in some case"))?;

    let csv = results.into_iter()
        .map(|(f, last_line)| format!("{}\t{}\n", f.file_name().unwrap().to_str().unwrap(), last_line))
        .collect::<Vec<_>>()
        .concat();
    println!("{}", csv);

    Ok(())
}

fn run_test_single_case(
    in_file: PathBuf,
    out_folder: &PathBuf,
    program: &str,
) -> Result<(PathBuf, String)> {
    let filename = in_file
        .file_name()
        .with_context(|| format!("could not get filename"))?;

    let input_file = File::open(in_file.clone())
        .with_context(|| format!("could not open input_file `{}`", in_file.to_str().unwrap()))?;

    let output_path = out_folder.join(filename);
    let output_file = File::create(output_path.clone()).with_context(|| {
        format!(
            "could not create output_file `{}`",
            output_path.to_str().unwrap()
        )
    })?;

    let stdin_pipe = Stdio::from(input_file);
    let stdout_pipe = Stdio::from(output_file);

    let output = Command::new(program)
        .stdin(stdin_pipe)
        .stdout(stdout_pipe)
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("could not execute command"))?;

    let stderr_str = String::from_utf8_lossy(&output.stderr);
    Ok((in_file.clone(), stderr_str.lines().last().map(|s| s.to_owned()).unwrap_or(format!(""))))
}
