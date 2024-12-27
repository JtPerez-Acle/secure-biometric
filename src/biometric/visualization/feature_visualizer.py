"""Visualize facial features and analysis results."""
import cv2
import numpy as np
from typing import Dict, List, Tuple, Optional
import plotly.graph_objects as go
from dataclasses import dataclass

@dataclass
class VisualizationConfig:
    """Configuration for feature visualization."""
    landmark_color: Tuple[int, int, int] = (0, 255, 0)
    connection_color: Tuple[int, int, int] = (255, 255, 255)
    text_color: Tuple[int, int, int] = (255, 255, 255)
    font_scale: float = 0.5
    line_thickness: int = 1
    point_size: int = 2

class FeatureVisualizer:
    """Visualizes facial features and analysis results."""
    
    def __init__(self, config: Optional[VisualizationConfig] = None):
        """Initialize the feature visualizer.
        
        Args:
            config: Optional visualization configuration
        """
        self.config = config or VisualizationConfig()
        
    def draw_landmarks(self, frame: np.ndarray, landmarks: np.ndarray,
                      connections: Optional[List[Tuple[int, int]]] = None) -> np.ndarray:
        """Draw facial landmarks and connections on frame.
        
        Args:
            frame: Input frame
            landmarks: Nx3 array of landmark coordinates
            connections: Optional list of landmark index pairs to connect
            
        Returns:
            Frame with landmarks drawn
        """
        vis_frame = frame.copy()
        
        # Draw connections
        if connections:
            for start_idx, end_idx in connections:
                start_point = tuple(landmarks[start_idx][:2].astype(int))
                end_point = tuple(landmarks[end_idx][:2].astype(int))
                cv2.line(vis_frame, start_point, end_point,
                        self.config.connection_color,
                        self.config.line_thickness)
        
        # Draw landmarks
        for point in landmarks:
            point_2d = tuple(point[:2].astype(int))
            cv2.circle(vis_frame, point_2d, self.config.point_size,
                      self.config.landmark_color, -1)
            
        return vis_frame
    
    def draw_face_metrics(self, frame: np.ndarray, metrics: Dict[str, float],
                         position: Tuple[int, int]) -> np.ndarray:
        """Draw facial metrics on frame.
        
        Args:
            frame: Input frame
            metrics: Dictionary of metrics to display
            position: (x, y) position to start drawing text
            
        Returns:
            Frame with metrics drawn
        """
        vis_frame = frame.copy()
        y_offset = position[1]
        
        for name, value in metrics.items():
            text = f"{name}: {value:.2f}"
            cv2.putText(vis_frame, text,
                       (position[0], y_offset),
                       cv2.FONT_HERSHEY_SIMPLEX,
                       self.config.font_scale,
                       self.config.text_color,
                       self.config.line_thickness)
            y_offset += 20
            
        return vis_frame
    
    def draw_expression(self, frame: np.ndarray, expression: str,
                       confidence: float, position: Tuple[int, int]) -> np.ndarray:
        """Draw expression classification on frame.
        
        Args:
            frame: Input frame
            expression: Detected expression
            confidence: Confidence score
            position: (x, y) position to draw text
            
        Returns:
            Frame with expression drawn
        """
        vis_frame = frame.copy()
        
        text = f"Expression: {expression} ({confidence:.2f})"
        cv2.putText(vis_frame, text,
                   position,
                   cv2.FONT_HERSHEY_SIMPLEX,
                   self.config.font_scale,
                   self.config.text_color,
                   self.config.line_thickness)
            
        return vis_frame
    
    def create_feature_dashboard(self, landmarks: np.ndarray,
                               metrics: Dict[str, Dict[str, float]]) -> go.Figure:
        """Create an interactive dashboard of facial features.
        
        Args:
            landmarks: Nx3 array of landmark coordinates
            metrics: Dictionary of feature metrics
            
        Returns:
            Plotly figure object
        """
        fig = go.Figure()
        
        # Add 3D scatter plot of landmarks
        fig.add_trace(go.Scatter3d(
            x=landmarks[:, 0],
            y=landmarks[:, 1],
            z=landmarks[:, 2],
            mode='markers',
            marker=dict(
                size=4,
                color='rgb(0, 255, 0)',
                opacity=0.8
            ),
            name='Landmarks'
        ))
        
        # Add metric plots
        for category, values in metrics.items():
            if isinstance(values, dict):
                fig.add_trace(go.Bar(
                    x=list(values.keys()),
                    y=list(values.values()),
                    name=category
                ))
        
        # Update layout
        fig.update_layout(
            title='Facial Feature Analysis',
            scene=dict(
                xaxis_title='X',
                yaxis_title='Y',
                zaxis_title='Z'
            ),
            height=800,
            showlegend=True
        )
        
        return fig
