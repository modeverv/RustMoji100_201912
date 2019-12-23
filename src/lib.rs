/// 00. 文字列の逆順
///　逆に（末尾から先頭に向かって）並べた文字列を得よ．
/// 一番最初の問題ですが、案外素直にいきませんでした...一度Charsに変換してから逆順にし、Stringに再変換して返しています。
pub fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}
/// 01. 「パタトクカシーー」
/// 「パタトクカシーー」という文字列の
/// 1,3,5,7文字目を取り出して連結した文字列を得よ．
pub fn odd_chars(text: &str) -> String {
    text.chars().step_by(2).collect()
}
/// 02. 「パトカー」＋「タクシー」＝「パタトクカシーー」
/// 「パトカー」＋「タクシー」の文字を先頭から
/// 交互に連結して文字列「パタトクカシーー」を得よ．
pub fn concat_alternately(left: &str, right: &str) -> String {
    let mut concated = String::new();
    left.chars().zip(right.chars()).for_each(|(l, r)| {
        concated.push(l);
        concated.push(r);
    });
    concated
}
/// 03. 円周率
/// "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."
/// という文を単語に分解し，
/// 各単語の（アルファベットの）文字数を先頭から
/// 出現順に並べたリストを作成せよ．
pub fn pi_from_text() -> Vec<usize> {
    let text = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    text.replace(",", "")
        .replace(".", "")
        .split_whitespace()
        .map(|s| s.len())
        .collect()
}
/// 04. 元素記号
/// "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can."
/// という文を単語に分解し，
/// 1, 5, 6, 7, 8, 9, 15, 16, 19番目の単語は先頭の1文字，
/// それ以外の単語は先頭に2文字を取り出し，
/// 取り出した文字列から単語の位置（先頭から何番目の単語か）
/// への連想配列（辞書型もしくはマップ型）を作成せよ．
use std::collections::HashMap;
pub fn element_symbols() -> HashMap<String, usize> {
    let text = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let mut result = HashMap::new();
    text.replace(".", "")
        .split_whitespace()
        .enumerate()
        .for_each(|(i, s)| {
            let symbol = match i {
                0 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 18 => s.get(0..1).unwrap(),
                _ => s.get(0..2).unwrap(),
            };
            result.insert(symbol.to_string(), i + 1);
        });
    result
}
/// 09. Typoglycemia
/// スペースで区切られた単語列に対して，
/// 各単語の先頭と末尾の文字は残し，
/// それ以外の文字の順序をランダムに並び替えるプログラムを作成せよ．
/// ただし，長さが４以下の単語は並び替えないこととする．
/// 適当な英語の文
/// （例えば"I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind ."）を与え，
/// その実行結果を確認せよ．
/// 第1章最後の問題、少し躓きました。とちゅう、
/// シャッフル部分はrandクレートを使用しています。
/// まず、文字列を並び替えなければいけないとなると、
/// strのままだと難しく、
/// さらに文字列の結合をするときは先頭がStringで後ろが
/// strという形にしないといけないみたいです。
/// また、Stringのスライスがcharのスライスになってくれればよいのですが、
/// そうはいかずにu8になってしまうようで一旦Charsに変換するという発想に至るのが
/// 難しかったです(ascii文字のみ想定にすればこの必要はないのですが)。
pub fn typoglycemia(text: &str) -> String {
    use rand::seq::*;
    text.split_whitespace()
        .map(|s| {
            if s.len() < 5 {
                s.to_string()
            } else {
                let (head, remaining) = s.split_at(1);
                let (body, tail) = remaining.split_at(remaining.len() - 1);
                let mut body: Vec<_> = body.chars().collect();
                body.shuffle(&mut rand::thread_rng());
                head.to_string() + &body.into_iter().collect::<String>() + tail
            }
        })
        .fold(String::new(), |result, s| {
            if result == "" {
                s
            } else {
                result + " " + &s
            }
        })
}

/// 10. 行数のカウント
/// 行数をカウントせよ．確認にはwcコマンドを用いよ
use std::fs::*;
use std::io::*;
use std::path::Path;
pub fn count_lines_number(path: &Path) -> Result<usize> {
    let f = File::open(path)?;
    let br = BufReader::new(f);
    let mut counter = 0;
    br.lines().for_each(|line| {
        counter += 1;
        println!("{} {}", counter, line.unwrap());
    });
    Ok(counter)
}

/// 11. タブをスペースに置換
/// タブ1文字につきスペース1文字に置換せよ．
/// 確認にはsedコマンド，trコマンド，もしくはexpandコマンドを用いよ
pub fn tab2space(path: &Path, tab_width: usize) -> Result<String> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let spaces = " ".repeat(tab_width);
    Ok(br
        .lines()
        .map(|s| match s {
            Ok(s) => s.replace("\t", &spaces) + "\n",
            Err(_) => "\0".to_string(),
        })
        .collect())
}

/// 14. 先頭からN行を出力
/// 自然数Nをコマンドライン引数などの手段で受け取り，
/// 入力のうち先頭のN行だけを表示せよ．確認にはheadコマンドを用いよ．
pub fn heads(path: &Path,n:usize) -> Result<Vec<String>> {
    let f = File::open(path)?;
    let br = BufReader::new(f);
    Ok(br.lines().take(n).map(|line| {
        line.unwrap()
    }).collect::<Vec<String>>())
}

/// 15. tail
pub fn tails(path:&Path,n:usize) ->Result<Vec<String>> {
    let f = File::open(path)?;
    let br = BufReader::new(f);
    let mut lines:Vec<String> = vec![];
    br.lines().for_each(|l| lines.push(l.unwrap()));
    Ok(lines.iter().rev().take(n).rev().map(|l| {String::from(l)} ).collect::<Vec<String>>())
}

pub fn str2sec(str: &str) -> Result<i32> {
    let v:Vec<&str> = str.split(":").collect();
    let min:i32 = v[0].parse().unwrap();
    let sec:i32 = v[1].parse().unwrap();
    Ok((min * 60) + sec)
}

pub fn format_sec(sec: i32) -> String {
    let min = (sec / 60) as i32;
    let secc = sec - min * 60 as i32;
    format!("{:02}:{:02}",min,secc)
}

/// 16. ファイルをN分割する
/// 自然数Nをコマンドライン引数などの手段で受け取り，
/// 入力のファイルを行単位でN分割せよ．同様の処理をsplitコマンドで実現せよ
pub fn split_file(path: &Path,n:usize) -> Result<Vec<String>> {
    let f = File::open(path)?;
    let br = BufReader::new(f);
    let lines = br.lines().collect::<Result<Vec<_>>>();
    lines.and_then(|lines| Ok(lines.chunks(n).map(|chunk| chunk.join("\n")).collect()))
}

/// 17. １列目の文字列の異なり
/// 1列目の文字列の種類（異なる文字列の集合）を求めよ．
/// 確認にはsort, uniqコマンドを用いよ．
pub fn sort_uniq(path: &Path) -> Result<Vec<String>>{
    let f = File::open(path)?;
    let br = BufReader::new(f);
    let mut map = HashMap::new();
    for line in br.lines() {
        //println!("{}",line.unwrap());
        map.entry(line.unwrap()).or_insert(1);
    }
    let mut vec:Vec<String> = vec![];
    for (k,_) in map {
        vec.push(k);
    }
    vec.sort();
    Ok(vec)
}