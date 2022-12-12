module TreeNode = struct
  type t = { name: string;
             size: int;
             parent: t option;
             children: t list; }

  let create name size = { name; size; parent = None; children = [] }

  let rec to_root node =
    match node.parent with
      Some p -> to_root p
    | None -> node

  let add_child node child =
    { node with size = node.size + child.size;
                children = child :: node.children }

  let change_dir node dirname =
    let child = List.find (fun c -> c.name = dirname) node.children
    in { child with parent = Some node }

  let goto_parent node =
    match node.parent with
      Some p -> { p with children = List.map (fun c -> if c.name = node.name then
                                                         node
                                                       else
                                                         c) p.children }
    | None -> raise (Failure "no parent")

  let rec get_size node =
    List.fold_left (fun acc child -> acc + get_size child) node.size node.children
  
  let from_list cmds =
    List.fold_left
      (fun node cmd ->
        match String.split_on_char ' ' cmd with
          ["$"; "ls"] -> node
        | ["$"; "cd"; ".."] -> goto_parent node
        | ["$"; "cd"; dname] -> change_dir node dname
        | ["dir"; dname] -> add_child node (create dname 0)
        | [size; _] -> { node with size = node.size + int_of_string size }
        | _ -> raise (Failure "unexpected input"))
      { name = "/"; size = 0; parent = None; children = [] }
      (List.tl cmds)
    |> goto_parent |> to_root
  
  let rec filter f node =
    List.fold_left
      (fun acc child -> acc @ filter f child)
      (if f node then [node] else [])
      node.children
end

let part1 tree =
  TreeNode.filter (fun node -> TreeNode.get_size node <= 100_000) tree
  |> List.fold_left (fun acc node -> acc + TreeNode.get_size node) 0

(* Part 2 *)
let part2 tree =
  let free_space = 70000000 - TreeNode.get_size tree in
  let needed_space = 30000000 - free_space in
  TreeNode.filter (fun node -> TreeNode.get_size node >= needed_space) tree
  |> List.fold_left
       (fun min node -> if TreeNode.get_size node < min then
                          TreeNode.get_size node
                        else
                          min)
       (TreeNode.get_size tree)

let input = Arg.read_arg "input"
            |> Array.to_list
            |> TreeNode.from_list

let () =
  Printf.printf "Part 1 result: %d\n" @@ part1 input;
  Printf.printf "Part 2 result: %d\n" @@ part2 input
