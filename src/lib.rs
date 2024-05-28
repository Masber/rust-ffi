use ffi_convert::RawBorrow;
use ffi_convert::{AsRust, CArray, CDrop, CReprOf, CStringArray, RawPointerConverter};
use futures::executor::block_on;
use std::ffi::{c_char, c_float, c_int, c_schar, c_uchar, c_uint, CStr};

// TEST 0
//
#[no_mangle]
pub extern "C" fn helloworld() {
    println!("hello, world!");
}

// TEST 1
//
#[no_mangle]
pub extern "C" fn hello(name: *const c_char, surname: *const c_char) {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap().to_string();

    let surname_cstr = unsafe { CStr::from_ptr(surname) };
    let surname = surname_cstr.to_str().unwrap().to_string();

    println!("hello {} {}!", name, surname);
}

// TEST 2
//
#[no_mangle]
pub extern "C" fn vector_int(data: *const c_int, length: c_int) {
    // print raw data (memory address of the pointer and its length)
    println!("data: {:?} and length: {}", data, length);
    // print each elem of the golang slice
    let slice = unsafe { std::slice::from_raw_parts(data, length as usize) };
    println!("slice: {:?}", slice);
}

// TEST 3
//
#[no_mangle]
pub extern "C" fn vector_string(data: *const *const c_char, length: c_int) {
    // print raw data (memory address of the pointer and its length)
    println!("data: {:?} and length: {}", data, length);

    // Convert input into Vec<String>
    let string_vec = unsafe {
        std::slice::from_raw_parts(data, length as usize)
            .iter()
            .map(|&cstr| CStr::from_ptr(cstr).to_string_lossy().into_owned())
            .collect::<Vec<String>>()
    };

    println!("slice: {:?}", string_vec);
}

// TEST 4
//
#[repr(C)]
#[derive(Debug)]
pub struct MyStruct {
    x: c_int,
    y: c_int,
}

#[no_mangle]
pub extern "C" fn my_struct(c_data: *const MyStruct) {
    let c_data = unsafe {
        assert!(!c_data.is_null());
        &*c_data
    };

    println!("x: {:#?}", c_data.x);
    println!("y: {:#?}", c_data.y);
    println!("struct: {:#?}", c_data);
}

// TEST 5
//
#[repr(C)]
#[derive(Debug)]
pub struct MyStruct2 {
    name: *const c_char,
    age: c_int,
}

#[no_mangle]
pub extern "C" fn my_struct_2(c_data: *const MyStruct2) {
    let c_data = unsafe {
        assert!(!c_data.is_null());
        &*c_data
    };

    let name_cstr = unsafe { CStr::from_ptr(c_data.name) };
    let name = name_cstr.to_str().unwrap().to_string();

    println!("name: {:#?}", name);
    println!("age: {:#?}", c_data.age);
    println!("struct: {:#?}", c_data);
}

// TEST 6
//
#[no_mangle]
pub extern "C" fn add(left: c_int, right: c_int) -> c_int {
    left + right
}

// TEST HSM GROUP
//
/* #[repr(C)]
#[derive(Debug)]
pub struct CMember {
    ids: *const *const libc::c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct CHsmGroup {
    label: *const libc::c_char,
    description: *const libc::c_char,
    members: *const CMember,
    exclusive_group: *const libc::c_char,
    tags: *const *const libc::c_char,
} */

#[derive(Debug, Default, Clone)]
pub struct HsmGroup {
    pub label: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub members: Option<Member>,
    pub exclusive_group: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Member {
    pub ids: Option<Vec<String>>,
}

#[repr(C)]
#[derive(CReprOf, AsRust, CDrop, RawPointerConverter, Debug)]
#[target_type(Member)]
pub struct CMember {
    #[nullable]
    ids: *const CStringArray,
}

#[repr(C)]
#[derive(CReprOf, AsRust, CDrop, RawPointerConverter, Debug)]
#[target_type(HsmGroup)]
pub struct CHsmGroup {
    label: *const libc::c_char,
    #[nullable]
    description: *const libc::c_char,
    #[nullable]
    members: *const CMember,
    #[nullable]
    exclusive_group: *const libc::c_char,
    #[nullable]
    tags: *const CStringArray,
}

#[no_mangle]
pub extern "C" fn get_hsm_group(
    /* shasta_token_c: *const c_char,
    shasta_base_url_c: *const c_char,
    shasta_root_cert_c: *const CArray<*const u8>, */
    data: CHsmGroup,
) {
    /* let get_future = mesa::hsm::group::http_client::get(
        shasta_token_c,
        shasta_base_url_c,
        &shasta_root_cert_c,
        None,
    );

    block_on(get_future); */

    println!("C struct: {:?}", data);
    // println!("Rust struct: {:?}", data.as_rust());
}
