let str_to_tuple list =
  List.map (fun s -> Scanf.sscanf s "%d,%d" (fun a b -> (b, a))) list

let rec window list =
  match list with x :: (y :: _ as tl) -> (x, y) :: window tl | _ -> []

let range_to_list ((x1, y1), (x2, y2)) =
  match compare x1 x2 with
  | 0 -> List.init (Int.abs (y1 - y2) + 1) (fun i -> (x1, Int.min y1 y2 + i))
  | _ -> List.init (Int.abs (x1 - x2) + 1) (fun i -> (Int.min x1 x2 + i, y1))

let get_rock_paths list =
  List.map
    (fun s ->
      Str.(split (regexp " -> ")) s
      |> str_to_tuple |> window |> List.map range_to_list |> List.flatten)
    list
  |> List.flatten

let get_min_max init list =
  List.fold_left
    (fun ((xmin, ymin), (xmax, ymax)) (x, y) ->
      ((Int.min x xmin, Int.min y ymin), (Int.max x xmax, Int.max y ymax)))
    init list

let to_matrix list =
  let hd = List.hd list in
  let (_, ymin), (xmax, ymax) = get_min_max (hd, hd) list in
  let rows = xmax + 1 in
  let cols = ymax - ymin + 1 in
  let sand_y = 500 - ymin in
  let arr = Array.make_matrix rows cols '.' in
  List.map (fun (x, y) -> (x, y - ymin)) list
  |> List.iter (fun (x, y) -> arr.(x).(y) <- '#');
  arr.(0).(sand_y) <- '+';
  (sand_y, arr)

let to_matrix2 list =
  let hd = List.hd list in
  let (_, ymin), (xmax, ymax) = get_min_max (hd, hd) list in
  let rows = xmax + 1 + 2 in
  let cols = ymax - ymin + 1 + (2 * rows) in
  let sand_y = 500 - ymin + rows in
  let arr = Array.make_matrix rows cols '.' in
  List.map (fun (x, y) -> (x, y - ymin + rows)) list
  |> List.iter (fun (x, y) -> arr.(x).(y) <- '#');
  arr.(0).(sand_y) <- '+';
  List.iter (fun i -> arr.(rows - 1).(i) <- '#') (List.init cols Fun.id);
  (sand_y, arr)

let simulate_sand sand_y (_, arr) =
  let rows = Array.length arr in
  let cols = Array.length arr.(0) in
  let rec aux x y =
    if x >= 0 && x < rows - 1 && y > 0 && y < cols - 1 then
      match arr.(x + 1).(y) with
      | '.' -> aux (x + 1) y
      | '#' | 'o' ->
          if arr.(x + 1).(y - 1) = '.' then aux (x + 1) (y - 1)
          else if arr.(x + 1).(y + 1) = '.' then aux (x + 1) (y + 1)
          else
            let () = arr.(x).(y) <- 'o' in
            if x = 0 && y = sand_y then true else false
      | _ -> true
    else true
  in
  (aux 0 sand_y, arr)

let count_sand_units (sand_y, arr) =
  Seq.iterate (simulate_sand sand_y) (false, arr)
  |> Seq.take_while (fun (over, _) -> not over)
  |> Seq.map (fun (_, arr) -> arr)
  |> List.of_seq |> List.rev |> List.hd
  |> Array.fold_left
       (fun acc arr ->
         acc
         + Array.fold_left (fun acc c -> if c = 'o' then acc + 1 else acc) 0 arr)
       0

let input = Arg.read_arg "input" |> Array.to_list |> get_rock_paths

let () =
  Printf.printf "Part 1 result: %d\n" @@ count_sand_units (to_matrix input);
  Printf.printf "Part 2 result: %d\n" @@ count_sand_units (to_matrix2 input)
