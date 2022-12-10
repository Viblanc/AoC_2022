type instruction = Noop | Addx of int

let to_instruction str =
  match String.split_on_char ' ' str with
    ["noop"] -> Noop
  | ["addx"; n] -> Addx (int_of_string n)
  | _ -> failwith "unexpected instruction"

let compute_instruction (x, cycle) = function
    Noop -> [x, cycle + 1]
  | Addx n -> [x + n, cycle + 2; x, cycle + 1]

let execute_program init instructions =
  let rec aux (x, cycle) = function
      [] -> []
    | hd :: tl ->
       let res = compute_instruction (x, cycle) hd in
       res :: aux (List.hd res) tl
  in init :: (List.flatten @@ aux init instructions)

let part1 instructions =
  execute_program (1, 1) instructions
  |> List.filter (fun (_, cycle) -> cycle = 20
                                    || cycle = 60
                                    || cycle = 100
                                    || cycle = 140
                                    || cycle = 180
                                    || cycle = 220)
  |> List.fold_left (fun acc (x, c) -> acc + c * x) 0

(* Part 2 *)
let lit_pixel idx register =
  let i = idx mod 40 in
  let range = [register - 1; register; register + 1] in
  match List.mem i range with
    true -> if i = 39 then "#\n" else "#"
  | false -> if i = 39 then ".\n" else "."

let draw_lines instructions =
  instructions
  |> List.mapi (fun i (x, _) -> lit_pixel i x)
  |> String.concat ""

let part2 instructions =
  execute_program (1, 1) instructions
  |> List.sort (fun (_, c1) (_, c2) -> compare c1 c2)
  |> draw_lines

let input = Arg.read_arg "input"
            |> Array.to_list
            |> List.map to_instruction

let part1_res = part1 input
let part2_res = part2 input

let () = Printf.printf "Part 1 result: %d\n" part1_res;
         Printf.printf "Part 2 result: \n%s" part2_res
