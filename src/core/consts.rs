use lazy_static::lazy_static;

lazy_static! {
    pub static ref JWT_SECRET: String = dotenvy::var("JWT_SECRET").expect("环境变量没有 JWT_SECRET");
    pub static ref JWT_KID: String = dotenvy::var("JWT_KID").expect("环境变量没有JWT_KID");
}
pub const JWT_LIVE: i64 = 60 * 60 * 24 * 7;
pub const JWT_EXPT: i64 = 60 * 60 * 24;

pub const UNAUTH_ROUTERS: [&str; 1] = [
    "/api/v1/auth/login",
];