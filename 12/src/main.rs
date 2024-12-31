use anyhow::{anyhow, ensure, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result_1 = exercise_1(&input_str)?;
    println!("Result 1: {}", result_1);

    let result_2 = exercise_2(&input_str)?;
    println!("Result 2: {}", result_2);

    Ok(())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let mut garden = Garden::from_str(input_str)?;
    garden.build_groups()?;
    garden.calculate_all_fences()?;

    ensure!(
        garden
            .groups
            .keys()
            .all(|g_key| garden.fences.contains_key(g_key)),
        "Not all groups have a fence length"
    );
    ensure!(
        garden
            .fences
            .keys()
            .all(|f_key| garden.groups.contains_key(f_key)),
        "Not all fences have a group"
    );

    garden
        .fences
        .iter()
        .map(|(group_id, (_, fence_length))| {
            let (_, group_members) = garden
                .groups
                .get(group_id)
                .ok_or(anyhow!("Group with id {} not found", group_id))?;
            let group_size = group_members.len();
            Ok(fence_length * group_size)
        })
        .sum()
}

fn exercise_2(input_str: &str) -> Result<usize> {
    let mut garden = Garden::from_str(input_str)?;
    garden.build_groups()?;
    garden.calculate_all_sides()?;

    ensure!(
        garden
            .groups
            .keys()
            .all(|g_key| garden.sides.contains_key(g_key)),
        "Not all groups have a fence length"
    );
    ensure!(
        garden
            .sides
            .keys()
            .all(|s_key| garden.groups.contains_key(s_key)),
        "Not all sides have a group"
    );

    garden
        .sides
        .iter()
        .map(|(group_id, (_, num_sides))| {
            let (_, group_members) = garden
                .groups
                .get(group_id)
                .ok_or(anyhow!("Group with id {} not found", group_id))?;
            let group_size = group_members.len();
            Ok(num_sides * group_size)
        })
        .sum()
}

#[derive(Debug, PartialEq, Clone)]
struct Garden {
    plants: Vec<Vec<Plant>>,
    groups: HashMap<usize, (char, Vec<(usize, usize)>)>,
    fences: HashMap<usize, (char, usize)>,
    sides: HashMap<usize, (char, usize)>,
}

#[derive(Debug, PartialEq, Clone)]
struct Plant {
    plant_type: char,
    group_id: Option<usize>,
}

#[derive(Debug, PartialEq)]
enum Fence {
    North(usize, usize),
    East(usize, usize),
    South(usize, usize),
    West(usize, usize),
}

impl Fence {
    fn get_coordinates(&self) -> (usize, usize) {
        match self {
            Fence::North(x, y) => (*x, *y),
            Fence::East(x, y) => (*x, *y),
            Fence::South(x, y) => (*x, *y),
            Fence::West(x, y) => (*x, *y),
        }
    }
}

impl Plant {
    fn new(plant_type: char) -> Self {
        Self {
            plant_type,
            group_id: None,
        }
    }
}

impl Garden {
    fn from_str(input_str: &str) -> Result<Self> {
        let plants: Vec<Vec<Plant>> = input_str
            .lines()
            .map(|line| line.chars().map(Plant::new).collect())
            .collect();

        ensure!(
            plants.iter().all(|row| row.len() == plants[0].len()),
            "All rows must have the same length"
        );
        ensure!(
            plants.iter().all(|row| row
                .iter()
                .all(|plant| plant.plant_type.is_ascii_uppercase())),
            "All plants must be uppercase letters"
        );

        Ok(Self {
            plants,
            groups: HashMap::new(),
            fences: HashMap::new(),
            sides: HashMap::new(),
        })
    }

    fn get_fence_neighbors(&self, (x, y): (usize, usize)) -> Vec<Fence> {
        // Coordinates in the fence-space.
        let plant_type = self.plants[x][y].plant_type;

        vec![
            Fence::North(x, y),
            Fence::East(x, y),
            Fence::South(x, y),
            Fence::West(x, y),
        ]
        .into_iter()
        .filter(|fence| {
            let (x_f, y_f) = fence.get_coordinates();
            // Automatically fence the edge of the garden.
            if (x_f == 0 && fence == &Fence::North(x, y))
                || (x_f == self.plants.len() - 1 && fence == &Fence::South(x, y))
                || (y_f == 0 && fence == &Fence::West(x, y))
                || (y_f == self.plants[0].len() - 1 && fence == &Fence::East(x, y))
            {
                return true;
            }
            // Check if the neighbor is a different plant.
            let (x, y) = match fence {
                Fence::North(x, y) => (*x - 1, *y),
                Fence::East(x, y) => (*x, *y + 1),
                Fence::South(x, y) => (*x + 1, *y),
                Fence::West(x, y) => (*x, *y - 1),
            };
            self.plants[x][y].plant_type != plant_type
        })
        .collect()
    }

    fn get_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.plants.len() - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.plants[0].len() - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn get_same_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let plant_type = self.plants[x][y].plant_type;
        self.get_neighbors((x, y))
            .into_iter()
            .filter(|&(x, y)| self.plants[x][y].plant_type == plant_type)
            .collect()
    }

    fn build_group(&mut self, (x, y): (usize, usize), group_id: usize) -> Result<()> {
        // Check that the group is empty before inserting to hashmaps.
        ensure!(
            !self.groups.contains_key(&group_id),
            "Group with id {} already exists",
            group_id
        );
        ensure!(
            !self.fences.contains_key(&group_id),
            "Fence with id {} already exists",
            group_id
        );
        ensure!(
            !self.sides.contains_key(&group_id),
            "Sides with id {} already exists",
            group_id
        );

        let mut to_visit = vec![(x, y)];
        let mut group: Vec<(usize, usize)> = Vec::new();
        while let Some((x, y)) = to_visit.pop() {
            // Specifying a group_id means that the plant has already been visited.
            if self.plants[x][y].group_id.is_some() {
                continue;
            }
            self.plants[x][y].group_id = Some(group_id);
            group.push((x, y));
            // Extend the queue with the neighbors of the current plant.
            to_visit.extend(
                self.get_same_neighbors((x, y))
                    .into_iter()
                    // Only keep neighbors that have not been visited yet (group_id is None).
                    .filter(|&(x, y)| self.plants[x][y].group_id.is_none()),
            );
        }

        // Update hashmaps.
        self.groups
            .insert(group_id, (self.plants[x][y].plant_type, group));
        self.fences
            .insert(group_id, (self.plants[x][y].plant_type, 0));
        self.sides
            .insert(group_id, (self.plants[x][y].plant_type, 0));

        Ok(())
    }

    fn build_groups(&mut self) -> Result<()> {
        let mut group_id = 0;
        for x in 0..self.plants.len() {
            for y in 0..self.plants[0].len() {
                if self.plants[x][y].group_id.is_none() {
                    self.build_group((x, y), group_id)?;
                    group_id += 1;
                }
            }
        }
        Ok(())
    }

    fn calculate_fence_length(&self, group_id: usize) -> Result<usize> {
        let fenced = self.get_fences(group_id)?;

        ensure!(
            fenced.len() >= 4,
            "Group with id {} has less than four fences!",
            group_id
        );

        Ok(fenced.len())
    }

    fn get_fences(&self, group_id: usize) -> Result<Vec<Fence>> {
        let (_, group) = self
            .groups
            .get(&group_id)
            .ok_or(anyhow!("Group with id {} not found", group_id))?;
        let mut fenced: Vec<Fence> = Vec::new();
        for (x, y) in group {
            let fence_neighbors = self.get_fence_neighbors((*x, *y));
            fenced.extend(fence_neighbors);
        }
        Ok(fenced)
    }

    fn calculate_number_sides(&self, group_id: usize) -> Result<usize> {
        let fenced = self.get_fences(group_id)?;

        let n_sides = calculate_sides_from_north_fences(group_id, &fenced)?;
        let s_sides = calculate_sides_from_south_fences(group_id, &fenced)?;
        let e_sides = calculate_sides_from_east_fences(group_id, &fenced)?;
        let w_sides = calculate_sides_from_west_fences(group_id, &fenced)?;

        ensure!(
            n_sides + s_sides + e_sides + w_sides >= 4,
            "Group with id {} has less than four sides!",
            group_id
        );
        Ok(n_sides + s_sides + e_sides + w_sides)
    }

    fn calculate_all_fences(&mut self) -> Result<()> {
        ensure!(
            self.groups.len() == self.fences.len(),
            "Number of groups and fences must be equal"
        );
        for group_id in self.groups.keys() {
            ensure!(
                self.fences.contains_key(group_id),
                "Fence length not found for group {}",
                group_id
            );
            ensure!(
                self.fences.get(group_id).unwrap().1 == 0,
                "Fence length already calculated for group {}",
                group_id,
            );
            let fence_length = self.calculate_fence_length(*group_id)?;
            self.fences.get_mut(group_id).unwrap().1 = fence_length;
        }
        Ok(())
    }

    fn calculate_all_sides(&mut self) -> Result<()> {
        ensure!(
            self.groups.len() == self.sides.len(),
            "Number of groups and sides must be equal"
        );
        for group_id in self.groups.keys() {
            ensure!(
                self.sides.contains_key(group_id),
                "Sides not found for group {}",
                group_id
            );
            ensure!(
                self.sides.get(group_id).unwrap().1 == 0,
                "Sides already calculated for group {}",
                group_id,
            );
            let num_sides = self.calculate_number_sides(*group_id)?;
            self.sides.get_mut(group_id).unwrap().1 = num_sides;
        }
        Ok(())
    }
}

fn calculate_sides_from_north_fences(group_id: usize, fenced: &[Fence]) -> Result<usize> {
    let mut north_fences: HashSet<(usize, usize)> = fenced
        .iter()
        .filter_map(|fence| match fence {
            Fence::North(x, y) => Some((*x, *y)),
            _ => None,
        })
        .collect();

    let mut n_sides: usize = 0;
    while let Some(&(h_x, h_y)) = north_fences.iter().next() {
        // Fetch a horizontal fence which will become a side.
        n_sides += 1;
        // Remove all adjacent horizontal fences.
        remove_horizontal(&mut north_fences, (h_x, h_y));
        // Then remove the original fence.
        north_fences.remove(&(h_x, h_y));
    }
    ensure!(
        n_sides >= 1,
        "Group with id {} has less than one north sides!",
        group_id
    );

    Ok(n_sides)
}

fn calculate_sides_from_south_fences(group_id: usize, fenced: &[Fence]) -> Result<usize> {
    let mut south_fences: HashSet<(usize, usize)> = fenced
        .iter()
        .filter_map(|fence| match fence {
            Fence::South(x, y) => Some((*x, *y)),
            _ => None,
        })
        .collect();

    let mut s_sides: usize = 0;
    while let Some(&(h_x, h_y)) = south_fences.iter().next() {
        // Fetch a horizontal fence which will become a side.
        s_sides += 1;
        // Remove all adjacent horizontal fences.
        remove_horizontal(&mut south_fences, (h_x, h_y));
        // Then remove the original fence.
        south_fences.remove(&(h_x, h_y));
    }
    ensure!(
        s_sides >= 1,
        "Group with id {} has less than one south sides!",
        group_id
    );

    Ok(s_sides)
}

fn calculate_sides_from_east_fences(group_id: usize, fenced: &[Fence]) -> Result<usize> {
    let mut east_fences: HashSet<(usize, usize)> = fenced
        .iter()
        .filter_map(|fence| match fence {
            Fence::East(x, y) => Some((*x, *y)),
            _ => None,
        })
        .collect();

    let mut e_sides: usize = 0;
    while let Some(&(v_x, v_y)) = east_fences.iter().next() {
        // Fetch a vertical fence which will become a side.
        e_sides += 1;
        // Remove all adjacent vertical fences.
        remove_vertical(&mut east_fences, (v_x, v_y));
        // Then remove the original fence.
        east_fences.remove(&(v_x, v_y));
    }
    ensure!(
        e_sides >= 1,
        "Group with id {} has less than one east sides!",
        group_id
    );

    Ok(e_sides)
}

fn calculate_sides_from_west_fences(group_id: usize, fenced: &[Fence]) -> Result<usize> {
    let mut west_fences: HashSet<(usize, usize)> = fenced
        .iter()
        .filter_map(|fence| match fence {
            Fence::West(x, y) => Some((*x, *y)),
            _ => None,
        })
        .collect();

    let mut w_sides: usize = 0;
    while let Some(&(w_x, w_y)) = west_fences.iter().next() {
        // Fetch a vertical fence which will become a side.
        w_sides += 1;
        // Remove all adjacent vertical fences.
        remove_vertical(&mut west_fences, (w_x, w_y));
        // Then remove the original fence.
        west_fences.remove(&(w_x, w_y));
    }
    ensure!(
        w_sides >= 1,
        "Group with id {} has less than one west sides!",
        group_id
    );

    Ok(w_sides)
}

fn remove_right(horizontal_fences: &mut HashSet<(usize, usize)>, (x, y): (usize, usize)) {
    // Base case, there is no neighbor to remove.
    if !horizontal_fences.contains(&(x, y + 1)) {
        return;
    }

    horizontal_fences.remove(&(x, y + 1));
    remove_right(horizontal_fences, (x, y + 1));
}

fn remove_left(horizontal_fences: &mut HashSet<(usize, usize)>, (x, y): (usize, usize)) {
    // Base case, there is no neighbor to remove.
    if y == 0 || !horizontal_fences.contains(&(x, y - 1)) {
        return;
    }

    horizontal_fences.remove(&(x, y - 1));
    remove_left(horizontal_fences, (x, y - 1));
}

fn remove_horizontal(horizontal_fences: &mut HashSet<(usize, usize)>, (x, y): (usize, usize)) {
    remove_right(horizontal_fences, (x, y));
    remove_left(horizontal_fences, (x, y));
}

fn remove_up(vertical_fences: &mut HashSet<(usize, usize)>, (x, y): (usize, usize)) {
    // Base case, there is no neighbor to remove.
    if x == 0 || !vertical_fences.contains(&(x - 1, y)) {
        return;
    }

    vertical_fences.remove(&(x - 1, y));
    remove_up(vertical_fences, (x - 1, y));
}

fn remove_down(vertical_fences: &mut HashSet<(usize, usize)>, (x, y): (usize, usize)) {
    // Base case, there is no neighbor to remove.
    if !vertical_fences.contains(&(x + 1, y)) {
        return;
    }

    vertical_fences.remove(&(x + 1, y));
    remove_down(vertical_fences, (x + 1, y));
}

fn remove_vertical(vertical_fences: &mut HashSet<(usize, usize)>, (x, y): (usize, usize)) {
    remove_up(vertical_fences, (x, y));
    remove_down(vertical_fences, (x, y));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_input() -> &'static str {
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
    }

    #[fixture]
    fn expected_output_1() -> usize {
        1930
    }

    #[fixture]
    fn expected_output_2() -> usize {
        1206
    }

    #[rstest]
    fn test_exercise_1(
        #[from(sample_input)] input: &str,
        #[from(expected_output_1)] expected: usize,
    ) {
        let result = exercise_1(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    fn test_exercise_2(
        #[from(sample_input)] input: &str,
        #[from(expected_output_2)] expected: usize,
    ) {
        let result = exercise_2(input).unwrap();
        assert_eq!(result, expected);
    }
}
