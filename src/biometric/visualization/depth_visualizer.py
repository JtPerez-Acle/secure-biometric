"""Visualize depth maps and 3D facial features."""
import cv2
import numpy as np
import plotly.graph_objects as go
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass

@dataclass
class DepthVisualizationConfig:
    """Configuration for depth visualization."""
    colormap: int = cv2.COLORMAP_VIRIDIS
    alpha: float = 0.6
    point_size: int = 2
    mesh_opacity: float = 0.7
    surface_colorscale: str = 'Viridis'

class DepthVisualizer:
    """Visualizes depth maps and 3D facial features."""
    
    def __init__(self, config: Optional[DepthVisualizationConfig] = None):
        """Initialize the depth visualizer.
        
        Args:
            config: Optional visualization configuration
        """
        self.config = config or DepthVisualizationConfig()
    
    def create_depth_overlay(self, frame: np.ndarray, depth_map: np.ndarray,
                           mask: Optional[np.ndarray] = None) -> np.ndarray:
        """Create a colored depth overlay on the frame.
        
        Args:
            frame: Input frame
            depth_map: Depth map array
            mask: Optional mask for valid depth values
            
        Returns:
            Frame with depth overlay
        """
        # Ensure depth map matches frame dimensions
        if depth_map.shape[:2] != frame.shape[:2]:
            depth_map = cv2.resize(depth_map, (frame.shape[1], frame.shape[0]))
        
        # Normalize depth values
        valid_mask = ~np.isnan(depth_map) if mask is None else mask
        if np.any(valid_mask):
            depth_norm = np.zeros_like(depth_map)
            depth_norm[valid_mask] = cv2.normalize(
                depth_map[valid_mask],
                None,
                0,
                255,
                cv2.NORM_MINMAX
            )
            
            # Apply colormap
            depth_colored = cv2.applyColorMap(
                depth_norm.astype(np.uint8),
                self.config.colormap
            )
            
            # Create overlay
            overlay = frame.copy()
            overlay[valid_mask] = cv2.addWeighted(
                frame[valid_mask],
                1 - self.config.alpha,
                depth_colored[valid_mask],
                self.config.alpha,
                0
            )
            
            return overlay
        
        return frame
    
    def create_3d_mesh_plot(self, vertices: np.ndarray, faces: np.ndarray,
                           values: Optional[np.ndarray] = None) -> go.Figure:
        """Create an interactive 3D mesh visualization.
        
        Args:
            vertices: Nx3 array of vertex coordinates
            faces: Mx3 array of face indices
            values: Optional array of values for vertex coloring
            
        Returns:
            Plotly figure object
        """
        fig = go.Figure()
        
        # Add mesh
        fig.add_trace(go.Mesh3d(
            x=vertices[:, 0],
            y=vertices[:, 1],
            z=vertices[:, 2],
            i=faces[:, 0],
            j=faces[:, 1],
            k=faces[:, 2],
            opacity=self.config.mesh_opacity,
            colorscale=self.config.surface_colorscale,
            intensity=values if values is not None else vertices[:, 2],
            showscale=True
        ))
        
        # Update layout
        fig.update_layout(
            scene=dict(
                aspectmode='data',
                xaxis_title='X',
                yaxis_title='Y',
                zaxis_title='Z'
            ),
            title='3D Facial Mesh'
        )
        
        return fig
    
    def create_depth_analysis_dashboard(self, depth_map: np.ndarray,
                                      metrics: Dict[str, float]) -> go.Figure:
        """Create an interactive dashboard for depth analysis.
        
        Args:
            depth_map: Depth map array
            metrics: Dictionary of depth-based metrics
            
        Returns:
            Plotly figure object
        """
        fig = go.Figure()
        
        # Add depth surface plot
        valid_mask = ~np.isnan(depth_map)
        if np.any(valid_mask):
            y, x = np.mgrid[:depth_map.shape[0], :depth_map.shape[1]]
            
            fig.add_trace(go.Surface(
                x=x,
                y=y,
                z=depth_map,
                colorscale=self.config.surface_colorscale,
                showscale=True,
                name='Depth Map'
            ))
        
        # Add metrics subplot
        fig.add_trace(go.Bar(
            x=list(metrics.keys()),
            y=list(metrics.values()),
            name='Depth Metrics'
        ))
        
        # Update layout
        fig.update_layout(
            title='Depth Analysis Dashboard',
            scene=dict(
                aspectmode='data',
                xaxis_title='X',
                yaxis_title='Y',
                zaxis_title='Depth'
            ),
            height=800,
            showlegend=True
        )
        
        return fig
    
    def visualize_vector_field(self, positions: np.ndarray,
                             vectors: np.ndarray) -> go.Figure:
        """Create an interactive visualization of the surface normal vector field.
        
        Args:
            positions: Nx2 array of vector positions
            vectors: Nx3 array of vector components
            
        Returns:
            Plotly figure object
        """
        fig = go.Figure()
        
        # Add vector field using cone plot
        fig.add_trace(go.Cone(
            x=positions[:, 0].flatten(),
            y=positions[:, 1].flatten(),
            z=np.zeros_like(positions[:, 0].flatten()),
            u=vectors[:, 0].flatten(),
            v=vectors[:, 1].flatten(),
            w=vectors[:, 2].flatten(),
            colorscale=self.config.surface_colorscale,
            showscale=True
        ))
        
        # Update layout
        fig.update_layout(
            title='Surface Normal Vector Field',
            scene=dict(
                aspectmode='data',
                xaxis_title='X',
                yaxis_title='Y',
                zaxis_title='Z'
            ),
            height=600
        )
        
        return fig
