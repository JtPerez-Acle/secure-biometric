import os
import json
import cv2
import numpy as np
from datetime import datetime
import pandas as pd

class NumpyJSONEncoder(json.JSONEncoder):
    """Custom JSON encoder for NumPy types."""
    def default(self, obj):
        if isinstance(obj, np.integer):
            return int(obj)
        if isinstance(obj, np.floating):
            return float(obj)
        if isinstance(obj, np.ndarray):
            return obj.tolist()
        return super().default(obj)

class BiometricDataHandler:
    """Handles storage and retrieval of biometric data."""
    
    def __init__(self, base_output_dir):
        """Initialize with base output directory."""
        self.base_dir = base_output_dir
        self.profiles_dir = os.path.join(base_output_dir, "profiles")
        self.temp_dir = os.path.join(base_output_dir, "temp")
        
        # Ensure directories exist
        for dir_path in [self.profiles_dir, self.temp_dir]:
            os.makedirs(dir_path, exist_ok=True)

    def _generate_timestamp(self):
        """Generate a timestamp for file naming."""
        return datetime.now().strftime("%Y%m%d_%H%M%S")

    def _convert_to_serializable(self, data):
        """Convert NumPy types to Python native types."""
        if isinstance(data, dict):
            return {k: self._convert_to_serializable(v) for k, v in data.items()}
        elif isinstance(data, list):
            return [self._convert_to_serializable(item) for item in data]
        elif isinstance(data, np.ndarray):
            return data.tolist()
        elif isinstance(data, (np.integer, np.floating)):
            return float(data)
        return data

    def save_biometric_profile(self, session_id, profile_data):
        """Save complete biometric profile including all essential data."""
        try:
            profile_path = os.path.join(self.profiles_dir, f"profile_{session_id}.json")
            
            # Structure the profile data
            profile = {
                "session_id": session_id,
                "timestamp": self._generate_timestamp(),
                "biometric_data": {
                    "landmarks": profile_data.get("landmarks", []),
                    "geometry": profile_data.get("geometry", {}),
                    "depth_features": profile_data.get("depth_features", {}),
                    "face_metrics": profile_data.get("face_metrics", {})
                },
                "metadata": {
                    "capture_duration": profile_data.get("capture_duration", 0),
                    "frame_count": profile_data.get("frame_count", 0),
                    "quality_score": profile_data.get("quality_score", 0)
                }
            }
            
            # Save the profile
            serializable_data = self._convert_to_serializable(profile)
            with open(profile_path, 'w') as f:
                json.dump(serializable_data, f, indent=4, cls=NumpyJSONEncoder)
            
            return profile_path
        except Exception as e:
            print(f"Error saving biometric profile: {str(e)}")
            raise

    def load_biometric_profile(self, session_id):
        """Load a complete biometric profile."""
        try:
            profile_path = os.path.join(self.profiles_dir, f"profile_{session_id}.json")
            if not os.path.exists(profile_path):
                return None
            
            with open(profile_path, 'r') as f:
                return json.load(f)
        except Exception as e:
            print(f"Error loading biometric profile: {str(e)}")
            return None

    def save_temp_data(self, session_id, data, data_type):
        """Save temporary data during analysis."""
        try:
            temp_path = os.path.join(self.temp_dir, f"{data_type}_{session_id}.json")
            serializable_data = self._convert_to_serializable(data)
            with open(temp_path, 'w') as f:
                json.dump(serializable_data, f, indent=4, cls=NumpyJSONEncoder)
            return temp_path
        except Exception as e:
            print(f"Error saving temporary data: {str(e)}")
            raise

    def clean_temp_data(self, session_id=None):
        """Clean temporary data files."""
        try:
            if session_id:
                # Clean specific session
                for file in os.listdir(self.temp_dir):
                    if session_id in file:
                        os.remove(os.path.join(self.temp_dir, file))
            else:
                # Clean all temp files
                for file in os.listdir(self.temp_dir):
                    os.remove(os.path.join(self.temp_dir, file))
        except Exception as e:
            print(f"Error cleaning temporary data: {str(e)}")
            raise

    def list_profiles(self):
        """List all available biometric profiles."""
        try:
            profiles = []
            for file in os.listdir(self.profiles_dir):
                if file.startswith("profile_") and file.endswith(".json"):
                    session_id = file.replace("profile_", "").replace(".json", "")
                    profile_path = os.path.join(self.profiles_dir, file)
                    with open(profile_path, 'r') as f:
                        profile = json.load(f)
                    profiles.append({
                        "session_id": session_id,
                        "timestamp": profile.get("timestamp", "Unknown"),
                        "frame_count": profile.get("metadata", {}).get("frame_count", 0),
                        "quality_score": profile.get("metadata", {}).get("quality_score", 0)
                    })
            return profiles
        except Exception as e:
            print(f"Error listing profiles: {str(e)}")
            return []

    def delete_profile(self, session_id):
        """Delete a specific biometric profile."""
        try:
            profile_path = os.path.join(self.profiles_dir, f"profile_{session_id}.json")
            if os.path.exists(profile_path):
                os.remove(profile_path)
                return True
            return False
        except Exception as e:
            print(f"Error deleting profile: {str(e)}")
            return False
