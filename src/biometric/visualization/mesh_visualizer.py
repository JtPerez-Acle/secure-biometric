"""Visualize 3D mesh data and surface properties."""
import numpy as np
import plotly.graph_objects as go
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass

@dataclass
class MeshVisualizationConfig:
    """Configuration for mesh visualization."""
    point_size: int = 3
    line_width: int = 2
    surface_opacity: float = 0.7
    colorscale: str = 'Viridis'
    background_color: str = 'rgb(17,17,17)'
    grid_color: str = 'rgb(50,50,50)'

class MeshVisualizer:
    """Visualizes 3D mesh data and surface properties."""
    
    def __init__(self, config: Optional[MeshVisualizationConfig] = None):
        """Initialize the mesh visualizer.
        
        Args:
            config: Optional visualization configuration
        """
        self.config = config or MeshVisualizationConfig()
    
    def create_wireframe_plot(self, vertices: np.ndarray,
                            edges: List[Tuple[int, int]]) -> go.Figure:
        """Create an interactive wireframe visualization.
        
        Args:
            vertices: Nx3 array of vertex coordinates
            edges: List of vertex index pairs forming edges
            
        Returns:
            Plotly figure object
        """
        fig = go.Figure()
        
        # Add vertices
        fig.add_trace(go.Scatter3d(
            x=vertices[:, 0],
            y=vertices[:, 1],
            z=vertices[:, 2],
            mode='markers',
            marker=dict(
                size=self.config.point_size,
                color=vertices[:, 2],
                colorscale=self.config.colorscale,
                opacity=1
            ),
            name='Vertices'
        ))
        
        # Add edges
        for start_idx, end_idx in edges:
            fig.add_trace(go.Scatter3d(
                x=[vertices[start_idx, 0], vertices[end_idx, 0]],
                y=[vertices[start_idx, 1], vertices[end_idx, 1]],
                z=[vertices[start_idx, 2], vertices[end_idx, 2]],
                mode='lines',
                line=dict(
                    width=self.config.line_width,
                    color='white'
                ),
                showlegend=False
            ))
        
        # Update layout
        fig.update_layout(
            scene=dict(
                aspectmode='data',
                xaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                ),
                yaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                ),
                zaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                )
            ),
            title='3D Wireframe Model'
        )
        
        return fig
    
    def create_surface_plot(self, vertices: np.ndarray, faces: np.ndarray,
                          values: Optional[np.ndarray] = None,
                          normals: Optional[np.ndarray] = None) -> go.Figure:
        """Create an interactive surface visualization with optional properties.
        
        Args:
            vertices: Nx3 array of vertex coordinates
            faces: Mx3 array of face indices
            values: Optional array of values for vertex coloring
            normals: Optional array of vertex normals
            
        Returns:
            Plotly figure object
        """
        fig = go.Figure()
        
        # Add surface mesh
        fig.add_trace(go.Mesh3d(
            x=vertices[:, 0],
            y=vertices[:, 1],
            z=vertices[:, 2],
            i=faces[:, 0],
            j=faces[:, 1],
            k=faces[:, 2],
            intensity=values if values is not None else vertices[:, 2],
            colorscale=self.config.colorscale,
            opacity=self.config.surface_opacity,
            name='Surface'
        ))
        
        # Add normal vectors if provided
        if normals is not None:
            # Sample points for normal visualization
            sample_idx = np.random.choice(
                vertices.shape[0],
                size=min(100, vertices.shape[0]),
                replace=False
            )
            
            # Add normal vectors as cones
            fig.add_trace(go.Cone(
                x=vertices[sample_idx, 0],
                y=vertices[sample_idx, 1],
                z=vertices[sample_idx, 2],
                u=normals[sample_idx, 0],
                v=normals[sample_idx, 1],
                w=normals[sample_idx, 2],
                colorscale=self.config.colorscale,
                name='Normals'
            ))
        
        # Update layout
        fig.update_layout(
            scene=dict(
                aspectmode='data',
                xaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                ),
                yaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                ),
                zaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                )
            ),
            title='3D Surface Model'
        )
        
        return fig
    
    def create_curvature_plot(self, vertices: np.ndarray,
                            faces: np.ndarray,
                            curvature: np.ndarray) -> go.Figure:
        """Create a visualization of surface curvature.
        
        Args:
            vertices: Nx3 array of vertex coordinates
            faces: Mx3 array of face indices
            curvature: Array of curvature values for vertices
            
        Returns:
            Plotly figure object
        """
        fig = go.Figure()
        
        # Add surface colored by curvature
        fig.add_trace(go.Mesh3d(
            x=vertices[:, 0],
            y=vertices[:, 1],
            z=vertices[:, 2],
            i=faces[:, 0],
            j=faces[:, 1],
            k=faces[:, 2],
            intensity=curvature,
            colorscale=self.config.colorscale,
            opacity=self.config.surface_opacity,
            name='Curvature',
            showscale=True,
            colorbar=dict(
                title='Curvature'
            )
        ))
        
        # Update layout
        fig.update_layout(
            scene=dict(
                aspectmode='data',
                xaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                ),
                yaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                ),
                zaxis=dict(
                    backgroundcolor=self.config.background_color,
                    gridcolor=self.config.grid_color,
                    showbackground=True
                )
            ),
            title='Surface Curvature Analysis'
        )
        
        return fig
