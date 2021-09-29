#![allow(dead_code)]
pub mod vector_examples{
    use std::collections::HashMap;
    // use std::hash::Hash;
    pub(crate) fn entry_point(){
        // let vec = Vec::new();
        // vec.push(10);
        hashmap_examples();
        ownership_with_hashmap();
    }
    fn hashmap_examples(){
        let mut _map = HashMap::new();
        _map.insert(String::from("Blue"), 10);
        _map.insert(String::from("Yellow"), 50);

        println!("{:#?}", _map);


        let teams = vec![
            String::from("Green"),
            String::from("Orange"),
            String::from("Pale") ];

        let init_values_for_teams = vec![100,200,300];

        let new_map: HashMap<_,_> = teams.into_iter().zip(init_values_for_teams.into_iter())
            .collect();

        println!("{:#?}", new_map);
    }
    fn ownership_with_hashmap(){
        let mut a = String::from("My color");
        let b = String::from("Always Blue");
        a = String::from("My Car");
        let mut map = HashMap::new();
        map.insert(&a,10);
        map.insert(&b,20);
        println!("{:#?}",map);

        //After updating reference you can't use map
        // a = String::from("My Car");
        println!("{:#?}",map);
        println!("{}",a);

        //Sample insert strings without references
        let mut map = HashMap::new();
        //This is shadowing of a and b;
        let  a = String::from("My color");
        let b = String::from("Always Blue");
        map.insert(a,10);
        map.insert(b,20);


        for (key,value) in &map {
            println!("Key: {}, value: {}", key, value);
        }
    }

fn updating_old_values_in_hashmap(){

}
}

















