# Tex Net: High-Performance Networking for Multiplayer Games

> Implementation of [WebTransport](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport) in Rust
***Tex Net*** brings a revolutionary networking approach to multiplayer games, combining Rust's performance and safety with advanced protocols QUIC and WebTransport. This blend results in a network solution that is not only fast and reliable but also tailored to handle the high throughput and real-time demands of multiplayer gaming environments.

At the heart of Tex Net's architecture lies a client-server model optimized for low latency and high concurrency. QUIC's connection-oriented yet streamlined nature allows for rapid packet delivery, essential for real-time interaction, while its in-built security features maintain a protected gaming environment. WebTransport complements this by enabling efficient transport of multiple data streams over a single connection, reducing overhead and improving responsiveness. Whether you're creating an action-packed FPS or a complex MMO, Tex Net is designed to scale seamlessly, providing an uninterrupted and engaging gaming experience for players worldwide.

Tex Net is specifically designed for WebGL and WebGPU multiplier games running in the browser. 

# Setup

- Install [xxd](https://www.tutorialspoint.com/unix_commands/xxd.htm) 
- `chmod +x ./cert/generate.sh` - Modify permissions to make generate executable
- `sh cert/generate.sh`
- `./cert/generate.sh` - Generate certificate (.crt & .key)


# What is Tex Net?

Tex net is a networking solution for multiplier games. It's built in rust and utilizes QUIC and webtransport for fast and scalable multiplier.


# References

- https://datatracker.ietf.org/doc/html/draft-frindell-webtrans-devious-baton
