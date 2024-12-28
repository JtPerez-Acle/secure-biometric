# Biometric Analysis Project Overview

## Project Description
The Biometric Analysis Project is a comprehensive system for facial feature analysis and recognition. It combines computer vision techniques with machine learning to analyze facial features, expressions, and depth information. The system is composed of two main components:

1. **Python Biometric Analysis Module**: Handles facial landmark detection, expression analysis, depth mapping, and visualization.
2. **Rust Backend Service**: Provides API endpoints for user management, authentication, and data storage.

## Key Features

### Python Module
- Real-time facial landmark detection using MediaPipe
- Depth map generation from facial landmarks
- Facial expression analysis (smile, frown, surprise, etc.)
- 3D mesh generation and visualization
- Change detection for facial movements
- Comprehensive visualization tools

### Rust Backend
- User authentication with JWT tokens
- Rate limiting and request logging
- Database management for users, sessions, and API keys
- RESTful API endpoints for all operations
- Secure storage of sensitive information

## Architecture

### Python Module Components
1. **Capture**: Handles video input and landmark detection
2. **Analysis**: Performs facial feature analysis and depth mapping
3. **Visualization**: Provides tools for 3D visualization and feature display
4. **Utils**: Configuration management and session handling

### Rust Backend Components
1. **API**: REST endpoints for all operations
2. **Middleware**: Authentication, rate limiting, and logging
3. **Services**: Core business logic (authentication, etc.)
4. **Repositories**: Database access layer
5. **Models**: Data structures and schemas

## Development Workflow
1. **Feature Development**: Create new features in separate branches
2. **Testing**: Write unit tests for all new functionality
3. **Documentation**: Update documentation with new features
4. **Code Review**: Submit pull requests for review
5. **Deployment**: Use CI/CD pipeline for deployment

## API Documentation
The Rust backend provides the following API endpoints:

- **Authentication**: /api/auth/register, /api/auth/login
- **User Management**: /api/users/{id}
- **Session Management**: /api/sessions
- **API Key Management**: /api/keys

## Configuration
The system uses a configuration file (config.json) for settings:
- Video capture resolution
- Analysis parameters
- Storage locations
- Visualization settings

## Dependencies
- Python: mediapipe, opencv, numpy, plotly
- Rust: actix-web, sqlx, jsonwebtoken
