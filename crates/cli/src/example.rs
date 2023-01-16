#![allow(clippy::clone_on_copy)]
#![allow(clippy::let_unit_value)]
use ascent::ascent;

ascent! {
    relation edge(i32, i32);
    relation path(i32, i32);

    path(x, y) <-- edge(x, y);
    path(x, z) <-- edge(x, y), path(y, z);
}

pub(crate) fn run(edges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // let mut prog = AscentProgram::default();
    // prog.edge = vec![(1, 2), (2, 3)];
    let mut prog = AscentProgram {
        edge: edges,
        ..Default::default()
    };
    prog.run();

    prog.path
}
