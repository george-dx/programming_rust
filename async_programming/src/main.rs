use async_std::io::prelude::*;
use async_std::{net, task};

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;
    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    Ok(response)
}

async fn cheapo_owning_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    cheapo_request(&host, port, &path).await
}

pub async fn many_requests(urls: &[String]) -> Vec<Result<String, surf::Exception>> {
    let client = surf::Client::new();
    let mut handles = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

fn main() -> std::io::Result<()> {
    let requests = &[
        "http://example.com".to_string(),
        "https://www.red-bean.com".to_string(),
        "https://en.wikipedia.org/wiki/Main_Page".to_string(),
    ];
    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("*** {}\n", response),
            Err(err) => eprintln!("error: {}\n", err),
        }
    }
    Ok(())
}
