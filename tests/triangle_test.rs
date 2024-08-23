use triangle::Triangle;

#[test]
fn test_valid_triangle() {
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    assert!(triangle.is_ok());
    let triangle = triangle.unwrap();
    assert_eq!(triangle.perimeter(), 12.0);
    assert!((triangle.area() - 6.0).abs() < 1e-10);
}

#[test]
fn test_invalid_triangle_zero_or_negative_sides() {
    assert!(Triangle::new(0.0, 4.0, 5.0).is_err());
    assert!(Triangle::new(-3.0, 4.0, 5.0).is_err());
    assert!(Triangle::new(3.0, 0.0, 5.0).is_err());
    assert!(Triangle::new(3.0, 4.0, -5.0).is_err());
}

#[test]
fn test_invalid_triangle_sum_of_two_sides_not_greater_than_third() {
    assert!(Triangle::new(1.0, 1.0, 2.0).is_err());
    assert!(Triangle::new(5.0, 5.0, 10.0).is_err());
}

#[test]
fn test_valid_triangle_edge_case() {
    let triangle = Triangle::new(1.0, 1.0, 1.9999999999);
    assert!(triangle.is_ok());
}

#[test]
fn test_degenerate_triangle() {
    assert!(Triangle::new(1.0, 2.0, 3.0).is_err());
}
