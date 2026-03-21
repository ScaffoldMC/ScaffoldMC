<p align="center">
	<img src="./logo.svg" height="256">
	A self-hosted server platform for Minecraft.
</p>

> [!NOTE]  
> This project is under heavy development, and is **not ready for use**.
> Features stated below may not be implemented yet, not functional, or untested.

## Mission

Minecraft server panels leave a lot to be desired. Many panels have involved
installation and configuration processes, and none go far beyond basic server
management features like file and console access. While some panels are great
options for hosting multiple different games, secure isolation of servers, and
commercial hosting, very few are aimed toward the average savvy player who wants
an easy way to deploy and manage their own servers. Panels that do promise ease
of use often have unappealing user interfaces, are platform dependent, and/or
are paid.

ScaffoldMC aims to be **the most integrated server panel for Minecraft**. It
intends to go beyond generic server management, providing easy mod management,
one-click loader installation, and more - bringing the seamless experience of
third-party client launchers to the server.

## Building

### Prerequisites

You will need the following programs installed globally:

- Cargo
- Node.js 24
- [pnpm](https://pnpm.io/)
- [nx](https://www.npmjs.com/package/nx)

### Build

Run `nx build` to build both the frontend and backend.

To start the project in dev mode, `nx run-many -t dev` will start both the
frontend and backend.
