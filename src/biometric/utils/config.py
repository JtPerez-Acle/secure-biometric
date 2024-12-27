"""Configuration management for biometric analysis."""
import os
import json
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Tuple

@dataclass
class CaptureConfig:
    """Configuration for video capture and frame processing."""
    frame_width: int = 1280
    frame_height: int = 720
    fps: int = 30
    min_face_size: Tuple[int, int] = (30, 30)
    quality_threshold: float = 0.8
    change_threshold: float = 0.15

@dataclass
class AnalysisConfig:
    """Configuration for facial analysis."""
    landmark_model: str = "mediapipe"
    depth_resolution: Tuple[int, int] = (320, 240)
    mesh_quality: str = "medium"  # low, medium, high
    feature_confidence_threshold: float = 0.7

@dataclass
class StorageConfig:
    """Configuration for data storage."""
    base_dir: str = "output"
    max_session_age_days: int = 30
    compress_older_than_days: int = 7
    max_disk_usage_gb: float = 10.0

@dataclass
class VisualizationConfig:
    """Configuration for visualization."""
    colormap: str = "viridis"
    point_size: int = 2
    line_thickness: int = 1
    overlay_opacity: float = 0.6

@dataclass
class BiometricConfig:
    """Main configuration container."""
    capture: CaptureConfig
    analysis: AnalysisConfig
    storage: StorageConfig
    visualization: VisualizationConfig
    
    @classmethod
    def load(cls, config_path: str) -> 'BiometricConfig':
        """Load configuration from file.
        
        Args:
            config_path: Path to configuration file
            
        Returns:
            BiometricConfig object
        """
        if not os.path.exists(config_path):
            return cls.get_default()
            
        with open(config_path, 'r') as f:
            config_dict = json.load(f)
            
        return cls(
            capture=CaptureConfig(**config_dict.get('capture', {})),
            analysis=AnalysisConfig(**config_dict.get('analysis', {})),
            storage=StorageConfig(**config_dict.get('storage', {})),
            visualization=VisualizationConfig(**config_dict.get('visualization', {}))
        )
    
    def save(self, config_path: str):
        """Save configuration to file.
        
        Args:
            config_path: Path to save configuration
        """
        config_dict = {
            'capture': asdict(self.capture),
            'analysis': asdict(self.analysis),
            'storage': asdict(self.storage),
            'visualization': asdict(self.visualization)
        }
        
        os.makedirs(os.path.dirname(config_path), exist_ok=True)
        with open(config_path, 'w') as f:
            json.dump(config_dict, f, indent=4)
    
    @classmethod
    def get_default(cls) -> 'BiometricConfig':
        """Get default configuration.
        
        Returns:
            BiometricConfig object with default values
        """
        return cls(
            capture=CaptureConfig(),
            analysis=AnalysisConfig(),
            storage=StorageConfig(),
            visualization=VisualizationConfig()
        )
