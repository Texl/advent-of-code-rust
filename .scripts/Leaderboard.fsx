#!/usr/bin/env -S dotnet fsi
#r "nuget: FsHttp"
#r "nuget: FSharp.Json"

open System
open System.IO
open FSharp.Json
open FsHttp

let session = "53616c7465645f5f95549ba54126951a7e726d8570cb97a71d40939bc4d531e62579ec03ea9a30da1581fbac093cf6b75e6dae599e3a2ed6766f0d134185986e"

let leaderboardPath = ".scripts/leaderboard.json"

module WireModels =

   type CompletionRecord =
      { star_index : int
        get_star_ts : int64 }

   type MemberRecord =
      { last_star_ts : int64
        stars : int
        id : int
        completion_day_level : Map<string, Map<string, CompletionRecord>>
        global_score : int
        local_score : int
        name : string }

   type Leaderboard =
      { event : string
        owner_id : int
        members : Map<string, MemberRecord> }


module Models =

   type Day = int
   
   type Part =
      | Part1
      | Part2
   
   type MemberId = int
   
   type Score = int
   
   type MemberRecord =
      { Name : string
        LocalScore : Score
        Stars : int
        CompletionDayLevel : (string * DateTimeOffset) array }

   type Leaderboard =
      { Event : string
        Members : MemberRecord array }


module Convert =
   module W = WireModels
   module M = Models

   let private convMember (o : W.MemberRecord) : M.MemberRecord option =
      let completionDayLevel =
         o.completion_day_level
         |> Map.toSeq
         |> Seq.collect (fun (day, parts) ->
            parts
            |> Map.toSeq
            |> Seq.map (fun (part, partCompletion) ->
               $"{day}.{part}", DateTimeOffset.FromUnixTimeSeconds(partCompletion.get_star_ts)))
         |> Seq.toArray
      
      if o.last_star_ts <> 0
      then
         let record : M.MemberRecord =
            { Name = o.name
              LocalScore = o.local_score
              Stars = o.stars
              CompletionDayLevel = completionDayLevel }
         Some record
      else None
   
   let leaderboard (o : W.Leaderboard) : M.Leaderboard =
      let members =
         o.members
         |> Map.toSeq
         |> Seq.choose (fun (k, v) -> v |> convMember)
         |> Seq.sortByDescending _.LocalScore
         |> Seq.toArray
      { Event = o.event
        Members = members }


let download () =
   let data =
      http {
         GET($"https://adventofcode.com/2024/leaderboard/private/view/837190.json")
         Cookie "session" session
      }
      |> Request.send
      |> Response.toText
    
   // Ensure path exists
   Directory.CreateDirectory(Path.GetDirectoryName(leaderboardPath)) |> ignore
   File.WriteAllText(leaderboardPath, data)

let parse () =
   let input = File.ReadAllText leaderboardPath
   let leaderboardW = Json.deserialize<WireModels.Leaderboard> input

   let leaderboardM = leaderboardW |> Convert.leaderboard
   
   printfn "%A" leaderboardM


// Update leaderboard if it's older than 15 minutes
if (DateTime.Now - FileInfo(leaderboardPath).LastWriteTime).TotalMinutes > 15.0 then
   printf "Updating leaderboard... "
   download ()
   printfn "done"
else
   printfn "Leaderboard is up to date"

parse ()
