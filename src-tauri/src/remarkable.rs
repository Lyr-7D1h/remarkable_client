use std::{
    collections::HashSet,
    error::Error,
    fs::read_dir,
    io::Read,
    net::{IpAddr, SocketAddr, TcpStream},
    path::Path,
};

use ssh2::Session;

use crate::RemarkableError;

/// A simple client for interacting with the remarkable
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

    /// Synchronize all files from the remarkable to a local path
    pub fn sync(&self, local_path: &Path) -> Result<(), RemarkableError> {
        todo!()
    }

    pub fn exec(&self, command: &str) -> Result<(i32, String), RemarkableError> {
        let mut channel = self.session.channel_session().unwrap();
        channel.exec(command).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        channel.wait_close();
        let exit_code = channel.exit_status()?;
        return Ok((exit_code, s));
    }
}
