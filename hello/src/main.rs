
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io;

fn main() -> Result<(),io::Error>{
    println!("hello word!");
    let num = rand::thread_rng().gen_range(1,100);
    
    // println!("the progress the guessing");
    // let mut guess_num = String::new();
    // io::stdin().read_line(&mut guess_num).expect("c错误信息");
    
    // println!("You guess:{}",guess_num);
    // println!("the rang number{}",num);

    let x = test(5);
    println!("the test value : {}",x);

    let num:(i32,u32) = (1,2);
    println!("the a[0] {},a[1] {}",num.0,num.1);
    let num = (2,3,4,5);
    println!("the a[0] {},a[1] {}",num.0,num.1);
    let num = [1;4];
    println!("the a[0] {},a[1] {}",num[0],num[1]);
    let num:[i32;5] = [1;5];
    let mut i = 0;
    println!("the a[0] {},a[1] {}",num[0],num[1]);
    for elem in num.iter() {
        println!("the array value {}",elem);
    }
    let aaa: i32 = 2;
    println!(" the aaa value {}",aaa);
    if aaa > 5 {
        println!("the num more 5");
    } else  if  aaa > 3 {
        println!("the num more 3");
    } else {
        println!("the num less 3");
    }

    let mut num = 1;
    let mut  aa = loop {
        num = num + 1 ;
        if num == 10 {
            break num * 2;
        }
    };
    println!("the loop value {}",aa);

    while aa != 0 {
        println!("the number value {}",aa);
        aa = aa - 1;
    }

    while  aa != 10 {
        aa = aa + 1;
        println!("the aa value {}",aa);
    }
    println!("the end");

    let hell = "hello word";
    let vat = String::from("adssd 1222");
    let result = slice(&vat);
    deala(hell);
    deal(&vat);
    println!("the splict last word {} ",result);
    let  a = String::from("aaaa");
    let mut b = a;
    b.clear();

    println!("the aa value {}",b);

    let user = User {
        use_name: String::from("yang"),
        age: 29,
        email: String::from("445482919@qq.com")
    };

    println!("the username {}",user.use_name);
    println!("the user email {}",user.email);

    let mut user1 = User {
        use_name: String::from("ma"),
        ..user
    };

    println!("the username {}",user1.use_name);
    println!("the username {}",user1.email);
    user1.age = 100;
    

    let rectage = Rectage {
        width: 30, height : 90
    };

    let rectage1 = Rectage {
        width: 40, height : 60
    };

    let area = rectage.area();
    let squery = Rectage::squery(30, 40);
    println!("the rectage area {}",area);
    println!("the rectage  {:#?}",rectage);
    println!("the rectage1 can hold other {}",rectage1.can_hold(&rectage));
    println!("the rectage  {:#?}",squery);
    let v4Ip = IpAddr::V4(127,0,0,0);
    let V6Ip = IpAddr::V6(String::from("::1"));
    let Aa  = IpAddr::Aa {aaa:String::from("dsds"),bbb:String::from("bbb")};
    println!("the v4 ipaddr {:?}",v4Ip);

    println!("the v4 ipaddr {:?}",Aa.call());
    let some = Some(5);
    let someV = Some("aaaa");
    let none:Option<u32> = None;
    println!("the some value {:?}",some);
    println!("the some value {:?}",someV);
    println!("the some value {:?}",none);
    let v = match v4Ip {
        IpAddr::V4(_,_,_,_) => {
            println!("the value is Aa");
            1
        },
        IpAddr::V6(_) => {
            2
        },
        IpAddr::Aa {aaa:_,bbb:_} => {
            3
        }
    };
    println!("the show enum value {}",v);
    let aa = num_plus(Some(2));
    println!("the value {:?}",aa);

   let u83 = Some(3);
   match u83  {
    Some(3) => {
        println!("the value is 3");
    },
    _ =>  {
        println!("the value is others");
    }
   }
   if let Some(3) = u83 {
       println!("the value is 3")
   }
   eat_restaurant();

   //Vector 
   let  aa :Vec<i32> = vec![1,2,3,4];
   let mut bb:Vec<i32> = Vec::new();
   bb.push(3);
   bb.push(4);
   bb.push(5);

   for elem in &bb {
       println!("the bb value {}",elem)
   }
   for i in &mut bb { //得到可变的数组引用，修改值
       *i += 50;
   }

   for elem in &bb {//得到不可变的引用，打印值
    println!("the bb value {}",elem);
   }

   //字符串
   let aa = "dsdsdd";
   let mut ss = aa.to_string();
   let sa = String::from("dsdsd");//字符常量值初始化
   ss.push_str(&sa);
   let a = ss + &sa;//第一参数被使用权转移了
   println!("the new value {}",sa);
   let new_str = format!("{}-{}",sa,sa);//格式化数据
   let index = &new_str[0..2];//获取字符变量的值，字符切片
   println!("the &str[0..2] value {}",index);
   for i in new_str.bytes() { //输出字节
       println!("the value in new_str {}",i);
   }
   for i in new_str.chars() {//输出标量值
    println!("the value in new_str {}",i);
   }

   //hashMap
   let mut hash_map:HashMap<String,String> = HashMap::new();
   hash_map.insert(String::from("1"), String::from("ddd"));
   hash_map.insert(String::from("2"), String::from("aaa"));
   let aa = hash_map.get("1") ;
   match aa {
       Some(a) => {
           println!("获取的值是 value {}",a)
       },
       None => {

       }
   }
   for (elem,value) in hash_map.iter() {
       println!("the key :{},value :{}",elem,value);
   }
   //如果没有key,则插入数据
   let value = hash_map.entry(String::from("3")).or_insert(String::from("333"));
   *value += "1"; //返回的引用进行操作
   println!("{:?}",hash_map);
   //异常
//    panic!("the error crash and burn"); //不可恢复信息的回调
   let vec = vec![1,2,3,4];
//    vec[99];
//RUST_BACKTRACE=1 cargo run 进行栈信息的跟踪



let file = File::open("hello.txt");
let f = match file {
    Ok(f) => f,
    Err(error) => match error.kind() {
            io::ErrorKind::NotFound =>
                match File::create("hello.txt") {
                    Ok(f) => f,
                    Err(error) => {
                        panic!("the file not create");
                    }
                }
            ,
            other_error =>  panic!("other file open error {:?}",other_error)
         },
};

//默认的提示
// let futh = File::open("h.txt").unwrap();
//期望的错误提示
// let futh = File::open("h.txt").expect("can't create file");


let s = read_use_duan1();
match s {
    Ok(s) => println!("the file content {}",s),
    Err(s) => panic!("the error {}",s)
}


let point = Point {
    x:1,y:2
};
let point2 = Point{
    x:"ddd",
    y:1
};
let bb = point2.new(point);
println!("the point x value {},point y {} ",bb.x,bb.y);

//定义trait
let aa = NewArticle {
    headline:String::from("dsds"),
    localhost: String::from("127.0.0.1")
};
let a = aa.print();
println!("the result a {}",a);
//默认实现
let bb = twwier {
    aa:String::from("ddd"),
    bb:String::from("aaa")
};
println!("the value default funtion {}",aa.eprint());

println!("the value default funtion {}",bb.eprint());



//全局方法方法的数据给调用者
let fs = std::fs::read_to_string("hello2.txt")?;
println!("the hello text content {}",fs);
Ok(())

//    match bb.get(3) {
//        Some(thirld) => {
//            println!("the third value {}",thirld);
//        },
//        None => {
//            println!("the no third value");
//        }
//    }
}

// fn large<T> (list:&[T]) -> T {
    // let mut larget = list[0];
    // for &elem in list.iter() {
        // if larget > elem {
        //     larget = elem;
        // }
    // };
    // larget
// }





struct Point<T,U> {
    x:T,
    y:U
}
impl<T,U> Point<T,U> {
    fn new<V,W> (self,a :Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: a.y
        }
    }
}

trait sumer {
    fn print(&self) -> String;
    fn eprint(&self) -> String  {
        format!("dd")
    }
}


pub struct twwier {
    aa:String,
    bb:String
}

impl sumer for twwier {
   fn print(&self)->String {
       String::from("dsdsds")
   }
   fn eprint(&self) ->String {
       String::from("aaa")
   }
}

pub struct NewArticle {
    pub headline: String,
    pub localhost:String
}

impl sumer for NewArticle {
    fn print(&self) -> String {
        self.headline.to_string()
    }
}



fn read_use_duan() -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_use_duan1() -> Result<String,io::Error> {
    std::fs::read_to_string("hello.txt")
}


fn num_plus ( a: Option<i32>) -> Option<i32> {
    match a  {
        None => None,
        Some(i)  => Some(i+1)
    }
}





  
struct User {
 use_name: String,
 age: i32,
 email: String
}

#[derive(Debug)]
struct Rectage {
 width: u32,
 height: u32
}

#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
    Aa {
        aaa:String,
        bbb:String
    }
}

impl IpAddr {
    fn call (&self) {
        println!("the v4 value");
    }
}

impl Rectage {

    fn squery (width: u32,height: u32) -> Rectage {
        return Rectage {
            width,height
        };
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self,other: &Rectage) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}




fn deal (a : &String ) {
    println!("the value {}", a);
}

fn deala (a : &str ) {
   
    println!("the value {}", a);
}

fn slice(a : &str) -> &str {
  let bytes = a.as_bytes();
  for (i,&item) in bytes.iter().enumerate() {
    if item == b' '{
        return &a[..i];
    }
  };
  &a[..]
}

fn test(mut x: i32) -> i32 {
    x = x + 1;
    return x;
}



fn read_str_file () -> Result<String,io::Error> {
    let f = File::open("hello.txt");
    let mut file = match f {
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
     Ok(_) => Ok(s),
     Err(s) => Err(s)
    }
}


 //从front_of_host文件中加载数据
use restaurant::front_of_house::{self,hosting,service_order};
pub fn eat_restaurant () {
//     //绝对路径
    hosting::add_to_watier();
    //相对路径
    hosting::add_to_table();
    let mut aa = front_of_house::Breakfast::summer(&String::from("ddsdd"));
   // aa.toast = String::from("dsdsd");//私有的结构属性，不能被赋值
    aa.key = String::from("dsdsd");
    println!("the breakfast value {:#?}",aa);
    let key1 = front_of_house::y_enum::a1;//使用作用域
    let kye2 = front_of_house::y_enum::a2;
    service_order::fix_incorret_order();
    restaurant::service_order(3);
}