use std::fs;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;

fn main() {

    // Boucle sans fin
    loop {

    // Lecture des fichiers locaux
    let file_ram = fs::read_to_string("/sys/class/powercap/intel-rapl:0:1/energy_uj")
    .expect("LogRocket: Should have been able to read the file");
    let file_cpu = fs::read_to_string("/sys/class/powercap/intel-rapl:0:0/energy_uj")
    .expect("LogRocket: Should have been able to read the file");
    let file_pkg = fs::read_to_string("/sys/class/powercap/intel-rapl:0/energy_uj")
    .expect("LogRocket: Should have been able to read the file");

    // Trim de string
    let file_ram = file_ram.trim();
    let file_cpu = file_cpu.trim();
    let file_pkg = file_pkg.trim();

    // Conversion string en f64
    let ram_first = file_ram.parse::<f64>().unwrap();
    let cpu_first = file_cpu.parse::<f64>().unwrap();
    let pkg_first = file_pkg.parse::<f64>().unwrap();


    // Creating time de 1 seconde
    let time = Duration::from_secs(1);
    // Sleep 1 seconde
    sleep(time);

    // Lecture des fichiers locaux	
    let file_ram = fs::read_to_string("/sys/class/powercap/intel-rapl:0:1/energy_uj")
    .expect("LogRocket: Should have been able to read the file");
    let file_cpu = fs::read_to_string("/sys/class/powercap/intel-rapl:0:0/energy_uj")
    .expect("LogRocket: Should have been able to read the file");
    let file_pkg = fs::read_to_string("/sys/class/powercap/intel-rapl:0/energy_uj")
    .expect("LogRocket: Should have been able to read the file");

    // Trim de string	
    let file_ram = file_ram.trim();
    let file_cpu = file_cpu.trim();
    let file_pkg = file_pkg.trim();

    // Conversion string en f64
    let ram_second = file_ram.parse::<f64>().unwrap();
    let cpu_second = file_cpu.parse::<f64>().unwrap();
    let pkg_second = file_pkg.parse::<f64>().unwrap();

    // Wattage 
    let watt_ram = (( ram_second - ram_first ) / 1000000.0) as f64;
    let watt_cpu = (( cpu_second - cpu_first ) / 1000000.0) as f64;
    let watt_pkg = (( pkg_second - pkg_first ) / 1000000.0) as f64;

    // f64 en string
    let var_ram = watt_ram.to_string();
    let var_cpu = watt_cpu.to_string();
    let var_pkg = watt_pkg.to_string();

    // Truncate string for 2 decimals	
    let mut new_ram = String::from(var_ram);
    if watt_ram > 10.0 {	
	new_ram.truncate(5);
    } else {
	new_ram.truncate(4);
    }

    // Truncate string for 2 decimals	
    let mut new_cpu = String::from(var_cpu);
    if watt_cpu > 10.0 {	
	new_cpu.truncate(5);
    } else {
	new_cpu.truncate(4);
    }

    // Truncate string for 2 decimals	
    let mut new_pkg = String::from(var_pkg);
	 if watt_pkg > 10.0 {	
	new_pkg.truncate(5);
    } else {
	new_pkg.truncate(4);
    }

    //std::env::set_var("WATT_RAM", "0");
    //std::env::set_var("WATT_CPU", "1");
    //std::env::set_var("WATT_PKG", "2");

    //Ecriture dans fichiers /tmp/conky
    let mut file_ram = std::fs::File::create("/tmp/conky/watt_ram").expect("create failed");
    file_ram.write_all(new_ram.as_bytes()).expect("write failed");
    let mut file_cpu = std::fs::File::create("/tmp/conky/watt_cpu").expect("create failed");
    file_cpu.write_all(new_cpu.as_bytes()).expect("write failed");
    let mut file_pkg = std::fs::File::create("/tmp/conky/watt_pkg").expect("create failed");
    file_pkg.write_all(new_pkg.as_bytes()).expect("write failed");

	println!("ram : {new_ram}");
	println!("cpu : {new_cpu}");
	println!("pkg : {new_pkg}");
    }
	
    
}
