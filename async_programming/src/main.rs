use std::{net, thread};

fn main() {
    let listener = net::TcpListener::bind(address)?;

    for socket_result in listener.incoming() {
        let socker = socker_result?;
        let groups = chat_group_table.clone();
        thread::spawn(|| { 
            log_error(serve(socker, groups));
        });
    }
}
