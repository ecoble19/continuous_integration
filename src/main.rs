use auto_deploy::dotnet::{BuildCmd, DeployCmd, TestCmd};
use std::path::PathBuf;
use auto_deploy::command::{Configuration, Cmd};
use auto_deploy::Project;

fn main() {
    let b = BuildCmd::default();
    let d = DeployCmd::new(PathBuf::from("/home/erick/rust/test_deploy"), Configuration::Release,
                           Some("netcoreapp3.1".to_owned()), false);
    let t = TestCmd::new(Configuration::Release, PathBuf::from("/home/erick/rust/test_results/output.xml"));

    let mut p = Project::new("LPP".to_owned(), "/home/erick/work/lpp");
    p.build_command(b.to_os_string())
        .deploy_command(d.to_os_string())
        .test_command(t.to_os_string());
    deploy(p);
}

fn deploy(mut p: Project) {
    match p.build() {
        Err(e) => panic!(e.as_ref().to_string()),
        _ => {}
    }
    match p.test() {
        Err(e) => panic!(e.as_ref().to_string()),
        _ => {}
    }

    p.change_dir(PathBuf::from("LoanPaymentPro"));
    match p.deploy() {
        Err(e) => panic!(e.as_ref().to_string()),
        _ => {}
    };
}
