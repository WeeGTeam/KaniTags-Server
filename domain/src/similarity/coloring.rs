use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct ColoredGroup<'i, I> {
    pub color: usize,
    pub items: &'i [&'i I],
}

pub fn color_and_merge_groups<'i, I: Hash + Eq + Clone>(item_groups: HashMap<&'i I, Vec<&'i I>>) -> Vec<Vec<I>> {
    let mut colored_item_groups = init_colored_groups(&item_groups);
    for item in item_groups.keys() {
        color_similarity_groups(&mut colored_item_groups, &item);
    }
    merge_groups(colored_item_groups)
}

fn init_colored_groups<'i, I: Eq + Hash>(item_groups: &'i HashMap<&'i I, Vec<&'i I>>) -> HashMap<&'i I, ColoredGroup<'i, I>> {
    item_groups.iter()
        .enumerate()
        .map(|(index, (&item_id, group))| {
            (item_id, ColoredGroup { color: index, items: &group })
        })
        .collect::<HashMap<_, _>>()
}

fn color_similarity_groups<'i, I: Eq + Hash>(all_groups: &mut HashMap<&'i I, ColoredGroup<'i, I>>, current_item_id: &I) {
    let colored_group = match all_groups.get(current_item_id) {
        Some(group) => group,
        None => return,
    };
    let group_items = colored_group.items;
    let colors = group_items.iter()
        .flat_map(|item_id| all_groups.get(item_id))
        .map(|n| n.color)
        .collect::<HashSet<_>>();
    if colors.len() <= 1 {
        return;
    }
    let new_color = colors.into_iter().min().unwrap();
    let mut other_groups_items_to_visit = Vec::new();
    for item in group_items {
        match all_groups.get_mut(item) {
            Some(g) => {
                if g.color != new_color {
                    g.color = new_color;
                    other_groups_items_to_visit.push(item);
                } else {
                }
            },
            None => continue,
        }
    }
    for other_item in other_groups_items_to_visit {
        color_similarity_groups(all_groups, other_item);
    }
}

fn merge_groups<'i, I: Eq + Hash + Clone>(item_groups: HashMap<&'i I, ColoredGroup<'i, I>>) -> Vec<Vec<I>> {
    let mut group_map = HashMap::new();
    for (_item_id, group) in item_groups {
        let group_color = group.color;
        let group_items = group_map.entry(group_color).or_insert_with(HashSet::new);
        group_items.extend(group.items.iter());
    }
    group_map.into_values()
        .map(|v| v.into_iter().cloned().collect())
        .collect::<Vec<_>>()
}
