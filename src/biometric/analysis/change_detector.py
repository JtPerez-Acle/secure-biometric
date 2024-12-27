import numpy as np
from datetime import datetime
import cv2

class ChangeDetector:
    """Detects significant changes in facial features and expressions."""
    
    def __init__(self, thresholds):
        """Initialize with threshold settings."""
        self.thresholds = thresholds
        self.last_landmarks = None
        self.last_pose = None
        self.last_save_time = None
        self.frame_count = 0
        
    def calculate_landmark_change(self, current_landmarks, previous_landmarks):
        """Calculate the relative change in landmark positions."""
        if previous_landmarks is None:
            return float('inf')
        
        # Calculate normalized distance between corresponding landmarks
        distances = np.linalg.norm(current_landmarks - previous_landmarks, axis=1)
        # Normalize by the face size (using bounding box diagonal)
        face_size = np.linalg.norm([
            np.max(current_landmarks[:, 0]) - np.min(current_landmarks[:, 0]),
            np.max(current_landmarks[:, 1]) - np.min(current_landmarks[:, 1])
        ])
        return np.mean(distances) / face_size if face_size > 0 else 0

    def calculate_expression_change(self, current_landmarks, previous_landmarks):
        """Calculate changes in facial expressions using key landmarks."""
        if previous_landmarks is None:
            return float('inf')

        # Eye aspect ratio (vertical/horizontal)
        def eye_aspect_ratio(eye_points):
            vertical = np.mean([
                np.linalg.norm(eye_points[1] - eye_points[5]),
                np.linalg.norm(eye_points[2] - eye_points[4])
            ])
            horizontal = np.linalg.norm(eye_points[0] - eye_points[3])
            return vertical / (2.0 * horizontal) if horizontal > 0 else 0

        # Calculate EAR for current and previous
        left_eye_curr = eye_aspect_ratio(current_landmarks[[33, 160, 158, 133, 153, 144]])
        left_eye_prev = eye_aspect_ratio(previous_landmarks[[33, 160, 158, 133, 153, 144]])
        
        # Mouth aspect ratio
        def mouth_aspect_ratio(landmarks):
            vertical = np.linalg.norm(landmarks[13] - landmarks[14])
            horizontal = np.linalg.norm(landmarks[78] - landmarks[308])
            return vertical / horizontal if horizontal > 0 else 0

        mouth_curr = mouth_aspect_ratio(current_landmarks)
        mouth_prev = mouth_aspect_ratio(previous_landmarks)

        # Eyebrow position relative to eye
        def eyebrow_position(landmarks):
            return np.mean([
                landmarks[282][1] - landmarks[257][1],  # right eyebrow
                landmarks[52][1] - landmarks[27][1]     # left eyebrow
            ])

        eyebrow_curr = eyebrow_position(current_landmarks)
        eyebrow_prev = eyebrow_position(previous_landmarks)

        changes = {
            "eye_aspect_ratio": abs(left_eye_curr - left_eye_prev),
            "mouth_aspect_ratio": abs(mouth_curr - mouth_prev),
            "eyebrow_position": abs(eyebrow_curr - eyebrow_prev) / 100  # Normalize
        }
        
        return changes

    def should_save_frame(self, landmarks_array, head_pose):
        """Determine if the current frame should be saved based on changes."""
        current_time = datetime.now()
        
        # Check time interval
        if self.last_save_time is not None:
            time_diff = (current_time - self.last_save_time).total_seconds()
            if time_diff < self.thresholds["MIN_FRAME_INTERVAL"]:
                return False

        # Check frame count limit
        if self.frame_count >= self.thresholds["MAX_STORED_FRAMES"]:
            return False

        # Calculate landmark changes
        landmark_change = self.calculate_landmark_change(
            landmarks_array, 
            self.last_landmarks
        )
        
        # Calculate expression changes
        if self.last_landmarks is not None:
            expression_changes = self.calculate_expression_change(
                landmarks_array,
                self.last_landmarks
            )
            # Initialize with False if no previous landmarks
            expression_changed = any(
                abs(expression_changes.get(k, 0)) > self.thresholds["EXPRESSION_METRICS"].get(k, float('inf'))
                for k in self.thresholds["EXPRESSION_METRICS"].keys()
            )
        else:
            expression_changed = True  # First frame should be saved

        # Check pose changes
        pose_changed = False
        if self.last_pose is not None and head_pose is not None:
            pose_changes = {
                k: abs(head_pose[k] - self.last_pose[k])
                for k in ["pitch", "yaw", "roll"]
            }
            pose_changed = any(
                pose_changes[k] > self.thresholds["HEAD_POSE_THRESHOLD"][k]
                for k in pose_changes
            )

        # Update last values if we should save
        should_save = (
            landmark_change > self.thresholds["LANDMARK_CHANGE_THRESHOLD"] or
            pose_changed or
            expression_changed
        )

        if should_save:
            self.last_landmarks = landmarks_array.copy()
            self.last_pose = head_pose.copy() if head_pose is not None else None
            self.last_save_time = current_time
            self.frame_count += 1

        return should_save
