#!/usr/bin/env python3
"""
画像最適化スクリプト
ノードの物理演算パフォーマンス向上のため、author_img.pngを最適化します。
"""

from PIL import Image
import os
import argparse
import sys
from pathlib import Path

# Add scripts directory to path for config import
sys.path.insert(0, str(Path(__file__).parent))
from config import get_config

def optimize_author_image(input_dir=None, output_dir=None):
    # Use configuration if not provided
    if input_dir is None:
        config = get_config()
        input_dir = config.get_path("images_dir")
    
    if output_dir is None:
        output_dir = input_dir
    
    input_path = os.path.join(input_dir, "author_img.png")
    
    if not os.path.exists(input_path):
        print(f"Error: {input_path} not found")
        return
    
    # 出力ディレクトリを作成
    os.makedirs(output_dir, exist_ok=True)
    
    # 設定から最適化パラメータを取得
    try:
        config = get_config()
        webp_quality = config.get_optimization_config("webp_quality")
        small_size = config.get_optimization_config("small_image_size")
        medium_size = config.get_optimization_config("medium_image_size")
    except:
        # Fallback values if config is not available
        webp_quality = 85
        small_size = 64
        medium_size = 128
    
    # 元画像を読み込み
    with Image.open(input_path) as img:
        print(f"Original size: {img.size}, format: {img.format}")
        
        # 小さいバージョンを作成（ノード用）
        small_img = img.resize((small_size, small_size), Image.Resampling.LANCZOS)
        small_path = os.path.join(output_dir, "author_img_small.png")
        small_img.save(small_path, "PNG", optimize=True)
        print(f"Created small version ({small_size}x{small_size}): {small_path} ({os.path.getsize(small_path)} bytes)")
        
        # WebP形式でも保存（さらに小さく）
        webp_path = os.path.join(output_dir, "author_img_small.webp")
        small_img.save(webp_path, "WEBP", quality=webp_quality, optimize=True)
        print(f"Created WebP version: {webp_path} ({os.path.getsize(webp_path)} bytes)")
        
        # 中サイズも作成（将来の拡張用）
        medium_img = img.resize((medium_size, medium_size), Image.Resampling.LANCZOS)
        medium_path = os.path.join(output_dir, "author_img_medium.png")
        medium_img.save(medium_path, "PNG", optimize=True)
        print(f"Created medium version ({medium_size}x{medium_size}): {medium_path} ({os.path.getsize(medium_path)} bytes)")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Optimize images for portfolio")
    parser.add_argument("--input-dir", help="Input directory for images (uses config if not specified)")
    parser.add_argument("--output-dir", help="Output directory for optimized images (defaults to input-dir)")
    
    args = parser.parse_args()
    
    try:
        optimize_author_image(args.input_dir, args.output_dir)
        print("Image optimization completed successfully!")
    except ImportError:
        print("PIL (Pillow) not available. Please install with: pip install Pillow")
    except Exception as e:
        print(f"Error: {e}")