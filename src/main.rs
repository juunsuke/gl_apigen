
use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator, StructGenerator};
use std::fs::File;
use std::path::Path;




fn gen(ver: (u8, u8), prof: Profile) {
	let pname = match prof {
		Profile::Core => "core",
		Profile::Compatibility => "compat",
	};

	let fname = format!("out/gl_{}{}_{}.rs", ver.0, ver.1, pname);
	println!("{}", fname);

    let mut file = File::create(&Path::new(&fname)).unwrap();
    Registry::new(Api::Gl, ver, prof, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}

fn struct_gen(ver: (u8, u8), prof: Profile) {
	let pname = match prof {
		Profile::Core => "core",
		Profile::Compatibility => "compat",
	};

	let fname = format!("out/gl_s_{}{}_{}.rs", ver.0, ver.1, pname);
	println!("{}", fname);

    let mut file = File::create(&Path::new(&fname)).unwrap();
    Registry::new(Api::Gl, ver, prof, Fallbacks::All, [])
        .write_bindings(StructGenerator, &mut file)
        .unwrap();
}

fn gen_both(ver: (u8, u8)) {
	gen(ver, Profile::Core);
	gen(ver, Profile::Compatibility);
}

fn struct_gen_both(ver: (u8, u8)) {
	struct_gen(ver, Profile::Core);
	struct_gen(ver, Profile::Compatibility);
}

fn main() {
	
	gen((2, 0), Profile::Core);
	gen((2, 1), Profile::Core);
	gen((3, 0), Profile::Core);
	gen((3, 1), Profile::Core);
	gen_both((3, 2));
	gen_both((3, 3));
	gen_both((4, 0));
	gen_both((4, 1));
	gen_both((4, 2));
	gen_both((4, 3));
	gen_both((4, 4));
	gen_both((4, 5));
	gen_both((4, 6));
	
	struct_gen((2, 0), Profile::Core);
	struct_gen((2, 1), Profile::Core);
	struct_gen((3, 0), Profile::Core);
	struct_gen((3, 1), Profile::Core);
	struct_gen_both((3, 2));
	struct_gen_both((3, 3));
	struct_gen_both((4, 0));
	struct_gen_both((4, 1));
	struct_gen_both((4, 2));
	struct_gen_both((4, 3));
	struct_gen_both((4, 4));
	struct_gen_both((4, 5));
	struct_gen_both((4, 6));
}



