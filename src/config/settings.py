"""Configuration settings for the biometric system."""
import os

# Base paths
BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT_DIR = os.path.join(BASE_DIR, "output")

# Camera settings
CAMERA_ID = 0
FRAME_WIDTH = 640
FRAME_HEIGHT = 480
FPS = 30

# Face detection settings
MIN_DETECTION_CONFIDENCE = 0.5
MIN_TRACKING_CONFIDENCE = 0.5
MAX_NUM_FACES = 1

# Visualization settings
WINDOW_NAME = "Face Analysis"
LANDMARK_COLOR = (0, 255, 0)  # BGR format
CONNECTION_COLOR = (255, 0, 0)  # BGR format

# Analysis settings
ENABLE_HEAD_POSE = True
ENABLE_COLOR_ANALYSIS = True
ENABLE_GEOMETRY_ANALYSIS = True
LANDMARK_CHANGE_THRESHOLD = 0.1

# Expression thresholds
EXPRESSION_METRICS = {
    "mouth_aspect_ratio": 0.2,    # Threshold for mouth movement
    "eye_aspect_ratio": 0.15,     # Threshold for eye blinking
    "eyebrow_position": 0.1       # Threshold for eyebrow movement
}

# Head pose thresholds
HEAD_POSE_THRESHOLD = {
    "pitch": 15.0,  # degrees
    "yaw": 15.0,    # degrees
    "roll": 15.0    # degrees
}

MIN_FRAME_INTERVAL = 0.5  # seconds
MAX_STORED_FRAMES = 100

# Output settings
SAVE_FRAMES = True
SAVE_LANDMARKS = True
SAVE_BIOMETRICS = True
GENERATE_REPORTS = True
EXPORT_FORMAT = "json"  # Options: "json", "csv"

# Window settings
WINDOW_FACE = "Face Analysis"
WINDOW_DEPTH = "Depth Map"
WINDOW_WIDTH = 800
WINDOW_HEIGHT = 600

# Storage thresholds
