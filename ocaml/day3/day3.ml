open Core

let split_in_two str =
  let rec aux res s =
    if List.length res = List.length s then
      (res, s)
    else
      aux (List.hd_exn s :: res) (List.tl_exn s)
in aux [] str

let input = In_channel.read_lines "day3/input"

module CharsSet = Set.Make(Char)
module CharsMap = Map.Make(Char)

let fill_priorities ?(start = 'a') () =
  let first_priority =
    match start with
    | 'a' -> 1
    | 'A' -> 27
    | _ -> raise (Failure "not a valid character")
  in
  List.init 26 ~f:(fun i -> int_of_char start + i)
  |> List.mapi ~f:(fun idx i -> (char_of_int i, idx + first_priority))

let priorities = CharsMap.of_alist_exn (fill_priorities ()
                                       |> List.append @@ fill_priorities ~start:'A' ())

let find_item (fst_pocket, snd_pocket) =
  CharsSet.find_exn ~f:(fun c -> CharsSet.mem snd_pocket c) fst_pocket

let sum l = List.fold ~init:0 ~f:Int.(+) l

let part1 = input
            |> List.map ~f:String.to_list
            |> List.map ~f:split_in_two
            |> List.map ~f:(fun (a, b) -> CharsSet.of_list a, CharsSet.of_list b)
            |> List.map ~f:find_item
            |> List.map ~f:(CharsMap.find_exn priorities)
            |> sum

let rec group_by_3 = function
    [] -> []
  | x :: y :: z :: tl -> (x, y, z) :: group_by_3 tl
  | _ -> raise (Failure "invalid number of elements in list")

let rec find_common_item (elf1, elf2, elf3) =
  let item = find_item (elf1, elf2) in
  match CharsSet.mem elf3 item with
    true -> item
  | false -> find_common_item (CharsSet.remove elf1 item,
                               CharsSet.remove elf2 item,
                               elf3)

let map_triple f (a, b, c) = f a, f b, f c

let part2 = input
            |> group_by_3
            |> List.map ~f:(map_triple String.to_list)
            |> List.map ~f:(map_triple CharsSet.of_list)
            |> List.map ~f:find_common_item
            |> List.map ~f:(CharsMap.find_exn priorities)
            |> sum

let () =
  Printf.printf "Part 1 result: %d\n" part1;
  Printf.printf "Part 2 result: %d\n" part2;
