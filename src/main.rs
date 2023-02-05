use docklink;
fn main() {
    let h = docklink::DockerBuilder::new().build();
    println!("{:?}", h.unwrap());
}