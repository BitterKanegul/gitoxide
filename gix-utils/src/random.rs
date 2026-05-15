//! A per-thread random number generator seeded with `getrandom`.

use fastrand::Rng;
use std::cell::Cell;

std::thread_local! {
    //FIXME: FIgure out when it can fail and what to do.
    static RNG: Cell<Rng> = Cell::new(Rng::with_seed(getrandom::u64().expect("RNG couldn't be initialized")));
}
