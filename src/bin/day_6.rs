const INPUT: &str = include_str!("../../input/day_6.txt");

fn main() {
    let groups = parse_groups(INPUT);

    let sum = sum_count(&groups, |g| g.anyone_yes_count());
    println!("sum of anyone counts is {}", sum);

    let sum = sum_count(&groups, |g| g.everyone_yes_count());
    println!("sum of everyone counts is {}", sum);
}

fn parse_groups(input: &str) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut group = Group(Vec::new());
    for line in input.lines() {
        if line.is_empty() {
            groups.push(group);
            group = Group(Vec::new());
            continue;
        }
        group.0.push(line)
    }

    if !group.0.is_empty() {
        groups.push(group);
    }
    groups
}

fn sum_count<F>(groups: &[Group], f: F) -> usize
where
    F: FnMut(&Group) -> usize,
{
    groups.iter().map(f).sum()
}

#[derive(Debug)]
pub struct Group<'a>(Vec<&'a str>);

impl<'a> Group<'a> {
    fn counts(&self) -> Vec<i32> {
        let mut counts = vec![0; 26];
        for answers in &self.0 {
            for answer in answers.trim().chars() {
                counts[(answer as u32 - 97) as usize] += 1;
            }
        }
        counts
    }

    pub fn anyone_yes_count(&self) -> usize {
        self.counts().into_iter().filter(|&c| c > 0).count()
    }

    pub fn everyone_yes_count(&self) -> usize {
        let people = self.0.len() as i32;
        self.counts().into_iter().filter(|&c| c == people).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_groups, sum_count, Group};

    #[test]
    fn test_anyone_yes_count() {
        let group = Group(vec!["abc"]);
        assert_eq!(group.anyone_yes_count(), 3);
    }

    #[test]
    fn test_anyone_yes_count_multiple() {
        let group = Group(vec!["abc", "abd"]);
        assert_eq!(group.anyone_yes_count(), 4);
    }

    #[test]
    fn test_anyone() {
        let groups = parse_groups(
            r#"abc

a
b
c

ab
ac

a
a
a
a

b"#,
        );

        assert_eq!(sum_count(&groups, |g| g.anyone_yes_count()), 11);
    }

    #[test]
    fn test_everyone() {
        let groups = parse_groups(
            r#"abc

a
b
c

ab
ac

a
a
a
a

b"#,
        );

        assert_eq!(sum_count(&groups, |g| g.everyone_yes_count()), 6);
    }
}
