use diary_cli::niceif;

#[test]
fn macro_niceif_true() {
    let result = niceif!(1 > 0, "more than", "not more than");
    assert_eq!(result, "more than");
}

#[test]
fn macro_niceif_false() {
    let result = niceif!(0 > 1, "more than", "not more than");
    assert_eq!(result, "not more than");
}

#[test]
fn macro_niceif_run_later() {
    let mut mutable = 0;
    niceif!(true, (), mutable += 1);
    assert_eq!(mutable, 0);
}