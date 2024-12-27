import cv2

class VideoCapture:
    def __init__(self, camera_id=0):
        self.cap = cv2.VideoCapture(camera_id)
        if not self.cap.isOpened():
            raise RuntimeError("Error: Could not open webcam.")

    def read_frame(self):
        """Read a frame from the video capture device."""
        success, frame = self.cap.read()
        if not success:
            return None
        return frame

    def release(self):
        """Release the video capture device."""
        self.cap.release()
        cv2.destroyAllWindows()
