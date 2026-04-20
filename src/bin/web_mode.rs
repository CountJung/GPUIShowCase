use std::{
    env, fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::{Component, Path, PathBuf},
};

const WEB_ROOT: &str = "web";
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 4173;

#[derive(Clone, Debug, PartialEq, Eq)]
struct ServerConfig {
    host: String,
    port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
        }
    }
}

fn main() -> io::Result<()> {
    let config = parse_args(env::args().skip(1))?;
    let listener = TcpListener::bind((config.host.as_str(), config.port))?;

    println!(
        "Listening on http://{}:{}",
        config.host.as_str(),
        config.port
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = handle_connection(stream) {
                    eprintln!("web_mode request failed: {error}");
                }
            }
            Err(error) => eprintln!("web_mode connection failed: {error}"),
        }
    }

    Ok(())
}

fn parse_args(args: impl IntoIterator<Item = String>) -> io::Result<ServerConfig> {
    let mut config = ServerConfig::default();
    let mut iter = args.into_iter();

    while let Some(argument) = iter.next() {
        match argument.as_str() {
            "--host" => {
                config.host = iter.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "missing value for --host")
                })?;
            }
            "--port" => {
                let raw = iter.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "missing value for --port")
                })?;
                config.port = raw.parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "invalid value for --port")
                })?;
            }
            _ => {}
        }
    }

    Ok(config)
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let path = request_path(&request_line).unwrap_or("/").to_string();

    while !request_line.trim().is_empty() {
        request_line.clear();
        reader.read_line(&mut request_line)?;
        if request_line == "\r\n" {
            break;
        }
    }

    if path == "/health" {
        return write_response(&mut stream, "200 OK", "text/plain; charset=utf-8", b"ok");
    }

    let asset_path = resolve_asset_path(&path)
        .and_then(|path| {
            let full_path = Path::new(WEB_ROOT).join(path);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
        .unwrap_or_else(|| Path::new(WEB_ROOT).join("index.html"));

    let body = fs::read(&asset_path)?;
    let mime_type = mime_type_for_path(&asset_path);
    write_response(&mut stream, "200 OK", mime_type, &body)
}

fn request_path(request_line: &str) -> Option<&str> {
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;

    if method.eq_ignore_ascii_case("GET") {
        Some(path)
    } else {
        None
    }
}

fn resolve_asset_path(request_path: &str) -> Option<PathBuf> {
    let trimmed = request_path.trim_start_matches('/');
    let candidate = if trimmed.is_empty() {
        PathBuf::from("index.html")
    } else {
        PathBuf::from(trimmed)
    };

    if candidate.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return None;
    }

    Some(candidate)
}

fn mime_type_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: &str,
    mime_type: &str,
    body: &[u8],
) -> io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {mime_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_PORT, mime_type_for_path, parse_args, request_path, resolve_asset_path};
    use std::path::Path;

    #[test]
    fn parses_host_and_port_arguments() {
        let config = parse_args([
            "--host".to_string(),
            "0.0.0.0".to_string(),
            "--port".to_string(),
            "9000".to_string(),
        ])
        .expect("config should parse");

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 9000);
    }

    #[test]
    fn defaults_when_no_arguments_are_given() {
        let config = parse_args(Vec::<String>::new()).expect("default config should parse");

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, DEFAULT_PORT);
    }

    #[test]
    fn request_path_extracts_get_path() {
        assert_eq!(
            request_path("GET /data/showcase.json HTTP/1.1\r\n"),
            Some("/data/showcase.json")
        );
        assert_eq!(request_path("POST /submit HTTP/1.1\r\n"), None);
    }

    #[test]
    fn resolve_asset_path_blocks_parent_segments() {
        assert_eq!(
            resolve_asset_path("/data/showcase.json").unwrap(),
            Path::new("data/showcase.json")
        );
        assert!(resolve_asset_path("/../secret.txt").is_none());
    }

    #[test]
    fn mime_type_matches_known_extensions() {
        assert_eq!(
            mime_type_for_path(Path::new("index.html")),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            mime_type_for_path(Path::new("styles.css")),
            "text/css; charset=utf-8"
        );
        assert_eq!(
            mime_type_for_path(Path::new("app.js")),
            "application/javascript; charset=utf-8"
        );
    }
}
