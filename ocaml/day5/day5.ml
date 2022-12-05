open Core

let input = In_channel.read_lines "day5/input"

let crates () =
  let rec aux res = function
      [] -> res
    | hd :: tl -> if String.is_empty hd then
                    res
                  else
                    aux (String.to_list hd :: res) tl in
  let lines = List.rev @@ List.tl_exn @@ aux [] input
              |> List.map ~f:(List.filteri ~f:(fun i _ -> not (i mod 4 = 3))) in
  List.transpose_exn lines
  |> List.map ~f:(List.filter ~f:(fun c -> not (Char.equal c '['
                                                || Char.equal c ']'
                                                || Char.is_whitespace c)))
  |> List.filter ~f:(fun l -> not (List.is_empty l))
  |> List.to_array

type operation =
  { n: int;
    from: int;
    dest: int;
  }

let parse_move s =
  let f _ x _ y _ z = { n = x; from = y - 1; dest = z - 1; } in
  Scanf.sscanf s "%s %d %s %d %s %d" f

let moves =
  let rec aux = function
      [] -> []
    | hd :: tl -> if String.is_empty hd then
                    tl
                  else
                    aux tl in
  aux input
  |> List.map ~f:parse_move

let do_step op arr ~model =
  let { n; from; dest } = op in
  let old_stack = Array.get arr from in
  let new_stack = Array.get arr dest in
  let shipment =
    let c = List.take old_stack n in
    match model with
      9000 -> List.rev c
    | 9001 -> c
    | _ -> raise (Failure "what the hell") in
  Array.set arr from (List.drop old_stack n);
  Array.set arr dest (List.append shipment new_stack)

let get_top_crate = function
    Some c -> c
  | None -> ' '
  
let part1 =
  let crates = crates () in
  let () = List.iter ~f:(fun op -> do_step ~model:9000 op crates) moves in
  crates
  |> Array.map ~f:(fun l -> get_top_crate @@ List.hd l)
  |> Array.to_list |> String.of_char_list

let part2 =
  let crates = crates () in
  let () = List.iter ~f:(fun op -> do_step ~model:9001 op crates) moves in
  crates
  |> Array.map ~f:(fun l -> get_top_crate @@ List.hd l)
  |> Array.to_list |> String.of_char_list

let () =
  Printf.printf "Part 1 result: %s\n" part1;
  Printf.printf "Part 2 result: %s\n" part2
