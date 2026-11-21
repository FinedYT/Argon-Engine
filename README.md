# Argon-Engine

An open-source 3D graphics engine written in Rust.

## Tech Stack
- **Graphics:** [wgpu](https://github.com/gfx-rs/wgpu)
- **Windowing:** [winit](https://github.com/rust-windowing/winit)
- **Math:** [glam](https://github.com/bitshifter/glam-rs)

## Getting Started

### Prerequisites
- [Rust](https://rust-lang.org) (stable toolchain)
- A GPU supporting Vulkan, DirectX 12, or Metal

### Build and Run
```bash
git clone https://github.com/FinedYT/Argon-Engine.git
cd Argon-Engine
cargo run
```

## Roadmap
- [x] Window creation and event loop (`winit`)
- [x] GPU context initialization and surface clearing (`wgpu`)
- [x] Basic shaders and render pipeline (WGSL)
- [ ] Vertex and index buffers
- [ ] 3D perspective camera and transforms (`glam`)
- [ ] Depth buffer (Z-buffer)
- [ ] Frame timing (`DeltaTime`) and keyboard/mouse input
- [ ] Textures and basic lighting

## License
This project is licensed under the [MIT License](LICENSE).
