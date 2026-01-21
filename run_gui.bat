@echo off
echo ====================================
echo Hunter Sim - Multi-Hunter Optimizer
echo ====================================
echo.

cd /d "%~dp0"
cd hunter-sim

echo Starting Multi-Hunter GUI...
python gui_multi.py

if errorlevel 1 (
    echo.
    echo Error running the GUI. Make sure Python 3.10+ is installed.
    echo Try running: python --version
    pause
)
