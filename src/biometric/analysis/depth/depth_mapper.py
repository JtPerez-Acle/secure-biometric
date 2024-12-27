"""Core functionality for creating and managing depth maps from facial landmarks."""
import numpy as np
import cv2
from scipy.interpolate import griddata

class DepthMapper:
    """Creates and manages depth maps from facial landmarks."""
    
    def __init__(self, image_size, feature_indices):
        """Initialize the depth mapper.
        
        Args:
            image_size: Tuple of (width, height) for the target image size
            feature_indices: Dictionary of facial feature indices
        """
        self.image_width, self.image_height = image_size
        self.feature_indices = feature_indices
        self.depth_map = None
    
    def _create_face_mask(self, points, resolution):
        """Create a mask for the face region using facial landmarks."""
        # Get face contour points and additional points for a complete mask
        contour_points = points[self.feature_indices['face_contour']]
        forehead_points = points[[10, 108, 67, 69, 104, 151, 337, 299, 333, 297, 338]]
        mask_points = np.vstack([contour_points, forehead_points])
        
        # Create convex hull
        hull = cv2.convexHull(mask_points.astype(np.float32))
        
        # Create empty mask
        mask = np.zeros(resolution, dtype=np.uint8)
        
        # Fill the hull area
        hull_points = hull.reshape(-1, 2)
        hull_points = hull_points * np.array([resolution[0]/self.image_width, 
                                            resolution[1]/self.image_height])
        cv2.fillConvexPoly(mask, hull_points.astype(np.int32), 1)
        
        return mask.astype(bool)

    def create_depth_map(self, landmarks_array, resolution=None):
        """Create a depth map from landmark points using interpolation.
        
        Args:
            landmarks_array: Nx3 array of landmark points (x, y, z)
            resolution: Optional tuple of (width, height) for the depth map resolution.
                       If None, uses the image dimensions
        
        Returns:
            Interpolated depth map as a 2D numpy array
        """
        # Use lower resolution for processing if not specified
        if resolution is None:
            resolution = (self.image_width // 4, self.image_height // 4)

        # Calculate relative depths from nose tip
        nose_tip_idx = self.feature_indices['nose_tip']
        nose_tip_depth = landmarks_array[nose_tip_idx, 2]
        relative_depths = landmarks_array[:, 2] - nose_tip_depth
        
        # Calculate face size for normalization
        face_points = landmarks_array[self.feature_indices['face_contour']]
        face_width = np.max(face_points[:, 0]) - np.min(face_points[:, 0])
        face_height = np.max(face_points[:, 1]) - np.min(face_points[:, 1])
        face_size = np.sqrt(face_width * face_height)
        
        # Normalize depths
        scale_factor = 0.1 * (resolution[0] / self.image_width)
        normalized_depths = relative_depths / (face_size * scale_factor)
        
        # Create grid for interpolation
        grid_x, grid_y = np.mgrid[0:resolution[0]:resolution[0]*1j,
                                 0:resolution[1]:resolution[1]*1j]
        
        # Scale landmark points to match the grid resolution
        points = landmarks_array[:, :2].copy()
        points[:, 0] *= (resolution[0] / self.image_width)
        points[:, 1] *= (resolution[1] / self.image_height)
        
        # Create face mask
        face_mask = self._create_face_mask(points, resolution)
        
        # Interpolate depths using linear method for speed
        depth_map = griddata(points, normalized_depths, (grid_x, grid_y), 
                           method='linear', fill_value=np.nan)
        
        # Fill remaining NaN values within face mask using nearest neighbor
        if np.any(np.isnan(depth_map) & face_mask):
            nn_interpolated = griddata(points, normalized_depths, (grid_x, grid_y), 
                                     method='nearest', fill_value=np.nan)
            depth_map = np.where(np.isnan(depth_map) & face_mask, 
                               nn_interpolated, depth_map)
        
        # Apply light smoothing to reduce noise
        valid_mask = ~np.isnan(depth_map)
        if np.any(valid_mask):
            valid_depths = depth_map[valid_mask]
            smoothed = cv2.GaussianBlur(valid_depths.reshape(-1, 1), (1, 3), 0).ravel()
            result = np.full_like(depth_map, np.nan)
            result[valid_mask] = smoothed
            depth_map = result
        
        self.depth_map = depth_map
        return depth_map

    def get_surface_gradients(self):
        """Calculate surface gradients from the depth map.
        
        Returns:
            Tuple of (dy, dx) gradients
        """
        if self.depth_map is None:
            return None, None
        
        return np.gradient(self.depth_map)
