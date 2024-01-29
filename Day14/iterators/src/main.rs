// iterator ==> to iterate over the each item of vector or hashmap

fn main() {
    println!("Hello, world!");

    let v1 = vec![4, 6, 7, 4];

    let v1_iter = v1.iter(); // iter() borrow the item of original vector

    // for i in v1_iter{
    //     println!("hi there here is your item {}", i);
    // }

    // sum method in iterator to sum each element

    let v1_sum: u32 = v1_iter.sum(); // here sum method (consuming adapters) takes ownership of v1_iter

    if v1_sum == 21 {
        println!("right!")
    } else {
        println!("right!")
    }

    // map() ==> iterator adpater similar like we use in python javascript

    let v2 = vec![45, 6, 9, 6];

    let v3_incr: Vec<_> = v2.iter().map(|x| x + 1).collect(); // here collect method call the closure
    println!("{:?}", v3_incr)

    //-------------------------------------------------------------------------

    // filter() adapter example
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    /*
       here we use into_iter because in the end it will return refrence of iterator while our function returning original vector
       so into_iter() will take ownership from shoes vectors and return original Shoes vector(array)
    */
    shoes.into_iter().filter(|s| s.size == size).collect()
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
  
    fn filters_by_shoe_size(){
        let shoes = vec![Shoe{size:10,style:String::from("sneaker")},Shoe{size:11,style:String::from("formal")},Shoe{size:12,style:String::from("sandal")}];

        let shoes_size_12 = shoes_in_size(shoes, 12);

        assert_eq!(shoes_size_12, vec![Shoe{size:12, style:String::from("sandal")}]);
    }
}
