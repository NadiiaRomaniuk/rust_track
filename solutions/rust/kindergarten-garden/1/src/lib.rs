//для себе на потім подивитись:https://users.rust-lang.org/t/string-str-when-to-use-vec-string-or-vec-str/98587

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let stud= [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let lines: Vec<&str> = diagram.lines().collect();
    let r1=lines[0];
    let r2=lines[1];

    let mut idx=0;
    for (i, name) in stud.iter().enumerate() {
        if *name== student {
            idx=i;
            break;
        }
    }

    let start=idx*2;

    let c1=r1.chars().nth(start).unwrap();
    let c2=r1.chars().nth(start+1).unwrap();
    let c3=r2.chars().nth(start).unwrap();
    
    let c4=r2.chars().nth(start+1).unwrap();

    let letters=vec![c1, c2, c3, c4];

    let mut plants_vec=Vec::new();
    for l in letters {
        let plant=match l {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => panic!("інша рослина"),
        };

        
        plants_vec.push(plant);
    }plants_vec
}

