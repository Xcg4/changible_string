fn main(){
	let mut a = "Hel";
	print!("{}", a);
	a = "lo";
	println!("{}", a);

	use std::mem::*;
	let a: &str = "";
	let b: &str = "0123456789";
	let c: &str = "abcdè";
	println!("{} {} {}",
	 size_of_val(a),
	 size_of_val(b),
	 size_of_val(c));

	println!("{} {} {}",
	 size_of_val(&*a),
	 size_of_val(&*b),
	 size_of_val(&*c));
	//--------------------//

	println!("{} {} {} ",
	 size_of_val(&a),
	 size_of_val(&b),
	 size_of_val(&c));

	println!("{} {} {}",
	 size_of_val(&&*a),
	 size_of_val(&&*b),
	 size_of_val(&&*c));
	//--------------------//	
	
	println!("{} {} {}",
	 size_of_val(&&a),
	 size_of_val(&&b),
	 size_of_val(&&c));

	println!("{} {} {}",
	 size_of_val(&&&*a),
	 size_of_val(&&&*b),
	 size_of_val(&&&*c));	
	//--------------------//
	
	let mut a: String = "He".to_string();
	a.push('l');
	a.push('l');
	a.push('o');
	println!("{}", a);
	//-----------------------//

	let mut a: String = "Xy".to_string(); // "Xy"
	a.remove(0); // "y"
	a.insert(0, 'H'); // "Hy"
	a.pop(); // "H"
	a.push('i'); // "Hi"
	println!("{}", a);
	//-------------------//

	let mut s1 = "".to_string();
	s1.push('e');
	let mut s2 = "".to_string();
	s2.push('è');
	let mut s3 = "".to_string();
	s3.push('€');
	println!("{} {} {}; ", s1.capacity(), s1.len(), size_of_val(&*s1));
	println!("{} {}; ", s2.capacity(), s2.len());
	println!("{} {}", s3.capacity(), s3.len());

	let ase:&str="Hello";
	println!("Length of {}: {}", ase, size_of_val(ase));
	println!("Length of {}: {}", ase,  size_of_val(&*ase));
	println!("Length of {}: {}", ase,  ase.len());

	println!("{}", size_of::<&*const str>());
	println!("{}", size_of::<&*mut str>());

	let mut s = "".to_string();
	for _ in 0..10 {
	 println!("{:?} {} {} {}",
	 s.as_ptr(), s.capacity(), s.len(), size_of_val(&*s));
	 s.push('a');
	}
	println!("{:?} {} {} {}: {}",
	 s.as_ptr(), s.capacity(), s.len(),size_of_val(&*s), s)


}