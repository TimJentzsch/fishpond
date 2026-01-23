# fishpond

![License: AGPL-3.0](https://img.shields.io/github/license/TimJentzsch/fishpond)
![CI status](https://img.shields.io/github/actions/workflow/status/TimJentzsch/fishpond/ci.yml?label=CI)
[![Bevy version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FTimJentzsch%2Ffishpond%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.workspace.dependencies.bevy&prefix=v&label=bevy)](https://bevy.org/)

Fishpond is a chess GUI targeted at amateur chess engine developers.

> [!WARNING]
>
> **This project is very much work in progress and not really usable yet!**

## Vision

Fishpond aims to serve as a GUI for amateur chess engine development.

- **Semantic logs**: UCI logs with syntax highlighting, visual preview of lines and error tracking.
- **Interactive debugging**: Query your engine on how it currently evaluates a given line (without influencing the search), visualize bitboards and evaluations by square.
- **Performance evaluation**: Compare your engine's strength against [Stockfish](https://stockfishchess.org/) or other engines, with automatic diluting to make it a fair fight.
- **Version management**: Create a snapshot of your current build and compare it against other versions of your engine.
- **Online play**: Face other engines (and humans) on [Lichess](https://lichess.org/) with a [bot account](https://lichess.org/@/lichess/blog/welcome-lichess-bots/WvDNticA).

## License

Contrary to most Rust/Bevy projects, this project is licensed under [**GNU Affero General Public License v3**](LICENSE-AGPL) or later.
This is in alignment with the general chess open-source ecosystem, like [Lichess](https://lichess.org/).
It also allows using more libraries from that ecosystem.

### Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
