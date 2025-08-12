use clap::Parser;
use main_error::MainError;
use regex::Regex;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::{fs, str};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("Not the same amount of video and sub files.")]
    MismatchError,
    #[error("{0} quality not supported.")]
    QualityError(Arc<str>),
    #[error("User cancelled.")]
    ExitError,
    #[error("IO error: {0}")]
    InputError(#[from] std::io::Error),
    #[error("Wrong Season: {0}")]
    SeasonError(#[from] ParseIntError),
    #[error("regex coulnd't find the capture groups'")]
    CaptureError,
    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Couldn't find any files to rename.")]
    EmptyError,
}

type ProgramResult<T> = Result<T, ProgramError>;
fn rename_mkv(params: Args) -> ProgramResult<()> {
    // Create list of files.
    let mut videofiles = Vec::with_capacity(12);
    let re = Regex::new(r"^(?<name>.+?)(?:\s+Disc\s+\d+)?_t(?<num>\d{2})\.mkv$")?;
    for file in fs::read_dir(params.dir.as_ref())? {
        let f: Rc<str> = file?.file_name().to_string_lossy().into();
        if re.is_match(f.as_ref()) {
            videofiles.push(f);
        }
    }

    if videofiles.is_empty() {
        return Err(ProgramError::EmptyError);
    }

    let quality_map = HashMap::from([
        ("bd", "[BD Remux][1080p]"),
        ("uhd", "[UHD Remux][2160p]"),
        ("dvd", "[DVD Remux][480p]"),
    ]);
    let quality: Arc<str> = (*quality_map
        .get(params.quality.as_ref())
        .ok_or(ProgramError::QualityError(params.quality.clone()))?)
    .into();

    // Renaming logic
    let mut newnames = Vec::with_capacity(12);
    let mut episode_num: u32 = params.number;
    for videofile in videofiles.iter() {
        let caps = re
            .captures(videofile.as_ref())
            .ok_or(ProgramError::CaptureError)?;
        let (_, [name, _]) = caps.extract();
        let newname = format!(
            "{} S{:02}E{:02} {}.mkv",
            name, params.season, episode_num, quality
        );
        episode_num += 1;
        newnames.push(newname);
    }

    // Check
    println!("Renaming file to the following:");
    let file_iter = newnames.iter().zip(videofiles.iter());

    for (sub, vid) in file_iter.clone() {
        println!("{}\t-->\t{}", sub, vid);
    }

    // User confirmation
    println!("Are these pairs correct? (Y/n): ");
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    if answer.contains("n") {
        return Err(ProgramError::ExitError);
    }

    // Create output folder

    // Run commands
    for (n, v) in file_iter {
        let new_file = Path::new(n);
        let old_file = Path::new(v.as_ref());
        fs::rename(old_file, new_file)?;
    }
    Ok(())
}

/// mkvmerge wrapper to bulk add subtitles to videofiles.
/// An output folder will be created with the multiplexed video files.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Directory with the video and sub files.
    #[arg(short, long, default_value = ".")]
    dir: Arc<str>,
    /// quality
    #[arg(short, long)]
    quality: Arc<str>,
    /// season
    #[arg(short, long, default_value = "1")]
    season: u32,
    /// starting episode number
    #[arg(short, long, default_value = "1")]
    number: u32,
}

fn main() -> Result<(), MainError> {
    let args = Args::parse();
    let _ = rename_mkv(args)?;
    Ok(())
}
