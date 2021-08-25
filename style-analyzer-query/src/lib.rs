use std::path::Path;

use djanco::*;
use djanco::attrib::sort::Direction;
use djanco::database::*;
use djanco::log::*;
use djanco::csv::*;

use djanco::objects::CommitId;
use djanco::objects::Head;
use djanco::objects::ItemWithData;
use djanco::objects::Language;
use djanco::objects::Project;
use djanco::objects::ProjectId;

use djanco::time::Duration;
use djanco_ext::*;

const SELECTIONS: usize = 10;
const SELECTED_PROJECTS: usize = 25;
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

#[djanco(May, 2021, subsets(JavaScript))]
pub fn all_projects(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .into_csv_in_dir(output, "javascript_projects.csv")
}

#[djanco(May, 2021, subsets(JavaScript))]
pub fn project_locs(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
    .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
    .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
    .map_into(Select!(project::URL, project::Locs))
    .into_csv_with_headers_in_dir(vec!["url", "locs"], output, "project_locs.csv")
}

#[macro_export]
macro_rules! one_per_selection {
    {$function:ident ($database:ident, $log:ident, $output:ident)} => {{
        for i in 0..SELECTIONS {
            random_projects($database, $log, $output, i)?;
        }
        Ok(())
    }}
}

#[djanco(May, 2021, subsets(JavaScript))] 
pub fn select_random_projects(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    one_per_selection!{ random_projects(database, log, output) }
}

pub fn random_projects(database: &Database, _log: &Log, output: &Path, seed_index: usize) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .filter(is_project_spec)
        .sample(Random(SELECTED_PROJECTS, Seed(SEEDS[seed_index])))
        .flat_map(project_spec)
        .into_csv_with_headers_in_dir(vec!["url", "to", "from"], output, format!("random_projects_{}_{}.csv", seed_index, BASE_COMMIT_OFFSET_RATIO))
}

#[djanco(May, 2021, subsets(JavaScript))] pub fn select_random_projects_by_size(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    one_per_selection!{ random_projects_by_size(database, log, output) }
}

pub fn random_projects_by_size(database: &Database, _log: &Log, output: &Path, seed_index: usize) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .filter(is_project_spec)
        .sample(Stratified(project::Locs, 
                                  Strata!("very large" -> Random(SELECTED_PROJECTS/4, Seed(SEEDS[seed_index])), 
                                          "large" -> Random(SELECTED_PROJECTS/4, Seed(SEEDS[seed_index])), 
                                          "medium" -> Random(SELECTED_PROJECTS/4, Seed(SEEDS[seed_index])),
                                          "small" -> Random(SELECTED_PROJECTS/4, Seed(SEEDS[seed_index]))),
                                  Thresholds::Inclusive(Conditions!("very large" -> 100_000,
                                                                    "large" -> 10_000,
                                                                    "medium" -> 1000),
                                                                    "small")))
        .flat_map(project_spec)
        .into_csv_with_headers_in_dir(vec!["url", "to", "from"], output, format!("random_projects_by_size_{}_{}.csv", seed_index, BASE_COMMIT_OFFSET_RATIO))
}

//#[djanco(May, 2021, subsets(JavaScript))] 
pub fn all_specs(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error> {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .flat_map(project_spec)
        .into_csv_with_headers_in_dir(vec!["url", "to", "from"], output, format!("all_project_specs_{}.csv", BASE_COMMIT_OFFSET_RATIO))
}

#[djanco(May, 2021, subsets(JavaScript))]
pub fn top_starred(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .sort_by(project::Stars)
        .filter(is_project_spec)
        .sample(Top(SELECTED_PROJECTS))
        .flat_map(project_spec)
        .into_csv_with_headers_in_dir(vec!["url", "to", "from"], output, "top_starred_projects.csv")
}
 
#[djanco(May, 2021, subsets(JavaScript))]
pub fn select_quality_projects(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    one_per_selection!{ quality_projects(database, log, output) }
}

pub fn quality_projects(database: &Database, _log: &Log, output: &Path, seed_index: usize) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
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
        .filter(is_project_spec)
        // Sample N projects at random (we're just going to do one selection, so take first seed)
        // Instead fo doing 10 selections of N, I'll do one selection of 10 * N
        .sample(Random(10 * SELECTED_PROJECTS, Seed(SEEDS[seed_index]))) 
        // Extract: url, head commit aka to, base commit aka from
        .flat_map(project_spec)
        .into_csv_with_headers_in_dir(vec!["url", "to", "from"], 
            output, 
            format!("quality_projects_{}_{}.csv", seed_index, BASE_COMMIT_OFFSET_RATIO))
}

pub fn debug(database: &Database, _log: &Log, output: &Path, project_id: usize) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Id, ProjectId::from(project_id)))       
        .map_into(project::Commits)
        .flat_map(|e| e )
        .flat_map(|e| e)
        .map_into(Select!(commit::Id, commit::Hash, commit::Message))
        .into_csv_in_dir(output, format!("{}_commits.csv", project_id))
}

//#[djanco(May, 2021, subsets(JavaScript))] 
pub fn debug_commits(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    database.commits()
        .sort_with_direction(Direction::Ascending, commit::Id)
        .map_into(Select!(commit::Id, commit::Hash))
        .into_csv_in_dir(output, "commits.csv") 
}
//#[djanco(May, 2021, subsets(JavaScript))]
pub fn debug_commits_from_source(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut hashes: Vec<(djanco::objects::CommitId, String)> = database.source().commit_hashes().collect();
    hashes.sort_by_key(|(id, _hash)| *id);
    hashes.into_iter().into_csv_in_dir(output, "commits_from_source.csv")
}

// #[djanco(May, 2021, subsets(JavaScript))] 
pub fn debug_heads(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    let mut heads = database.projects()
        .map_into(Select!(project::Id, project::Heads))
        .map(|(project_id, heads)| (project_id, heads.unwrap_or_else(Vec::new)))
        .map(|(project_id, heads)| (project_id, heads.into_iter().map(|head| {
                (head.name(), (head.commit_id(), head.commit_with_data().unwrap().hash().unwrap()))
            }).collect::<Vec<(String, (CommitId, String))>>())
        )
        .map(|(project_id, mut heads)| {heads.sort(); (project_id, heads)})
        .collect::<Vec<(ProjectId, Vec<(String, (CommitId, String))>)>>();
    heads.sort();
    heads.into_iter().into_csv_in_dir(output, "heads.csv")
}

// #[djanco(May, 2021, subsets(JavaScript))] 
pub fn debug_heads_from_source(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    let mut heads = database.source()
        .project_heads()
        .map(|(project_id, heads)| 
            (project_id, heads.into_iter().collect::<Vec<(String, (djanco::objects::CommitId, String))>>())
        )
        .map(|(project_id, mut heads)| {heads.sort(); (project_id, heads)})
        .collect::<Vec<(djanco::objects::ProjectId, Vec<(String, (djanco::objects::CommitId, String))>)>>();
    heads.sort();
    heads.into_iter().into_csv_in_dir(output, "heads_from_source.csv")
}

// Helper functions:
type ProjectURL = String;
type CommitHash = String;

fn is_project_spec<'a>(project: &ItemWithData<'a, Project>) -> bool {
    _project_spec(project).is_some()
}
fn project_spec<'a>(project: ItemWithData<'a, Project>) -> Option<(ProjectURL, CommitHash, CommitHash)> {
    _project_spec(&project)
}
fn _project_spec<'a>(project: &ItemWithData<'a, Project>) -> Option<(ProjectURL, CommitHash, CommitHash)> {
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

    let head_commit = default_branch_head.commit_with_data();
    if head_commit.is_none() {
        eprintln!("WARNING: Head commit inaccessible from branch {} in project {} ({:?}), skipping.", 
                  default_branch, project.id(), url);
        return None;
    }
    let head_commit = head_commit.unwrap();
    
    let head_commit_hash = head_commit.hash();
    if head_commit_hash.is_none() {
        eprintln!("WARNING: Head commit hash unavaiable for head commit {} from branch {} in project {} ({:?}), skipping.", 
                  head_commit.id(), default_branch, project.id(), url);
        return None;
    }
    let head_commit_hash = head_commit_hash.unwrap();

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