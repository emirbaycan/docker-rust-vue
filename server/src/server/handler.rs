use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use sysinfo::{Components, Disks, Networks, System};

pub async fn server_info() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    let mut serialized_processes = Vec::new();

    for (pid, process) in sys.processes() {
        let disk_usage = process.disk_usage();
        let serialized_process = serde_json::json!({
            "pid": pid.to_string(),
            "name": process.name(),
            "disk_usage": {
                "total": disk_usage.total_read_bytes,
                "used": disk_usage.total_written_bytes,
            },
        });
        serialized_processes.push(serialized_process);
    }

    let mut serialized_disks = Vec::new();
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let serialized_disk = serde_json::json!({
            "name": disk.name(),
            "mount_point": disk.mount_point(),
            "total_space": disk.total_space(),
            "available_space": disk.available_space(),
            // Add other fields as needed
        });
        serialized_disks.push(serialized_disk);
    }

    let mut serialized_networks = Vec::new();
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        let serialized_network = serde_json::json!({
            "interface_name": interface_name,
            "total_received": data.total_received(),
            "total_transmitted": data.total_transmitted(),
            // Add other fields as needed
        });
        serialized_networks.push(serialized_network);
    }

    let mut serialized_components = Vec::new();
    let components = Components::new_with_refreshed_list();
    for component in &components {
        let serialized_component = serde_json::json!({
            "label": component.label(),
            "temperature": component.temperature(),
            "max": component.max(),
            "critical": component.critical(),
        });
        serialized_components.push(serialized_component);
    }


    let mut serialized_cpus = Vec::new();
    sys.refresh_cpu();
    for cpu in sys.cpus() {
        let serialized_cpu = serde_json::json!({
            "cpu": cpu.cpu_usage()
        });
        serialized_cpus.push(serialized_cpu);
    }

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "memory":sys.total_memory(),
            "used_memory":sys.used_memory(),
            "total_swap":sys.total_swap(),
            "used_swap":sys.used_swap(),
            "name":System::name(),
            "kernel_version":System::kernel_version(),
            "os_version":System::os_version(),
            "host_name":System::host_name(),
            "cpu_number":sys.cpus().len(),
            "cpus":serialized_cpus,
            "process":serialized_processes,
            "disks":serialized_disks,
            "networks":serialized_networks,
            "components":serialized_components,
        })
    });
    Ok(Json(json_response))
}
