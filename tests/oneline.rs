use oneline_eyre::eyre::{eyre, Report, WrapErr};

#[test]
fn ok() {
    oneline_eyre::install().unwrap();

    let error: Report = eyre!("cause");
    let wrapped: Result<(), _> = Err(error).wrap_err("middle").wrap_err("outer");

    let output = format!("{:?}", wrapped.unwrap_err());
    assert_eq!(output, "outer: middle: cause");
}
