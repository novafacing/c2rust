use std::ptr;

// CHECK-LABEL: final labeling for "alias3_copy_bad1"
// CHECK-DAG: ([[#@LINE+1]]: x): {{.*}}type = READ | WRITE#
pub unsafe fn alias3_copy_bad1(x: *mut i32) {
    // CHECK-DAG: ([[#@LINE+1]]: p): {{.*}}type = READ#
    let p = x;
    // CHECK-DAG: ([[#@LINE+1]]: q): {{.*}}type = READ | WRITE#
    let q = x;
    *q = *p;
}

// CHECK-LABEL: final labeling for "alias3_copy_bad2"
// CHECK-DAG: ([[#@LINE+1]]: x): {{.*}}type = READ | WRITE#
pub unsafe fn alias3_copy_bad2(x: *mut i32) {
    // CHECK-DAG: ([[#@LINE+1]]: p): {{.*}}type = READ | WRITE#
    let p = x;
    // CHECK-DAG: ([[#@LINE+1]]: q): {{.*}}type = READ#
    let q = x;
    *p = *q;
}

#[cfg(not(compiling_for_test))]
pub unsafe fn alias3_addr_of_bad1(x: *mut i32) {
    let p = ptr::addr_of_mut!(*x);
    let q = ptr::addr_of_mut!(*x);
    *q = *p;
}

#[cfg(not(compiling_for_test))]
pub unsafe fn alias3_addr_of_bad2(x: *mut i32) {
    let p = ptr::addr_of_mut!(*x);
    let q = ptr::addr_of_mut!(*x);
    *p = *q;
}
