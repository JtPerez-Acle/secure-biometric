"""Analyze facial expressions using landmark positions and movements."""
import numpy as np
from dataclasses import dataclass
from typing import Dict, List, Optional
from enum import Enum

class Expression(Enum):
    """Enumeration of basic facial expressions."""
    NEUTRAL = "neutral"
    SMILE = "smile"
    FROWN = "frown"
    SURPRISE = "surprise"
    SQUINT = "squint"

@dataclass
class ExpressionFeatures:
    """Container for expression-related features."""
    primary_expression: Expression
    confidence: float
    expression_metrics: Dict[str, float]
    movement_metrics: Dict[str, float]

class ExpressionAnalyzer:
    """Analyzes facial expressions using landmark positions and movements."""
    
    def __init__(self, feature_indices: Dict[str, List[int]],
                 neutral_threshold: float = 0.1):
        """Initialize the expression analyzer.
        
        Args:
            feature_indices: Dictionary mapping feature names to landmark indices
            neutral_threshold: Threshold for considering an expression neutral
        """
        self.feature_indices = feature_indices
        self.neutral_threshold = neutral_threshold
        self.previous_landmarks = None
        
    def analyze_expression(self, landmarks: np.ndarray) -> ExpressionFeatures:
        """Analyze facial expression from landmarks.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            ExpressionFeatures object containing expression analysis
        """
        # Calculate expression metrics
        expression_metrics = self._calculate_expression_metrics(landmarks)
        
        # Calculate movement metrics if we have previous landmarks
        movement_metrics = self._calculate_movement_metrics(landmarks)
        
        # Determine primary expression
        primary_expression, confidence = self._classify_expression(
            expression_metrics, movement_metrics
        )
        
        # Update previous landmarks
        self.previous_landmarks = landmarks.copy()
        
        return ExpressionFeatures(
            primary_expression=primary_expression,
            confidence=confidence,
            expression_metrics=expression_metrics,
            movement_metrics=movement_metrics
        )
    
    def _calculate_expression_metrics(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Calculate metrics that characterize facial expression.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of expression metrics
        """
        metrics = {}
        
        # Mouth metrics
        mouth_points = landmarks[self.feature_indices['mouth']]
        metrics['mouth_openness'] = self._calculate_mouth_openness(mouth_points)
        metrics['mouth_width_ratio'] = self._calculate_mouth_width_ratio(mouth_points)
        
        # Eye metrics
        left_eye_points = landmarks[self.feature_indices['left_eye']]
        right_eye_points = landmarks[self.feature_indices['right_eye']]
        metrics['left_eye_openness'] = self._calculate_eye_openness(left_eye_points)
        metrics['right_eye_openness'] = self._calculate_eye_openness(right_eye_points)
        
        # Eyebrow metrics
        left_brow_points = landmarks[self.feature_indices['left_eyebrow']]
        right_brow_points = landmarks[self.feature_indices['right_eyebrow']]
        metrics['brow_height'] = self._calculate_brow_height(
            left_brow_points, right_brow_points,
            left_eye_points, right_eye_points
        )
        
        return metrics
    
    def _calculate_movement_metrics(self, landmarks: np.ndarray) -> Dict[str, float]:
        """Calculate metrics related to facial movement.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            
        Returns:
            Dictionary of movement metrics
        """
        metrics = {}
        
        if self.previous_landmarks is not None:
            # Calculate overall movement
            movement = landmarks - self.previous_landmarks
            metrics['total_movement'] = float(np.mean(np.linalg.norm(movement, axis=1)))
            
            # Calculate feature-specific movements
            for feature_name, indices in self.feature_indices.items():
                feature_movement = movement[indices]
                metrics[f'{feature_name}_movement'] = float(
                    np.mean(np.linalg.norm(feature_movement, axis=1))
                )
        
        return metrics
    
    def _classify_expression(self, expression_metrics: Dict[str, float],
                           movement_metrics: Dict[str, float]) -> tuple[Expression, float]:
        """Classify the facial expression based on metrics.
        
        Args:
            expression_metrics: Dictionary of expression metrics
            movement_metrics: Dictionary of movement metrics
            
        Returns:
            Tuple of (Expression enum, confidence score)
        """
        # Define expression characteristics
        characteristics = {
            Expression.SMILE: {
                'mouth_width_ratio': 1.2,
                'mouth_openness': 0.3,
                'brow_height': 0.0
            },
            Expression.FROWN: {
                'mouth_width_ratio': 0.8,
                'mouth_openness': -0.2,
                'brow_height': -0.2
            },
            Expression.SURPRISE: {
                'mouth_openness': 0.5,
                'brow_height': 0.3
            },
            Expression.SQUINT: {
                'left_eye_openness': -0.3,
                'right_eye_openness': -0.3,
                'brow_height': -0.1
            }
        }
        
        # Calculate match scores for each expression
        scores = {}
        for expression, chars in characteristics.items():
            score = self._calculate_expression_score(expression_metrics, chars)
            scores[expression] = score
        
        # Find best matching expression
        max_score = max(scores.values())
        if max_score < self.neutral_threshold:
            return Expression.NEUTRAL, 1.0 - max_score
        
        best_expression = max(scores.items(), key=lambda x: x[1])[0]
        return best_expression, max_score
    
    @staticmethod
    def _calculate_mouth_openness(mouth_points: np.ndarray) -> float:
        """Calculate relative mouth openness."""
        top_point = mouth_points[2]  # top middle point
        bottom_point = mouth_points[3]  # bottom middle point
        return float(np.linalg.norm(top_point - bottom_point))
    
    @staticmethod
    def _calculate_mouth_width_ratio(mouth_points: np.ndarray) -> float:
        """Calculate mouth width relative to neutral position."""
        left_point = mouth_points[0]
        right_point = mouth_points[1]
        return float(np.linalg.norm(right_point - left_point))
    
    @staticmethod
    def _calculate_eye_openness(eye_points: np.ndarray) -> float:
        """Calculate relative eye openness."""
        top_points = eye_points[1:4]  # upper eyelid points
        bottom_points = eye_points[4:7]  # lower eyelid points
        return float(np.mean(np.linalg.norm(top_points - bottom_points, axis=1)))
    
    @staticmethod
    def _calculate_brow_height(left_brow: np.ndarray, right_brow: np.ndarray,
                             left_eye: np.ndarray, right_eye: np.ndarray) -> float:
        """Calculate relative eyebrow height."""
        left_height = np.mean(left_brow[:, 1]) - np.mean(left_eye[:, 1])
        right_height = np.mean(right_brow[:, 1]) - np.mean(right_eye[:, 1])
        return float(np.mean([left_height, right_height]))
    
    @staticmethod
    def _calculate_expression_score(metrics: Dict[str, float],
                                  characteristics: Dict[str, float]) -> float:
        """Calculate how well metrics match expression characteristics."""
        differences = []
        for key, target in characteristics.items():
            if key in metrics:
                diff = abs(metrics[key] - target)
                differences.append(diff)
        
        if not differences:
            return 0.0
            
        return float(1.0 - np.mean(differences))
