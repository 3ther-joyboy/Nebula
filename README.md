---
title: "**Nebulaal**"
event: Maturita 2025/26
author: Lukáš Valla

options:
    implicit_slide_ends: true
theme:
    name: tokyonight-night
    override:
        intro_slide:
            title: 
                font_size: 1
        slide_title:
            font_size: 1
        footer:
            style: template
            left: "{event}"
            center: "{current_slide} / {total_slides} "
            right: "{author}" 
            height: 1

---

Nebula
=
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
Rust?
=
<!-- column_layout: [1, 2] -->
<!-- column: 0 -->
<!-- pause -->
![](./presentation/java.jpg)
<!-- column: 1 -->
<!-- pause -->
```rust {2-7} +exec +id:rust_example
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
Multiplayer
=
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
* Jitter
<!-- pause -->
![](./presentation/boat_race.png)
<!-- reset_layout -->
Client
=
<!-- column_layout: [3, 2] -->
<!-- column: 0 -->
![image:w:100%](./presentation/client_display.png)
<!-- column: 1 -->
Jede na 2 vláknech.
<!-- pause -->
# Network
* "Kommpresor"
* Map parser
<!-- pause -->
# Display
* Okno (Winit)
* Texture buffer
* Animation buffer
* OpenGl (Glium)
    * Vertex shader
    * Fragment shader
<!-- pause -->
# Soubory
* JSON (serde_json)
* Obrázky (image)

Kam Dál?
=
<!-- pause -->
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
![image:w:80%](./presentation/smash_bros.jpg)
<!-- column: 1 -->
![image:w:100%](./presentation/brawhala.jpg)
<!-- column_layout: [1, 1] -->
<!-- pause -->
<!-- column: 0 -->
# Mám Hotové
<!-- column: 1 -->
# Musim dodělat
<!-- pause -->
<!-- column: 0 -->
* Server 
    * Networking
    * Event Loop
    * Permise
    * Inputy
    * Animace
        * Eventy
* Client 
    * Rendrování
    * Input Handeling
        * Input Map
* Ostatní 
    * Multi-threading 
    * Parsování 
    * Object Structure 

<!-- column: 1 -->
<!-- pause -->
* Loadování Map
* Fyzika 

* Soubory
    * Mapy
    * Charaktery
<!-- pause -->
## Kdybych se nudil
Dynamické objekty:
* Projektyly
* Power-Upy
* ...
