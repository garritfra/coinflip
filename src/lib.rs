extern crate rand;
use rand::distributions::Range;

pub fn flip() -> bool {
    let step = Range::new(0, 1);
    let mut rng = rand::thread_rng();
    let choice = step.ind_sample(&mut rng);
    match choice {
        1 => true,
        0 => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let coin = flip();
        assert!(coin == true || coin == false);
    }
}
