use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn write_obj_file(words: &[u16]) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("rustvm-syscall-{unique}.obj"));
    let mut file = File::create(&path).expect("failed to create temp obj file");

    for word in words {
        file.write_all(&word.to_be_bytes())
            .expect("failed to write obj word");
    }

    path
}

fn run_vm(obj_path: &Path, stdin_data: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustvm"))
        .arg(obj_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rustvm");

    if !stdin_data.is_empty() {
        let stdin = child.stdin.as_mut().expect("failed to open child stdin");
        stdin
            .write_all(stdin_data.as_bytes())
            .expect("failed to write child stdin");
    }

    child.wait_with_output().expect("failed to wait rustvm output")
}

fn runtime_stdout(full_stdout: &str) -> &str {
    full_stdout
        .split_once("Executing now")
        .map(|(_, runtime)| runtime)
        .unwrap_or(full_stdout)
}

#[test]
fn syscall_out_prints_character_from_r0() {
    // x3000: LD R0, #2
    // x3001: TRAP x21 (OUT)
    // x3002: TRAP x25 (HALT)
    // x3003: 'A'
    let obj = write_obj_file(&[0x3000, 0x2002, 0xF021, 0xF025, 0x0041]);
    let output = run_vm(&obj, "");
    std::fs::remove_file(&obj).ok();

    assert!(
        output.status.success(),
        "process failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let runtime = runtime_stdout(&stdout);
    assert!(
        runtime.contains('A'),
        "expected runtime output to contain 'A', got:\n{runtime}"
    );
}

#[test]
fn syscall_getc_reads_character_into_r0() {
    // x3000: TRAP x20 (GETC)
    // x3001: TRAP x21 (OUT)  -- prints what GETC put in R0
    // x3002: TRAP x25 (HALT)
    let obj = write_obj_file(&[0x3000, 0xF020, 0xF021, 0xF025]);
    let output = run_vm(&obj, "Z");
    std::fs::remove_file(&obj).ok();

    assert!(
        output.status.success(),
        "process failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let runtime = runtime_stdout(&stdout);
    assert!(
        runtime.contains('Z'),
        "expected runtime output to contain echoed 'Z', got:\n{runtime}"
    );
}

#[test]
fn syscall_in_char_prompts_reads_and_echoes() {
    // x3000: TRAP x23 (IN)
    // x3001: TRAP x21 (OUT)  -- prints R0 again to prove IN stored it
    // x3002: TRAP x25 (HALT)
    let obj = write_obj_file(&[0x3000, 0xF023, 0xF021, 0xF025]);
    let output = run_vm(&obj, "k\n");
    std::fs::remove_file(&obj).ok();

    assert!(
        output.status.success(),
        "process failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let runtime = runtime_stdout(&stdout);
    let k_count = runtime.matches('k').count();
    assert!(
        k_count >= 2,
        "expected IN to echo 'k' and OUT to print it again; got:\n{runtime}"
    );
}

#[test]
fn syscall_putsp_prints_packed_string() {
    // x3000: LEA R0, #2      -- R0 -> x3003
    // x3001: TRAP x24 (PUTSP)
    // x3002: TRAP x25 (HALT)
    // x3003: 0x6948 ('H','i')
    // x3004: 0x0021 ('!')
    // x3005: 0x0000 (terminator)
    let obj = write_obj_file(&[0x3000, 0xE002, 0xF024, 0xF025, 0x6948, 0x0021, 0x0000]);
    let output = run_vm(&obj, "");
    std::fs::remove_file(&obj).ok();

    assert!(
        output.status.success(),
        "process failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let runtime = runtime_stdout(&stdout);
    assert!(
        runtime.contains("Hi!"),
        "expected packed string output \"Hi!\", got:\n{runtime}"
    );
}
