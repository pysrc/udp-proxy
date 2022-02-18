use std::{
    net::{SocketAddr, UdpSocket},
    thread::JoinHandle,
};
const MTU: usize = 1500;

fn proxy(local: &str, remote: &str) {
    println!("{} {}", local, remote);
    let localcon = UdpSocket::bind(local).unwrap();
    let mut recv_buf = [0u8; MTU];
    let mut org: Option<SocketAddr> = None;
    loop {
        if let Ok((siz, addr)) = localcon.recv_from(&mut recv_buf) {
            if addr.to_string() == remote && org != None {
                localcon.send_to(&recv_buf[..siz], org.unwrap()).unwrap();
            } else {
                if org == None {
                    org = Some(addr);
                } else if org.unwrap() != addr {
                    org = Some(addr);
                }
                localcon.send_to(&recv_buf[..siz], remote).unwrap();
            }
        }
    }
}

fn main() {
    let cfg = std::fs::read_to_string("config.txt").unwrap();
    let cfg_line: Vec<&str> = cfg.split('\n').collect();
    let mut ths = Vec::<JoinHandle<()>>::new();
    for ele in cfg_line {
        if ele.starts_with("#") {
            continue;
        }
        let addrs: Vec<&str> = ele.split("->").collect();
        if addrs.len() != 2 {
            continue;
        }
        let src = String::from(addrs[0]);
        let dst = String::from(addrs[1]);
        let th = std::thread::spawn(move || {
            proxy(src.trim(), dst.trim());
        });
        ths.push(th);
    }
    for ele in ths {
        ele.join().unwrap();
    }
}
