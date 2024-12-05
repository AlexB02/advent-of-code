use std::{collections::{HashMap, HashSet}, fs};

#[derive(Debug)]
struct Rule {
    page: u32,
    depends_on: u32
} 

fn parse_rule(rule: &str) -> Rule {
    // Parse a rule from a string like "1|2" to mean page 2 depends on page 1 or 1 must be printed before 2
    let mut parts = rule.split("|");
    let depends_on = parts.next().unwrap().parse::<u32>().unwrap();
    let page = parts.next().unwrap().parse::<u32>().unwrap();

    Rule {
        page,
        depends_on
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>
}

#[derive(Debug)]
struct RulesAndUpdates {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}


fn read_file(file_path: &str) -> RulesAndUpdates {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut rules_and_updates = RulesAndUpdates {
        rules: Vec::new(),
        updates: Vec::new(),
    };

    for line in contents.lines() {
        if line.contains("|") {
            rules_and_updates.rules.push(parse_rule(line));
        } else if line.contains(",") {
            let parts = line.split(",");
            let pages = parts.map(|part| part.parse::<u32>().unwrap()).collect();
            rules_and_updates.updates.push(Update { pages });
        }
    }

    rules_and_updates
}

fn rules_to_deps(rules: &Vec<Rule>) -> HashMap<u32, Vec<u32>> {
    let mut deps: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in rules {
        let entry = deps.entry(rule.page).or_insert(Vec::new());
        entry.push(rule.depends_on);
    }
    deps
}

fn is_update_valid(deps: &HashMap<u32, Vec<u32>>, update: &Update) -> bool {
    let mut seen_pages: HashSet<u32> = HashSet::new();
    for i in 0..update.pages.len() {
        let page = update.pages[i];
        seen_pages.insert(page);
        let depends_on = match deps.get(&page) {
            None => &Vec::new(),
            Some(deps) => deps
        };
        // Have we seen all the pages that this page depends on?
        // Only consider dependencies that are in the list of updates. i.e. no update, no dependency
        for dep in depends_on {
            if !seen_pages.contains(dep) && update.pages.contains(dep) {
                return false;
            }
        }
    }

    true
}

fn get_corrected_update(deps: &HashMap<u32, Vec<u32>>, update: &Update) -> Update {
    // Insert into corrected_pages while maintaining the invariant
    // that all elements are placed after their dependencies.
    let mut corrected_pages: Vec<u32> = Vec::new();
    
    for element in &update.pages {
        let element_deps: &Vec<u32> = match deps.get(&element) {
            Some(deps) => deps,
            None => &Vec::new(),
        };
        let filtered_deps: HashSet<u32> = HashSet::from_iter(element_deps.iter().filter(|x| update.pages.contains(x) && corrected_pages.contains(x)).cloned());
        let mut already_seen: HashSet<u32> = HashSet::new();
        if corrected_pages.len() == 0 {
            corrected_pages.push(element.clone());
        } else {
            for i in 0..=corrected_pages.len() {
                let pending_dependencies: HashSet<&u32> = filtered_deps.difference(&already_seen).collect();
                if pending_dependencies.len() == 0 {
                    if i == corrected_pages.len() {
                        corrected_pages.push(element.clone());
                    } else {
                        corrected_pages.insert(i, element.clone());
                    }
                    break;
                }
                if i == corrected_pages.len() {
                    break;
                }
                already_seen.insert(corrected_pages[i]);
            }
        }
    }
    Update { pages: corrected_pages }
}

fn main() {
    let file_path = "day5/input.txt";
    let rules_and_updates = read_file(file_path);

    let deps = rules_to_deps(&rules_and_updates.rules);

    let mut sum_of_middle_pages = 0;

    for update in rules_and_updates.updates {
        let is_valid_update = is_update_valid(&deps, &update);
        if !is_valid_update {
            let corrected_update = get_corrected_update(&deps, &update);
            let is_valid_update = is_update_valid(&deps, &corrected_update);
            if is_valid_update {
                let middle_page = corrected_update.pages[corrected_update.pages.len() / 2];
                sum_of_middle_pages += middle_page;
            }
        }
    }

    println!("Sum of middle pages: {}", sum_of_middle_pages);
}