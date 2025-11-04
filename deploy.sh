#!/bin/bash
set -e  # exit on any error

TARGET_BRANCH="gh-pages"
BUILD_DIR="/home/runner/work/ecvlab.github.io/ecvlab.github.io/target/dx/ecvlab-github-io/release/web/public"
CNAME="ecvlab.davidfastovich.com"

# Build the Dioxus web bundle
dx bundle

# Ensure build directory exists
if [ ! -d "$BUILD_DIR" ]; then
    echo "Error: build directory '$BUILD_DIR' not found."
    exit 1
fi

# Set up a clean gh-pages branch in a temporary directory
git clone --depth 1 --branch $TARGET_BRANCH https://github.com/$GITHUB_REPOSITORY.git gh-pages

# Clear existing content
rm -rf gh-pages/*

# Copy new site files
cp -r $BUILD_DIR/* gh-pages/
echo "$CNAME" > gh-pages/CNAME

# Add a fallback 404 page for GitHub Pages
cp gh-pages/index.html gh-pages/404.html