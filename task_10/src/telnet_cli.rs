use std::{error::Error, io::{self, stdin, stdout, Read, Write}, net::{AddrParseError, SocketAddr, TcpStream}, num::ParseIntError, time::Duration};

use clap::Parser;

#[derive(Parser)]
struct Cli{
    #[arg(long,short='t',default_value_t=String::from("10s"))]
    timeout:String,
    host:String,
    port:u16,
}

impl Cli{
    fn get_duration(&self)->Result<Duration, ParseIntError>{

        let last_index=&self.timeout[self.timeout.len()..];
        let sec=&self.timeout[..self.timeout.len()-1].parse::<u64>()?;
        match last_index {
            "s"=>{
                return Ok(Duration::from_secs(*sec))
            }
            "m"=>{
                return Ok(Duration::from_secs(60*sec));
            }
            "ml"=>{
                return Ok(Duration::from_millis(*sec));
            }
            _=>{
                return Ok(Duration::from_secs(*sec))

            }
            
        }
    }
    fn get_addr(&self)->Result<SocketAddr,AddrParseError>{
        format!("{}:{}",self.host,self.port).parse::<SocketAddr>()
    }
}

pub struct TelnetCli{
    stream:TcpStream
}

impl TelnetCli{
    pub fn new()->Result<Self,Box<dyn Error>>{
        let cli=Cli::parse();
        let timeout=cli.get_duration()?;
        let addr=cli.get_addr()?;
        let stream=TcpStream::connect_timeout(&addr, timeout)?;
        Ok(Self { stream})
    }
    fn write(&mut self,data:&[u8])->io::Result<()>{
        self.stream.write_all(data)
    }
    fn read(&mut self,mut buff:&mut [u8])->io::Result<usize>{
        self.stream.read(&mut buff)
    }
    pub fn run(mut self){

        loop {
            print!("telnet>");
            stdout().flush().unwrap();
            let mut buff=[0;512];
            match stdin().read(&mut buff){
                Ok(0)=>{
                    println!("CTRLD");
                    break;
                }
                Ok(_)=>{
                        if let Err(err) =&mut self.write(&buff)  {
                            eprintln!("Error while write into socket:{}",err);
                            continue;
                        }
                        let mut resp_buff=[0;128];
                        &mut self.read(&mut resp_buff).unwrap();
                        println!("Response from server:{:?}",String::from_utf8_lossy(resp_buff.as_slice()))
                }
                Err(err)=>{
                    eprintln!("Error while read from stdin:{}",err)
                }
            }
        }
    }
}




