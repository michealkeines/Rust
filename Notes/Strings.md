two strings cant be concatinated directly


format!("{} {} {}",s1,s2,s3) // all alows to create a new string with other strings

strings dont support indexing 

as rust used utf8 encoded bytes by eveey char is just u8 byte

a String is warpper over a Vec<u8>
	
to interate over a string, we need get its char array and they we can get individual bytes
	
	string.char()
	