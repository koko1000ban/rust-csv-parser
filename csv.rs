//parse follow RFC 4180

export parse_line, parse, open;

#[doc = "Split lines"]
fn split_csv(data: str) -> [str] {
  let mut splited = [];

  //lines_anyを使うことで複数の改行コードをサポート
  let mut lines = str::lines_any(data);
  let mut linenum = 0;
  
  //#はじまりと空行はskip (log warn)
  // 正規表現ほしいなあ
  lines = vec::filter(lines) {|line|
    let f = (str::is_empty(line) || str::char_at(line, 0u) == '#');
    if f {
      log(warn, #fmt("ignore line:%d. it may be empty or comment line.", linenum+1));
    }
    linenum+=1;
    !f
  };
  
  let mut i=0, n=vec::len(lines) as int;
  while i < n {
    let mut line = lines[i];
    let mut quote_count = vec::count(str::chars(lines[i]), '"') as int;
    
    //splitした行のダブルクォーテーションが奇数であれば偶数になるまで次行と結合
    while quote_count % 2 != 0 {
      if i+1 >= n {
        // reach eof
        line += "\"";
        break;
      }
      log(debug, "concat next line..");
      i+=1;
      line += "\n"+lines[i];
      quote_count += vec::count(str::chars(lines[i]), '"') as int;
    }
    splited += [line];
    i+=1;
  }
  ret splited;
}

#[doc = "Parse line to csv row"]
fn parse_line(line: str) -> [str] {
  let mut row = [];
  let mut i=0u, n=str::len(line);
  let mut elem="";
  let mut quoted = false;

  while i < n {
    let mut {ch, next} = str::char_range_at(line, i);
    log(debug, #fmt("%c | %b", ch, quoted));

    alt ch {
     '"' {
      // 初めてでてきた
      // 閉じるためクオート
      // エスケープされたクウォート
      let res = str::char_range_at(line, next);
      if quoted && i < n && res.ch == '"' {
        log(debug, "discover escaped double quote!");
        str::push_char(elem, '"');
        next=res.next;
      } else {
        quoted = !quoted;
      }
     }
     ',' {
      if quoted {
        str::push_char(elem, ch);
      } else {
        log(debug, #fmt("push elem: %s", elem));
        row += [elem];
        elem = "";
      }
     }
     _ {
      str::push_char(elem, ch);
     }
    }
    i=next;
  }
  row += [elem];
  ret row;
}

#[doc = "
open file and split, parse 
# Example

~~~
csv::open(\"/path/to/csv_file\") { |row|
   // row is comma splited lien
}
~~~

# Arguments

* p - The file path
* receiver - The closure that receive splited line

# Failure

If `p` path is not found.
"]
fn open(p: path, receiver: fn([str])) {
  let res = io::read_whole_file_str(p);
  alt res {
   result::ok(s) {
    parse(s, receiver);
   }
   result::err(e){
    log(error, e);
   }
  }
}

#[doc = "
parse csv and yield receiver

# Example

~~~
csv::parse(\"a,b,c\") { |row|
   let i = 0u;
   while i < vec::len(row) {
      std::io::println(#fmt(\"%u: %s\", i, row[i]));
      i += 1u;
   }
}
~~~

# Example output

~~~
0: a
1: b
2: c
~~~
"]
fn parse(data:str, receiver: fn([str])) {
  let splited = split_csv(data);
  for splited.each {|line|
    receiver(parse_line(line));
  }
}

#[cfg(test)]
mod test{
  // ぱくり https://github.com/grahame/rust-csv/blob/master/csv.rs
  fn rowmatch(testdata: str, expected: [[str]]) {
    let runchecks = fn@(testdata: str) {
      let splited = split_csv(testdata);
      let mut i = 0u;
      let n = vec::len(splited);
      if n != vec::len(expected) {
        log(info, (splited, expected));
        assert(n == vec::len(expected));
      }
      
      while i < n {
        let row = parse_line(splited[i]);
        let expect = expected[i];
        assert(row.len() == vec::len(expect));
        
        let mut j = 0u;
        while j < row.len() {
          assert(row[j] == expect[j]);
          j += 1u;
        }
        i += 1u;
      }
      ret;
    };

    // so we can test trailing newline case, testdata
    // must not end in \n - leave off the last newline
    runchecks(testdata);
    runchecks(testdata+"\n");
    runchecks(str::replace(testdata, "\n", "\r\n"));
  }
  
  #[test]
  fn test_parse(){
    let mut rows = [];
    parse("9404,あやめ公園,0,ｱﾔﾒｺｳｴﾝ\n4778,一本松(埼玉県),0,ｲｯﾎﾟﾝﾏﾂ\n9482,羽田空港第１ビル,0,ﾊﾈﾀﾞｸｳｺｳﾀﾞｲｲﾁﾋﾞﾙ") { |row|
      let mut cells = [];
      for row.each {|cell|
        cells += [cell];
      }
      rows += [cells];
    };
    assert(rows == [["9404","あやめ公園","0","ｱﾔﾒｺｳｴﾝ"],["4778","一本松(埼玉県)","0","ｲｯﾎﾟﾝﾏﾂ"], ["9482","羽田空港第１ビル","0","ﾊﾈﾀﾞｸｳｺｳﾀﾞｲｲﾁﾋﾞﾙ"]]);
  }

  #[test]
  fn test_multiline_1() {
    rowmatch("\naaa,bbbb,\"ccc\ndddd\",\"fffff\"\n111,222,333,444,555", 
             [["aaa", "bbbb", "ccc\ndddd","fffff"], ["111", "222", "333", "444", "555"]]);
  }

  #[test]
  fn test_multiline_2(){
    rowmatch("jjj,\"kk\nlll", [["jjj", "kk\nlll"]]);
  }

  #[test]
  fn test_complex_blank_quote() {
    rowmatch("\"aaa\",\"b\nbb\",\"ccc\",zzz,\"y\"\"Y\"\"y\",xxx,\"\"", 
             [["aaa", "b\nbb", "ccc", "zzz", "y\"Y\"y", "xxx", ""]]);
  }

  #[test]
  fn test_simple_number() {
    rowmatch("1,2,3",[["1", "2", "3"]]);
  }

  #[test]
  fn test_multibyte(){
    rowmatch("9404,あやめ公園,0,ｱﾔﾒｺｳｴﾝ,", 
            [["9404", "あやめ公園", "0", "ｱﾔﾒｺｳｴﾝ", ""]]);
  }

  #[test]
  fn test_simple() {
    rowmatch("a,b,c,d\n1,2,3,4",
             [["a", "b", "c", "d"], ["1", "2", "3", "4"]]);
  }
  
  #[test]
  fn test_trailing_comma() {
    rowmatch("a,b,c,d\n1,2,3,4,",
             [["a", "b", "c", "d"], ["1", "2", "3", "4", ""]]);
  }
  
  #[test]
  fn test_leading_comma() {
    rowmatch("a,b,c,d\n,1,2,3,4",
             [["a", "b", "c", "d"], ["", "1", "2", "3", "4"]]);
  }
  
  #[test]
  fn test_quote_simple() {
    rowmatch("\"Hello\",\"There\"\na,b,\"c\",d",
             [["Hello", "There"], ["a", "b", "c", "d"]]);
  }
  
  #[test]
  fn test_quote_nested() {
    rowmatch("\"Hello\",\"There is a \"\"fly\"\" in my soup\"\na,b,\"c\",d",
             [["Hello", "There is a \"fly\" in my soup"], ["a", "b", "c", "d"]]);
  }
  
  #[test]
  fn test_quote_with_comma() {
    rowmatch("\"1,2\"",
                 [["1,2"]])
    }
  
  #[test]
  fn test_quote_with_other_comma() {
    rowmatch("1,2,3,\"a,b,c\"",
             [["1", "2", "3", "a,b,c"]])
  }
  
  #[test]
  fn test_blank_line() {
    rowmatch("\n\n", []);
  }
}

