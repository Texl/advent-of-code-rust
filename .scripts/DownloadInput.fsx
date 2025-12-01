#!/usr/bin/env -S dotnet fsi
#r "nuget: FsHttp"

open System.IO
open FsHttp

let session = "53616c7465645f5f95549ba54126951a7e726d8570cb97a71d40939bc4d531e62579ec03ea9a30da1581fbac093cf6b75e6dae599e3a2ed6766f0d134185986e"

let downloadInput year day =
    let input =
        http {
            GET($"https://adventofcode.com/{year}/day/{day}/input")
            Cookie "session" session
        }
        |> Request.send
        |> Response.toText
    
    let inputPath = $"data/day{day:D2}.txt"
    
    // Ensure path exists
    Directory.CreateDirectory(Path.GetDirectoryName(inputPath)) |> ignore
    
    File.WriteAllText(inputPath, input)


match fsi.CommandLineArgs |> Array.tail with
| [| yearStr; dayStr |] ->
    let year = int yearStr
    let day = int dayStr
    downloadInput year day
| _ -> ()
