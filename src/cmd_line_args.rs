#[derive(Debug)]
pub struct CommandLineArguments<'a> {
    port: &'a String,
    uuid: &'a String,
    event: &'a String,
    info: &'a String,
}

pub fn get_arguments<'a>(args: &'a Vec<String>) -> Option<CommandLineArguments<'a>> {
    let port = get_value_for("-port", args);
    let uuid = get_value_for("-pluginUUID", args);
    let event = get_value_for("-registerEvent", args);
    let info = get_value_for("-info", args);

    match (port, uuid, event, info) {
        (Some(p), Some(u), Some(e), Some(i)) => {
            return Some(CommandLineArguments {
                port: p,
                uuid: u,
                event: e,
                info: i,
            })
        }
        _ => {
            println!(
                "Could not find all the expected values, found '{:?}' instead.",
                (port, uuid, event, info)
            );
            return None;
        }
    }
}

fn get_value_for<'a, 'b>(term: &'b str, args: &'a Vec<String>) -> Option<&'a String> {
    let size = args.len();

    if size % 2 == 1 {
        for x in (1..size).step_by(2) {
            if is_equal(term, &args, x) {
                return args.get(x + 1);
            }
        }
    }
    return None;
}

fn is_equal(term: &str, args: &Vec<String>, index: usize) -> bool {
    if let Some(value) = args.get(index) {
        return value.eq(term);
    } else {
        return false;
    }
}
