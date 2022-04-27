// pong
// 
// Mandatory:
//
//  • The executable must be named "pong".
//
//  • You have to manage the -v -h options.
//  The -v option here will also allow us to see the results in case of a
//  problem or an error linked to the packets, which logically shouldn’t
//  force the program to stop (the modification of the TTL value can help
//  to force an error).
//
//  • You will have to manage a simple IPv4 (address/hostname) as parameters.
//
//  • You will have to manage FQDN without doing the DNS resolution in the packet
//  return
//
// Allowed fn:
//      inet_ntop        
//      inet_pton        
//      getpid           
//      getuid           
//      getaddrinfo	   
//      freeaddrinfo
//      signal
//      alarm
//      setsockopt
//      recvmsg
//      sendto
//      socket
//      printf(family)
//      stdlib
//      exit 
//
// ATTENTION: Usage of fcntl, poll et ppoll is strictly forbidden.
//
// =====================================================================
//
// Bonus:
//
// • IPv6 management
//
// • Additional -f -m -l -I -m -M -n -w -W -p -Q -S -t -T flags...
//
// =====================================================================
//
// Sources I used during the task:
// • https://www.techtarget.com/searchnetworking/definition/ICMP
//
// =====================================================================
//
//    NOTE
//    8 byte header
//
//        0x0               0x7 0x8              0xf 0x10                      0x1f
//        -------------------------------------------------------------------------
//        |        type        |       code         |         Checksum            |
//        -------------------------------------------------------------------------
//        |                            rest of header                             |
//        -------------------------------------------------------------------------
//

#![allow(unused)]

use clap::Parser;
use libc;

static EXIT_FAILURE: i32 = 1;
static EXIT_SUCCESS: i32 = 1;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The destination to send ICMP packets.
    /// TODO: make it possible to use with domain.
    destination: Option<String>,
    
    #[clap(long, short)]
    verbose: Option<std::net::IpAddr>,
}

fn main() {
    let args = Cli::parse();
    dbg!(&args);

    if args.destination.is_none() {
        eprintln!("ping: usage error: Destination address required");
        std::process::exit(EXIT_FAILURE);
    }
    //let s = args.destination.unwrap_or(String::from(""));
    
}
