#!/bin/bash

echo "⏳ Downloading the Pomi (v1.0.0)..."
curl -fsSL -o /tmp/pomi.dmg "https://github.com/rios-pedro/pomi-app/releases/download/v1.0.0/pomi_1.0.0.dmg"

echo "📦 Installing in the Applications folder..."
hdiutil attach /tmp/pomi.dmg -nobrowse -quiet
cp -R "/Volumes/pomi/pomi.app" /Applications/
hdiutil detach "/Volumes/pomi" -quiet
rm /tmp/pomi.dmg

echo "✅ Pomi installed successfully!"
echo "👉 You can now open it from the Launchpad or Spotlight."