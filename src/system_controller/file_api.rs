use std::process::Command;


pub fn execute_script() -> Result<String, String> {
    let output = Command::new("sh")
        .arg("/Volumes/External SSD/code/InvictusRust/ovs-rust/test.sh")
        .output()
        .expect("failed to execute process");

    let status = output.status;

    if status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}