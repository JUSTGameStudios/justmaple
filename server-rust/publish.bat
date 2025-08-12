@echo off
echo Publishing Rust server to local SpacetimeDB...
echo.
choice /C YN /M "Do you want to use --delete-data flag (this will clear all data)?"
if errorlevel 2 (
    echo Publishing WITHOUT deleting data...
    spacetime publish --server local justmaple
) else (
    echo Publishing WITH --delete-data flag...
    echo y | spacetime publish --server local justmaple --delete-data
)
echo.
echo Publish complete!