use petra_frontend::{Frontend, PetraFrontendError};

#[test]
fn check_error() {
    let test_data = "dsadsadsa;dsa;dsa;d;sa".to_string();
    let frontend = Frontend::default();
    let res: Result<petra_core::Document, PetraFrontendError> =
        frontend.parse(test_data.as_bytes());
    assert!(
        matches!(res, Err(PetraFrontendError::ParseError(_))),
        "result is not the correct error: {res:?}"
    );
}
#[test]
fn check_sanity() {
    let test_data = r#"
    /* rem
    ark */
    // cfir
    // cfir2
    cfir:string   ="cfir";
    "#
    .to_string();
    let frontend: Frontend = Frontend::default();
    let res: Result<petra_core::Document, PetraFrontendError> =
        frontend.parse(test_data.as_bytes());
    assert!(res.is_ok(), "got an error: {res:?}");
}
