use crate::Loc;

fn spfa(map: &super::Map, s: usize) -> Vec<f64> {

    let mut q = crate::queue::Queue::new();
    let mut dist = vec![1e9; map.n_nodes() + 1];
    let mut tag = vec![false; map.n_nodes() + 1];

    q.push(s);
    dist[s] = 0.0;
    tag[s] = true;

    while !q.is_empty() {
        let x = q.pop().unwrap();
        
        for (y, d) in map.to(x) {
            let y = *y;
            if dist[x] + d < dist[y] {
                dist[y] = dist[x] + d;
                if !tag[y] {
                    q.push(y);
                    tag[y] = true;
                }
               
            }
        }
        tag[x] = false;
    }

    dist
}

pub fn get_path(map: &super::Map, loc_s: Loc, loc_t: Loc) -> Vec<Loc> {

    let s = map.nearest(loc_s);
    let t = map.nearest(loc_t);

    let dist = spfa(map, t);

    let mut v = vec![loc_s, map.loc(s)];
    let mut x = s;
    while x != t {

        for (y, d) in map.to(x) {
            if (dist[*y] + d - dist[x]).abs() < 1. {
                x = *y;
                break;
            }
        };
        
        v.push(map.loc(x));
    }
    v.push(loc_t);
    v
}
