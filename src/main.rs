extern crate mojiretsu;
use std::path::Path;
fn main() {
    println!("Hello, world!");
    // 00
    println!("{}", mojiretsu::reverse("abc"));
    // 01. 「パタトクカシーー」
    println!("{}", mojiretsu::odd_chars("パタトクカシーー"));
    // 02. 「パトカー」＋「タクシー」＝「パタトクカシーー」
    println!("{}", mojiretsu::concat_alternately("パトカー", "タクシー"));
    // 03. 円周率
    println!("{:?}", mojiretsu::pi_from_text());
    // 04. 元素記号
    println!("{:?}", mojiretsu::element_symbols());
    // contains
    {
        let a = "hello world";
        if a.contains("world") {
            println!("{}", "contains");
        }
    }
    {
        // 09
        let s = mojiretsu::typoglycemia("I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind .");
        println!("{}", s);
    }
    {
        println!(
            "{}",
            mojiretsu::count_lines_number(&Path::new("src/main.rs")).unwrap()
        );
    }
    {
        println!(
            "{}",
            mojiretsu::tab2space(&Path::new("src/main.rs"), 2).unwrap()
        );
    }
    {
        println!("14.");
        mojiretsu::heads(Path::new("src/main.rs"), 10)
            .unwrap()
            .iter()
            .for_each(|line| {
                println!("{}", line);
            });
    }
    {
        println!("15.");
        mojiretsu::tails(Path::new("src/main.rs"), 10)
            .unwrap()
            .iter()
            .for_each(|line| {
                println!("{}", line);
            });
    }
    {
        
    }
}
