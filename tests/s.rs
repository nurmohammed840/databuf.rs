use databuf::{*, config::num::LE};

#[test]
fn test_name() {

    #[derive(Encode, Decode, Debug)]
    struct Car<'a> {
        year: u16,
        is_new: bool,
        name: &'a str,
    }
    
    #[derive(Encode, Decode, Debug)]
    struct Company<'a> { name: String, cars: Vec<Car<'a>> }
    
    let old = Company {
        name: "Tesla".into(),
        cars: vec![
            Car { name: "Model S", year: 2018, is_new: true },
            Car { name: "Model X", year: 2019, is_new: false },
        ],
    };
    let bytes = old.to_bytes::<LE>();
    let new = Company::from_bytes::<LE>(&bytes);
    println!("{:?}", new);
    
}
