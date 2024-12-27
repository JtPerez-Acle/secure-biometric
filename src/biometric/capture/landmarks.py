import mediapipe as mp
import numpy as np
import cv2

class FaceLandmarkDetector:
    def __init__(self):
        self.mp_face_mesh = mp.solutions.face_mesh
        self.face_mesh = self.mp_face_mesh.FaceMesh(
            static_image_mode=False,
            max_num_faces=1,
            refine_landmarks=True,
            min_detection_confidence=0.5,
            min_tracking_confidence=0.5
        )

    def detect_landmarks(self, frame):
        """Detect facial landmarks in the given frame."""
        frame_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        results = self.face_mesh.process(frame_rgb)
        return results.multi_face_landmarks[0] if results.multi_face_landmarks else None

    def extract_landmark_points(self, frame, face_landmarks):
        """Convert normalized MediaPipe FaceMesh landmark coordinates into pixel coordinates."""
        h, w, _ = frame.shape
        landmark_points = []
        for landmark in face_landmarks.landmark:
            x_px = int(landmark.x * w)
            y_px = int(landmark.y * h)
            z_val = landmark.z  # Normalized depth
            landmark_points.append([x_px, y_px, z_val])
        return np.array(landmark_points, dtype=np.float32)

    def close(self):
        """Release resources."""
        self.face_mesh.close()
