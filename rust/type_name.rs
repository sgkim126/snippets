#![feature(core_intrinsics)]

fn get_name<T>(_t: &T) -> String {
    unsafe {
        (*std::intrinsics::type_name::<T>()).to_string()
    }
}

fn main() {
    println!("{}", get_name(&get_name::<i32>));
}
