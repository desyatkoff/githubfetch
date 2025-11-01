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
use reqwest::blocking::Client;
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct UserInfo {
    login: Option<String>,
    id: Option<usize>,
    name: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    public_repos: Option<usize>,
    public_gists: Option<usize>,
    followers: Option<usize>,
    following: Option<usize>,
    created_at: Option<String>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (help, version, username) = parse_args(&args);

    if help || username.is_empty() {
        println!(
            r#"
USAGE:
    githubfetch [OPTIONS] <USERNAME>

OPTIONS:
    -h, --help       Print help
    -V, --version    Print version
            "#
        );

        return;
    }

    if version {
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

        return;
    }

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
        .expect("Failed to query GitHub");

    let user_profile_data: UserInfo = response
        .json()
        .expect("Failed to parse JSON");

    println!(
        r#"
{}@github
{}-------
ID: {}
Name: {}
Company: {}
Blog: {}
Location: {}
Email: {}
Bio: {}
Public Repos: {}
Public Gists: {}
Followers: {}
Following: {}
Created At: {}
        "#,
        user_profile_data.login.clone().unwrap_or_default(),
        "-".repeat(user_profile_data.login.clone().unwrap_or_default().len()),
        user_profile_data.id.unwrap_or_default(),
        user_profile_data.name.unwrap_or_default(),
        user_profile_data.company.unwrap_or_default(),
        user_profile_data.blog.unwrap_or_default(),
        user_profile_data.location.unwrap_or_default(),
        user_profile_data.email.unwrap_or_default(),
        user_profile_data.bio.unwrap_or_default(),
        user_profile_data.public_repos.unwrap_or_default(),
        user_profile_data.public_gists.unwrap_or_default(),
        user_profile_data.followers.unwrap_or_default(),
        user_profile_data.following.unwrap_or_default(),
        user_profile_data.created_at.unwrap_or_default()
    );

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
