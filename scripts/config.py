#!/usr/bin/env python3
"""
Simple project configuration loader
Follows DRY, ETC, and KISS principles
"""

import os
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:
    try:
        import tomli as tomllib
    except ImportError:
        print("Error: tomli required for Python < 3.11. Install: pip install tomli")
        sys.exit(1)

class ProjectConfig:
    def __init__(self, config_path="project.toml"):
        self.project_root = Path(__file__).parent.parent
        self.config_path = self.project_root / config_path
        self._config = self._load_config()
    
    def _load_config(self):
        """Load configuration from TOML file"""
        if not self.config_path.exists():
            raise FileNotFoundError(f"Config not found: {self.config_path}")
        
        with open(self.config_path, "rb") as f:
            return tomllib.load(f)
    
    def get_path(self, key, relative=False):
        """Get path for a configuration key"""
        path = self._config["paths"][key]
        return path if relative else str(self.project_root / path)
    
    def get_config(self, section, key):
        """Get configuration value from any section"""
        return self._config[section][key]
    
    def get_public_url(self, mode=None):
        """Get public URL based on mode"""
        deployment = self._config["deployment"]
        if mode == "github-pages" or os.getenv("GITHUB_PAGES_MODE") == "1":
            return deployment["github_pages_path"]
        return deployment["local_dev_path"]

# Global config instance
_config = None

def get_config():
    global _config
    if _config is None:
        _config = ProjectConfig()
    return _config

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Get project configuration")
    parser.add_argument("key", help="Configuration key")
    parser.add_argument("--relative", action="store_true", help="Return relative path")
    parser.add_argument("--section", default="paths", help="Configuration section")
    parser.add_argument("--mode", choices=["local", "github-pages"], help="Mode for public_url")
    
    args = parser.parse_args()
    
    try:
        config = get_config()
        
        if args.key == "public_url":
            print(config.get_public_url(args.mode))
        elif args.section == "paths":
            print(config.get_path(args.key, args.relative))
        else:
            print(config.get_config(args.section, args.key))
    except (KeyError, FileNotFoundError) as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)