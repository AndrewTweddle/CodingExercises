use num::integer::Roots;
use pathfinding::prelude::astar;
use petgraph::algo::bellman_ford;
use petgraph::graph::{NodeIndex, UnGraph};
use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;

fn main() {
    let mut start_time = Instant::now();
    let contents = fs::read_to_string("data/day23_input.txt").unwrap();
    for rep in 0..=NUM_REPETITIONS {
        let solution = solve(&contents).expect("No solution could be found");
        if rep == 0 {
            println!("Day 23 part 1 solution: {}", solution);
            println!("Duration, incl I/O: {:?}", start_time.elapsed());
            start_time = Instant::now();
        }
    }

    if NUM_REPETITIONS > 0 {
        let avg_duration = start_time.elapsed() / NUM_REPETITIONS;
        println!(
            "Avg duration, excl I/O, over {} further repetitions: {:?}",
            NUM_REPETITIONS, avg_duration
        );
    }
}

fn solve(diagram: &str) -> Option<Energy> {
    let initial_pod_positions = convert_diagram_to_pod_positions(diagram);

    let distance_and_mask_lkp = get_distances_and_mask_lookup();

    // The lookup above uses petgraph to do the calculations over a graph of empty cells.
    // However, for the graph of states, it doesn't make sense to use petgraph.
    // Firstly, it takes over a second just to initialize the nodes for a graph of this size.
    // Secondly, many states will never be visited, so why represent them?
    //
    // Instead treat the graph of states as a conceptual idea, rather than trying to instantiate it.
    //
    // This imagined graph has nodes in the range 0..M (u32's),
    // where M = (15 choose 2) * (13 choose 2) * (11 choose 2) * (9 choose 2) = 16,216,200.
    // since this is the number of unique states with 2 A's, 2 B's, 2 C's and 2 D's.

    let initial_state = convert_amphipod_positions_to_state(&initial_pod_positions);
    let target_state = convert_amphipod_positions_to_state(&TARGET_POD_POSITIONS);
    let result = astar(
        &initial_state,
        |state| state.get_successors(&distance_and_mask_lkp),
        |state| state.get_lower_bound_cost_to_reach_target_state(&distance_and_mask_lkp),
        |state| *state == target_state,
    );
    result.map(|(_path, energy)| energy)
}

// ----------------------------------------------------------------------------
// 1. Occupiable cells (each '.') in order from top to bottom and left to right:
//
// #############
// #..x.x.x.x..#   0..7        Note that the x's are positions that can be moved through, but not occupied.
// ###.#.#.#.###   7..11
//   #.#.#.#.#    11..15
//   #########
//
// When we need all cells will also include the 4 x's, from left to right, in positions 15 to 18.
const TOTAL_CELL_COUNT: usize = 19;
const OCCUPIABLE_CELL_COUNT: usize = 15;
const OCCUPIABLE_HALLWAY_SLOT_COUNT: u8 = 7;

// ----------------------------------------------------------------------------
// 2. For each pair of occupiable cells, a source and a destination,
//    we will need to know the distance in steps to move from the source to the destination.
//
//    We also want to know which cells along the way (excluding source, including the destination)
//    must be unoccupied, for the move to be legal.
//
//    Since there are only 15 occupiable cells, a u16 bit mask can be used to store these cells.
//    The bit in position i, 0 <= i < 15, will be set to 1 if cell i needs to be empty, because it's on the path.
//    This makes it easy to do a bitwise and of these slots with the bitwise flag of
//    actual occupied slots in the current state, to see if there is any overlap.
//
//    How can we check the path between an amphipod position and target positions?
//      a. We can create a graph with indices 0 to 18.
//         Positions 0 to 14 are the normal ones. But 15 to 18 are the "lobbies" outside of each side room.
//      b. We can create a graph of these 18 indices, with adjacent edges,
//         and a u16 bit mask with a single bit set for indices (bit positions) 0 to 14.
//      c. Then create a mapping for each source and destination index, from 0 to 14 (so a 15x15 matrix)
//          whose entries are:
//              i. The distance from source to destination, for cost calculations.
//             ii. The bit mask of all cells on the path from source to destination,
//                 excluding the source, including the destination.

#[derive(Copy, Clone, Default, Debug)]
struct DistanceAndMask {
    distance: Energy,
    mask: u16,
}

// A lookup of distance and mask data, where the first index is the src slot, and the second is the destination slot:
type DistanceAndMaskLookup = [[DistanceAndMask; OCCUPIABLE_CELL_COUNT]; OCCUPIABLE_CELL_COUNT];

fn get_distances_and_mask_lookup() -> DistanceAndMaskLookup {
    let mut lookup = [[DistanceAndMask::default(); OCCUPIABLE_CELL_COUNT]; OCCUPIABLE_CELL_COUNT];
    let cell_graph = get_cell_graph();

    // use Bellman-Ford, not Floyd-Warshall algorithm, since we want to calculate masks, so need the path back
    for (src, src_lookup) in lookup.iter_mut().enumerate() {
        let bf_result = bellman_ford(&cell_graph, (src as u8).into());
        let paths = bf_result.unwrap_or_else(|_| panic!("Bellman-Ford algorithm failed with node {}", src));
        for (dst, (&weight, predecessor)) in
            paths.distances.iter().zip(&paths.predecessors).enumerate()
        {
            if (dst == src) || (dst >= OCCUPIABLE_CELL_COUNT) {
                continue;
            }
            src_lookup[dst].distance = weight as Energy;
            let mut mask: u16 = 1 << dst;
            let mut next_predecessor: &Option<NodeIndex<u8>> = predecessor;
            while let Some(prev_node) = next_predecessor {
                let prev_cell_ix: u8 = prev_node.index() as u8;
                if prev_cell_ix == src as u8 {
                    break;
                }
                if prev_cell_ix < OCCUPIABLE_CELL_COUNT as u8 {
                    mask |= 1 << prev_cell_ix;
                }
                next_predecessor = &paths.predecessors[prev_cell_ix as usize];
            }
            src_lookup[dst].mask = mask;
        }
    }

    lookup
}

const CELL_GRAPH_EDGE_COUNT: usize = 18;
const CELL_GRAPH_EDGES: [(u8, u8, f32); CELL_GRAPH_EDGE_COUNT] = [
    (0, 1, 1.0),
    (1, 15, 1.0),
    (15, 2, 1.0),
    (2, 16, 1.0),
    (16, 3, 1.0),
    (3, 17, 1.0),
    (17, 4, 1.0),
    (4, 18, 1.0),
    (18, 5, 1.0),
    (5, 6, 1.0),
    (15, 7, 1.0),
    (16, 8, 1.0),
    (17, 9, 1.0),
    (18, 10, 1.0),
    (7, 11, 1.0),
    (8, 12, 1.0),
    (9, 13, 1.0),
    (10, 14, 1.0),
];

fn get_cell_graph() -> UnGraph<u8, f32, u8> {
    let mut cell_graph = UnGraph::with_capacity(TOTAL_CELL_COUNT, CELL_GRAPH_EDGE_COUNT);
    for cell_id in 0..TOTAL_CELL_COUNT {
        assert_eq!(cell_graph.add_node(1_u8), (cell_id as u8).into());
    }
    cell_graph.extend_with_edges(CELL_GRAPH_EDGES);
    cell_graph
}

// ----------------------------------------------------------------------------
// 3. The state can be expressed conveniently as a set of positions of the amphipods:
//      [a0, a1, b0, b1, c0, c1, d0, d1],
//          where these are all unique,
//          0 <= a0 < a1 < 15 and similarly for each other pair.

type PodPositions = [u8; 8];

const TARGET_POD_POSITIONS: PodPositions = [7, 11, 8, 12, 9, 13, 10, 14];

#[rustfmt::skip]
const CELL_INDEXES_TO_MAP_POSITIONS: [(usize, usize); OCCUPIABLE_CELL_COUNT] = [
    (1, 1), (1, 2),         (1, 4),         (1, 6),         (1, 8),         (1, 10), (1, 11),
    (2, 3),         (2, 5),         (2, 7),         (2, 9),
    (3, 3),         (3, 5),         (3, 7),         (3, 9),
];

fn convert_diagram_to_pod_positions(diagram: &str) -> PodPositions {
    let mut pod_positions = [OCCUPIABLE_CELL_COUNT as u8; 8];

    let map_rows: Vec<&[u8]> = diagram.lines().map(|line| line.as_bytes()).collect();
    for (i, &(row, col)) in CELL_INDEXES_TO_MAP_POSITIONS.iter().enumerate() {
        let symbol = map_rows[row][col];
        if let b'A'..=b'D' = symbol {
            let pod_type_index = (symbol - b'A') as usize;
            if pod_positions[2 * pod_type_index] == 15 {
                pod_positions[2 * pod_type_index] = i as u8;
            } else {
                pod_positions[2 * pod_type_index + 1] = i as u8;
            }
        }
    }

    pod_positions
}

// ----------------------------------------------------------------------------
// 4. How can we represent states compactly as an integer?
//
//     One option, is that each cell can contain one of 5 symbols:  'ABCD.' (0..5)
//
//     We could use 3 bits per cells, and 15 cells, using 45 bits, and a u64 representation.
//     But a lot of these states are unachievable, since there must always be exactly 2 A's, 2 B's, 2 C's and 2 D's.
//
//     Rather, represent intermediate states in the search tree:
//
//     A's: 15 choose 2 = 15 * 7  =         105    14 choices for the lower index, and any higher index for the other A
//     B's: 13 choose 2 = 13 * 6  =          78
//     C's: 11 choose 2 = 11 * 5  =          55
//     D's:  9 choose 2 =  9 * 4 =           36
//     Number of states (product) =  16 216 200
//
//     Since every 4 bits can be used to represent the position of a single amphipod, and 8 bits can be used for each pair,
//     we can use 4 x 8 = 32 bits to represent all the positions.
//     This is still 4,294,967,296 states, and only 16.2 million of these are valid.
//
//     Instead we could represent each combination of two amphipods as follows:
//
//     Row:
//       0        x                0 = (0,n-1)
//       1       x x               1 = (0,n-2), 2 = (1,n-1)
//       2      x x x              3 = (0,n-3), 4 = (1,n-2), 5 = (2,n-1)
//       3     x x x x
//       4    x x x x x            ... T[r] + k = (k, n - 1 - r + k) ...
//       5   x x x x x x
//       6  x x x x x x x          T[n-1] = (0, 1), ..., T[n-1] + k = (k, k+1)..., T[n] - 1 = (n-1, n)
//       7 * * * * * * * *
//     n=8
//
//     Each pair in the final row (of *'s) maps to a unique position in the tree of x's above it, and vice versa.

// ----------------------------------------------------------------------------
// 5. We need to be able to convert an index into this triangular array into an ordered pair of points...
//
// Convert n and p to a pair of numbers in the range {0, 1, ..., n-1} as follows:
//
//     8 * T[r] + 1 = (2r + 1)^2, so given a number p (for pair) in {0, 1,... T[n]-1}, we can convert it to a pair using:
//
//     a. Get r by choosing p such that T[r] <= p < T[r+1], so r = tri_inverse(p).
//        To calculate tri_inverse(p), set 8p + 1 = (2r+1)^2, then solve for p:
//        tri_inverse(p) = (int_sqrt(8 * p + 1) - 1) / 2, where the division is integral
//     b. Get k as p - T[r]
//     c. Given (r, k), the pair of numbers are (k, n - r + k).

fn convert_triangular_index_to_ordered_pair(p: u8, n: u8) -> (u8, u8) {
    let r = inv_triangular(p as u32) as u8;
    let t = triangular(r);
    let k = p - t;
    (k, n - 1 - r + k)
}

fn inv_triangular(p: u32) -> u32 {
    ((8 * p + 1).sqrt() - 1) / 2
}

// ----------------------------------------------------------------------------
// 6. Convert ordered pair (a0, a1), and n, with 0 <= a0 < a1 < n, to a single number A:
//
//     a. Let k = a0, r = n - a1 + a0
//     b. Return T[r-1] + k, where T[m] = triangular(m) = m * (m + 1) / 2
//     c. Call this function convert_ordered_pair_to_triangular_index(a0: u8, a1: u8, n: u8) -> u32

fn convert_ordered_pair_to_triangular_index(pair: (u8, u8), n: u8) -> u8 {
    let k = pair.0;
    let r = n - pair.1 + k;
    triangular(r - 1) + k
}

#[inline(always)]
fn triangular(n: u8) -> u8 {
    n * (n + 1) / 2
}

// ----------------------------------------------------------------------------
// 7. Create State struct and methods to encapsulate the u32 state id, and be able to:
//
//    a. Determine a lower bound on the cost of moving from this state to the target state,
//       to be used in the A* search for the cheapest path to the target state.
//    b. Determine all moves that can transform this state into another state,
//       along with the energy cost of each move. This is also used by the A* search algorithm.
//
//      Notes on heuristic cost of each state for A* search:
//
//          i. For each amphipod, calculate its contribution to the estimated cost, as follows:
//         ii. If the pod is in its side-room already, the cost is zero.
//        iii. If the pod is not in its side-room, use the cost to move it to the top slot of its side-room.
//         iv. If the pod is in the upper slot of its side-room, but there is a different type of amphipod
//             in the lower slot, then calculate the minimum cost to move out of the side-room,
//             then move back after the other amphipod has moved out.

type StateId = u32;
type Energy = u32;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct State {
    id: StateId,
}

const MAX_EDGES_PER_STATE: usize = OCCUPIABLE_CELL_COUNT - 1;

type Move = (State, Energy);

impl State {
    fn get_positional_representation(&self) -> PodPositions {
        convert_state_to_amphipod_positions(self)
    }

    fn get_lower_bound_cost_to_reach_target_state(
        &self,
        distance_and_mask_lkp: &DistanceAndMaskLookup,
    ) -> Energy {
        let mut energy: Energy = 0;
        let pod_positions = self.get_positional_representation();
        for pod_type_index in 0..4 {
            let cost_per_step: Energy = 10_u32.pow(pod_type_index as u32);
            let pos0_index = 2 * pod_type_index;
            let pos1_index = 2 * pod_type_index + 1;
            let current_posns = (pod_positions[pos0_index], pod_positions[pos1_index]);
            let upper_slot = 7 + pod_type_index as u8;
            let lower_slot = 11 + pod_type_index as u8;

            let is_lower_slot_occupied_by_pos_0 = current_posns.0 == lower_slot;
            let is_lower_slot_occupied_by_pos_1 = current_posns.1 == lower_slot;
            let is_lower_slot_correct =
                is_lower_slot_occupied_by_pos_0 || is_lower_slot_occupied_by_pos_1;
            let is_lower_slot_occupied_by_another_type = !is_lower_slot_correct
                && pod_positions
                    .iter()
                    .any(|&pos_index| pos_index == lower_slot);

            let is_upper_slot_occupied_by_pos_0 = current_posns.0 == upper_slot;
            let is_upper_slot_occupied_by_pos_1 = current_posns.1 == upper_slot;
            let is_upper_slot_correct =
                is_upper_slot_occupied_by_pos_0 || is_upper_slot_occupied_by_pos_1;

            if !is_lower_slot_correct {
                if is_lower_slot_occupied_by_another_type && is_upper_slot_correct {
                    // The amphipod in the upper slot must move out the minimum distance
                    // to let the lower amphipod out, since it is not of the correct type.
                    // This involves moving 1 space up, then 1 space left or right,
                    // then 3 spaces to the bottom slot after the other amphipod has vacated the bottom slot.
                    energy += 5 * cost_per_step;
                } else {
                    // Either the lower slot is open,
                    // or the amphipod in the lower slot could move to its own side-room,
                    // perhaps allowing the lower slot to be moved to directly
                    energy += cost_per_step;
                }
            }

            if !(is_lower_slot_occupied_by_pos_0 || is_upper_slot_occupied_by_pos_0) {
                // Add the energy cost for the amphipod in pos.0 to move into the upper slot
                let distance =
                    distance_and_mask_lkp[current_posns.0 as usize][upper_slot as usize].distance;
                energy += distance * cost_per_step;
            }

            if !(is_lower_slot_occupied_by_pos_1 || is_upper_slot_occupied_by_pos_1) {
                // Add the energy cost for the amphipod in pos.0 to move into the upper slot
                let distance =
                    distance_and_mask_lkp[current_posns.1 as usize][upper_slot as usize].distance;
                energy += distance * cost_per_step;
            }
        }
        energy
    }

    fn get_successors(&self, distance_and_mask_lkp: &DistanceAndMaskLookup) -> Vec<Move> {
        let mut moves = Vec::with_capacity(MAX_EDGES_PER_STATE);

        let mut pod_positions = self.get_positional_representation();
        let mut occupied_mask: u16 = 0;
        for p in pod_positions {
            occupied_mask |= 1 << p;
        }

        for pod_type_index in 0..4 {
            let cost_per_step: Energy = 10_u32.pow(pod_type_index as u32);
            let upper_slot = 7 + pod_type_index as u8;
            let lower_slot = 11 + pod_type_index as u8;

            for pod_index in 0..2 {
                let this_pod_index = 2 * pod_type_index + pod_index;
                let this_pod_slot = pod_positions[this_pod_index];

                if this_pod_slot == lower_slot {
                    // already in position, so don't move...
                    continue;
                }

                let that_pod_index = 2 * pod_type_index + 1 - pod_index;
                let that_pod_slot = pod_positions[that_pod_index];

                // Don't try to move into the hallway, if already in the hallway
                let mut should_try_to_move_to_hallway =
                    this_pod_slot >= OCCUPIABLE_HALLWAY_SLOT_COUNT;

                // Don't try to move into the lower side-room slot, if the other pod of the same type is already there
                let mut should_try_to_move_to_lower_sideroom_slot = that_pod_slot != lower_slot;

                // ...and conversely, try to move into the upper side-room slot
                // if the other pod of the same type is in the lower side-room slot
                let mut should_try_to_move_to_upper_sideroom_slot =
                    !should_try_to_move_to_lower_sideroom_slot;

                if this_pod_slot == upper_slot {
                    if that_pod_slot == lower_slot {
                        // both pods are in position, so don't move...
                        continue;
                    }
                    // We're in a strange situation where the upper slot is correct, but not the lower slot.
                    // It's not possible to reach a position where the lower slot is empty,
                    // so it must have another pod type in that slot, which will need to get out.
                    // So try to move this pod out of its side-room, back into the hall, to let the other pod out.
                    should_try_to_move_to_lower_sideroom_slot = false;
                    should_try_to_move_to_upper_sideroom_slot = false;
                    should_try_to_move_to_hallway = true;
                }

                if should_try_to_move_to_lower_sideroom_slot {
                    if let Some(move_into_lower_pos) = try_move(
                        &mut pod_positions,
                        this_pod_index,
                        lower_slot,
                        cost_per_step,
                        occupied_mask,
                        distance_and_mask_lkp,
                    ) {
                        // If we can move into the correct position directly, then do so as the only move,
                        // since there is never a reason not to, and it reduces the search space
                        moves.clear();
                        moves.push(move_into_lower_pos);
                        return moves;
                    }
                }

                if should_try_to_move_to_upper_sideroom_slot {
                    if let Some(move_into_upper_pos) = try_move(
                        &mut pod_positions,
                        this_pod_index,
                        upper_slot,
                        cost_per_step,
                        occupied_mask,
                        distance_and_mask_lkp,
                    ) {
                        // If we can move into the correct position directly, then do so as the only move,
                        // since there is never a reason not to, and it reduces the search space
                        moves.clear();
                        moves.push(move_into_upper_pos);
                        return moves;
                    }
                }

                if should_try_to_move_to_hallway {
                    for dst_slot in 0..OCCUPIABLE_HALLWAY_SLOT_COUNT {
                        if let Some(move_out) = try_move(
                            &mut pod_positions,
                            this_pod_index,
                            dst_slot,
                            cost_per_step,
                            occupied_mask,
                            distance_and_mask_lkp,
                        ) {
                            moves.push(move_out);
                        }
                    }
                }
            }
        }

        moves
    }
}

fn try_move(
    pod_positions: &mut PodPositions,
    this_pod_index: usize,
    dst_slot: u8,
    cost_per_step: Energy,
    occupied_mask: u16,
    distance_and_mask_lkp: &DistanceAndMaskLookup,
) -> Option<Move> {
    let src_slot = pod_positions[this_pod_index];
    if dst_slot == src_slot {
        // don't move to the same position
        return None;
    }

    let lookup = &distance_and_mask_lkp[src_slot as usize][dst_slot as usize];
    let path_mask = lookup.mask;
    if path_mask & occupied_mask != 0 {
        None
    } else {
        let (that_pod_index, expected_order) = if this_pod_index % 2 == 0 {
            (this_pod_index + 1, Ordering::Less)
        } else {
            (this_pod_index - 1, Ordering::Greater)
        };
        let that_slot = pod_positions[that_pod_index];

        if dst_slot.cmp(&that_slot) == expected_order {
            pod_positions[this_pod_index] = dst_slot;
        } else {
            // Switch positions of the 2 slots, so that the lower index is always first
            pod_positions[this_pod_index] = that_slot;
            pod_positions[that_pod_index] = dst_slot;
        }

        let new_state = convert_amphipod_positions_to_state(pod_positions);
        pod_positions[this_pod_index] = src_slot;
        pod_positions[that_pod_index] = that_slot;
        let cost_of_move = lookup.distance * cost_per_step;
        let new_move = (new_state, cost_of_move);
        Some(new_move)
    }
}

// ----------------------------------------------------------------------------
// 8. Helper method to convert state id S to a valid set of unique positions,
//     [a0, a1, b0, b1, c0, c1, d0, d1], with a0 < a1, ..., d0 < d1:
//

fn convert_state_to_amphipod_positions(state: &State) -> PodPositions {
    let mut state_id = state.id;
    let mut pod_posns = [0_u8; 8];
    let mut taken_pos_flags: u16 = 0;
    set_next_2_pod_positions(&mut state_id, &mut pod_posns, &mut taken_pos_flags, 0);
    set_next_2_pod_positions(&mut state_id, &mut pod_posns, &mut taken_pos_flags, 2);
    set_next_2_pod_positions(&mut state_id, &mut pod_posns, &mut taken_pos_flags, 4);
    set_next_2_pod_positions(&mut state_id, &mut pod_posns, &mut taken_pos_flags, 6);
    pod_posns
}

fn set_next_2_pod_positions(
    state_id: &mut StateId,
    pod_posns: &mut [u8; 8],
    taken_pos_flags: &mut u16,
    next_index: u8,
) {
    let n = 15 - next_index;
    let base = n_choose2(n) as u32;
    let pair_id = (*state_id) % base;
    *state_id /= base;
    let mut pair = convert_triangular_index_to_ordered_pair(pair_id as u8, n);

    // The indices of the pair are with respect to the remaining positions.
    // So they need to be adjusted upwards, skipping over any numbers that have already been taken.
    let mut flag_mask = 1;
    let mut flag_mask_0 = 0;

    for i in 0..OCCUPIABLE_CELL_COUNT {
        if *taken_pos_flags & flag_mask != 0 {
            if i as u8 <= pair.0 {
                pair.0 += 1;
            }
            pair.1 += 1;
        } else {
            if pair.0 == i as u8 {
                // Save this flag for later (updating taken_posn_flags now will influence the adjustment of pair.1)
                flag_mask_0 = flag_mask;
            } else if pair.1 == i as u8 {
                // We are done with the adjustments, since the second index is in place.
                *taken_pos_flags |= flag_mask_0 | flag_mask;
                break;
            }
        }
        flag_mask *= 2;
    }
    pod_posns[next_index as usize] = pair.0;
    pod_posns[next_index as usize + 1] = pair.1;
}

#[inline(always)]
fn n_choose2(n: u8) -> u8 {
    triangular(n - 1)
}

// ----------------------------------------------------------------------------
// 9. Helper method to convert positions P = [a0, a1, b0, b1, c0, c1, d0, d1] into a compact state S.
//

fn convert_amphipod_positions_to_state(positions: &PodPositions) -> State {
    let mut state_id: StateId = 0;
    let mut multiplier = 1;
    let mut taken_posn_flags: u16 = 0;

    update_state_id_with_next_2_positions(
        positions,
        &mut state_id,
        &mut multiplier,
        &mut taken_posn_flags,
        0,
    );
    update_state_id_with_next_2_positions(
        positions,
        &mut state_id,
        &mut multiplier,
        &mut taken_posn_flags,
        2,
    );
    update_state_id_with_next_2_positions(
        positions,
        &mut state_id,
        &mut multiplier,
        &mut taken_posn_flags,
        4,
    );
    update_state_id_with_next_2_positions(
        positions,
        &mut state_id,
        &mut multiplier,
        &mut taken_posn_flags,
        6,
    );

    State { id: state_id }
}

fn update_state_id_with_next_2_positions(
    &positions: &PodPositions,
    state_id: &mut StateId,
    multiplier: &mut u32,
    taken_posn_flags: &mut u16,
    next_index: u8,
) {
    let mut pair = (
        positions[next_index as usize],
        positions[next_index as usize + 1],
    );

    // The indices of the pair are of the cells.
    // But they need to be with respect to the remaining positions.
    // So they need to be adjusted downwards for each lower number that has already been taken.
    let flag0: u16 = 1 << pair.0;
    let mask0: u16 = flag0 - 1;
    pair.0 -= (*taken_posn_flags & mask0).count_ones() as u8;
    let flag1 = 1 << pair.1;
    let mask1 = flag1 - 1;
    pair.1 -= (*taken_posn_flags & mask1).count_ones() as u8;

    // Now mark these two numbers as taken
    *taken_posn_flags |= flag0;
    *taken_posn_flags |= flag1;

    let pair_digit = convert_ordered_pair_to_triangular_index(pair, 15 - next_index);

    *state_id += *multiplier * pair_digit as u32;
    *multiplier *= n_choose2(15 - next_index) as u32;
}

// ----------------------------------------------------------------------------
// 10. Unit tests
#[cfg(test)]
mod tests {
    mod triangular_pair_conversions {
        use super::super::{
            convert_ordered_pair_to_triangular_index, convert_triangular_index_to_ordered_pair,
        };

        #[test]
        fn convert_smallest_and_largest_to_index() {
            let pair = (0, 14);
            let index = convert_ordered_pair_to_triangular_index(pair, 15);
            assert_eq!(index, 0);
        }

        #[test]
        fn convert_zero_to_smallest_and_largest_pair() {
            let pair = convert_triangular_index_to_ordered_pair(0, 15);
            assert_eq!((pair), (0, 14));
        }

        #[test]
        fn convert_largest_pair_to_index() {
            let pair = (13, 14);
            let index = convert_ordered_pair_to_triangular_index(pair, 15);
            assert_eq!(index, 14 * 15 / 2 - 1);
        }

        #[test]
        fn convert_first_pair_to_index_and_back() {
            let pair1 = (10, 14);
            let id = convert_ordered_pair_to_triangular_index(pair1, 15);
            let pair2 = convert_triangular_index_to_ordered_pair(id, 15);
            assert_eq!(pair1, pair2)
        }
    }

    mod search_algorithm_tests {
        use super::super::solve;

        const EXAMPLE_DIAGRAM: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

        #[test]
        fn test_example() {
            let minimum_energy = solve(EXAMPLE_DIAGRAM).unwrap();
            assert_eq!(minimum_energy, 12521);
        }
    }
}
