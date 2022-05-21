// count
let amount = User::count()
.where()
    .id("2")
.execute();

// create 
let user = User {
    id: "John".to_string(),
    name: "John".to_string(),
    age: Some(20),
    isAdmin: true,
};
user.insert();

// find/findMany
let users = User::find()
.where()
    .name(FindConfig::Contains("teste")) 
    .email(FindConfig::Equals("teste@teste.com")) 
.many() 
.order(OrderBy::Asc("name"))
.skip(10)
.take(10)
.relations(vec![UserRelations::Projets]) 
.execute();

// update
let mut user = User::update()
.where()
    .id("1".to_string()) 
.values()
    .name("Matheus".to_string())
.result() // return the updated user
.execute();

// delete
User::delete()
.where()
    .id("1".to_string()) 
.execute();