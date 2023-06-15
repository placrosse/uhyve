use byte_unit::Byte;
use criterion::{criterion_group, Criterion};
#[cfg(target_os = "linux")]
use uhyvelib::linux::x86_64::kvm_cpu::KvmCpu;
#[cfg(target_os = "macos")]
use uhyvelib::macos::x86_64::vcpu::XhyveCpu;
use uhyvelib::{params::Params, vm::UhyveVm};

pub fn load_vm_hello_world(c: &mut Criterion) {
	let path = [env!("CARGO_MANIFEST_DIR"), "benches_data/hello_world"]
		.iter()
		.collect();
	let params = Params {
		memory_size: Byte::from_bytes(1024 * 100000).try_into().unwrap(),
		..Default::default()
	};

	#[cfg(target_os = "linux")]
	let mut vm = UhyveVm::<KvmCpu>::new(path, params).expect("Unable to create VM");
	#[cfg(target_os = "macos")]
	let mut vm = UhyveVm::<XhyveCpu>::new(path, params).expect("Unable to create VM");

	c.bench_function("vm::load_kernel(hello world)", |b| {
		b.iter(|| vm.load_kernel().unwrap())
	});
}

criterion_group!(load_kernel_benchmark_group, load_vm_hello_world);
