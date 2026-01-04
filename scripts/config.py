#!/usr/bin/env python3
"""
Project configuration loader
Centralizes all path configurations following DRY, ETC, and KISS principles
"""

import os
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:
    # Python < 3.11 fallback
    try:
        import tomli as tomllib
    except ImportError:
        print("Error: tomli package required for Python < 3.11. Install with: pip install tomli")
        sys.exit(1)

class ProjectConfig:
    def __init__(self, config_path="project.toml"):
        self.project_root = Path(__file__).parent.parent
        self.config_path = self.project_root / config_path
        self._config = self._load_config()
    
    def _load_config(self):
        """Load configuration from TOML file"""
        if not self.config_path.exists():
            raise FileNotFoundError(f"Configuration file not found: {self.config_path}")
        
        with open(self.config_path, "rb") as f:
            return tomllib.load(f)
    
    def get_path(self, key):
        """Get absolute path for a configuration key"""
        relative_path = self._config["paths"][key]
        return str(self.project_root / relative_path)
    
    def get_relative_path(self, key):
        """Get relative path for a configuration key"""
        return self._config["paths"][key]
    
    def get_build_config(self, key):
        """Get build configuration value"""
        return self._config["build"][key]
    
    def get_optimization_config(self, key):
        """Get optimization configuration value"""
        return self._config["optimization"][key]

# Global configuration instance
_config = None

def get_config():
    """Get the global configuration instance"""
    global _config
    if _config is None:
        _config = ProjectConfig()
    return _config

if __name__ == "__main__":
    # CLI interface for getting configuration values
    import argparse
    
    parser = argparse.ArgumentParser(description="Get project configuration values")
    parser.add_argument("key", help="Configuration key (e.g., articles_dir, assets_dir)")
    parser.add_argument("--relative", action="store_true", help="Return relative path instead of absolute")
    
    args = parser.parse_args()
    
    try:
        config = get_config()
        if args.relative:
            print(config.get_relative_path(args.key))
        else:
            print(config.get_path(args.key))
    except KeyError:
        print(f"Error: Configuration key '{args.key}' not found", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)