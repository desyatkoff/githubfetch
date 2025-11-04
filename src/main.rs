/*
 * This file is part of GitHubFetch
 *
 * Copyright (C) 2025 Sergey Desyatkov
 *
 * GitHubFetch is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License,
 * or (at your option) any later version
 *
 * GitHubFetch is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details
 *
 * You should have received a copy of the GNU General Public License
 * along with GitHubFetch. If not, see <https://www.gnu.org/licenses/>
 */

use std::env;
use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize)]
struct UserInfo {
    login: String,
    id: usize,
    name: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    public_repos: usize,
    public_gists: usize,
    followers: usize,
    following: usize,
    created_at: String
}

#[derive(Deserialize)]
struct RepoInfo {
    stargazers_count: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (help, version, username) = parse_args(&args);

    if help {
        print_help();
    }

    if version {
        print_version();
    }

    if !username.is_empty() {
        let user_url = format!(
            "https://api.github.com/users/{}",
            username
        );

        let client = Client::new();
        let response = client
            .get(&user_url)
            .header(
                "User-Agent",
                format!("GitHubFetch/{}", VERSION)
            )
            .send()
            .expect(&format!(
                "{}: failed to query GitHub",
                "error".red().bold()
            ));

        let user_profile_data: UserInfo = response
            .json()
            .expect(&format!(
                "{}: failed to parse JSON",
                "error".red().bold()
            ));

        let mut page = 1;
        let mut total_stars = 0;

        loop {
            let repos: Vec<RepoInfo> = client
                .get(format!(
                    "{}/repos",
                    &user_url
                ))
                .query(&[
                    ("per_page", "100"),
                    ("page", &page.to_string())
                ])
                .header(
                    "User-Agent",
                    format!("GitHubFetch/{}", VERSION)
                )
                .send()
                .expect(&format!(
                    "{}: failed to query GitHub",
                    "error".red().bold()
                ))
                .json()
                .expect(&format!(
                    "{}: failed to parse JSON",
                    "error".red().bold()
                ));

            if repos.is_empty() {
                break;
            }

            total_stars += repos.iter().map(|r| r.stargazers_count).sum::<usize>();
            page += 1;
        }

        println!(
            r#"
            ▟██████████████▙               {}@{}
         ▟████████████████████▙            {}-------
       ▟████████████████████████▙          {}: {}
     ▟████████████████████████████▙        {}: {}
    ▟█████▛  ▜████████████▛  ▜█████▙       {}: {}
   ▟██████                    ██████▙      {}: {}
  ▟██████▛                    ▜██████▙     {}: {}
 ▟██████▛                      ▜██████▙    {}: {}
 ███████                        ███████    {}: {}
 ███████                        ███████    {}: {}
 ███████                        ███████    {}: {}
 ███████                        ███████    {}: {}
 ▜██████▙                      ▟██████▛    {}: {}
  ███████▙                    ▟███████     {}: {}
  ▜███  ▜████▙            ▟██████████▛     {}: {}
   ▜██▙   ▜████▙        ▟███████████▛      
    ▜███▙   ▜██▛        ▜██████████▛       
      ▜██▙               ████████▛         
        ▜█████▙          ██████▛           
           ▜███          ███▛              
            "#,
            user_profile_data.login.clone().blue(),
            "github".blue(),
            "-".repeat(user_profile_data.login.clone().len()),
            "ID".blue(),
            user_profile_data.id,
            "Name".blue(),
            user_profile_data.name.unwrap_or_default(),
            "Company".blue(),
            user_profile_data.company.unwrap_or_default(),
            "Blog".blue(),
            user_profile_data.blog.unwrap_or_default(),
            "Location".blue(),
            user_profile_data.location.unwrap_or_default(),
            "Email".blue(),
            user_profile_data.email.unwrap_or_default(),
            "Bio".blue(),
            user_profile_data.bio.unwrap_or_default(),
            "Public Repos".blue(),
            user_profile_data.public_repos,
            "Public Gists".blue(),
            user_profile_data.public_gists,
            "Followers".blue(),
            user_profile_data.followers,
            "Following".blue(),
            user_profile_data.following,
            "Total Stars".blue(),
            total_stars,
            "Created At".blue(),
            user_profile_data.created_at
        );
    } else {
        if !help && !version {
            eprintln!(
                "{}: username not specified",
                "error".red().bold()
            );

            println!(
                "{}: use `-h` or `--help` to get usage help",
                "help".cyan().bold()
            );
        }
    }

    return;
}

fn parse_args(args: &[String]) -> (bool, bool, String) {
    let mut help = false;
    let mut version = false;
    let mut username = String::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                help = true;
            },
            "-V" | "--version" => {
                version = true;
            },
            _ if arg.starts_with('-') => {},
            _ => {
                if username.is_empty() {
                    username = arg.clone();
                }
            }
        }
    }

    return (help, version, username);
}

fn print_help() {
    println!(
        r#"
USAGE:
    githubfetch [OPTIONS] <USERNAME>

OPTIONS:
    -h, --help       Print help
    -V, --version    Print version
        "#
    );
}

fn print_version() {
    println!(
        r#"
  ____ _ _   _   _       _     _____    _       _     
 / ___(_) |_| | | |_   _| |__ |  ___|__| |_ ___| |__  
| |  _| | __| |_| | | | | '_ \| |_ / _ \ __/ __| '_ \ 
| |_| | | |_|  _  | |_| | |_) |  _|  __/ || (__| | | |
 \____|_|\__|_| |_|\__,_|_.__/|_|  \___|\__\___|_| |_|

GitHubFetch v{}
Fetch GitHub profile info by username

Copyright (C) 2025 Desyatkov Sergey
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version
        "#,
        VERSION
    );
}
