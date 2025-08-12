@echo off
echo Starting full deployment (publish with --delete-data + generate)...
echo.

echo Step 1: Publishing to local SpacetimeDB server with --delete-data...
echo y | spacetime publish --server local justmaple --delete-data
echo Publish complete!
echo.

echo Step 2: Generating client code...
spacetime generate --lang csharp --out-dir ../JustMaple/Assets/autogen
echo Generation complete!
echo.

echo Full deployment complete!