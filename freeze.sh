#!/usr/bin/env bash
pyinstaller --onefile --console --exclude-module tkinter -n image_classifier main.py