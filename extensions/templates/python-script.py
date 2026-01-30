#!/usr/bin/env python3
"""
Example Python script extension
Make executable: chmod +x python-script.py
"""

import os
import sys
from pathlib import Path

def main():
    # Get context from environment variables
    repo_root = Path(os.environ['DEVKIT_REPO_ROOT'])
    is_quiet = os.environ['DEVKIT_QUIET'] == '1'
    has_docker = os.environ['DEVKIT_FEATURE_DOCKER'] == '1'
    has_git = os.environ['DEVKIT_FEATURE_GIT'] == '1'
    has_cargo = os.environ['DEVKIT_FEATURE_CARGO'] == '1'
    has_node = os.environ['DEVKIT_FEATURE_NODE'] == '1'
    has_database = os.environ['DEVKIT_FEATURE_DATABASE'] == '1'

    print(f"🐍 Python script running in {repo_root}")

    if not is_quiet:
        print(f"✓ Docker: {'available' if has_docker else 'not available'}")
        print(f"✓ Git: {'available' if has_git else 'not available'}")
        print(f"✓ Cargo: {'available' if has_cargo else 'not available'}")
        print(f"✓ Node.js: {'available' if has_node else 'not available'}")
        print(f"✓ Database: {'configured' if has_database else 'not configured'}")

    # Your custom logic here
    print("Running custom Python workflow...")

    # Example: count Python files
    py_files = list(repo_root.rglob('*.py'))
    print(f"Found {len(py_files)} Python files")

    print("✓ Done!")
    return 0

if __name__ == '__main__':
    sys.exit(main())
