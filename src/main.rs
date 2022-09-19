use actix::{Actor, Addr, Context, System, Message, Handler, Arbiter};
use std::io;

// fn main() {
//     println!("Hello, world!");
// }

struct CriticalActor(u32, Addr<ReadActor>);
// struct WriteActor(Addr<CriticalActor>);
struct ReadActor;

#[derive(Message)]
#[rtype(result = "()")]
struct IncRequest;

#[derive(Message)]
#[rtype(result = "()")]
struct PrinRequest(u32);


impl Actor for CriticalActor {
    type Context = Context<Self>;
}

// impl Actor for WriteActor {
//     type Context = Context<Self>;
// }

impl Actor for ReadActor {
    type Context = Context<Self>;
}

impl Handler<IncRequest> for CriticalActor {
    type Result = ();

    fn handle(&mut self, msg: IncRequest, _ctx: &mut Context<Self>) -> Self::Result {
        self.1.do_send(PrinRequest(self.0));
        self.0 += 1;
    }
}

impl Handler<PrinRequest> for ReadActor {
    type Result = ();

    fn handle(&mut self, msg: PrinRequest, _ctx: &mut Context<Self>) -> Self::Result {
        println!("{}", msg.0);
    }
}

#[actix_rt::main]
async fn main() {
    // let system = System::new();
    // let wr_arb = Arbiter::new();
    let crit_arb = Arbiter::new();
    let rd_arb = Arbiter::new();

    let read_addr = ReadActor::start_in_arbiter(&rd_arb.handle(), |_ctx: &mut Context<ReadActor>| ReadActor);
    let crit_addr = CriticalActor::start_in_arbiter(&crit_arb.handle(), |_ctx: &mut Context<CriticalActor>| CriticalActor(0, read_addr));

    // let read_addr = ReadActor.start();
    // let crit_addr = CriticalActor(0, read_addr).start();

    // let write_addr = WriteActor::start_in_arbiter(&wr_arb.handle(), |_ctx: &mut Context<WriteActor>| WriteActor(crit_addr));

    loop {
        crit_addr.send(IncRequest).await;
    }

    // actix::spawn(inc_loop);

    // system.run();
}