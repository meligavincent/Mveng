# Mveng
🌍 African AI assistant powered by Axum + Whisper + Ollama. Digital griot sharing African stories, history &amp; culture. Rooted in Cameroon's Yesum traditions. Named after scholar Engelbert Mveng. Your bridge to African wisdom. 🇨🇲✨


# VoiceAI Assistant 🎙️🤖

A conversational AI agent that processes voice input, transcribes it using OpenAI Whisper, generates responses with Ollama (open-source LLM), and provides both text and optional voice responses.

## 🚀 Features

- **Voice Input Processing**: Upload audio files for transcription
- **Real-time Transcription**: Uses OpenAI Whisper for accurate speech-to-text
- **Open Source LLM**: Powered by Ollama with Llama 2 or other open models
- **RESTful API**: Built with Axum (Rust) for high performance
- **Containerized**: Complete Docker Compose setup
- **WebSocket Support**: Real-time conversation updates
- **Multi-format Audio**: Supports WAV, MP3, M4A, and more
- **Response Streaming**: Stream LLM responses in real-time

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client App    │───▶│   Axum Server   │───▶│  Whisper Model  │
│  (Web/Mobile)   │    │    (Rust)       │    │  (Transcription)│
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │  Ollama Server  │
                       │ (Open Source LLM)│
                       └─────────────────┘
```

## 📋 Prerequisites

- Docker and Docker Compose
- At least 8GB RAM (for LLM models)
- NVIDIA GPU (optional, for faster inference)

## 🛠️ Installation

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/voiceai-assistant
cd voiceai-assistant
```

2. **Start the services**
```bash
docker-compose up -d
```

3. **Pull the LLM model** (first run only)
```bash
docker-compose exec ollama ollama pull llama2
```

4. **Verify installation**
```bash
curl http://localhost:3000/health
```

## 🐳 Docker Services

### Core Services
- **axum-server**: Main API server (Port 3000)
- **whisper-service**: Speech-to-text transcription (Port 8001)
- **ollama**: Open-source LLM inference (Port 11434)
- **redis**: Session and cache management (Port 6379)

### Optional Services
- **postgres**: Conversation history storage (Port 5432)
- **nginx**: Load balancer and static file serving (Port 80)

## 📡 API Endpoints

### Health Check
```bash
GET /health
```

### Upload Audio for Conversation
```bash
POST /api/v1/chat/voice
Content-Type: multipart/form-data

Form data:
- audio: <audio_file>
- session_id: <optional_session_id>
```

### Text-based Chat
```bash
POST /api/v1/chat/text
Content-Type: application/json

{
  "message": "Hello, how are you?",
  "session_id": "optional-session-id"
}
```

### WebSocket Real-time Chat
```bash
WS /ws/chat
```

### Get Conversation History
```bash
GET /api/v1/conversations/{session_id}
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file:

```env
# Server Configuration
RUST_LOG=info
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Whisper Configuration
WHISPER_MODEL=base
WHISPER_LANGUAGE=auto

# Ollama Configuration
OLLAMA_HOST=ollama:11434
OLLAMA_MODEL=llama2

# Database (Optional)
DATABASE_URL=postgresql://voiceai:password@postgres:5432/voiceai
REDIS_URL=redis://redis:6379

# Audio Processing
MAX_AUDIO_SIZE=50MB
SUPPORTED_FORMATS=wav,mp3,m4a,flac,ogg

# Response Configuration
ENABLE_STREAMING=true
MAX_RESPONSE_LENGTH=2000
```

### Model Selection

Supported Ollama models:
- `llama2` (7B) - Good balance of speed and quality
- `codellama` (7B) - Better for technical questions
- `mistral` (7B) - Fast and efficient
- `llama2:13b` - Higher quality, slower

Change model in docker-compose.yml or via API:
```bash
curl -X POST http://localhost:11434/api/pull -d '{"name":"mistral"}'
```

## 📁 Project Structure

```
voiceai-assistant/
├── src/
│   ├── main.rs              # Main application entry
│   ├── handlers/            # HTTP request handlers
│   │   ├── chat.rs
│   │   ├── health.rs
│   │   └── websocket.rs
│   ├── services/            # Business logic
│   │   ├── whisper.rs       # Whisper integration
│   │   ├── ollama.rs        # Ollama LLM client
│   │   └── audio.rs         # Audio processing
│   ├── models/              # Data structures
│   │   ├── conversation.rs
│   │   └── message.rs
│   └── utils/               # Utilities
│       ├── config.rs
│       └── error.rs
├── docker-compose.yml       # Container orchestration
├── Dockerfile              # Rust app container
├── whisper.Dockerfile      # Whisper service container
├── migrations/             # Database migrations
├── static/                 # Static web files
├── tests/                  # Integration tests
└── README.md
```

## 🚀 Usage Examples

### Voice Conversation (cURL)
```bash
# Record audio and send to API
curl -X POST \
  -F "audio=@recording.wav" \
  -F "session_id=my-session" \
  http://localhost:3000/api/v1/chat/voice
```

### Text Conversation
```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Explain quantum computing in simple terms",
    "session_id": "my-session"
  }' \
  http://localhost:3000/api/v1/chat/text
```

### WebSocket (JavaScript)
```javascript
const ws = new WebSocket('ws://localhost:3000/ws/chat');
ws.send(JSON.stringify({
  type: 'message',
  content: 'Hello AI!',
  session_id: 'browser-session'
}));
```

## 📊 Performance & Monitoring

### Metrics Endpoint
```bash
GET /metrics  # Prometheus metrics
```

### Key Metrics
- Request latency (transcription, LLM inference)
- Audio processing time
- Memory usage
- Active WebSocket connections
- Error rates

### Logging
Logs are structured JSON and can be viewed:
```bash
docker-compose logs -f axum-server
```

## 🧪 Development

### Local Development Setup
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Start dependencies only
docker-compose up -d whisper-service ollama redis

# Run the Axum server locally
cargo run

# Run tests
cargo test
```

### Adding New Models
1. Update the supported models in `src/services/ollama.rs`
2. Add model-specific configurations
3. Test with different model sizes

### Custom Audio Processing
Extend `src/services/audio.rs` for additional formats or preprocessing.

## 🔒 Security Considerations

- Audio files are temporarily stored and cleaned up after processing
- Rate limiting on API endpoints
- Input validation for all uploads
- Session-based conversation isolation
- Optional authentication middleware ready

## 🚀 Deployment

### Production Deployment
```bash
# Build optimized images
docker-compose -f docker-compose.prod.yml up -d

# Scale services
docker-compose up -d --scale axum-server=3
```

### Cloud Deployment
- Configure persistent volumes for model storage
- Use managed Redis/PostgreSQL for production
- Set up proper SSL/TLS termination
- Configure horizontal pod autoscaling

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙋‍♂️ Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/voiceai-assistant/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/voiceai-assistant/discussions)
- **Wiki**: [Project Wiki](https://github.com/yourusername/voiceai-assistant/wiki)

## 🎯 Roadmap

- [ ] **v1.1**: Voice synthesis for AI responses (TTS)
- [ ] **v1.2**: Multiple language support
- [ ] **v1.3**: Custom model fine-tuning interface
- [ ] **v1.4**: Mobile app (React Native)
- [ ] **v1.5**: Plugin system for custom integrations
- [ ] **v2.0**: Multi-modal inputs (text + images)

---

**Built with ❤️ using Rust, Axum, Whisper, and Ollama**
