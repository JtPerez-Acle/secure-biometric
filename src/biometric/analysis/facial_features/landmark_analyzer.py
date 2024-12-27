"""Analyze facial landmarks and their relationships."""
import numpy as np
from dataclasses import dataclass
from typing import Dict, List, Tuple, Optional

@dataclass
class LandmarkFeatures:
    """Container for landmark-based facial features."""
    distances: Dict[str, float]
    angles: Dict[str, float]
    ratios: Dict[str, float]
    symmetry: Dict[str, float]

class LandmarkAnalyzer:
    """Analyzes facial landmarks to extract geometric features."""
    
    def __init__(self, feature_indices: Dict[str, List[int]]):
        """Initialize the landmark analyzer.
        
        Args:
            feature_indices: Dictionary mapping feature names to landmark indices
        """
        self.feature_indices = feature_indices
        
    def calculate_distances(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Calculate key distances between facial landmarks.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of distance measurements
        """
        distances = {}
        
        # Eye width measurements
        distances['left_eye_width'] = self._point_distance(
            landmarks[self.feature_indices['left_eye'][0]],
            landmarks[self.feature_indices['left_eye'][-1]]
        )
        distances['right_eye_width'] = self._point_distance(
            landmarks[self.feature_indices['right_eye'][0]],
            landmarks[self.feature_indices['right_eye'][-1]]
        )
        
        # Eye separation
        left_eye_center = np.mean(landmarks[self.feature_indices['left_eye']], axis=0)
        right_eye_center = np.mean(landmarks[self.feature_indices['right_eye']], axis=0)
        distances['eye_separation'] = self._point_distance(left_eye_center, right_eye_center)
        
        # Nose measurements
        nose_tip = landmarks[self.feature_indices['nose_tip']]
        nose_bridge_top = landmarks[self.feature_indices['nose_bridge'][0]]
        distances['nose_height'] = self._point_distance(nose_tip, nose_bridge_top)
        
        # Mouth measurements
        mouth_points = landmarks[self.feature_indices['mouth']]
        distances['mouth_width'] = self._point_distance(
            mouth_points[0],  # left corner
            mouth_points[1]   # right corner
        )
        
        return distances
    
    def calculate_angles(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Calculate important angles between facial features.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of angle measurements in degrees
        """
        angles = {}
        
        # Eye angles
        left_eye_angle = self._calculate_feature_angle(
            landmarks[self.feature_indices['left_eye']]
        )
        right_eye_angle = self._calculate_feature_angle(
            landmarks[self.feature_indices['right_eye']]
        )
        angles['left_eye_angle'] = left_eye_angle
        angles['right_eye_angle'] = right_eye_angle
        
        # Eyebrow angles
        left_brow_angle = self._calculate_feature_angle(
            landmarks[self.feature_indices['left_eyebrow']]
        )
        right_brow_angle = self._calculate_feature_angle(
            landmarks[self.feature_indices['right_eyebrow']]
        )
        angles['left_eyebrow_angle'] = left_brow_angle
        angles['right_eyebrow_angle'] = right_brow_angle
        
        # Nose bridge angle
        nose_bridge_points = landmarks[self.feature_indices['nose_bridge']]
        angles['nose_bridge_angle'] = self._calculate_vertical_angle(nose_bridge_points)
        
        return angles
    
    def calculate_ratios(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Calculate facial proportion ratios.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of facial ratios
        """
        distances = self.calculate_distances(landmarks)
        
        ratios = {
            'eye_width_ratio': distances['left_eye_width'] / distances['right_eye_width'],
            'eye_separation_ratio': distances['eye_separation'] / distances['mouth_width'],
            'nose_mouth_ratio': distances['nose_height'] / distances['mouth_width']
        }
        
        return ratios
    
    def analyze_symmetry(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Analyze facial symmetry using landmarks.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of symmetry metrics
        """
        symmetry = {}
        
        # Calculate facial midline
        nose_bridge = landmarks[self.feature_indices['nose_bridge']]
        midline = self._calculate_facial_midline(nose_bridge)
        
        # Measure feature distances from midline
        symmetry['eye_symmetry'] = self._calculate_feature_symmetry(
            landmarks[self.feature_indices['left_eye']],
            landmarks[self.feature_indices['right_eye']],
            midline
        )
        
        symmetry['eyebrow_symmetry'] = self._calculate_feature_symmetry(
            landmarks[self.feature_indices['left_eyebrow']],
            landmarks[self.feature_indices['right_eyebrow']],
            midline
        )
        
        return symmetry
    
    def extract_features(self, landmarks: np.ndarray) -> LandmarkFeatures:
        """Extract all landmark-based features.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            LandmarkFeatures object containing all computed features
        """
        return LandmarkFeatures(
            distances=self.calculate_distances(landmarks),
            angles=self.calculate_angles(landmarks),
            ratios=self.calculate_ratios(landmarks),
            symmetry=self.analyze_symmetry(landmarks)
        )
    
    @staticmethod
    def _point_distance(p1: np.ndarray, p2: np.ndarray) -> float:
        """Calculate Euclidean distance between two points."""
        return float(np.linalg.norm(p1 - p2))
    
    @staticmethod
    def _calculate_feature_angle(points: np.ndarray) -> float:
        """Calculate the angle of a feature relative to horizontal."""
        direction = points[-1] - points[0]
        angle = np.arctan2(direction[1], direction[0])
        return float(np.degrees(angle))
    
    @staticmethod
    def _calculate_vertical_angle(points: np.ndarray) -> float:
        """Calculate angle relative to vertical axis."""
        direction = points[-1] - points[0]
        angle = np.arctan2(direction[0], direction[1])  # relative to vertical
        return float(np.degrees(angle))
    
    @staticmethod
    def _calculate_facial_midline(nose_bridge: np.ndarray) -> Tuple[np.ndarray, np.ndarray]:
        """Calculate facial midline from nose bridge points."""
        direction = nose_bridge[-1] - nose_bridge[0]
        normalized_direction = direction / np.linalg.norm(direction)
        midpoint = (nose_bridge[-1] + nose_bridge[0]) / 2
        return midpoint, normalized_direction
    
    def _calculate_feature_symmetry(self, left_points: np.ndarray, 
                                  right_points: np.ndarray,
                                  midline: Tuple[np.ndarray, np.ndarray]) -> float:
        """Calculate symmetry score for paired features."""
        midpoint, direction = midline
        
        # Calculate distances to midline
        left_distances = self._point_set_line_distance(left_points, midpoint, direction)
        right_distances = self._point_set_line_distance(right_points, midpoint, direction)
        
        # Compare distance distributions
        distance_diff = np.mean(np.abs(left_distances - right_distances))
        max_distance = np.max(np.concatenate([left_distances, right_distances]))
        
        # Normalize symmetry score (0 = perfect symmetry, 1 = maximum asymmetry)
        symmetry_score = distance_diff / max_distance if max_distance > 0 else 0
        return float(symmetry_score)
    
    @staticmethod
    def _point_set_line_distance(points: np.ndarray, line_point: np.ndarray,
                               line_direction: np.ndarray) -> np.ndarray:
        """Calculate distances from points to a line."""
        vectors = points - line_point
        projections = np.dot(vectors, line_direction)
        projected_points = line_point + np.outer(projections, line_direction)
        return np.linalg.norm(points - projected_points, axis=1)
