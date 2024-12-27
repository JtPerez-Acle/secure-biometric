"""Biometric analysis pipeline orchestrator."""
import cv2
import numpy as np
from typing import Dict, Optional, Tuple
import time

from .capture.video import VideoCapture
from .capture.landmarks import FaceLandmarkDetector
from .analysis.facial_features.landmark_analyzer import LandmarkAnalyzer
from .analysis.facial_features.geometry_analyzer import GeometryAnalyzer
from .analysis.facial_features.expression_analyzer import ExpressionAnalyzer
from .analysis.depth.depth_mapper import DepthMapper
from .analysis.depth.feature_extractor import DepthFeatureExtractor
from .analysis.depth.mesh_generator import MeshGenerator
from .analysis.change_detector import ChangeDetector
from .analysis.color_analysis import ColorAnalyzer
from .visualization.feature_visualizer import FeatureVisualizer
from .visualization.depth_visualizer import DepthVisualizer
from .visualization.mesh_visualizer import MeshVisualizer
from .utils.config import BiometricConfig
from .utils.data_handler import BiometricDataHandler
from .utils.session_manager import SessionManager

class BiometricPipeline:
    """Orchestrates the biometric analysis pipeline."""
    
    def __init__(self, config: BiometricConfig):
        """Initialize the pipeline with configuration.
        
        Args:
            config: Configuration object
        """
        self.config = config
        self._init_components()
        
    def _init_components(self):
        """Initialize all pipeline components."""
        # Capture components
        self.video_capture = VideoCapture()
        self.landmark_detector = FaceLandmarkDetector()
        
        # Analysis components
        self.landmark_analyzer = LandmarkAnalyzer(self.landmark_detector.FEATURE_INDICES)
        self.geometry_analyzer = GeometryAnalyzer(self.landmark_detector.FEATURE_INDICES)
        self.expression_analyzer = ExpressionAnalyzer(self.landmark_detector.FEATURE_INDICES)
        self.depth_mapper = DepthMapper(
            (self.config.capture.frame_width, self.config.capture.frame_height),
            self.landmark_detector.FEATURE_INDICES
        )
        self.depth_extractor = DepthFeatureExtractor(self.landmark_detector.FEATURE_INDICES)
        self.mesh_generator = MeshGenerator()
        self.change_detector = ChangeDetector()
        self.color_analyzer = ColorAnalyzer()
        
        # Visualization components
        self.feature_visualizer = FeatureVisualizer()
        self.depth_visualizer = DepthVisualizer()
        self.mesh_visualizer = MeshVisualizer()
        
        # Data handling
        self.data_handler = BiometricDataHandler(self.config.storage.base_dir)
        self.session_manager = SessionManager(self.config.storage.base_dir)
        
    def run_capture_phase(self, duration: int = 10) -> Tuple[str, Dict]:
        """Run the initial capture phase.
        
        Args:
            duration: Capture duration in seconds
            
        Returns:
            Tuple of (session_id, captured_data)
        """
        session_id = self.session_manager.start_new_session()
        start_time = time.time()
        captured_data = {
            "landmarks": [],
            "frames": [],
            "metrics": {
                "geometry": [],
                "expression": [],
                "color": []
            }
        }
        
        print(f"\nStarting capture phase (Session: {session_id})")
        print("Please maintain a neutral expression and follow the prompts...")
        
        while (time.time() - start_time) < duration:
            # Capture and process frame
            frame = self.video_capture.read_frame()
            if frame is None:
                continue
                
            # Detect landmarks
            landmarks = self.landmark_detector.detect_landmarks(frame)
            if landmarks is None:
                continue
                
            # Check for significant changes
            if self.change_detector.is_significant_change(landmarks):
                # Analyze frame
                geometry_features = self.geometry_analyzer.analyze_geometry(landmarks)
                expression_features = self.expression_analyzer.analyze_expression(landmarks)
                color_features = self.color_analyzer.analyze(frame, landmarks)
                
                # Store data
                captured_data["landmarks"].append(landmarks)
                captured_data["frames"].append(frame)
                captured_data["metrics"]["geometry"].append(geometry_features)
                captured_data["metrics"]["expression"].append(expression_features)
                captured_data["metrics"]["color"].append(color_features)
                
                # Visualize progress
                vis_frame = self.feature_visualizer.draw_landmarks(frame, landmarks)
                cv2.imshow("Capture Progress", vis_frame)
                
            if cv2.waitKey(1) & 0xFF == 27:  # ESC to exit
                break
                
        cv2.destroyAllWindows()
        return session_id, captured_data
        
    def run_depth_analysis(self, captured_data: Dict) -> Dict:
        """Run depth analysis on captured frames.
        
        Args:
            captured_data: Data from capture phase
            
        Returns:
            Dictionary of depth analysis results
        """
        print("\nStarting depth analysis phase...")
        depth_results = {
            "depth_maps": [],
            "depth_features": [],
            "meshes": []
        }
        
        for landmarks in captured_data["landmarks"]:
            # Generate depth map
            depth_map = self.depth_mapper.create_depth_map(landmarks)
            
            # Extract depth features
            depth_features = self.depth_extractor.calculate_depth_features(landmarks)
            
            # Generate 3D mesh
            mesh = self.mesh_generator.create_3d_mesh(landmarks)
            
            # Store results
            depth_results["depth_maps"].append(depth_map)
            depth_results["depth_features"].append(depth_features)
            depth_results["meshes"].append(mesh)
            
        return depth_results
        
    def visualize_results(self, session_id: str, captured_data: Dict,
                         depth_results: Dict):
        """Create visualizations of analysis results.
        
        Args:
            session_id: Session identifier
            captured_data: Data from capture phase
            depth_results: Results from depth analysis
        """
        print("\nGenerating visualizations...")
        
        # Create feature dashboard
        feature_fig = self.feature_visualizer.create_feature_dashboard(
            captured_data["landmarks"][-1],  # Use last frame
            captured_data["metrics"]
        )
        feature_fig.write_html(f"output/visualization/{session_id}_features.html")
        
        # Create depth analysis dashboard
        depth_fig = self.depth_visualizer.create_depth_analysis_dashboard(
            depth_results["depth_maps"][-1],
            depth_results["depth_features"][-1]
        )
        depth_fig.write_html(f"output/visualization/{session_id}_depth.html")
        
        # Create 3D mesh visualization
        mesh = depth_results["meshes"][-1]
        mesh_fig = self.mesh_visualizer.create_surface_plot(
            mesh["vertices"],
            mesh["faces"]
        )
        mesh_fig.write_html(f"output/visualization/{session_id}_mesh.html")
        
    def run_full_pipeline(self) -> str:
        """Run the complete biometric analysis pipeline.
        
        Returns:
            Session ID of the completed analysis
        """
        try:
            # Phase 1: Capture
            session_id, captured_data = self.run_capture_phase()
            
            # Phase 2: Depth Analysis
            depth_results = self.run_depth_analysis(captured_data)
            
            # Phase 3: Visualization
            self.visualize_results(session_id, captured_data, depth_results)
            
            # Save results
            self.data_handler.save_biometric_profile(session_id, {
                "captured_data": captured_data,
                "depth_results": depth_results
            })
            
            print(f"\nAnalysis complete! Session ID: {session_id}")
            return session_id
            
        except Exception as e:
            print(f"\nError during analysis: {e}")
            raise
        
        finally:
            self.video_capture.release()
            cv2.destroyAllWindows()
