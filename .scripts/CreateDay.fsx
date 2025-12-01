#!/usr/bin/env -S dotnet fsi

open System.IO

let write (path : string) (text : string) =
   if not (File.Exists(path)) then
      Directory.CreateDirectory(Path.GetDirectoryName(path)) |> ignore
      File.WriteAllText(path, text)

let createDay year day =
   let sourceText =
      $"""// https://adventofcode.com/{year}/day/{day}
static INPUT_FILE: &'static str = include_str!("../data/day{day:D2}.test.txt");

pub fn part1() -> () {{
    let _lines: Vec<_> = INPUT_FILE.trim().split("\n").collect();

    println!("Day {day:D2}, Part 1");
}}

pub fn part2() -> () {{
    let _lines: Vec<_> = INPUT_FILE.trim().split("\n").collect();

    println!("Day {day:D2}, Part 2");
}}
"""

   write $"src/day{day:D2}.rs" sourceText
   write $"data/day{day:D2}.test.txt" "\n"
   write $"data/day{day:D2}.txt" "\n"

match fsi.CommandLineArgs |> Array.tail with
| [| yearStr; dayStr |] ->
   let year = int yearStr
   let day = int dayStr
   createDay year day
| _ -> ()
