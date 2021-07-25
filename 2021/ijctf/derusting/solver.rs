mod rev;
use rand::{SeedableRng, prelude::{SliceRandom, StdRng}};

fn main() {
    for xor_key in 0..=0xff {
        let mut rng: StdRng = 
            SeedableRng::seed_from_u64(xor_key as u64);

        let mut output = [
            66u8, 67, 36, 53, 123, 127, 12, 104, 125, 36, 36, 86, 73, 94, 41, 61, 41, 4, 87, 55, 58, 109, 51, 66, 60, 11, 77, 17, 56, 109, 76, 41, 84, 125, 100, 90, 101, 47, 36, 40,
        ];
        let mut order = vec![];
        let mut mapp = vec![];
        for i in 0..output.len() {
            order.push(i);
            mapp.push(i);
        }
        let mut orders = vec![];
        let mut mapps = vec![];
        (0..output.len()).for_each(|_| {
            order.shuffle(&mut rng);
            orders.push(order.clone());
            mapp.shuffle(&mut rng);
            mapps.push(mapp.clone());
        });
        let mut temp = [0; 40];
        let mut inp = [0; 40];
        for i in (0..output.len()).rev() {
            let ord = &orders[i];
            let map = &mapps[i];
            for j in 0..ord.len() {
                temp[map[j]] = output[j];
            }
            inp[inp.len()-1] = temp[temp.len()-1];
            let mut index = ord.iter()
                .position(|&x| x == inp.len()-1).unwrap();
            let mut saved = index;
            while index > 0 {
                index -= 1;
                inp[ord[index]] = temp[index] ^ inp[ord[index+1]];
            }
            saved += 1;
            while saved < ord.len() {
                inp[ord[saved]] = temp[saved-1] ^ inp[ord[saved-1]];
                saved += 1;
            }
            let r = enc_map(&inp[..], ord, map);
            assert_eq!(output, r.as_slice());
            output.clone_from_slice(&inp[..inp.len()]);
        }
        let cs = String::from_utf8_lossy(&output[..]);
        if cs.contains("IJCTF{") {
            println!("{}", cs);
        }
    }
}

// passing u8 slices and returning vecs are far efficient than strings
fn enc_map(inp: &[u8], order: &[usize], mapp: &[usize]) -> Vec<u8> {
    let mut temp = vec![];
    for i in 1..inp.len() {
        temp.push(inp[order[i-1]] ^ inp[order[i]]);
    }
    temp.push(inp[inp.len()-1]);
    let mut ans = vec![];
    for i in 0..temp.len() {
        ans.push(temp[mapp[i]]);
    }
    ans
}
