/*
BSD 3-Clause License

Copyright (c) 2019, Giacomo Picchiarelli
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

extern crate unveil;
extern crate pledge;

mod sigs;
mod daemon;

use unveil::unveil;
use pledge::{pledge, Promise, ToPromiseString};
use daemon::get_daemon;
use std::env;

fn main(){
    println!("Dungeon Honeypot");
    test_unveil();
//    test_pledge();
    get_daemon();
}

fn test_unveil(){
    let path = env::current_dir().unwrap().as_os_str().to_os_string();
    unveil("/tmp", "rw").unwrap();
    unveil(&path, "rw").unwrap();
}

fn test_pledge(){
        match pledge(&vec![Promise::Audio,
    Promise::Chown,
    Promise::CPath,
    Promise::DiskLabel,
    Promise::Dns,
    Promise::DPath,
    Promise::Drm,
    Promise::Exec,
    Promise::Fattr,
    Promise::Flock,
    Promise::Getpw,
    Promise::Id,
    Promise::Inet,
    Promise::Ioctl,
    Promise::MCast,
    Promise::Pf,
    Promise::Proc,
    Promise::ProtExec,
    Promise::Ps,
    Promise::Recvfd,
    Promise::Route,
    Promise::RPath,
    Promise::Sendfd,
    Promise::Settime,
    Promise::Stdio,
    Promise::TMPPath,
    Promise::Tty,
    Promise::Unix,
    Promise::Vminfo,
    Promise::Vmm,
Promise::WPath].to_promise_string()) {
        Err(_) => println!("Failed to pledge"),
        _ => ()
    }
}

