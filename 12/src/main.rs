use anyhow::{anyhow, ensure, Result};
use std::{
    collections::BTreeMap,
    fs,
};

fn main() -> Result<()> {
    let input_str = fs::read_to_string("input.txt")?;

    let result = exercise_1(&input_str)?;
    println!("Result: {}", result);

    Ok(())
}

fn exercise_1(input_str: &str) -> Result<usize> {
    let mut garden = Garden::from_str(input_str)?;
    garden.build_groups()?;
    garden.calculate_fence_lengths()?;

    ensure!(garden.groups.keys().all(|g_key| garden.fences.contains_key(g_key)), "Not all groups have a fence length");
    ensure!(garden.fences.keys().all(|f_key| garden.groups.contains_key(f_key)), "Not all fences have a group");

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

#[derive(Debug, PartialEq, Clone)]
struct Garden {
    plants: Vec<Vec<Plant>>,
    groups: BTreeMap<usize, (char, Vec<(usize, usize)>)>,
    fences: BTreeMap<usize, (char, usize)>,
}

#[derive(Debug, PartialEq, Clone)]
struct Plant {
    plant_type: char,
    group_id: Option<usize>,
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
            groups: BTreeMap::new(),
            fences: BTreeMap::new(),
        })
    }

    fn get_fence_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        // Coordinates in the fence-space.
        let (x_p, y_p) = (x + 1, y + 1);
        let plant_type = self.plants[x][y].plant_type;

        vec![
            (x_p - 1, y_p),
            (x_p + 1, y_p),
            (x_p, y_p - 1),
            (x_p, y_p + 1),
        ]
        .into_iter()
        .filter(|(x_p, y_p)| {
            // Automatically fence the edge of the garden.
            if *x_p == 0
                || *x_p == self.plants.len() + 1
                || *y_p == 0
                || *y_p == self.plants[0].len() + 1
            {
                return true;
            }
            // Check if the neighbor is a different plant.
            let (x, y) = (x_p - 1, y_p - 1);
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
        let (_, group) = self
            .groups
            .get(&group_id)
            .ok_or(anyhow!("Group with id {} not found", group_id))?;

        let mut fenced: Vec<(usize, usize)> = Vec::new();
        for (x, y) in group {
            let fence_neighbors = self.get_fence_neighbors((*x, *y));
            fenced.extend(fence_neighbors);
        }

        // println!("Group {} ({}): {} => {:?}", group_id, p, fenced.len(), fenced);

        ensure!(
            fenced.len() >= 4,
            "Group with id {} has less than four fences!",
            group_id
        );

        Ok(fenced.len())
    }

    fn calculate_fence_lengths(&mut self) -> Result<()> {
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
    fn expected_output() -> usize {
        1930
    }

    #[rstest]
    fn test_exercise_1(
        #[from(sample_input)] input: &str,
        #[from(expected_output)] expected: usize,
    ) {
        let result = exercise_1(input).unwrap();
        assert_eq!(result, expected);
    }
}
