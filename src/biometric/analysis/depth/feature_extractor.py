"""Extract and analyze depth-based facial features."""
import numpy as np

class DepthFeatureExtractor:
    """Extracts and analyzes depth-based features from facial landmarks."""
    
    def __init__(self, feature_indices):
        """Initialize the feature extractor.
        
        Args:
            feature_indices: Dictionary of facial feature indices
        """
        self.feature_indices = feature_indices
    
    def extract_feature_depths(self, landmarks_array):
        """Extract depth values for key facial features.
        
        Args:
            landmarks_array: Nx3 array of landmark points
            
        Returns:
            Dictionary of feature names and their depth characteristics
        """
        feature_depths = {}
        
        for feature_name, indices in self.feature_indices.items():
            if isinstance(indices, list):
                # Calculate statistics for features with multiple points
                depths = landmarks_array[indices, 2]
                feature_depths[feature_name] = {
                    'mean_depth': float(np.mean(depths)),
                    'std_depth': float(np.std(depths)),
                    'relative_depths': depths.tolist()
                }
            else:
                # Single point feature
                feature_depths[feature_name] = {
                    'depth': float(landmarks_array[indices, 2])
                }
        
        return feature_depths
    
    def calculate_depth_features(self, landmarks_array):
        """Calculate advanced depth-based features for face recognition.
        
        Args:
            landmarks_array: Nx3 array of landmark points
            
        Returns:
            Dictionary of depth-based features
        """
        feature_depths = self.extract_feature_depths(landmarks_array)
        
        # Calculate distinctive depth relationships
        depth_features = {
            'facial_topology': {
                'nose_prominence': feature_depths['nose_tip']['depth'] - 
                                 np.mean([feature_depths['left_eye']['mean_depth'],
                                        feature_depths['right_eye']['mean_depth']]),
                
                'eye_depth_asymmetry': abs(feature_depths['left_eye']['mean_depth'] - 
                                         feature_depths['right_eye']['mean_depth']),
                
                'nose_bridge_angle': np.std(feature_depths['nose_bridge']['relative_depths'])
            },
            
            'facial_structure': {
                'face_depth_variance': np.std(landmarks_array[:, 2]),
                'face_depth_range': np.ptp(landmarks_array[:, 2]),
                'face_depth_profile': self._calculate_depth_profile(landmarks_array)
            },
            
            'feature_depths': feature_depths
        }
        
        return depth_features
    
    def _calculate_depth_profile(self, landmarks_array):
        """Calculate depth profile characteristics.
        
        Args:
            landmarks_array: Nx3 array of landmark points
            
        Returns:
            Dictionary of depth profile metrics
        """
        # Get vertical depth profile
        vertical_profile = np.sort(landmarks_array[:, 2])
        percentiles = [10, 25, 50, 75, 90]
        
        return {
            'depth_percentiles': {
                f'p{p}': float(np.percentile(vertical_profile, p))
                for p in percentiles
            },
            'depth_quartile_ratio': float(
                np.percentile(vertical_profile, 75) /
                np.percentile(vertical_profile, 25)
            )
        }
    
    def analyze_feature_symmetry(self, landmarks_array):
        """Analyze facial feature symmetry using depth information.
        
        Args:
            landmarks_array: Nx3 array of landmark points
            
        Returns:
            Dictionary of symmetry metrics
        """
        symmetry_metrics = {}
        
        # Compare left-right feature pairs
        feature_pairs = [
            ('left_eye', 'right_eye'),
            ('left_eyebrow', 'right_eyebrow')
        ]
        
        for left_feature, right_feature in feature_pairs:
            left_depths = landmarks_array[self.feature_indices[left_feature], 2]
            right_depths = landmarks_array[self.feature_indices[right_feature], 2]
            
            symmetry_metrics[f'{left_feature}_{right_feature}'] = {
                'depth_difference': float(np.mean(left_depths) - np.mean(right_depths)),
                'pattern_correlation': float(np.corrcoef(left_depths, right_depths)[0, 1]),
                'variance_ratio': float(np.var(left_depths) / np.var(right_depths))
            }
        
        return symmetry_metrics
