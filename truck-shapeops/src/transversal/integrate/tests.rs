use truck_meshalgo::prelude::*;
use truck_modeling::*;

#[test]
fn punched_cube() {
    let v = builder::vertex(Point3::origin());
    let e = builder::tsweep(&v, Vector3::unit_x());
    let f = builder::tsweep(&e, Vector3::unit_y());
    let cube: Solid = builder::tsweep(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(0.5, 0.25, -0.5));
    let w = builder::rsweep(
        &v,
        Point3::new(0.5, 0.5, 0.0),
        Vector3::unit_z(),
        Rad(7.0),
        3,
    );
    let f = builder::try_attach_plane(&[w]).unwrap();
    let mut cylinder = builder::tsweep(&f, Vector3::unit_z() * 2.0);
    cylinder.not();
    let and = crate::and(&cube, &cylinder, 0.05).unwrap();

    let poly = and.triangulation(0.01).to_polygon();
    let file = std::fs::File::create("punched-cube.obj").unwrap();
    obj::write(&poly, file).unwrap();
}

fn save_step(solid: &Solid, path: &str) {
    // compress solid data.
    let compressed = solid.compress();
    // step format display
    let display = truck_stepio::out::CompleteStepDisplay::new(truck_stepio::out::StepModel::from(&compressed), Default::default());
    // content of step file
    let step_string: String = display.to_string();
    std::fs::write(path, &step_string).unwrap();
}


#[test]
fn adjacent_cubes_or() {
    let v = builder::vertex(Point3::origin());
    let e = builder::tsweep(&v, Vector3::unit_x());
    let f = builder::tsweep(&e, Vector3::unit_y());
    let cube: Solid = builder::tsweep(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(0.0, 0.0, 1.0));
    let w = builder::tsweep(&v, Vector3::unit_x());
    let f = builder::tsweep(&w, Vector3::unit_y());
    let cube2: Solid = builder::tsweep(&f, Vector3::unit_z());

    let result = crate::or(&cube, &cube2, 0.05);
    match &result {
        Some(r) => {
            eprintln!("Result is Some");
            save_step(r, "/tmp/test.step");
        },
        None => eprintln!("Result is None"),
    }

    assert!(
        result.is_some(),
        "Boolean OR of adjacent cubes should succeed"
    );
}

#[test]
fn coplanar_faces_or() {
    let v = builder::vertex(Point3::origin());
    let e = builder::tsweep(&v, Vector3::unit_x());
    let f = builder::tsweep(&e, Vector3::unit_y());
    let cube: Solid = builder::tsweep(&f, Vector3::unit_z());
    let v = builder::vertex(Point3::new(-0.5, -0.5, 0.0));
    let w = builder::tsweep(&v, Vector3::unit_x());
    let f = builder::tsweep(&w, Vector3::unit_y());
    let cube2: Solid = builder::tsweep(&f, Vector3::unit_z() * 2.0);
    let result = crate::or(&cube, &cube2, 0.05);
    match &result {
        Some(_) => eprintln!("Result is Some"),
        None => eprintln!("Result is None"),
    }
    assert!(
        result.is_some(),
        "Boolean OR of coplanar faces should succeed"
    );
}
