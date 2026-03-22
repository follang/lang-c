use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::driver::{self, Config};
use crate::extract;
use crate::ir::SourcePackage;

fn known_system_headers() -> Vec<&'static str> {
    vec!["stdint.h", "stdio.h", "linux/stddef.h", "linux/input.h"]
}

fn find_header<'a>(candidates: &'a [&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .copied()
        .find(|candidate| Path::new(candidate).exists())
}

fn unique_temp_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    env::temp_dir().join(format!("pac-system-headers-{}", stamp))
}

fn write_wrapper(dir: &Path, header: &str) -> PathBuf {
    let wrapper = dir.join("wrapper.c");
    let source = format!(
        "#include <{}>\nint pac_header_probe(void) {{ return 0; }}\n",
        header
    );
    fs::write(&wrapper, source).expect("writing temporary wrapper");
    wrapper
}

fn parse_header_wrapper(path: &Path) -> Result<(), String> {
    let mut config = Config::with_gcc();
    config.cpp_options.push("-D_GNU_SOURCE".to_owned());
    driver::parse(&config, path)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

#[test]
fn system_header_wrappers_parse_when_headers_exist() {
    let mut attempted = 0usize;
    let mut failures = Vec::new();

    for header in known_system_headers() {
        let full_path = Path::new("/usr/include").join(header);
        if !full_path.exists() {
            continue;
        }

        attempted += 1;
        let dir = unique_temp_dir();
        fs::create_dir_all(&dir).expect("creating temporary wrapper directory");
        let wrapper = write_wrapper(&dir, header);

        if let Err(err) = parse_header_wrapper(&wrapper) {
            failures.push(format!("{}: {}", header, err));
        }

        fs::remove_file(&wrapper).expect("removing temporary wrapper");
        fs::remove_dir(&dir).expect("removing temporary wrapper directory");
    }

    assert!(
        attempted > 0,
        "expected at least one system header to exist"
    );
    if !failures.is_empty() {
        panic!(
            "{} system header wrappers failed:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}

#[test]
fn resilient_parser_recovers_items_from_linux_headers() {
    let header_path = Path::new("/usr/include/linux/input.h");
    if !header_path.exists() {
        return; // Skip if header not available
    }

    let dir = unique_temp_dir();
    fs::create_dir_all(&dir).expect("creating temp dir");
    let wrapper = dir.join("wrapper.c");
    fs::write(&wrapper, "#include <linux/input.h>\n").expect("writing wrapper");

    let mut config = Config::with_gcc();
    config.cpp_options.push("-D_GNU_SOURCE".to_owned());

    let processed = match preprocess_for_test(&config, &wrapper) {
        Some(s) => s,
        None => {
            let _ = fs::remove_file(&wrapper);
            let _ = fs::remove_dir(&dir);
            return;
        }
    };

    let tu = driver::parse_preprocessed_resilient(&config, processed);
    assert!(
        tu.unit.0.len() > 0,
        "resilient parser should recover at least some declarations from linux/input.h"
    );

    let _ = fs::remove_file(&wrapper);
    let _ = fs::remove_dir(&dir);
}

fn preprocess_for_test(config: &Config, source: &Path) -> Option<String> {
    use std::process::Command;
    let mut cmd = Command::new(&config.cpp_command);
    for item in &config.cpp_options {
        cmd.arg(item);
    }
    cmd.arg(source);
    let output = cmd.output().ok()?;
    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}

fn parse_wrapper_package(path: &Path) -> Option<SourcePackage> {
    let mut config = Config::with_gcc();
    config.cpp_options.push("-D_GNU_SOURCE".to_owned());
    let parsed = driver::parse(&config, path).ok()?;
    Some(extract::extract_from_translation_unit(&parsed.unit, None))
}

#[test]
fn openssl_wrapper_extracts_public_surface_when_headers_exist() {
    let Some(header) = find_header(&[
        "/usr/include/openssl/ssl.h",
        "/usr/include/x86_64-linux-gnu/openssl/ssl.h",
    ]) else {
        return;
    };

    let dir = unique_temp_dir();
    fs::create_dir_all(&dir).expect("creating temporary wrapper directory");
    let wrapper = write_wrapper(&dir, Path::new(header).strip_prefix("/usr/include/").ok().and_then(|p| p.to_str()).unwrap_or("openssl/ssl.h"));

    let pkg = parse_wrapper_package(&wrapper).expect("openssl wrapper should parse and extract");

    assert!(pkg.find_function("SSL_new").is_some() || pkg.find_function("SSL_CTX_new").is_some());
    assert!(pkg.find_type_alias("SSL").is_some());
    assert!(pkg.find_type_alias("SSL_CTX").is_some());
    assert!(pkg.item_count() >= 20);

    fs::remove_file(&wrapper).expect("removing temporary wrapper");
    fs::remove_dir(&dir).expect("removing temporary wrapper directory");
}

#[test]
fn openssl_wrapper_extracts_deterministically_when_headers_exist() {
    let Some(header) = find_header(&[
        "/usr/include/openssl/ssl.h",
        "/usr/include/x86_64-linux-gnu/openssl/ssl.h",
    ]) else {
        return;
    };

    let make = || {
        let dir = unique_temp_dir();
        fs::create_dir_all(&dir).expect("creating temporary wrapper directory");
        let wrapper = write_wrapper(
            &dir,
            Path::new(header)
                .strip_prefix("/usr/include/")
                .ok()
                .and_then(|p| p.to_str())
                .unwrap_or("openssl/ssl.h"),
        );

        let pkg = parse_wrapper_package(&wrapper).expect("openssl wrapper should parse and extract");
        let json = serde_json::to_string(&pkg).expect("openssl package json");

        fs::remove_file(&wrapper).expect("removing temporary wrapper");
        fs::remove_dir(&dir).expect("removing temporary wrapper directory");
        json
    };

    assert_eq!(make(), make());
}

#[test]
fn linux_event_loop_wrapper_extracts_combined_surface_when_headers_exist() {
    let headers = [
        "/usr/include/sys/epoll.h",
        "/usr/include/sys/timerfd.h",
        "/usr/include/sys/signalfd.h",
        "/usr/include/x86_64-linux-gnu/sys/epoll.h",
        "/usr/include/x86_64-linux-gnu/sys/timerfd.h",
        "/usr/include/x86_64-linux-gnu/sys/signalfd.h",
    ];

    let epoll_candidates = [headers[0], headers[3]];
    let timerfd_candidates = [headers[1], headers[4]];
    let signalfd_candidates = [headers[2], headers[5]];
    let epoll = find_header(&epoll_candidates);
    let timerfd = find_header(&timerfd_candidates);
    let signalfd = find_header(&signalfd_candidates);
    if epoll.is_none() || timerfd.is_none() || signalfd.is_none() {
        return;
    }

    let dir = unique_temp_dir();
    fs::create_dir_all(&dir).expect("creating temporary wrapper directory");
    let wrapper = dir.join("wrapper.c");
    fs::write(
        &wrapper,
        "#include <sys/epoll.h>\n#include <sys/timerfd.h>\n#include <sys/signalfd.h>\n",
    )
    .expect("writing temporary wrapper");

    let pkg = parse_wrapper_package(&wrapper).expect("combined linux wrapper should parse");

    assert!(pkg.find_function("epoll_create1").is_some());
    assert!(pkg.find_function("timerfd_create").is_some());
    assert!(pkg.find_function("signalfd").is_some());
    assert!(pkg.find_record("epoll_event").is_some());
    assert!(pkg.find_record("signalfd_siginfo").is_some());
    assert!(pkg.item_count() >= 20);

    fs::remove_file(&wrapper).expect("removing temporary wrapper");
    fs::remove_dir(&dir).expect("removing temporary wrapper directory");
}

#[test]
fn linux_event_loop_wrapper_extracts_deterministically_when_headers_exist() {
    let epoll_candidates = ["/usr/include/sys/epoll.h", "/usr/include/x86_64-linux-gnu/sys/epoll.h"];
    let timerfd_candidates = ["/usr/include/sys/timerfd.h", "/usr/include/x86_64-linux-gnu/sys/timerfd.h"];
    let signalfd_candidates = ["/usr/include/sys/signalfd.h", "/usr/include/x86_64-linux-gnu/sys/signalfd.h"];
    if find_header(&epoll_candidates).is_none()
        || find_header(&timerfd_candidates).is_none()
        || find_header(&signalfd_candidates).is_none()
    {
        return;
    }

    let make = || {
        let dir = unique_temp_dir();
        fs::create_dir_all(&dir).expect("creating temporary wrapper directory");
        let wrapper = dir.join("wrapper.c");
        fs::write(
            &wrapper,
            "#include <sys/epoll.h>\n#include <sys/timerfd.h>\n#include <sys/signalfd.h>\n",
        )
        .expect("writing temporary wrapper");

        let pkg = parse_wrapper_package(&wrapper).expect("combined linux wrapper should parse");
        let json = serde_json::to_string(&pkg).expect("combined event-loop package json");

        fs::remove_file(&wrapper).expect("removing temporary wrapper");
        fs::remove_dir(&dir).expect("removing temporary wrapper directory");
        json
    };

    assert_eq!(make(), make());
}
