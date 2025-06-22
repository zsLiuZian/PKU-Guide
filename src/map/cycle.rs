use rand::Rng;
use crate::Loc;

const N_REGIONS: usize = 10;

pub fn get_cycle(map: &super::Map, loc: Loc) -> Vec<Loc> {

    let s = map.nearest(loc);
    let s_region = region(map.loc(s));

    let mut v = vec![loc, map.loc(s)];

    let mut last = s;
    for step in 1..=N_REGIONS {
        
        let next: usize = if step == N_REGIONS {
            s
        }
        else {
            loop {
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(1..=map.n_nodes());
                if region(map.loc(x)) == (s_region + step) % N_REGIONS {
                    break x;
                }
            }
        };

        v.extend(&(super::path::get_path(map, map.loc(last), map.loc(next)))[1..]);
        last = next;

    }
    
    filter(v)

}

fn filter(v: Vec<Loc>) -> Vec<Loc> {
    let mut filter_v = vec![];
    let mut len = 0;
    for x in &v {
        if len > 0 && *x == filter_v[len - 1] {
            continue;
        }
        if len > 1 && *x == filter_v[len - 2] {
            filter_v.pop();
            len -= 1;
            continue;
        }
        filter_v.push(*x);
        len += 1;
    }
    filter_v
}


fn region((x, y): Loc) -> usize {
    let w = crate::window::WIDTH / (N_REGIONS / 2);
    let h = crate::window::HEIGHT / 2;
    match (x / w, y / h) {
        (n, 0) => n,
        (n, 1) => N_REGIONS - n - 1,
        _ => unreachable!(),
    }
}