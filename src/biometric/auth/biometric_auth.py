import numpy as np
from pathlib import Path
import json

class BiometricAuthenticator:
    """Handles biometric authentication using facial features."""

    def __init__(self, tolerance_thresholds=None):
        """Initialize with custom tolerance thresholds."""
        self.tolerance_thresholds = tolerance_thresholds or {
            # Distance measurements (in pixels)
            "eye_distance": 0.1,        # 10% variation allowed
            "mouth_width": 0.15,        # 15% variation allowed
            "nose_to_chin": 0.1,        # 10% variation allowed
            
            # Facial ratios (absolute differences)
            "width_ratio_temple_jaw": 0.05,
            "width_ratio_cheek_jaw": 0.05,
            "height_ratio_nose_face": 0.05,
            "face_width_height_ratio": 0.05,
            
            # Angles (in degrees)
            "nose_angle": 5.0,
            "left_eye_angle": 5.0,
            "right_eye_angle": 5.0,
            "mouth_symmetry_angle": 5.0,
            
            # Head pose (in degrees)
            "pitch": 15.0,
            "yaw": 15.0,
            "roll": 15.0,
            
            # Color variation (BGR values, 0-255)
            "color_tolerance": 25.0
        }

    def create_biometric_profile(self, session_data):
        """Create a biometric profile from session data."""
        profile = {
            "geometry": self._process_geometry_data(session_data.get("geometries", [])),
            "pose": self._process_pose_data(session_data.get("poses", [])),
            "color": self._process_color_data(session_data.get("colors", []))
        }
        return profile

    def _process_geometry_data(self, geometry_data):
        """Process geometric measurements into a profile."""
        if not geometry_data:
            return None

        # Convert list of measurements to numpy array for each metric
        metrics = {}
        for metric in geometry_data[0].keys():
            values = [frame[metric] for frame in geometry_data]
            metrics[metric] = {
                "mean": float(np.mean(values)),
                "std": float(np.std(values)),
                "min": float(np.min(values)),
                "max": float(np.max(values))
            }
        return metrics

    def _process_pose_data(self, pose_data):
        """Process head pose data into a profile."""
        if not pose_data:
            return None

        pose_metrics = {}
        for axis in ["pitch", "yaw", "roll"]:
            values = [frame[axis] for frame in pose_data]
            pose_metrics[axis] = {
                "mean": float(np.mean(values)),
                "std": float(np.std(values)),
                "range": float(np.ptp(values))
            }
        return pose_metrics

    def _process_color_data(self, color_data):
        """Process color data into a profile."""
        if not color_data:
            return None

        color_array = np.array(color_data)
        return {
            "mean": np.mean(color_array, axis=0).tolist(),
            "std": np.std(color_array, axis=0).tolist()
        }

    def verify_biometric(self, profile, current_data):
        """Verify current biometric data against a stored profile."""
        if not profile or not current_data:
            return False, {"error": "Missing profile or current data"}

        results = {
            "geometry": self._verify_geometry(
                profile.get("geometry"), 
                current_data.get("geometry")
            ),
            "pose": self._verify_pose(
                profile.get("pose"), 
                current_data.get("pose")
            ),
            "color": self._verify_color(
                profile.get("color"), 
                current_data.get("color")
            )
        }

        # Calculate overall match score
        match_scores = []
        for category, result in results.items():
            if isinstance(result, dict) and "match_score" in result:
                match_scores.append(result["match_score"])

        overall_score = np.mean(match_scores) if match_scores else 0.0
        
        return overall_score >= 0.8, {
            "overall_score": overall_score,
            "details": results
        }

    def _verify_geometry(self, profile_geometry, current_geometry):
        """Verify geometric measurements."""
        if not profile_geometry or not current_geometry:
            return {"match_score": 0.0, "error": "Missing geometry data"}

        matches = []
        for metric, profile_values in profile_geometry.items():
            if metric not in current_geometry:
                continue

            current_value = current_geometry[metric]
            threshold = self.tolerance_thresholds.get(metric, 0.1)
            
            # Calculate normalized difference
            diff = abs(current_value - profile_values["mean"])
            max_allowed_diff = profile_values["mean"] * threshold
            
            # Calculate match score for this metric
            match_score = max(0.0, 1.0 - (diff / max_allowed_diff))
            matches.append(match_score)

        return {
            "match_score": np.mean(matches) if matches else 0.0,
            "metric_scores": dict(zip(profile_geometry.keys(), matches))
        }

    def _verify_pose(self, profile_pose, current_pose):
        """Verify head pose."""
        if not profile_pose or not current_pose:
            return {"match_score": 0.0, "error": "Missing pose data"}

        matches = []
        for axis in ["pitch", "yaw", "roll"]:
            if axis not in current_pose or axis not in profile_pose:
                continue

            threshold = self.tolerance_thresholds[axis]
            diff = abs(current_pose[axis] - profile_pose[axis]["mean"])
            match_score = max(0.0, 1.0 - (diff / threshold))
            matches.append(match_score)

        return {
            "match_score": np.mean(matches) if matches else 0.0,
            "axis_scores": dict(zip(["pitch", "yaw", "roll"], matches))
        }

    def _verify_color(self, profile_color, current_color):
        """Verify color match."""
        if not profile_color or not current_color:
            return {"match_score": 0.0, "error": "Missing color data"}

        threshold = self.tolerance_thresholds["color_tolerance"]
        color_diff = np.linalg.norm(
            np.array(current_color) - np.array(profile_color["mean"])
        )
        
        match_score = max(0.0, 1.0 - (color_diff / (threshold * np.sqrt(3))))
        
        return {
            "match_score": match_score,
            "color_difference": float(color_diff)
        }

    def save_profile(self, profile, filepath):
        """Save biometric profile to file."""
        Path(filepath).parent.mkdir(parents=True, exist_ok=True)
        with open(filepath, 'w') as f:
            json.dump(profile, f, indent=4)

    def load_profile(self, filepath):
        """Load biometric profile from file."""
        with open(filepath, 'r') as f:
            return json.load(f)
