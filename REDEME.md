---
title: "**Nebula**"
sub_title: Maturita 2025/26
author: Lukáš Valla
---

Nebula
===
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
# Hra
<!-- pause -->
* **bez** herního enginu
<!-- column: 1 -->
![](./assets/opengl.png)
<!-- column: 0 -->
<!-- pause -->
* v **Rustu**
<!-- column: 1 -->
![](./presentation/trpl.jpg)
<!-- column: 0 -->
<!-- pause -->
* s **multiplayerem**
<!-- pause -->
```toml
winit = "0.30.12"# Abstrakce
glium = "0.36.0" # OpenGl
image = "0.25.9" # Protokoly

serde_json = "1.0.145" # JSON
reqwest = "0.12.24" # Network
```
<!-- pause -->
\+ std
```rust
use std::*;
```
<!-- end_slide -->

Rust?
===
<!-- column_layout: [1, 2] -->
<!-- column: 0 -->
<!-- pause -->
![](./presentation/java.jpg)
<!-- column: 1 -->
<!-- pause -->
```rust +exec +id:rust_example
fn main() {
    let ptr: &mut usize;
    {
        let mut num = 3;
        ptr = &mut num;
    }
    println!("{}",*ptr);

}
```
<!-- reset_layout -->
<!-- snippet_output: rust_example -->
<!-- end_slide -->

Multiplayer
==
<!-- pause -->
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
![image:w:100%](./presentation/server_client.png)
<!-- pause -->
![](./presentation/minecraft.png)
<!-- column: 1 -->
<!-- pause -->
# Výhody
* Synchronizace
* Jednoduchost implementace
<!-- jump_to_middle -->
<!-- pause -->
# Nevýhody
* TPS == FPS
* Delay
<!-- pause -->
![](./presentation/boat_race.png)
<!-- reset_layout -->
