use rand::distributions::Slice;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::time::SystemTime;

lazy_static! {
    static ref TAGLINES: Vec<&'static str> =
        include_str!("taglines").lines().collect::<Vec<&str>>();
}

pub(crate) fn cheap_rng() -> SmallRng {
    let time_since_unix_epoch = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("SystemTime before unix epoch ");
    SmallRng::seed_from_u64(time_since_unix_epoch.as_secs())
}

pub(crate) fn random_tagline<R: Rng>(mut rng: R) -> &'static str {
    let taglines_dist = Slice::new(&TAGLINES).expect("failed to load taglines");
    rng.sample(taglines_dist)
}
