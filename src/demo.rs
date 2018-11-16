pub mod unsafes {
    use std::net::IpAddr;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[derive(Debug)]
    enum ServerAddr {
        SocketAddr(SocketAddr),
        DomainName(String),
    }

    impl ServerAddr {
        fn listen_addr(self) -> SocketAddr {
            match self {
                ServerAddr::SocketAddr(s) => s,
                _ => panic!("Cannot use domain name as server listen address"),
            }
        }
    }

    pub fn madd(a: u64) -> u64 {
        let mut counter = Arc::new(Mutex::new(a));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter = counter.clone();
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });

            handlers.push(handle);
        }

        for h in handlers {
            h.join().unwrap();
        }

        println!("{}", *counter.lock().unwrap());
        let result = *counter.lock().unwrap();
        result
    }

    struct Context<'s>(&'s str);

    struct Parser<'c, 's: 'c> {
        context: &'c Context<'s>,
    }

    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            Err(&self.context.0[1..])
        }
    }

    fn parse_content(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }

    pub fn param() {
        // let mut a = ServerAddr::SocketAddr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),8080));
    }

    fn fa(s: &mut String) -> &str {
        s.push_str("aaa");
        s
    }

}
//EOF
