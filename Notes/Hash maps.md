std::collections::HashMap;

HashMap<k,v>

let mut users = HashMap::new();

users.insert(String::form("admin"), true);

just like vectors all the keys must have same type and all the values must have same type

hasmap takes owership of types that doesnt support copy trait like strings

hashmap returns option<> as output