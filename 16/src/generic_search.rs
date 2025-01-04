use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

pub struct Node<T> {
    state: T,
    parent: Option<Rc<Node<T>>>,
    cost: usize,
    heuristic: usize,
}

impl<T> Node<T> {
    pub fn new(state: T, parent: Option<Rc<Node<T>>>, cost: usize, heuristic: usize) -> Self {
        Self {
            state,
            parent,
            cost,
            heuristic,
        }
    }

    pub fn get_cost(&self) -> usize {
        self.cost
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.cost + self.heuristic) == (other.cost + other.heuristic)
    }
}

impl<T> Eq for Node<T> {}

impl<T> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn node_to_path(&self) -> Vec<T> {
        let mut current_node = self;
        let mut path = vec![current_node.state.clone()];

        while let Some(ref parent) = current_node.parent {
            current_node = parent.as_ref();
            path.push(current_node.state.clone());
        }
        path.reverse();
        path
    }
}

pub fn dfs<T, F, G>(initial: T, goal_test: F, successors: G) -> Option<Node<T>>
where
    T: Clone + Eq + std::hash::Hash,
    F: Fn(&T) -> bool,
    G: Fn(&T) -> Vec<T>,
{
    let mut frontier = Vec::new();
    let mut explored = HashSet::new();
    frontier.push(Node::new(initial.clone(), None, 0, 0));
    explored.insert(initial);

    while let Some(current_node) = frontier.pop() {
        if goal_test(&current_node.state) {
            return Some(current_node);
        }
        let current_node = Rc::new(current_node);
        successors(&current_node.state)
            .into_iter()
            .filter(|c| explored.insert(c.clone()))
            .for_each(|c| frontier.push(Node::new(c, Some(Rc::clone(&current_node)), 0, 0)));
    }
    None
}

pub fn bfs<T, F, G>(initial: T, goal_test: F, successors: G) -> Option<Node<T>>
where
    T: Clone + Eq + std::hash::Hash,
    F: Fn(&T) -> bool,
    G: Fn(&T) -> Vec<T>,
{
    let mut frontier = VecDeque::new();
    let mut explored = HashSet::new();
    frontier.push_back(Node::new(initial.clone(), None, 0, 0));
    explored.insert(initial);

    while let Some(current_node) = frontier.pop_front() {
        if goal_test(&current_node.state) {
            return Some(current_node);
        }
        let current_node = Rc::new(current_node);
        successors(&current_node.state)
            .into_iter()
            .filter(|c| explored.insert(c.clone()))
            .for_each(|c| frontier.push_back(Node::new(c, Some(Rc::clone(&current_node)), 0, 0)));
    }
    None
}

pub fn astar<T, F, G, H, C>(
    initial: T,
    goal_test: F,
    successors: G,
    heuristic: H,
    cost: C,
) -> Option<Node<T>>
where
    T: Clone + Eq + std::hash::Hash,
    F: Fn(&T) -> bool,
    G: Fn(&T) -> Vec<T>,
    H: Fn(&T) -> usize,
    C: Fn(&Node<T>, &T) -> usize,
{
    let mut frontier = BinaryHeap::new();
    let mut explored = HashMap::new();
    frontier.push(Node::new(initial.clone(), None, 0, heuristic(&initial)));
    explored.insert(initial, 0);

    while let Some(current_node) = frontier.pop() {
        if goal_test(&current_node.state) {
            return Some(current_node);
        }
        let current_node = Rc::new(current_node);
        successors(&current_node.state)
            .into_iter()
            .map(|child| (cost(&current_node, &child), child))
            .for_each(|(new_cost, child)| match explored.entry(child.clone()) {
                Entry::Vacant(e) => {
                    e.insert(new_cost);
                    let h = heuristic(&child);
                    frontier.push(Node::new(
                        child,
                        Some(Rc::clone(&current_node)),
                        new_cost,
                        h,
                    ));
                }
                Entry::Occupied(mut e) if new_cost < *e.get() => {
                    e.insert(new_cost);
                    let h = heuristic(&child);
                    frontier.push(Node::new(
                        child,
                        Some(Rc::clone(&current_node)),
                        new_cost,
                        h,
                    ));
                }
                _ => {}
            });
    }
    None
}