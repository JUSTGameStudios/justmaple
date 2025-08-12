@echo off
echo Generating Unity client bindings from Rust server...
spacetime generate --lang csharp --out-dir ../JustMaple/Assets/autogen
echo Generation complete!