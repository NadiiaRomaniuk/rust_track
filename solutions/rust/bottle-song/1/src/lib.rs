pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn num_word(n: u32) -> &'static str {
        match n {
            0 => "no",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            _ => "??",
        }
    }

    fn cap(x: &str) -> String {
        let mut it = x.chars();
        if let Some(f) = it.next() {
            let mut s = String::new();
            s.push_str(&f.to_uppercase().to_string());
            s.push_str(it.as_str());
            s
        } else {
            "".to_string()
        }
    }

    let mut out = Vec::new();
    let mut k = start_bottles;

    for _ in 0..take_down {
        if k == 0 {
            break;
        }

        let tmp = cap(num_word(k));
        let bb = if k == 1 { "bottle" } else { "bottles" };

        let nxt = if k > 0 { k - 1 } else { 0 };
        let nxt_word = if nxt == 0 { "no".to_string() } else { num_word(nxt).to_string() };
        let nxt_b = if nxt == 1 { "bottle" } else { "bottles" };

        let txt = format!(
            "{A} green {B} hanging on the wall,\n\
             {A} green {B} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {C} green {D} hanging on the wall.",
            A = tmp,
            B = bb,
            C = nxt_word,
            D = nxt_b
        );

        out.push(txt);
        k -= 1; 
    }


    

    out.join("\n\n")
}
