"""Analyze geometric relationships and proportions in facial features."""
import numpy as np
from dataclasses import dataclass
from typing import Dict, List, Optional, Tuple
from scipy.spatial.transform import Rotation

@dataclass
class GeometricFeatures:
    """Container for geometric facial features."""
    proportions: Dict[str, float]
    head_pose: Dict[str, float]
    face_shape: Dict[str, float]
    contour_metrics: Dict[str, float]

class GeometryAnalyzer:
    """Analyzes geometric properties of facial features."""
    
    def __init__(self, feature_indices: Dict[str, List[int]]):
        """Initialize the geometry analyzer.
        
        Args:
            feature_indices: Dictionary mapping feature names to landmark indices
        """
        self.feature_indices = feature_indices
        
    def analyze_geometry(self, landmarks: np.ndarray) -> GeometricFeatures:
        """Analyze geometric properties of facial landmarks.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            GeometricFeatures object containing geometric analysis
        """
        return GeometricFeatures(
            proportions=self.calculate_proportions(landmarks),
            head_pose=self.estimate_head_pose(landmarks),
            face_shape=self.analyze_face_shape(landmarks),
            contour_metrics=self.analyze_face_contour(landmarks)
        )
    
    def calculate_proportions(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Calculate facial proportions and ratios.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of facial proportions
        """
        proportions = {}
        
        # Face width to height ratio
        contour_points = landmarks[self.feature_indices['face_contour']]
        face_width = np.max(contour_points[:, 0]) - np.min(contour_points[:, 0])
        face_height = np.max(contour_points[:, 1]) - np.min(contour_points[:, 1])
        proportions['width_height_ratio'] = float(face_width / face_height)
        
        # Eye spacing proportions
        left_eye = np.mean(landmarks[self.feature_indices['left_eye']], axis=0)
        right_eye = np.mean(landmarks[self.feature_indices['right_eye']], axis=0)
        eye_distance = np.linalg.norm(right_eye - left_eye)
        proportions['eye_spacing_ratio'] = float(eye_distance / face_width)
        
        # Nose proportions
        nose_bridge = landmarks[self.feature_indices['nose_bridge']]
        nose_length = np.linalg.norm(nose_bridge[-1] - nose_bridge[0])
        proportions['nose_face_ratio'] = float(nose_length / face_height)
        
        # Mouth proportions
        mouth_points = landmarks[self.feature_indices['mouth']]
        mouth_width = np.linalg.norm(mouth_points[0] - mouth_points[1])
        proportions['mouth_face_ratio'] = float(mouth_width / face_width)
        
        return proportions
    
    def estimate_head_pose(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Estimate 3D head pose from facial landmarks.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary containing head pose angles
        """
        # Define canonical points for pose estimation
        canonical_points = self._get_canonical_points()
        
        # Get corresponding points from landmarks
        actual_points = self._get_pose_points(landmarks)
        
        # Estimate rotation matrix
        rotation_matrix = self._estimate_rotation(actual_points, canonical_points)
        
        # Convert to Euler angles
        euler_angles = rotation_matrix.as_euler('xyz', degrees=True)
        
        return {
            'pitch': float(euler_angles[0]),  # up/down
            'yaw': float(euler_angles[1]),    # left/right
            'roll': float(euler_angles[2])    # tilt
        }
    
    def analyze_face_shape(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Analyze overall face shape characteristics.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of face shape metrics
        """
        shape_metrics = {}
        
        # Get face contour points
        contour = landmarks[self.feature_indices['face_contour']]
        
        # Calculate face shape metrics
        shape_metrics['jaw_width'] = float(
            np.linalg.norm(contour[0] - contour[-1])
        )
        
        # Calculate face roundness
        center = np.mean(contour, axis=0)
        distances = np.linalg.norm(contour - center, axis=1)
        shape_metrics['roundness'] = float(
            1.0 - np.std(distances) / np.mean(distances)
        )
        
        # Calculate jaw angle
        jaw_angle = self._calculate_jaw_angle(contour)
        shape_metrics['jaw_angle'] = float(jaw_angle)
        
        return shape_metrics
    
    def analyze_face_contour(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Analyze facial contour properties.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of contour metrics
        """
        contour_metrics = {}
        
        # Get face contour points
        contour = landmarks[self.feature_indices['face_contour']]
        
        # Calculate contour length
        contour_length = np.sum(
            np.linalg.norm(contour[1:] - contour[:-1], axis=1)
        )
        contour_metrics['length'] = float(contour_length)
        
        # Calculate contour smoothness
        smoothness = self._calculate_contour_smoothness(contour)
        contour_metrics['smoothness'] = float(smoothness)
        
        # Calculate contour symmetry
        symmetry = self._calculate_contour_symmetry(contour)
        contour_metrics['symmetry'] = float(symmetry)
        
        return contour_metrics
    
    @staticmethod
    def _get_canonical_points() -> np.ndarray:
        """Get canonical 3D points for pose estimation."""
        return np.array([
            [0.0, 0.0, 0.0],    # Nose tip
            [-1.0, 0.0, 0.0],   # Left eye
            [1.0, 0.0, 0.0],    # Right eye
            [0.0, -1.0, 0.0],   # Mouth center
            [0.0, 1.0, 0.0]     # Bridge of nose
        ])
    
    def _get_pose_points(self, landmarks: np.ndarray) -> np.ndarray:
        """Extract points for pose estimation from landmarks."""
        return np.array([
            landmarks[self.feature_indices['nose_tip']],
            np.mean(landmarks[self.feature_indices['left_eye']], axis=0),
            np.mean(landmarks[self.feature_indices['right_eye']], axis=0),
            np.mean(landmarks[self.feature_indices['mouth']], axis=0),
            landmarks[self.feature_indices['nose_bridge'][0]]
        ])
    
    @staticmethod
    def _estimate_rotation(points: np.ndarray, canonical_points: np.ndarray) -> Rotation:
        """Estimate rotation matrix from point correspondences."""
        # Center the points
        centered_points = points - np.mean(points, axis=0)
        centered_canonical = canonical_points - np.mean(canonical_points, axis=0)
        
        # Calculate rotation matrix using SVD
        H = centered_points.T @ centered_canonical
        U, _, Vt = np.linalg.svd(H)
        R = Vt.T @ U.T
        
        # Ensure proper rotation matrix
        if np.linalg.det(R) < 0:
            Vt[-1, :] *= -1
            R = Vt.T @ U.T
        
        return Rotation.from_matrix(R)
    
    @staticmethod
    def _calculate_jaw_angle(contour: np.ndarray) -> float:
        """Calculate jaw angle from contour points."""
        # Use last few points of each side to estimate jaw angle
        left_jaw = contour[:3]
        right_jaw = contour[-3:]
        
        left_vector = np.mean(np.diff(left_jaw, axis=0), axis=0)
        right_vector = np.mean(np.diff(right_jaw, axis=0), axis=0)
        
        angle = np.arccos(np.dot(left_vector, right_vector) /
                         (np.linalg.norm(left_vector) * np.linalg.norm(right_vector)))
        return np.degrees(angle)
    
    @staticmethod
    def _calculate_contour_smoothness(contour: np.ndarray) -> float:
        """Calculate smoothness of the contour."""
        # Calculate second derivatives along the contour
        second_derivatives = np.diff(contour, n=2, axis=0)
        curvature = np.linalg.norm(second_derivatives, axis=1)
        
        # Normalize and invert so higher values mean smoother
        return 1.0 / (1.0 + np.mean(curvature))
    
    @staticmethod
    def _calculate_contour_symmetry(contour: np.ndarray) -> float:
        """Calculate symmetry of the face contour."""
        # Find vertical axis
        center_line = np.mean(contour, axis=0)[0]
        
        # Split points into left and right
        left_points = contour[contour[:, 0] < center_line]
        right_points = contour[contour[:, 0] > center_line]
        
        # Mirror right points
        right_points_mirrored = right_points.copy()
        right_points_mirrored[:, 0] = 2 * center_line - right_points_mirrored[:, 0]
        
        # Calculate average distance between corresponding points
        min_points = min(len(left_points), len(right_points_mirrored))
        left_points = left_points[:min_points]
        right_points_mirrored = right_points_mirrored[:min_points]
        
        asymmetry = np.mean(np.linalg.norm(left_points - right_points_mirrored, axis=1))
        
        # Convert to symmetry score (1 = perfect symmetry, 0 = maximum asymmetry)
        face_width = np.max(contour[:, 0]) - np.min(contour[:, 0])
        return 1.0 - (asymmetry / face_width)
