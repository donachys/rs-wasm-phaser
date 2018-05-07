mod js;
extern{
    pub fn phaser();
}

#[no_mangle]
pub fn call_js() {
    js::alert("Hello, world!");
    js::log("Hello, world!");
    unsafe{phaser();}
}
