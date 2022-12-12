module MonkeyMap = Map.Make(Int)

type monkey = { id: int;
                items: int list;
                operation: int -> int;
                test: int -> bool;
                if_true: int;
                if_false: int;
                inspections: int; }

let parse_id str =
  Scanf.sscanf str "Monkey %d:" Fun.id

let parse_items str =
  let get_items s =
    let numbers = Str.(split (regexp ", ")) s in
    List.map int_of_string numbers
  in
  match Str.(split (regexp ": ")) str with
    ["Starting items"; s] -> get_items s
  | _ -> raise (Failure "no list of items found")

let parse_operation str =
  let get_operation sign n =
    let f i = if n = "old" then i else int_of_string n in
    match sign with
      '+' -> fun i -> i + f i
    | '-' -> fun i -> i - f i
    | '*' -> fun i -> i * f i
    | '/' -> fun i -> i / f i
    | _ -> raise (Failure "invalid operation")
  in Scanf.sscanf str "Operation: new = old %c %s" get_operation

let get_number_at_end str =
  String.split_on_char ' ' str |> List.rev |> List.hd |> int_of_string

let parse_test test =
  let number = get_number_at_end test in
  (fun i -> i mod number = 0)

let parse_monkey arr =
  let id = parse_id arr.(0) in
  let items = parse_items arr.(1) in
  let operation = parse_operation arr.(2) in
  let test = parse_test arr.(3) in
  let if_true = get_number_at_end arr.(4) in
  let if_false = get_number_at_end arr.(5) in
  let inspections = 0 in
  id, { id; items; operation; test; if_true; if_false; inspections }

let monkeys lines =
  lines |> Str.(split (regexp "\n\n"))
  |> List.map (String.split_on_char '\n')
  |> List.map (List.map String.trim)
  |> List.map Array.of_list
  |> List.map parse_monkey |> List.to_seq
  |> MonkeyMap.of_seq

let rec throw_to_monkey f monkey monkeys =
  match monkey.items with
    [] -> MonkeyMap.update monkey.id (function | None -> failwith "lol" | Some _ -> Some monkey) monkeys
  | item :: tl ->
     let worry_lvl = f (monkey.operation item) in
     let target = if monkey.test worry_lvl then monkey.if_true else monkey.if_false in
     throw_to_monkey
       f
       { monkey with items = tl; inspections = monkey.inspections + 1 }
       (MonkeyMap.update target
          (function | None -> failwith "err" | Some m -> Some { m with items = m.items @ [worry_lvl] })
          monkeys)

let round f monkeys =
  MonkeyMap.fold (fun i _ map ->
      let monkey = MonkeyMap.find i map in
      throw_to_monkey f monkey map) monkeys monkeys

let launch_round f monkeys n =
  Seq.iterate (round f) monkeys
  |> Seq.take (n + 1) |> Seq.drop n |> Seq.uncons
  |> function
      Some (map, _) -> map
    | None -> failwith "something went wrong"

let get_monkey_business monkeys f n =
  launch_round f monkeys n
  |> MonkeyMap.to_seq |> List.of_seq
  |> List.map (fun (_, m) -> m.inspections)
  |> List.sort (fun i1 i2 -> compare i2 i1)
  |> List.to_seq |> Seq.take 2
  |> Seq.fold_left (fun acc i -> acc * i) 1

let part1 monkeys = get_monkey_business monkeys (fun i -> i / 3) 20
let part2 monkeys = get_monkey_business monkeys (fun i -> i mod 9699690) 10000

let input = In_channel.with_open_bin "input" In_channel.input_all
            |> monkeys

let () = Printf.printf "Part 1 result: %d\n" @@ part1 input;
         Printf.printf "Part 2 result: %d\n" @@ part2 input
