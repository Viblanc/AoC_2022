module Hand = struct
  type t = Rock | Paper | Scissor

  let create = function
      "A" | "X" -> Rock
      | "B" | "Y" -> Paper
      | "C" | "Z" -> Scissor
      | _ -> raise (Failure "not a valid hand")

  let get_value = function
      Rock -> 1
    | Paper -> 2
    | Scissor -> 3

  let winning = function
      Rock -> Paper
    | Paper -> Scissor
    | Scissor -> Rock

  let losing h = winning @@ winning h

  let cmp hand1 hand2 =
    match (hand1, hand2) with
      (Rock, Scissor) -> -1
    | (Scissor, Rock) -> 1
    | _ -> Int.compare (get_value hand2) (get_value hand1)
end

module Result = struct
  type t = Win | Draw | Lose

  let create = function
      "X" -> Lose
    | "Y" -> Draw
    | "Z" -> Win
    | _ -> raise (Failure "not a valid result")

  let get_value = function
      Win -> 6
    | Draw -> 3
    | Lose -> 0
end

let list_to_tuple = function
    a :: b :: [] -> (a, b)
  | _ -> raise (Failure "not a tuple")

let input =
  In_channel.with_open_bin "day2/input" In_channel.input_all
  |> String.split_on_char '\n'
  |> List.filter (fun s -> (String.length s != 0))
  |> List.map (fun s -> list_to_tuple @@ String.split_on_char ' ' s)

let first_strategy hands =
  let (elf_hand, my_hand) = hands in
  let my_hand_value = Hand.get_value my_hand in
  match Hand.cmp elf_hand my_hand with
    1 -> my_hand_value + Result.get_value Win
  | -1 -> my_hand_value + Result.get_value Lose
  | _ -> my_hand_value + Result.get_value Draw

let part1 =
  input
  |> List.map (fun (a, b) -> Hand.create a, Hand.create b)
  |> List.map first_strategy
  |> List.fold_left (+) 0

let second_strategy (elf_hand, (wanted_result: Result.t)) =
  match wanted_result with
    Win -> first_strategy (elf_hand, Hand.winning elf_hand)
  | Lose -> first_strategy (elf_hand, Hand.losing elf_hand)
  | Draw -> first_strategy(elf_hand, elf_hand)

let part2 =
  input
  |> List.map (fun (a, b) -> Hand.create a, Result.create b)
  |> List.map second_strategy
  |> List.fold_left (+) 0

let () =
  Printf.printf "Part 1 result: %d\n" part1;
  Printf.printf "Part 2 result: %d\n" part2;
