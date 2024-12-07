fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("*******the value s1:{s2}*******");
    
    /*無法執行下面的語句，因為S1的引用已經給S2了，無法S2和S1同時擁有對堆上數據“hello”的引用，否則會產生兩次釋放
    這個是rust所不允許的，所以被限制了，S1給到S2後，S1就無效了*/
    //println!("*******the value s1:{s1}*******");
}
