let input =
  let content = In_channel.with_open_bin "day1/input" In_channel.input_all in
  String.split_on_char '\n' content

let count_calories list =
  let rec aux l res cur =
    match l with
      [] -> res
    | h :: t -> if String.length h = 0 then
                  aux t (cur :: res) []
                else
                  aux t res (h :: cur)
  in aux list [] []

let sum acc x = acc + int_of_string x

let calories =
  input
  |> count_calories
  |> List.map (List.fold_left sum 0)
  |> List.sort (fun a b -> b - a)

let part1 =
  calories
  |> List.hd

let rec take n l =
  match n with
    0 -> []
  | m -> match l with
           [] -> []
         | h :: t -> h :: (take (m - 1) t)

let part2 =
  calories
  |> take 3
  |> List.fold_left (+) 0

let () =
  Printf.printf "Part 1 result: %d\n" part1;
  Printf.printf "Part 2 result: %d\n" part2
