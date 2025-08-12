# Clipstream Frontend 🎮

Next.js 14+ frontend for the Clipstream gaming video platform with direct MinIO integration and optimized Axum backend communication.

## 🏗️ Architecture

### Frontend-Backend Separation
- **Decoupled Architecture**: Next.js frontend communicates with Axum backend via REST API
- **Optimized Database Access**: Backend uses SQLx connection pooling for high performance
- **Direct Storage Access**: Frontend uploads directly to MinIO via presigned URLs
- **Stateless API Design**: JWT-based authentication supports horizontal backend scaling
- **Multi-Client Ready**: Backend's SQLx pooling enables web, mobile, and desktop clients

### Key Integrations
- **Axum Backend API**: High-performance Rust backend with integrated SQLx pooling
- **MinIO Object Storage**: Direct file uploads/downloads via presigned URLs
- **Google OAuth**: Authentication flow through NextAuth.js
- **Real-time Updates**: WebSocket connections for processing status updates
```javascript
// Google Sign-In with NextAuth.js
import { signIn, useSession } from "next-auth/react"

const { data: session } = useSession()
const googleIdToken = session?.accessToken // Google ID token

// Send to backend for verification
const response = await fetch('/api/auth/verify', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${googleIdToken}`,
    'Content-Type': 'application/json'
  }
})
const { app_token } = await response.json()
```

### API Client Configuration
```javascript
// lib/api-client.js - Optimized for Axum backend with SQLx pooling
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'

class ApiClient {
  constructor() {
    this.baseURL = API_BASE_URL
    this.token = null
  }
  
  setToken(token) {
    this.token = token
  }
  
  async request(endpoint, options = {}) {
    const url = `${this.baseURL}${endpoint}`
    const config = {
      headers: {
        'Content-Type': 'application/json',
        ...(this.token && { 'Authorization': `Bearer ${this.token}` }),
        ...options.headers,
      },
      ...options,
    }
    
    // Axum backend with SQLx pooling provides consistent low-latency responses
    const response = await fetch(url, config)
    
    if (!response.ok) {
      throw new Error(`API request failed: ${response.status}`)
    }
    
    return response.json()
  }
}

// Connection health monitoring
const checkBackendHealth = async () => {
  const health = await apiClient.request('/health');
  console.log('Backend pool status:', health.pool_connections, 'active connections');
}
```

## 📱 Pages & Components

### Authentication Pages
```
/auth/signin                    # Google Sign-In page
/auth/signout                   # Sign-out confirmation
/auth/callback                  # OAuth callback handler
```

### Main Application Pages
```
/                              # Dashboard with user's streams
/streams                       # Stream discovery/browse
/streams/[id]                  # Stream detail view with videos
/streams/[id]/settings         # Stream management (admin only)
/streams/create                # Create new stream

/videos/[id]                   # Video player with timeline view
/videos/[id]/similar           # Similar clips view
/videos/[id]/timeline          # Timeline view with trimmed clips

/upload                        # Video upload interface
/search                        # Search videos by content/speech

/profile                       # User profile management
/admin                         # Admin dashboard (admin users only)
```

### Key React Components
```
components/
├── auth/
│   ├── AuthButton.tsx         # Sign in/out button
│   ├── AuthGuard.tsx          # Route protection wrapper
│   └── GoogleSignIn.tsx       # Google OAuth component
│
├── streams/
│   ├── StreamCard.tsx         # Stream preview card
│   ├── StreamList.tsx         # Grid of streams
│   ├── CreateStream.tsx       # Stream creation form
│   ├── InviteManager.tsx      # Invite link management
│   └── MemberList.tsx         # Stream members with roles
│
├── videos/
│   ├── VideoPlayer.tsx        # HLS video player with controls
│   ├── VideoUpload.tsx        # Drag & drop upload with progress
│   ├── VideoCard.tsx          # Video thumbnail with metadata
│   ├── VideoList.tsx          # Grid of videos
│   ├── TimelineView.tsx       # Timeline with aligned clips
│   ├── ProcessingStatus.tsx   # Processing progress indicator
│   └── SimilarVideos.tsx      # Similar clips recommendations
│
├── search/
│   ├── SearchBar.tsx          # Search input with suggestions
│   ├── SearchResults.tsx      # Search results display
│   └── SearchFilters.tsx      # Filter by game, date, etc.
│
└── ui/
    ├── Button.tsx             # Reusable button component
    ├── Modal.tsx              # Modal dialog
    ├── Toast.tsx              # Notification system
    ├── LoadingSpinner.tsx     # Loading indicators
    └── ErrorBoundary.tsx      # Error handling wrapper
```

## 🔧 API Integration Hooks

### Custom React Hooks
```javascript
// hooks/useAuth.js
export function useAuth() {
  const { data: session } = useSession()
  const [appToken, setAppToken] = useState(null)
  
  const verifyWithBackend = async () => {
    if (session?.accessToken) {
      const response = await fetch('/api/auth/verify', {
        method: 'POST',
        headers: { 'Authorization': `Bearer ${session.accessToken}` }
      })
      const { app_token } = await response.json()
      setAppToken(app_token)
      apiClient.setToken(app_token)
    }
  }
  
  return { session, appToken, verifyWithBackend }
}

// hooks/useStreams.js
export function useStreams() {
  return useQuery('streams', () => apiClient.request('/streams'))
}

// hooks/useMinioUpload.js
export function useMinioUpload() {
  const [uploadState, setUploadState] = useState({
    progress: 0,
    status: 'idle', // 'uploading' | 'processing' | 'complete' | 'error'
    presignedUrl: null,
    videoId: null
  });

  const uploadVideo = async (file, streamId) => {
    try {
      // 1. Get presigned URL from Axum backend
      const response = await apiClient.request('/videos/upload-url', {
        method: 'POST',
        body: JSON.stringify({
          filename: file.name,
          contentType: file.type,
          streamId
        })
      });
      
      const { presignedUrl, videoId } = response;
      setUploadState(prev => ({ ...prev, presignedUrl, videoId }));

      // 2. Upload directly to MinIO
      setUploadState(prev => ({ ...prev, status: 'uploading' }));
      
      const uploadResponse = await fetch(presignedUrl, {
        method: 'PUT',
        body: file,
        headers: { 'Content-Type': file.type }
      });

      if (uploadResponse.ok) {
        setUploadState(prev => ({ ...prev, status: 'processing', progress: 100 }));
        
        // 3. Notify backend of completed upload
        await apiClient.request(`/videos/${videoId}/uploaded`, {
          method: 'POST'
        });
        
        return videoId;
      }
    } catch (error) {
      setUploadState(prev => ({ ...prev, status: 'error' }));
      throw error;
    }
  };

  return { uploadVideo, uploadState };
}

// hooks/useVideoProcessing.js - Enhanced with MinIO status
export function useVideoProcessing(videoId) {
  return useQuery(
    ['video-processing', videoId],
    () => apiClient.request(`/videos/${videoId}/processing`),
    { 
      refetchInterval: (data) => {
        // Stop polling when processing complete (status 0)
        return data?.processingIndex === 0 ? false : 3000;
      },
      enabled: !!videoId 
    }
  )
}

// hooks/useMinioStream.js - Direct MinIO streaming
export function useMinioStream(videoId, processingStatus) {
  const [streamUrls, setStreamUrls] = useState({
    hlsUrl: null,
    thumbnailUrl: null,
    loading: true
  });

  useEffect(() => {
    if (processingStatus === 0) {
      // Video is fully processed, get direct MinIO URLs
      apiClient.request(`/videos/${videoId}/stream-urls`)
        .then(({ hlsUrl, thumbnailUrl }) => {
          setStreamUrls({ hlsUrl, thumbnailUrl, loading: false });
        });
    }
  }, [videoId, processingStatus]);

  return streamUrls;
}
```

## 🎨 UI/UX Features

### Video Processing Indicators
- **Upload Progress**: Real-time progress bar during file upload
- **Processing Stages**: Visual indicators for each processing stage
- **Thumbnail States**: Template → Processing overlay → Final thumbnail
- **Error Handling**: Clear error messages with retry options

### Interactive Timeline View
- **Aligned Clips**: Visual timeline showing trimmed clips positioned relative to original
- **Hover Previews**: Thumbnail previews on timeline hover
- **Click Navigation**: Jump to clip timestamps
- **Sorting**: Clips sorted longest to shortest

### Search & Discovery
- **Real-time Search**: Search-as-you-type with debouncing
- **Speech Content**: Search within video transcripts
- **Visual Results**: Thumbnail previews with timestamp highlights
- **Suggestions**: Auto-complete based on popular searches

### Responsive Design
- **Mobile-first**: Optimized for all screen sizes
- **Touch-friendly**: Large tap targets for mobile
- **Adaptive UI**: Components adjust to available space
- **Dark/Light Mode**: User preference support

## 🛠️ Development Setup

### Tech Stack
- **Framework**: Next.js 14+ with App Router
- **Authentication**: NextAuth.js with Google provider
- **Styling**: Tailwind CSS with shadcn/ui components
- **State Management**: React Query for server state, Zustand for client state
- **Video Player**: HLS.js for adaptive streaming from MinIO
- **Forms**: React Hook Form with Zod validation
- **Backend**: Optimized communication with Axum API (SQLx pooling)

### Environment Variables
```env
# Authentication
NEXTAUTH_SECRET=your-nextauth-secret
NEXTAUTH_URL=http://localhost:3000
GOOGLE_CLIENT_ID=your-google-oauth-client-id
GOOGLE_CLIENT_SECRET=your-google-oauth-client-secret

# Axum Backend API (separated architecture)
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws

# MinIO Object Storage (for direct uploads)
NEXT_PUBLIC_MINIO_ENDPOINT=http://localhost:9000
NEXT_PUBLIC_MINIO_USE_SSL=false

# Optional: Analytics, monitoring
NEXT_PUBLIC_ANALYTICS_ID=your-analytics-id
```

### Development Commands
```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Run tests
npm run test

# Lint code
npm run lint
```

## 📊 Performance Optimizations

### Image & Video Optimization
- **Next.js Image**: Automatic image optimization and lazy loading
- **Video Thumbnails**: WebP format with fallbacks
- **Progressive Loading**: Load videos as user scrolls
- **Caching**: Browser caching for static assets

### Code Splitting
- **Route-based**: Automatic code splitting per page
- **Component-based**: Dynamic imports for heavy components
- **Library Splitting**: Separate bundles for large dependencies

### State Management
- **React Query**: Efficient server state with caching
- **Local Storage**: Persist user preferences
- **Session Storage**: Temporary UI state
- **Memory Management**: Proper cleanup of video players

## 🚀 Deployment

### Production Build
- **Static Generation**: Pre-render public pages
- **API Routes**: Server-side API integration
- **CDN Integration**: Optimized asset delivery
- **Error Monitoring**: Sentry integration for production errors

### Environment Configuration
- **Development**: Local API server
- **Staging**: Staging backend API
- **Production**: Production backend API with CDN
