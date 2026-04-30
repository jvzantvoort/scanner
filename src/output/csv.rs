use crate::output::helpers::get_port_info;
use crate::scanner::host::HostScanResult;
use csv::Writer;

pub fn format_csv(results: &[HostScanResult]) -> Result<String, csv::Error> {
    let mut wtr = Writer::from_writer(vec![]);

    // Write header
    wtr.write_record(&["IP", "Hostname", "Port", "State", "Service"])?;

    // Write data rows
    for result in results {
        let hostname = result.hostname.as_deref().unwrap_or("-");
        
        for (&port, &state) in &result.ports {
            let info = get_port_info(port);
            wtr.write_record(&[
                result.ip.to_string(),
                hostname.to_string(),
                port.to_string(),
                state.to_string(),
                info.service,
            ])?;
        }
    }

    wtr.flush()?;
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    Ok(data)
}
