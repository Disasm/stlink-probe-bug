use probe_rs::Probe;

fn main() {
    pretty_env_logger::init();

    let probes = Probe::list_all();
    println!("Available probes: {:?}", probes);
    if probes.len() == 0 {
        return;
    }

    let probe: Probe = probes[0].open().unwrap();
    println!("Probe opened");

    let session = probe.attach("stm32f401").unwrap();
    let mem = session.attach_to_memory(0).unwrap();

    let v = mem.read32(0x2000_0000).unwrap();
    println!("Data: {:08x}", v);
}
