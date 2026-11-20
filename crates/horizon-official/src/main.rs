//! The official game executable (docs/layers.md, "How a Horizon crate plugs in").
#![forbid(unsafe_code)]

#[global_allocator]
static ALLOCATOR: horizon_game::Allocator = horizon_game::ALLOCATOR;

/// The game identity marker used by current official Horizon Launcher imports.
static GAME_IDENTITY: [u8; 24] = *b"horizon-game-executable;";

/// Retain the legacy mod API marker for compatibility with older launcher builds.
static MOD_API_LEVEL: [u8; horizon_game::MOD_API_LEVEL.len()] = horizon_game::MOD_API_LEVEL;

fn main() {
    std::hint::black_box(&GAME_IDENTITY);
    std::hint::black_box(&MOD_API_LEVEL);
    horizon_official::register();
    horizon_game::main();
}
