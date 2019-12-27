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
        let vc = vec!("03:10","02:55","03:30","03:20");
        let allsec:i32 = vc.iter().map(|s| mojiretsu::str2sec(s).unwrap()).fold(0,|result, t| result + t);
        println!("all sec is {}",allsec);
        println!("formated is {}",mojiretsu::format_sec(allsec));
    }
    {
        // 16
        let vc = mojiretsu::split_file(Path::new("src/main.rs"), 3);
        if let Ok(vc) = vc {
            for i in vc {
                println!("{}\n\n",i);
            }
        }
    }
    {
        // 17
        let vc = mojiretsu::sort_uniq(Path::new("17.txt"));
        if let Ok(vc) = vc {
            for item in vc {
                println!("{}",item);
            }
        }
    }
    {
        // 18
        let vc = mojiretsu::sort_by_column(Path::new("sec2.txt"),2);
        if let Ok(vc) = vc {
            for item in vc {
                println!("{}",item);
            }
        }
    }
    {
        // 19
        println!("19");
        let vc = mojiretsu::sort_by_freakency(Path::new("sec2.txt"));
        if let Ok(vc) = vc {
            for line in vc {
                println!("{}",line);
            }
        }
    }
}
