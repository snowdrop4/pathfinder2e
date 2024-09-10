#[allow(dead_code)]
#[derive(Debug)]
pub struct Currency(i64);

pub const fn pp(v: i64) -> Currency {
    gp(v * 10)
}

pub const fn gp(v: i64) -> Currency {
    sp(v * 10)
}

pub const fn sp(v: i64) -> Currency {
    cp(v * 10)
}

pub const fn cp(v: i64) -> Currency {
    Currency(v)
}
