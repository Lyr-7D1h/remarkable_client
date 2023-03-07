use std::{
    error::Error,
    net::{IpAddr, SocketAddr, TcpStream},
};

use ssh2::Session;

pub mod filesystem;

pub struct Remarkable {
    session: Session,
}

impl Remarkable {
    pub fn connect(
        ip: IpAddr,
        username: &String,
        password: &String,
    ) -> Result<Remarkable, Box<dyn Error>> {
        let tcp = TcpStream::connect(SocketAddr::new(ip, 22))?;
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();

        session.userauth_password(username, password)?;

        if session.authenticated() {
            return Err("Failed to authenticated".into());
        }

        Ok(Remarkable { session })
    }
}
