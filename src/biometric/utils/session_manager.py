import os
import shutil
from datetime import datetime
import json
import glob

class SessionManager:
    """Manages biometric capture sessions and output directory cleanup."""
    
    def __init__(self, output_dir):
        self.output_dir = output_dir
        self.session_id = None
        self.session_start_time = None
        self._create_directory_structure()
        
    def _create_directory_structure(self):
        """Create the required directory structure."""
        directories = [
            os.path.join(self.output_dir, "profiles"),  # For permanent biometric profiles
            os.path.join(self.output_dir, "temp")       # For temporary processing files
        ]
        
        for directory in directories:
            os.makedirs(directory, exist_ok=True)
    
    def list_sessions(self):
        """List all existing sessions with their profiles."""
        profiles_dir = os.path.join(self.output_dir, "profiles")
        session_details = []
        
        try:
            for file in os.listdir(profiles_dir):
                if file.startswith("profile_") and file.endswith(".json"):
                    profile_path = os.path.join(profiles_dir, file)
                    try:
                        with open(profile_path, 'r') as f:
                            profile = json.load(f)
                            
                        session_details.append({
                            "session_id": profile.get("session_id"),
                            "timestamp": profile.get("timestamp"),
                            "frame_count": profile.get("metadata", {}).get("frame_count", 0),
                            "quality_score": profile.get("metadata", {}).get("quality_score", 0),
                            "capture_duration": profile.get("metadata", {}).get("capture_duration", 0)
                        })
                    except Exception as e:
                        print(f"Error reading profile {file}: {e}")
                        continue
            
            # Sort by timestamp (newest first)
            return sorted(session_details, key=lambda x: x["timestamp"], reverse=True)
        except Exception as e:
            print(f"Error listing sessions: {e}")
            return []
    
    def clean_session(self, session_id):
        """Remove a specific session's profile and any temporary files."""
        files_removed = 0
        
        try:
            # Remove profile
            profile_path = os.path.join(self.output_dir, "profiles", f"profile_{session_id}.json")
            if os.path.exists(profile_path):
                os.remove(profile_path)
                files_removed += 1
            
            # Remove any temporary files
            temp_dir = os.path.join(self.output_dir, "temp")
            for file in os.listdir(temp_dir):
                if session_id in file:
                    file_path = os.path.join(temp_dir, file)
                    try:
                        os.remove(file_path)
                        files_removed += 1
                    except Exception as e:
                        print(f"Error removing temporary file {file}: {e}")
            
            return files_removed
        except Exception as e:
            print(f"Error cleaning session {session_id}: {e}")
            return files_removed
    
    def clean_all_sessions(self):
        """Remove all profiles and temporary files."""
        try:
            # Remove all files in profiles directory
            profiles_dir = os.path.join(self.output_dir, "profiles")
            for file in os.listdir(profiles_dir):
                try:
                    os.remove(os.path.join(profiles_dir, file))
                except Exception as e:
                    print(f"Error removing profile {file}: {e}")
            
            # Clean temporary directory
            temp_dir = os.path.join(self.output_dir, "temp")
            for file in os.listdir(temp_dir):
                try:
                    os.remove(os.path.join(temp_dir, file))
                except Exception as e:
                    print(f"Error removing temporary file {file}: {e}")
            
            return True
        except Exception as e:
            print(f"Error during cleanup: {e}")
            return False
    
    def start_new_session(self):
        """Start a new capture session."""
        self.session_id = datetime.now().strftime("%Y%m%d_%H%M%S")
        self.session_start_time = datetime.now()
        return self.session_id
    
    def get_session_duration(self):
        """Get the duration of the current session in seconds."""
        if self.session_start_time:
            return (datetime.now() - self.session_start_time).total_seconds()
        return 0
    
    def clean_old_directories(self):
        """Remove old directory structure that's no longer needed."""
        old_directories = [
            os.path.join(self.output_dir, "scans"),
            os.path.join(self.output_dir, "biometrics"),
            os.path.join(self.output_dir, "reports"),
            os.path.join(self.output_dir, "depth"),
            os.path.join(self.output_dir, "sessions")
        ]
        
        for directory in old_directories:
            if os.path.exists(directory):
                try:
                    shutil.rmtree(directory)
                    print(f"Removed old directory: {directory}")
                except Exception as e:
                    print(f"Error removing directory {directory}: {e}")
