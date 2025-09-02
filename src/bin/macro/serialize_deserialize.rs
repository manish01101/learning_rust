trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}
trait Deserialize: Sized {
    fn deserialize(v: &Vec<u8>) -> Option<Self>;
}

#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::new();

        let temp1 = self.qty_1.to_be_bytes();
        for ele in temp1 {
            v.push(ele);
        }
        let temp2 = self.qty_2.to_be_bytes();
        for ele in temp2 {
            v.push(ele);
        }
        // or
        //  // push bytes of qty_1
        // v.extend_from_slice(&self.qty_1.to_be_bytes());

        // // push bytes of qty_2
        // v.extend_from_slice(&self.qty_2.to_be_bytes());

        v
    }
}
impl Deserialize for Swap {
    fn deserialize(v: &Vec<u8>) -> Option<Self> {
        if v.len() < 8 {
            return None;
        }
        let qty_1 = u32::from_be_bytes(v[0..4].try_into().unwrap());

        // next 4 bytes → u32
        let qty_2 = u32::from_be_bytes(v[4..8].try_into().unwrap());

        Some(Swap { qty_1, qty_2 })
    }
}

fn main() {
    let s = Swap {
        qty_1: 200000,
        qty_2: 3,
    };
	// SERIALIZE
    let v = s.serialize();
    for e in &v {
        print!("{} ", e)
    } println!();
    // or
    // Print nicely in hex
    println!("{:?}", v);

    // Or as hex string
    for byte in &v {
        print!("{:02X} ", byte);
    } println!();

	// DESERIALIZE
	let ans: Option<Swap> = Swap::deserialize(&v);
	if ans.is_some() {
		println!("{:#?}", ans);
	}
	if let Some(ans) = Swap::deserialize(&v) {
		println!("{:#?}", ans);
	} else {
        println!("Failed to deserialize!");
    }
	
}
