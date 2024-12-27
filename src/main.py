"""Main entry point for biometric analysis application."""
import os
import sys
import time
from biometric.pipeline import BiometricPipeline
from biometric.utils.config import BiometricConfig
from biometric.utils.session_manager import SessionManager

def print_menu():
    """Print the main menu options."""
    print("\nBiometric Analysis System")
    print("=" * 30)
    print("1. Start New Analysis")
    print("2. List Sessions")
    print("3. View Session Results")
    print("4. Clean Old Sessions")
    print("5. Exit")
    print("=" * 30)

def list_sessions(session_manager):
    """Display all existing sessions."""
    sessions = session_manager.list_sessions()
    if not sessions:
        print("\nNo sessions found.")
        return None
    
    print("\nExisting Sessions:")
    print("=" * 70)
    print(f"{'Session ID':<20} {'Timestamp':<20} {'Quality Score':<15}")
    print("-" * 70)
    
    for session in sessions:
        print(f"{session['session_id']:<20} "
              f"{session['timestamp']:<20} "
              f"{session['quality_score']:<15.2f}")
    
    return sessions

def view_session_results(session_id: str):
    """Open visualization dashboards for a session.
    
    Args:
        session_id: Session identifier
    """
    base_path = f"output/visualization/{session_id}"
    dashboards = [
        f"{base_path}_features.html",
        f"{base_path}_depth.html",
        f"{base_path}_mesh.html"
    ]
    
    for dashboard in dashboards:
        if os.path.exists(dashboard):
            os.system(f"open {dashboard}")
        else:
            print(f"Warning: Dashboard not found: {dashboard}")

def main():
    """Main application entry point."""
    # Load configuration
    config = BiometricConfig.load("config/biometric_config.json")
    session_manager = SessionManager(config.storage.base_dir)
    
    while True:
        print_menu()
        choice = input("\nEnter your choice (1-5): ")
        
        if choice == "1":
            # Start new analysis
            pipeline = BiometricPipeline(config)
            try:
                session_id = pipeline.run_full_pipeline()
                print("\nAnalysis complete!")
                print(f"Session ID: {session_id}")
                print("\nWould you like to view the results? (y/n)")
                if input().lower() == 'y':
                    view_session_results(session_id)
            except Exception as e:
                print(f"\nError during analysis: {e}")
                
        elif choice == "2":
            # List sessions
            list_sessions(session_manager)
            
        elif choice == "3":
            # View session results
            sessions = list_sessions(session_manager)
            if sessions:
                session_id = input("\nEnter session ID to view: ")
                view_session_results(session_id)
                
        elif choice == "4":
            # Clean old sessions
            sessions = list_sessions(session_manager)
            if sessions:
                print("\nEnter session ID to clean (or 'all' for all sessions):")
                session_id = input()
                if session_id.lower() == 'all':
                    session_manager.clean_all_sessions()
                    print("\nAll sessions cleaned.")
                else:
                    session_manager.clean_session(session_id)
                    print(f"\nSession {session_id} cleaned.")
                    
        elif choice == "5":
            print("\nExiting...")
            break
            
        else:
            print("\nInvalid choice. Please try again.")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nExiting...")
        sys.exit(0)
