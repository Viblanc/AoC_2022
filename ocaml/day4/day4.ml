module Pairs : sig
  type 'a t = 'a * 'a
  val of_list : 'a list -> 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
  val contains : 'a t -> 'a t -> bool
  val overlaps : 'a t -> 'a t -> bool
end
=
struct
  type 'a t = 'a * 'a

  let of_list = function
      x :: y :: [] -> (x, y)
    | _ -> raise (Failure ("cannot convert list to pairs"))

  let map f pair =
    let a, b = pair in
    f a, f b

  let contains p1 p2 =
    let a, b = p1 in
    let c, d = p2 in
    a <= c && b >= d || c <= a && d >= b

  let overlaps p1 p2 =
    let a, b = p1 in
    let c, d = p2 in
    if a < c then
      b >= c
    else if a > c then
      d >= a
    else true
end

let parse_str str =
  Pairs.of_list @@ String.split_on_char ',' str
  |> Pairs.map (fun s -> Pairs.of_list @@ String.split_on_char '-' s)
  |> Pairs.map (Pairs.map int_of_string)

let input = In_channel.with_open_bin "day4/input" In_channel.input_all
            |> String.split_on_char '\n'
            |> List.filter (fun s -> String.length s != 0)
            |> List.map parse_str

let part1 = input
            |> List.filter (fun (a, b) -> Pairs.contains a b)
            |> List.length

let part2 = input
            |> List.filter (fun (a, b) -> Pairs.overlaps a b)
            |> List.length

let () =
  Printf.printf "Part 1 result: %d\n" part1;
  Printf.printf "Part 2 result: %d\n" part2;
