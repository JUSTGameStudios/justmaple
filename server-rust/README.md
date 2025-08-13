# JustMaple Rust Server

This is the Rust implementation of the JustMaple SpacetimeDB server..

## Structure

```
server-rust/
├── Cargo.toml                 # Rust project configuration
├── src/
│   ├── lib.rs                 # Main module entry point
│   ├── config/
│   │   ├── mod.rs
│   │   └── game_constants.rs  # Game configuration constants
│   ├── tables/                # Database table definitions
│   │   ├── mod.rs
│   │   ├── player.rs          # Player and LoggedOutPlayer tables
│   │   ├── entity.rs          # Base entity table
│   │   ├── circle.rs          # Player circle table
│   │   ├── config.rs          # Game configuration table
│   │   └── timers.rs          # Scheduled timer tables
│   ├── reducers/              # Server game logic
│   │   ├── mod.rs
│   │   ├── connection_reducers.rs  # Player connection/disconnection
│   │   ├── game_reducers.rs        # Core game mechanics and timers
│   │   └── player_reducers.rs      # Player-specific actions
│   ├── types/                 # Custom data types
│   │   ├── mod.rs
│   │   └── db_vector2.rs      # 2D vector type for SpacetimeDB
│   └── utils/                 # Helper utilities
│       ├── mod.rs
│       ├── game_math.rs       # Game mathematics
│       ├── collision_detection.rs  # Physics collision logic
│       └── random_extensions.rs    # Random number utilities
├── publish.bat                # Publish module to SpacetimeDB
├── generate.bat               # Generate Unity client bindings
├── deploy.bat                 # Full deployment (publish + generate)
└── .editorconfig             # Code style configuration
```

## Features

- **Same directory structure**: Maintains familiar organization
- **Type-safe SpacetimeDB integration**: Uses Rust's type system for database operations
- **Performance optimized**: Benefits from Rust's zero-cost abstractions
- **Ready for Rapier2D**: Prepared for physics engine integration

## Prerequisites

1. Install Rust: https://rustup.rs/
2. Add WebAssembly target: `rustup target add wasm32-unknown-unknown`
3. Install SpacetimeDB CLI: https://spacetimedb.com/install

## Development Commands

- `publish.bat` - Publish module to local SpacetimeDB instance
- `generate.bat` - Generate Unity client bindings
- `deploy.bat` - Full deployment (publish with --delete-data + generate)
- `cargo check` - Check for compilation errors
- `cargo build --target wasm32-unknown-unknown` - Build WebAssembly module

## API References

All code follows the documentation-first development protocol:

- **SpacetimeDB**: https://docs.rs/spacetimedb/latest/spacetimedb/
- **Rapier2D**: https://docs.rs/rapier2d/latest/rapier2d/ (for future physics integration)
- **Unity**: https://docs.unity3d.com/ScriptReference/index.html

## Next Steps

This Rust server is ready for:
1. **Rapier2D physics integration** - Server-authoritative physics simulation
2. **Enhanced platformer mechanics** - Jump, gravity, collision responses
3. **Performance optimizations** - Rust's zero-cost abstractions
4. **Advanced game features** - More complex gameplay mechanics
