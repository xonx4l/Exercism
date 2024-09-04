use time::Duration;
use time::PrimitiveDateTime as DateTime;

pub fn after(start_date: DateTime) -> DateTime {
    start_date + Duration::seconds(1_000_000_000)
}

#[test]

fn test_after() {
    assert_eq!(1_000_000_000);
}
