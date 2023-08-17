#[allow(dead_code)]


mod pair;
mod vec3;
mod counter;

use pair::pair::default_pair;
use vec3::vec3::default_vec3;
use counter::counter::next_signed;

#[allow(dead_code)]
fn call_all() -> () {   
    default_pair();
    default_vec3();
    next_signed(0);
    ()
}