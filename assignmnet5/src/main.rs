
fn main() {
    let mut number : f32 = 5.0;

    number = module_a :: function1(number);
    println!("{}",number );

    number = module_d :: function4(number);
    println!("{}",number );
    
}

pub mod module_a {
    pub fn function1 (mut _x: f32) -> f32 {
        _x = _x * 9.0;
        _x = crate :: module_b :: function2(_x);
        _x 
    }
}

pub mod module_b {
    pub fn function2 (mut _y: f32) -> f32 {
        _y = _y / 5.0;
        _y = crate :: module_c :: function3(_y);
        _y

    }
}

pub mod module_c {
    pub fn function3 (mut _z: f32) -> f32 {
        _z = _z + 32.0;
        _z
    }
}

pub mod module_d {
    pub fn function4 (mut _m: f32) -> f32 {
        _m = _m - 32.0;
        _m = crate :: module_e :: function5(_m);
        _m
    }
}

pub mod module_e {
    pub fn function5 (mut _n: f32) -> f32 {
        _n = _n * 5.0;
        _n = crate :: module_f :: function6 (_n);
        _n
    }
}

pub mod module_f {
    pub fn function6 (mut _o: f32) -> f32 {
        _o = _o / 9.0;
        _o
    }
}
