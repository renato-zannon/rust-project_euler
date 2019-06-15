#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

use failure::Error;
use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::str;

#[derive(Debug, StructOpt)]
struct Opts {
    /// Problem number
    problem: u32,

    /// Output file. Defaults to stdout
    #[structopt(parse(from_os_str))]
    output_file: Option<PathBuf>,
}

fn main() {
    run().unwrap()
}

fn run() -> Result<(), Error> {
    let opts = Opts::from_args();
    let html = fetch_problem_html(opts.problem)?;
    let problem = parse_problem(opts.problem, &html)?;
    let output = render_problem(&problem)?;

    if let Some(path) = opts.output_file {
        let mut output_file = BufWriter::new(File::create(path)?);
        write!(output_file, "{}", output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct Problem {
    number: u32,
    title: String,
    description_pars: Vec<String>,
}

fn render_problem(problem: &Problem) -> Result<String, Error> {
    let hbs = handlebars::Handlebars::new();

    hbs.render_template(include_str!("problem.hbs"), problem)
        .map_err(Error::from)
}

fn parse_problem(number: u32, html: &str) -> Result<Problem, Error> {
    use scraper::{ElementRef, Html, Node, Selector};
    let problem_info_selector = Selector::parse("#problem_info").unwrap();
    let description_paragraphs_selector = Selector::parse(".problem_content p").unwrap();

    let document = Html::parse_document(html);
    let problem_info_el = document
        .select(&problem_info_selector)
        .next()
        .ok_or_else(|| format_err!("Problem info not found"))?;

    let problem_title_el = problem_info_el
        .prev_siblings()
        .filter_map(ElementRef::wrap)
        .find(|sibling| sibling.value().name() == "h2")
        .ok_or_else(|| format_err!("Problem title not found"))?;

    let title = problem_title_el.inner_html().trim().to_string();

    let description_pars: Vec<String> = document
        .select(&description_paragraphs_selector)
        .flat_map(|par_el| {
            par_el
                .descendants()
                .filter_map(|node| {
                    if let Node::Text(t) = node.value() {
                        return Some(t.to_string());
                    }

                    let el = ElementRef::wrap(node)?;
                    match el.value().name() {
                        "br" => Some("\n".to_string()),
                        "b" => Some(el.text().collect::<String>()),
                        _ => None,
                    }
                })
                .collect::<String>()
                .lines()
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .filter(|s| s.len() > 0)
        .collect();

    Ok(Problem {
        number,
        title,
        description_pars,
    })
}

fn fetch_problem_html(number: u32) -> Result<String, Error> {
    use reqwest::{Client, StatusCode};

    let client = Client::new();

    let mut response = client
        .get(&format!("https://projecteuler.net/problem={}", number))
        .send()?;

    match response.status() {
        StatusCode::OK => response.text().map_err(Error::from),
        status => Err(format_err!("Unexpected response status: {:?}", status)),
    }
}
