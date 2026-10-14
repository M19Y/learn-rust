fn connect_to_database(host: Option<String>) {
    match host {
        None => {
            panic!("No database host provided");
        }
        Some(x) => {
            println!("Success connect to {}", x);
        }
    }
}

#[test]
fn erorr_handling_using_micro_panic() {
    connect_to_database(Some(String::from("MySQL")));
}

// unrecovable error
#[test]
fn erorr_handling_using_micro_panic_throw_panic() {
    connect_to_database(None);
}

// recovable error

fn connect_to_cache(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err("No cache host provided".to_string()),
    }
}

#[test]
fn error_handling_using_recovable_error() {
    let cache = connect_to_cache(None);
    println!("Start programm");
    match cache {
        Ok(x) => {
            println!("Connect to cache at {}", x);
        }
        Err(err) => {
            println!("Error to connect cache -> {}", err);
        }
    }
    println!("End programm");
}

fn connect_to_email(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err("No email host provided".to_string()),
    }
}

fn connect_to_application(host: Option<String>) -> Result<String, String> {
    connect_to_cache(host.clone())?;
    connect_to_email(host.clone())?;
    Ok("Connect to application".to_string())
}

#[test]
fn error_handling_using_recovable_error_with_question_mark_checking() {
    println!("================ Start ================");
    let connect = connect_to_application(Some("Postgresql".to_string()));

    match connect {
        Ok(z) => {
            println!("Success connect => {}", z)
        }
        Err(y) => {
            println!("Failed connect => {}", y)
        }
    }

    println!("================ End ================");
}
