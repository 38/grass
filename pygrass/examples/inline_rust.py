from pygrass import RustEnv

RustEnv().inline_rust("""
    println!("Hello World");
""")