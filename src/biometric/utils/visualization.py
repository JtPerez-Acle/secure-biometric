import mediapipe as mp
import cv2

class FaceMeshVisualizer:
    def __init__(self):
        self.mp_drawing = mp.solutions.drawing_utils
        self.mp_face_mesh = mp.solutions.face_mesh
        self.drawing_spec_landmark = self.mp_drawing.DrawingSpec(
            thickness=1, 
            circle_radius=1, 
            color=(0, 255, 0)
        )
        self.drawing_spec_connection = self.mp_drawing.DrawingSpec(
            thickness=1, 
            color=(255, 0, 0)
        )

    def draw_landmarks(self, frame, face_landmarks):
        """Draw facial landmarks and connections on the frame."""
        self.mp_drawing.draw_landmarks(
            image=frame,
            landmark_list=face_landmarks,
            connections=self.mp_face_mesh.FACEMESH_TESSELATION,
            landmark_drawing_spec=self.drawing_spec_landmark,
            connection_drawing_spec=self.drawing_spec_connection
        )
        return frame

    def display_frame(self, frame, window_name="Face Analysis"):
        """Display the frame in a window."""
        cv2.imshow(window_name, frame)
        return cv2.waitKey(5) & 0xFF  # Return key pressed
