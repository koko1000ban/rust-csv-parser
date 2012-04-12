use std;
use csv;

#[test]
fn test_open(){
  
  let mut rows = [];

  csv::open("./test/test1.csv") { |row|
    let mut cells = [];
    for row.each {|cell|
      cells += [cell];
    }
    rows += [cells];
  };
  assert(rows == [["9404","あやめ公園","0","ｱﾔﾒｺｳｴﾝ"],["4778","一本松(埼玉県)","0","ｲｯﾎﾟﾝﾏﾂ"], ["9482","羽田空港第１ビル","0","ﾊﾈﾀﾞｸｳｺｳﾀﾞｲｲﾁﾋﾞﾙ"]]);
}