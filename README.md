<!-- markdownlint-disable MD004 MD033 MD034 -->

<div align="center">

# Refit

</div>
<p align="center">
	<img alt="GitHub" src="https://img.shields.io/github/license/Wombatlord/refit?logo=Github&logoColor=green">
	<img alt="GitHub" src="https://img.shields.io/github/languages/top/Wombatlord/refit?label=Rust&logo=rust&logoColor=red"></p>

## Introduction
Refit is a simple utility for switching profiles when using the Starship prompt.

## Installation
To build from source, clone the repo and run `cargo build --release` in the project root.

## Usage

In the top of your starship.toml file add a single comment with the name of your profile. This can be whatever you like.
```
# name_of_profile
-- starship.toml starts here --
```

All `starship.toml` files should be named in the format `starship.name_of_profile.toml` except for your active one.

When you run refit, pass it the name of the profile you would like to switch to.
`$ refit different_profile`

If you are running refit for the first time, it will create a file called `refit.conf` alongside the executable, and prompt you to add the path to your starship.toml files.

Refit will then search the directory specified in refit.conf for profiles, and if it finds a match, rename your current profile to `starship.%profile_name.toml` where `%profile_name` is taken from the first line comment of the profile. It will then rename your selected profile from `starship.%passed_profile_name.toml` to `starship.toml`, where `%passed_profile_name` is the name you passed to refit, as long as this name matches a first line comment in one of your profiles.

✨ Enjoy your switched out profile! ✨
