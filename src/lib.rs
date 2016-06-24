#![allow(non_snake_case)]

extern crate celix;
use celix::{
    celix_status_t,
    CELIX_SUCCESS,
    bundle_context_pt,
};
mod activator;
use activator::Activator;
use std::os::raw::c_void;

trait BundleActivator {
	fn create (&mut self, context: bundle_context_pt) -> celix_status_t;
	fn start(&self, context: bundle_context_pt) -> celix_status_t;
	fn stop (&self, context: bundle_context_pt) -> celix_status_t;
	fn destroy (&mut self, context: bundle_context_pt) -> celix_status_t;
}

#[no_mangle]
pub extern "C" fn bundleActivator_create(ctx: bundle_context_pt, userData: *mut *mut c_void) -> celix_status_t {
    let status = CELIX_SUCCESS;
    let mut a : Activator = Activator::new();
    a.create(ctx);
    let a : *mut Activator = Box::into_raw(Box::new(a));
    unsafe { *userData = a as *mut c_void }
    status
}

#[no_mangle]
pub extern "C" fn bundleActivator_start(userData: *mut c_void, ctx: bundle_context_pt) -> celix_status_t {
    let activator = userData as *mut Activator;
    let status = unsafe { (*activator).start(ctx) };
    status
}

#[no_mangle]
pub extern "C" fn bundleActivator_stop(userData: *mut c_void, ctx: bundle_context_pt) -> celix_status_t {
    let activator = userData as *mut Activator;
    let status = unsafe { (*activator).stop(ctx) };
    status
}

#[no_mangle]
pub extern "C" fn bundleActivator_destroy(userData: *mut c_void, ctx: bundle_context_pt) -> celix_status_t {
    let activator = userData as *mut Activator;
    let status : celix_status_t;
    unsafe {
        status = (*activator).destroy(ctx);
        let _ = Box::from_raw(activator);
    }
    status
}
