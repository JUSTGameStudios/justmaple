# JustMaple Project

## Project Overview
JustMaple is a multiplayer Unity game based on the SpacetimeDB Unity tutorial (parts 1-4). It's an agar.io-style game where players control circles that can eat food and smaller circles to grow larger.

## Technology Stack
- **Unity 2022.3+** - Game engine and client
- **SpacetimeDB** - Real-time multiplayer database and server
- **C#** - Unity client code
- **Rust** - Server code
- **Universal Render Pipeline (URP)** - Unity rendering
- **Input System** - Unity's new input handling

## Project Structure

### Unity Client (`/JustMaple/`)
```
JustMaple/
├── Assets/
│   ├── Scripts/                 # Core client scripts
│   │   ├── GameManager.cs       # Main connection and game state management
│   │   ├── PlayerController.cs  # Local player input and management
│   │   ├── EntityController.cs  # Base class for game entities
│   │   ├── CircleController.cs  # Player circle behavior
│   │   ├── FoodController.cs    # Food entity behavior
│   │   ├── CameraController.cs  # Camera follow logic
│   │   ├── PrefabManager.cs     # Entity spawning management
│   │   └── Extensions.cs        # Utility extensions
│   ├── Prefabs/                 # Game object prefabs
│   │   ├── PlayerPrefab.prefab
│   │   ├── CirclePrefab.prefab
│   │   └── FoodPrefab.prefab
│   ├── Scenes/
│   │   └── SampleScene.unity    # Main game scene
│   └── autogen/                 # SpacetimeDB generated code
│       ├── Tables/              # Generated table classes
│       ├── Reducers/            # Generated reducer classes
│       └── Types/               # Generated type definitions
```

### SpacetimeDB Server (`/server-rust/`)
```
server-rust/
├── Cargo.toml                   # Rust project configuration
├── src/
│   ├── lib.rs                   # Main module entry point
│   ├── config/                  # Game configuration
│   │   ├── mod.rs               # Config module exports
│   │   └── game_constants.rs    # Game configuration constants
│   ├── reducers/                # Server game logic
│   │   ├── mod.rs               # Reducer module exports
│   │   ├── connection_reducers.rs # Player connection/disconnection
│   │   ├── game_reducers.rs     # Core game mechanics
│   │   └── player_reducers.rs   # Player-specific actions
│   ├── tables/                  # Database schema
│   │   ├── mod.rs               # Table module exports
│   │   ├── player.rs            # Player data table
│   │   ├── entity.rs            # Base entity table
│   │   ├── circle.rs            # Player circle table
│   │   ├── food.rs              # Food entity table
│   │   ├── config.rs            # Game configuration table
│   │   └── timers.rs            # Game timer tables
│   ├── types/                   # Custom types
│   │   ├── mod.rs               # Type module exports
│   │   └── db_vector2.rs        # Custom 2D vector type
│   └── utils/                   # Helper utilities
│       ├── mod.rs               # Utils module exports
│       ├── collision_detection.rs # Physics collision logic
│       ├── game_math.rs         # Game mathematics
│       └── random_extensions.rs # Random number utilities
├── generate.bat                 # Generate Unity client bindings
├── publish.bat                  # Build the SpacetimeDB module
└── deploy.bat                   # Deploy module to local instance
```

## Key Game Features
- **Real-time multiplayer** - Powered by SpacetimeDB
- **Circle-based gameplay** - Players control circles that can grow by eating
- **Food system** - Randomly spawned food items to consume
- **Player vs Player** - Larger circles can eat smaller ones
- **Arena boundaries** - Constrained world with visible borders
- **Smooth movement** - Client-side prediction with server authority

## Development Commands

### Server Development
From `/server-rust/`:
- `generate.bat` - Generate Unity client bindings from server schema
- `publish.bat` - Build the SpacetimeDB module
- `deploy.bat` - Deploy module to local SpacetimeDB instance

### Unity Development
- Open `/JustMaple/` in Unity Editor
- Run from editor for development
- Build standalone for distribution

## Network Architecture
- **Client-Server** - Unity client connects to SpacetimeDB server
- **Real-time sync** - All game state stored and synchronized via SpacetimeDB
- **Generated bindings** - Type-safe client code auto-generated from server schema
- **Event-driven** - Client reacts to database change events

## Configuration
- **Server URL**: `http://127.0.0.1:3000` (local development)
- **Module Name**: `justmaple`
- **World Size**: Configurable via server constants
- **Update Rates**: 
  - Player input: 20 Hz
  - Food spawning: Configurable interval
  - Player movement: Configurable interval

## Key Dependencies
- **SpacetimeDB SDK** - `com.clockworklabs.spacetimedbsdk`
- **Unity Input System** - New input handling
- **Universal Render Pipeline** - Modern Unity rendering
- **TextMeshPro** - Text rendering

## Development Notes
- The project follows SpacetimeDB best practices with clear separation between client and server
- Server logic is organized by domain (connection, game, player)
- Client uses Unity's modern systems (URP, Input System)
- Code generation creates type-safe client bindings from server schema
- Game uses server-authoritative physics and collision detection

## DOCUMENTATION-FIRST DEVELOPMENT PROTOCOL

Before writing any SpacetimeDB, Rapier2D, or Unity code, you MUST:
1. Use WebFetch to get current documentation for the APIs you're using
2. Cite specific documentation URLs in your response
3. Include inline comments with doc references
4. Verify API exists and usage is correct

Documentation Sources (always use latest):
- SpacetimeDB: https://docs.rs/spacetimedb/latest/spacetimedb/
- Rapier2D: https://docs.rs/rapier2d/latest/rapier2d/
- Unity: https://docs.unity3d.com/ScriptReference/index.html

Output Format:
- Start with "## API References" section listing docs fetched
- Include inline comments: // See: [URL] for [specific API]
- End with verification checklist

## Next Steps for Transformation
This project will be significantly enhanced with:
1. **Rapier2D integration** - Advanced 2D physics with Rust performance
2. **Server-authoritative platformer mechanics** - Full physics simulation control
3. **Enhanced Unity client** - Improved platformer controls and rendering

The current foundation provides a solid multiplayer game architecture that can be expanded into a more complex and feature-rich platformer experience.