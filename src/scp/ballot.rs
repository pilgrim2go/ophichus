use scp::slot::SlotValue;
use scp::local_node::QSet;

enum Phase {
    Prepare,
    Confirm,
    Externalize
}
/*
enum Message<T: SlotValue> {
    Prepare: {d: QSet, b: Ballot<T>, p0: Ballot<T>, p1: Ballot<T>,
              nc: u64, nh: u64},
    Confirm: {d: QSet, },
}*/

struct State<T: SlotValue> {
    phase: Phase,
    b: Ballot<T>,
    p0: Ballot<T>,
    p1: Ballot<T>,
    c: Ballot<T>,
    h: Ballot<T>,
    z: Option<T>,
}

#[derive(PartialEq, PartialOrd)]
struct Ballot<T: SlotValue> {
    n: u64,
    x: Option<T>,
}


impl<T: SlotValue> State<T> {
    pub fn new() -> State<T> {
        State {phase: Phase::Prepare,
               b: Ballot {n: 0, x: None},
               p0: Ballot {n: 0, x: None},
               p1: Ballot {n: 0, x: None},
               c: Ballot {n: 0, x: None},
               h: Ballot {n: 0, x: None},
               z: None}
    }
}

