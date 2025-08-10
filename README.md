> [!NOTE]  
> This project is under heavy development, and is **not ready for use**.
> Features stated below may not be implemented yet, not functional, or untested.

# ScaffoldHost

A self-hosted Minecraft server panel for Minecraft: Java Edition.

## Mission

Minecraft server panels leave a lot to be desired. Many panels have involved
installation and configuration processes, and none go far beyond basic server
management features like file and console access. While some panels are great
options for hosting multiple different games, secure isolation of servers, and
commercial hosting, very few are aimed toward the average savvy player who wants
an easy way to deploy and manage their own servers. Panels that do promise ease
of use often have unappealing user interfaces, are platform dependent, and/or
are paid.

ScaffoldHost aims to be **the most integrated server panel for Minecraft**. It
intends to go beyond generic server management, providing easy mod management,
one-click loader installation, and more - bringing the seamless experience of
third-party client launchers to the server.

### Scope

This project is intended to be a server manager for Minecraft Java Servers and
Proxies. No other services are intended to be supported.

## Building

### Prerequisites

You will need the following programs installed globally:

- Cargo
- Node.js 22
- [pnpm](https://pnpm.io/)
- [nx](https://www.npmjs.com/package/nx)
- [just](https://just.systems/) (Optional)

### Build

Run `nx build` to build both the frontend and backend.
