use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use regex;
use ::csv as csv_reader;
use serde::Deserialize;

use djanco::*;

use djanco::database::*;
use djanco::log::*;
use djanco::csv::*;

use djanco::objects::Head;
use djanco::objects::ItemWithData;
use djanco::objects::Language;
use djanco::objects::Project;

use djanco::time::Duration;
use djanco_ext::*;
use regex::Regex;

const SELECTIONS: usize = 10;
const SELECTED_PROJECTS: usize = 30;
const SEEDS: [u128; 10] = [1,2,3,5,7,11,13,17,19,23]; // one seed per selection

// Base commit is going to be a commit this many percent commits in the past.
//
// Eg. if there are 12 commits and BASE_COMMIT_OFFSET_RATIO is 25, then
// the base commit will be 12 * 25 / 100 = 3 commits pushed back from the head.
//
// A B C D E F G H I J K L
//                 ^     ^
//                 |     |
//                 BASE  HEAD
//
// All math is done on integers.
const BASE_COMMIT_OFFSET_RATIO: usize = 10;

#[djanco(May, 2021, subsets(Generic))]
pub fn all_projects(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        //.filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .into_csv_in_dir(output, "info/javascript_projects.csv")
}

#[djanco(May, 2021, subsets(Generic))]
pub fn all_projects_extended(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        //.filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .into_extended_csv_in_dir(output, "info/javascript_projects_extended.csv")
}

#[djanco(May, 2021, subsets(Generic))]
pub fn project_locs(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
    //.filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
    .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
    .map_into(Select!(project::URL, project::Locs))
    .into_csv_with_headers_in_dir(vec!["url", "locs"], output, "info/project_locs.csv")
}

#[macro_export]
macro_rules! one_per_selection {
    {$function:ident ($database:ident, $log:ident, $output:ident)} => {{
        for i in 0..SELECTIONS {
            $function($database, $log, $output, i)?;
        }
        Ok(())
    }}
}

#[djanco(May, 2021, subsets(Generic))] 
pub fn select_random_projects(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    one_per_selection!{ random_projects(database, log, output) }
}

pub fn random_projects(database: &Database, _log: &Log, output: &Path, seed_index: usize) -> Result<(), std::io::Error>  {
    database.projects()
        //.filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        //.filter(is_project_spec)
        .sample(Random(SELECTED_PROJECTS, Seed(SEEDS[seed_index])))
        //.flat_map(project_spec)
        .map_into(project::URL)
        .into_csv_with_headers_in_dir(vec!["url"], output, format!("selections/random_projects_{}.csv", seed_index))
}

#[djanco(May, 2021, subsets(Generic))] pub fn select_random_projects_by_size(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    one_per_selection!{ random_projects_by_size(database, log, output) }
}

pub fn random_projects_by_size(database: &Database, _log: &Log, output: &Path, seed_index: usize) -> Result<(), std::io::Error> {
    database.projects()
        //.filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        //.filter(is_project_spec)
        .sample(Stratified(project::Locs, 
                                  Strata!("very large" -> Random(SELECTED_PROJECTS/4, Seed(SEEDS[seed_index])), 
                                          "large" -> Random(SELECTED_PROJECTS/4, Seed(SEEDS[seed_index])), 
                                          "medium" -> Random(SELECTED_PROJECTS/4, Seed(SEEDS[seed_index])),
                                          "small" -> Random(SELECTED_PROJECTS/4, Seed(SEEDS[seed_index]))),
                                  Thresholds::Inclusive(Conditions!("very large" -> 100_000,
                                                                    "large" -> 10_000,
                                                                    "medium" -> 1000),
                                                                    "small")))
        //.flat_map(project_spec)
        .map_into(project::URL)
        .into_csv_with_headers_in_dir(vec!["url"], output, format!("selections/random_projects_by_size_{}.csv", seed_index))
}

//#[djanco(May, 2021, subsets(Generic))] 
pub fn all_specs(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error> {
    database.projects()
        //.filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .flat_map(project_spec)
        .into_csv_with_headers_in_dir(vec!["url", "to", "from"], output, "selections/all_project_specs.csv")
}

#[djanco(May, 2021, subsets(Generic))]
pub fn select_top_starred(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut top_starred = database.projects()
        //.filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .sort_by(project::Stars)
        //.filter(is_project_spec)
        .sample(Top(SELECTIONS * SELECTED_PROJECTS))
        //.flat_map(project_spec)
        .collect::<Vec<ItemWithData<Project>>>();

    for seed_index in 0..SELECTIONS {
        // Select N projects from the pool of top-starred projects
        let selection = top_starred.clone().into_iter()
            .sample(Random(SELECTED_PROJECTS, Seed(SEEDS[seed_index])))
            .collect::<Vec<ItemWithData<Project>>>();
        
        // Remove the N selected projects from the pool of top-starred projects
        top_starred
            .retain(|project| !selection.contains(project));

        // Write out selection. 
        selection.into_iter()
            .map_into(project::URL)
            .into_csv_with_headers_in_dir(
                vec!["url"], output, 
                format!("selections/top_starred_projects_{}.csv", seed_index))?
    }

    Ok(())
}
 
#[djanco(May, 2021, subsets(Generic))]
pub fn select_quality_projects(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    one_per_selection!{ quality_projects(database, log, output) }
}

pub fn quality_projects(database: &Database, _log: &Log, output: &Path, seed_index: usize) -> Result<(), std::io::Error>  {
    database.projects()
        //.filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        // Contains at least 80% JavaScript code
        .filter(|project| {
            project.language_composition().map_or(false, |languages| {
                languages.into_iter()
                    .any(|(language, propotion)| {
                        language == Language::JavaScript && propotion >= 80
                    })
            })
        })
        // Contains at least 5KLOC in the head tree.
        .filter_by(AtLeast(project::Locs, 5_000))
        // The spanm between first and last commit is at least 1 year
        .filter_by(AtLeast(project::Age, Duration::from_months(12)))
        // Contains at least 100 commits total
        .filter_by(AtLeast(Count(project::Commits), 100))        
        // Has at least 2 users
        .filter_by(AtLeast(Count(project::Users), 2))
        // Only pick projects for which we can generate a base and head commit        
        //.filter(is_project_spec)
        // Sample N projects at random (we're just going to do one selection, so take first seed)        
        .sample(Random(SELECTED_PROJECTS, Seed(SEEDS[seed_index]))) 
        // Extract: url, head commit aka to, base commit aka from
        //.flat_map(project_spec)
        .map_into(project::URL)
        .into_csv_with_headers_in_dir(vec!["url"], 
            output, 
            format!("selections/quality_projects_{}.csv", seed_index))
}

#[derive(Deserialize, Debug, PartialEq, Eq, std::hash::Hash)]
struct Url { url: String }

#[djanco(May, 2021, subsets(Generic))]
pub fn xxx_generate_project_spec_form_selections(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut selections_dir = PathBuf::from(output);
    selections_dir.push("selections");
    
    let mut project_selection_assignments: HashMap<Url, Vec<String>> = HashMap::new();

    let extension = Regex::new(".csv$").unwrap();
    let selections = std::fs::read_dir(&selections_dir)?;
    for selection in selections {
        let selection = selection?;
        let selection_path = selection.path();
        let selection_file = selection.file_name().to_str().unwrap().to_owned();            
        let selection_name = extension.replace(&selection_file, "").to_string();
        
        // if selection_name.as_str() == "all" { continue }
        // if !extension.is_match(selection_file) { continue }

        let mut csv = csv_reader::Reader::from_path(selection_path)?;
        
        for row in csv.deserialize() {
            let project_url: Url = row?;

            //println!("Adding {} to {:?}", selection_name, project_url);
            project_selection_assignments.entry(project_url)
                .and_modify(|vector| vector.push(selection_name.clone()))
                .or_insert(vec![selection_name.clone()]);
        }
    }

    // let project_urls = project_selection_assignments.iter()
    //     .map(|(url, _)| url.url.as_str().to_owned())
    //     .collect::<HashSet<String>>();

    //database.clone().projects().for_each(|x| println!("{}", x.url()));

    let project_specs = database.projects().into_iter()
        //.filter(|project| project_urls.contains(project.url()))
        .map(|p| { println!(">> {}", p.url()); p })
        .flat_map(project_spec)
        .map(|p| { println!(">> {:?}", p); p })
        .map(|(url, to, from)| (url.clone(), (url, to, from)))
        .collect::<HashMap<String, (String, String, String)>>();

    let mut selection_specs: HashMap<String, Vec<(String, String, String)>> = HashMap::new();
    for (url, selections) in project_selection_assignments {
        //println!("{:?} -> {:?}", url, selections);
        let project_spec = project_specs.get(&url.url);
        if project_spec.is_none() {
            //println!("No spec for project {}, skipping. (Should appear in selections: {})", url.url, selections.join(", "));
            continue
        }
        //println!("!");
        let project_spec = project_spec.unwrap().clone();

        for selection in selections {
            selection_specs.entry(selection)
                .and_modify(|vector| vector.push(project_spec.clone()))
                .or_insert(vec![project_spec.clone()]);
        }
    }

    for (selection, project_specs) in selection_specs {
        //println!("SELECTION: {}", selection);
        project_specs.into_iter().into_csv_with_headers_in_dir(
            vec!["url","to","from"], 
            &output, 
            format!("specs/{}.csv", selection))?
    }

    Ok(())
}

// Helper functions:
type ProjectURL = String;
type CommitHash = String;

pub fn is_project_spec<'a>(project: &ItemWithData<'a, Project>) -> bool {
    _project_spec(project).is_some()
}
pub fn project_spec<'a>(project: ItemWithData<'a, Project>) -> Option<(ProjectURL, CommitHash, CommitHash)> {
    _project_spec(&project)
}
pub fn _project_spec<'a>(project: &ItemWithData<'a, Project>) -> Option<(ProjectURL, CommitHash, CommitHash)> {
    let url = project.url();
    
    let default_branch = project.default_branch();
    if default_branch.is_none() {
        eprintln!("WARNING: Default branch not found for project {} ({:?}), skipping.", project.id(), url);
        return None;
    }
    let default_branch = default_branch.unwrap();
    let default_branch_path = format!("refs/heads/{}", default_branch);

    let heads = project.heads_with_data();
    if heads.is_none() {
        eprintln!("WARNING: Heads not found for project {} ({:?}), skipping.", project.id(), url);
        return None;
    }
    let heads = heads.unwrap();

    //eprintln!("INFO: heads in project {} ({:?}): {}", project.id(), url, heads.iter().map(|head| format!("{}:{}:{:?}", head.name(), head.commit_id(), head.commit_with_data().unwrap().hash().unwrap()) ).collect::<Vec<String>>().join("\n"));

    let default_branch_head = 
        heads.into_iter()
            .filter(|head| head.name() == default_branch_path)            
            .collect::<Vec<ItemWithData<Head>>>();

    if default_branch_head.len() == 0 {
        eprintln!("WARNING: No branch {} found in project {} ({:?}), skipping.", default_branch, project.id(), url);
        return None;
    }
    if default_branch_head.len() > 1 {
        eprintln!("WARNING: More than one ({}) branch {} found in project {} ({:?}), continuing.", 
                  default_branch_head.len(), default_branch, project.id(), url);
    }
    let default_branch_head = default_branch_head[0].clone();   
    
    let head_commit_hash = default_branch_head.hash();    
    // Now getting it dfirectly from head.
    // let head_commit = default_branch_head.commit_with_data();
    // if head_commit.is_none() {
    //     eprintln!("WARNING: Head commit inaccessible from branch {} in project {} ({:?}), skipping.", 
    //               default_branch, project.id(), url);
    //     return None;
    // }
    // let head_commit = head_commit.unwrap();
    //
    // let head_commit_hash = head_commit.hash();
    // if head_commit_hash.is_none() {
    //     eprintln!("WARNING: Head commit hash unavaiable for head commit {} from branch {} in project {} ({:?}), skipping.", 
    //               head_commit.id(), default_branch, project.id(), url);
    //     return None;
    // }
    // let head_commit_hash = head_commit_hash.unwrap();

    let mut commits = default_branch_head.commits_with_data();

    // Newest first.
    commits.sort_by_key(|commit| commit.committer_timestamp());
    commits.reverse();

    let total_commits = commits.len();
    let base_commit_offset = (BASE_COMMIT_OFFSET_RATIO * total_commits) / 100;

    eprintln!("INFO: Base commit offset is {} (of {}) for project {} ({:?})", 
              base_commit_offset, total_commits, project.id(), url);

    let base_commit = commits.iter().take(base_commit_offset).last();
    if base_commit.is_none() {
        eprintln!("WARNING: Base commit unavaiable for for branch {} in project {} ({:?}), skipping.", 
                  default_branch, project.id(), url);
        return None;
    }
    let base_commit = base_commit.unwrap();


    let base_commit_hash = base_commit.hash();
    if base_commit_hash.is_none() {
        eprintln!("WARNING: Base commit hash unavaiable for base commit {} from branch {} in project {} ({:?}), skipping.", 
                  base_commit.id(), default_branch, project.id(), url);
        return None;
    }
    let base_commit_hash = base_commit_hash.unwrap();

    return Some((url, head_commit_hash, base_commit_hash))
}
