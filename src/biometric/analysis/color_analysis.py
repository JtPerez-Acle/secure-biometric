import cv2
import numpy as np

class FaceColorAnalyzer:
    def calculate_average_color(self, frame, landmarks_array):
        """Calculate average color in the face region."""
        x_coords = landmarks_array[:, 0]
        y_coords = landmarks_array[:, 1]

        # Get bounding box
        x_min, x_max = int(np.min(x_coords)), int(np.max(x_coords))
        y_min, y_max = int(np.min(y_coords)), int(np.max(y_coords))

        # Clip to image boundaries
        h, w = frame.shape[:2]
        x_min = max(0, x_min)
        x_max = min(w - 1, x_max)
        y_min = max(0, y_min)
        y_max = min(h - 1, y_max)

        # Extract face region
        face_roi = frame[y_min:y_max, x_min:x_max]
        if face_roi.size == 0:
            return (0, 0, 0)

        # Calculate mean color (BGR)
        mean_color = cv2.mean(face_roi)[:3]
        return mean_color
