use std::iter::{FromIterator, Iterator};

const INPUT: &str = include_str!("../../input/day_3.txt");

fn main() {
    let map: Map = INPUT
        .lines()
        .map(|l| l.chars().filter_map(Cell::try_from).collect())
        .collect();

    let tree_count = map.path(3, 1).tree_count();

    println!("tree count is {}", tree_count);

    let tree_product: usize = vec![
        map.path(1, 1),
        map.path(3, 1),
        map.path(5, 1),
        map.path(7, 1),
        map.path(1, 2),
    ]
    .into_iter()
    .map(|mut p| p.tree_count())
    .product();

    println!("tree product is {}", tree_product);
}

struct Map {
    grid: Vec<Vec<Cell>>,
    height: usize,
}

impl Map {
    pub fn path(&self, slope_x: usize, slope_y: usize) -> Path {
        Path {
            map: self,
            x: 0,
            y: 0,
            slope_x,
            slope_y,
        }
    }
}

struct Path<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
    slope_x: usize,
    slope_y: usize,
}

impl<'a> Path<'a> {
    pub fn tree_count(&mut self) -> usize {
        self.into_iter().filter(|c| c.is_tree()).count()
    }
}

impl<'a> Iterator for Path<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.y += self.slope_y;
        self.x += self.slope_x;

        if self.y >= self.map.height {
            return None;
        }

        let row = &self.map.grid[self.y];
        row.get(self.x % row.len())
    }
}

impl FromIterator<Vec<Cell>> for Map {
    fn from_iter<I: IntoIterator<Item = Vec<Cell>>>(iter: I) -> Self {
        let mut grid = Vec::new();
        for row in iter {
            grid.push(row);
        }
        Map {
            height: grid.len(),
            grid,
        }
    }
}

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Tree,
}

impl Cell {
    pub fn try_from(c: char) -> Option<Self> {
        match c {
            '.' => Some(Cell::Empty),
            '#' => Some(Cell::Tree),
            _ => None,
        }
    }

    pub fn is_tree(&self) -> bool {
        matches!(self, Cell::Tree)
    }
}
