mod keyworld lets us sperate thing with public and private notaiton

mod can contain other mods inside

mod User {
	mod admin {
		fn isadmin(){
		}
	}
	mod normal_user {
		fn isanon {
		}
	}
}

everything inside a module is private by default

we use pub keyword to make function or module public for everyone

mod user {
	pub mod admin {
		pub fn isadmin();
	}
}

user::admin::isadmin() // this accessible as it is public

everything should be explicitly add pub, if not it will stay private even if the outer layer mod is public

we can use super to access function that are in parent 

fn test() {}

mod admin {
	fn ok () {
		is(); // accessing the function inside the same mod
		super::test(); // accessing the funtion from parent
	}
	fn is() {
	}
}

we can set one pub for seperate feilds in structs

for enum if we make it pub all the fiels will automaticall ybecome public

use keyword make the path accessible as symlink

use user::admin;

now we can access admin direclty as admin::ok()

with as we can define an temp for a path

use user::admin as omg;

omg.ok();

use key make  the imported fun only accessible with that file so 

we have to use pub to make it publice event he imported ones

pub use user::admin();

