use celix::{
    celix_status_t,
    CELIX_SUCCESS,
    bundle_context_pt,
};

use BundleActivator;

#[repr(C)]
pub struct Activator {
	data: i32
}

impl Activator {
	pub fn new () -> Activator {
		Activator { data: 0 }
	}
}

#[allow(unused_variables)]
impl BundleActivator for Activator {
	fn create (&mut self, context: bundle_context_pt) -> celix_status_t {
		let status = CELIX_SUCCESS;
		self.data = 100;
		status
	}

	fn start (&self, context: bundle_context_pt) -> celix_status_t {
		println!("{:?}", self.data);
		CELIX_SUCCESS
	}

	fn stop (&self, context: bundle_context_pt) -> celix_status_t {
		println!("{:?}", self.data);
		CELIX_SUCCESS
	}

	fn destroy (&mut self, context: bundle_context_pt) -> celix_status_t {
		let status = CELIX_SUCCESS;
        println!("{:?}", self.data);
		drop(self.data);
		status
	}
}
