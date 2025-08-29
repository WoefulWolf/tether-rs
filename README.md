<h1 align="center">🦀➰ tether 🔗🦀</h1>
<h3 align="center">Drag & Drop DLL Proxy</h2>
<p align = "center">
Rust "port" of my <a href = "https://github.com/WoefulWolf/tether">C++ library.</a>
</p>

## Usage
```rust
#[allow(unused_imports)] // Optional :)
use tether;
```

## Proxies
| Library | Procedures |
| ----------------------------------- | ------------- |
| dxgi.dll  | CreateDXGIFactory <br> CreateDXGIFactory1 <br> CreateDXGIFactory2 |
| dinput8.dll  | DirectInput8Create |
| d3d11.dll  | D3D11CreateDevice <br> D3D11CreateDeviceAndSwapChain |
<br>
⚠️ Note: This library must be built using the MSVC toolchain.

## Why does this exist?
The repo exists so I can quickly and easily clone and include it in projects instead of having to type/copy the same stuff every time.

## What if it doesn't have the proxy I'm looking for?
Look at the existing examples and add it. I'll also be happy to merge pull requests.
