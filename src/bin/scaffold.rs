use clap::Parser;
use regex::Regex;
use reqwest::header::COOKIE;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::ops::Range;
use std::path::Path;

const DAY_TEMPLATE: &str = r##"pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#""#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), None);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), None);
    }
}
"##;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to scaffold
    #[arg(short, long)]
    day: u8,
}

/// Creates a file and its parent directories and writes the given contents to the created file.\
/// If the file already exists, the contents of that file will not be overwritten.
fn create_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(file_path);

    // Create required directories
    fs::create_dir_all(path.parent().unwrap())?;

    // Create the file and write to it.
    // If the file already exists, return `Ok(())`.
    match fs::File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(mut file) => write!(&mut file, "{}", content),
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(()),
            _ => Err(err),
        },
    }
}

/// Gets the puzzle input using the user's session cookie.
fn get_puzzle_input(day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let session_cookie =
        std::env::var("AOC_SESSION").expect("The `AOC_SESSION` environment variable should be set");
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("https://adventofcode.com/2022/day/{day}/input"))
        .header(COOKIE, format!("session={session_cookie}"))
        .send()?;
    Ok(res.text()?.trim_end().to_string())
}

/// Tries to get the puzzle description and parse it to markdown.
///
/// If part 1 of the puzzle isn't completed or the session cookie can't be found, the 2nd part's description won't be available.
fn _get_puzzle_description(day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let session_cookie =
        std::env::var("AOC_SESSION").expect("The `AOC_SESSION` environment variable should be set");
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("https://adventofcode.com/2022/day/{day}"))
        .header(COOKIE, format!("session={session_cookie}"))
        .send()?;

    let page_html = res.text()?;

    // Get only the puzzle parts from the page
    let re = Regex::new(r#"<article class="day-desc">(.|\n)*?</article>"#)?;
    let puzzle_parts_html = re
        .find_iter(&page_html)
        .map(|part| part.as_str())
        .collect::<Vec<&str>>()
        .join("\n");
    let markdown = html2md::parse_html(&puzzle_parts_html);
    Ok(markdown)
}

/// Finds the index of a line where code for a given day should be inserted below it.
///
/// The appropriate line is found by matching each line in a given file to a given [`Regex`] pattern.
/// To get the line's corresponding day number, this functions requires a range for where the day is in that pattern.
///
/// This function returns [`None`] if the index isn't found or if the line already exists.
///
/// ## Example
///
/// Let's say we had a file with this code:
///
/// ```
/// match day {
///     1 => (),
///     3 => (),
///     _ => ()
/// }
/// ```
///
/// Now we want to add a match clause for the 2nd day below line 2 (index 1). We can find the index of that line like this:
///
/// ```
/// let line_idx = index_of_missing_day_line(
///     2,
///     std::path::Path::new("/path/to/file.rs"),
///     regex::Regex::new(r"[\d_]+ => ()").unwrap(),
///     0..2,
/// );
///
/// assert_eq!(line_idx, Some(1));
/// ```
fn index_of_missing_day_line(
    day: u8,
    path: &Path,
    pattern: Regex,
    pattern_day_range: Range<usize>,
) -> Option<usize> {
    let file = fs::File::options().read(true).open(path).unwrap();
    let reader = BufReader::new(file);

    let mut prev_line_day = None;
    let mut line_idx = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let matches = pattern
            .find_iter(&line)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        if let Some(match_case) = matches.first() {
            // Try to parse the given range in the pattern into a day number
            if let Ok(line_day) = match_case[pattern_day_range.clone()]
                .to_string()
                .trim()
                .parse::<u8>()
            {
                if prev_line_day.is_none() {
                    prev_line_day = Some(line_day);
                }

                // If the line already exists, do nothing
                if line_day == day {
                    return None;
                }

                // If the day falls between these two lines, the index has been found
                if prev_line_day.unwrap() < day && (line_day > day) {
                    return Some(line_idx);
                }
            }
        } else if prev_line_day.is_some() {
            // If the line can't be parsed but the previous day is defined, the index is the previous line's index
            return Some(line_idx - 1);
        }
        line_idx += 1;
    }

    // If EOF reached but the previous day is defined, the index is at the end
    if prev_line_day.is_some() {
        return Some(line_idx);
    }
    None
}

fn main() {
    let args = Args::parse();

    // Find absolute path to project directory
    let exe_path = std::env::current_exe().unwrap();
    let project_dir_idx = exe_path
        .iter()
        .position(|str| str.to_str().unwrap() == "advent-of-code-2022")
        .unwrap();
    let project_path =
        exe_path.to_str().unwrap().split('/').collect::<Vec<&str>>()[0..=project_dir_idx].join("/");

    // Create the day's files

    let day_dir_path_str = format!("{}/src/days/day{:02}", project_path, args.day);

    create_file(&format!("{}/mod.rs", day_dir_path_str), DAY_TEMPLATE).unwrap();

    create_file(
        &format!("{}/input.txt", day_dir_path_str),
        &get_puzzle_input(args.day).expect("Couldn't get puzzle input"),
    )
    .unwrap();

    // Uncomment to enable generation of puzzle descriptions
    /*
    if let Ok(desc) = _get_puzzle_description(args.day) {
        create_file(&format!("{}/README.md", day_dir_path_str), &desc).unwrap();
    }
    */

    let main_path_str = format!("{}/src/main.rs", project_path);
    let main_path = std::path::Path::new(&main_path_str);
    let target_line_idx = index_of_missing_day_line(
        args.day,
        main_path,
        Regex::new(r"[\d_]+ => ").unwrap(),
        0..2,
    );
    // If the correct line index has been found, insert a line for the new match case below that line
    if let Some(i) = target_line_idx {
        insert_below_line(
            &format!(
                "        {} => DayResult::from_solvers(input, day{:02}::part_one, day{:02}::part_two),",
                args.day, args.day, args.day
            ),
            i,
            main_path,
        )
        .unwrap();
    }

    let mod_path_str = format!("{}/src/days/mod.rs", project_path);
    let mod_path = std::path::Path::new(&mod_path_str);
    let target_line_idx = index_of_missing_day_line(
        args.day,
        mod_path,
        Regex::new(r"pub mod day\d\d;").unwrap(),
        11..13,
    );

    // If the correct line index has been found, insert a line for the new match case below that line
    if let Some(i) = target_line_idx {
        insert_below_line(&format!("pub mod day{:02};", args.day), i, mod_path).unwrap();
    }
}

fn insert_below_line(content: &str, line_idx: usize, path: &Path) -> std::io::Result<()> {
    let file = fs::File::options().read(true).open(path).unwrap();
    let reader = BufReader::new(file.try_clone().unwrap());
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let new_content = vec![
        &lines[0..line_idx],
        &[content.to_string()],
        &lines[line_idx..],
    ]
    .into_iter()
    .flatten()
    .map(|l| l.as_str())
    .collect::<Vec<&str>>()
    .join("\n");
    fs::File::options()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap()
        .write_all(new_content.as_bytes())?;
    Ok(())
}
