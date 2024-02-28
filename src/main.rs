use dialoguer;

#[derive(Debug)]
struct ProjectInfo {
    project_type: usize,
    typescript: bool,
}

fn main() {
    let project_info = get_project_info();

    match project_info.project_type {
        0 => express_server(project_info.typescript),
        1 => discord_bot(project_info.typescript),
        2 => any_node_project(project_info.typescript),
        _ => panic!("Error, non selectable option chosen."),
    }
}

fn get_project_info() -> ProjectInfo {
    let project_types = vec![
        "ExpressJS Backend Server",
        "Discord Bot",
        "Any NodeJS Project",
    ];

    let selection_idx = dialoguer::FuzzySelect::new()
        .with_prompt("Project Type")
        .items(&project_types)
        .clear(false)
        .default(0)
        .interact()
        .unwrap();

    let use_typescript = dialoguer::Confirm::new()
        .with_prompt("Do you want to use Typescript?")
        .default(true)
        .interact()
        .unwrap();

    ProjectInfo {
        project_type: selection_idx,
        typescript: use_typescript,
    }
}

fn express_server(_typescript: bool) {}
fn discord_bot(_typescript: bool) {}
fn any_node_project(_typescript: bool) {}
