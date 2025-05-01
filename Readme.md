# Stack: Rust (Backend) + React (Frontend)

**Frontend Location**: `/frontend` (Vite/React)

---

## 🚀 Quick Start

### 1. Run the Frontend (React)

```bash
cd frontend    # Navigate to frontend folder
npm install    # Install dependencies
npm run dev    # Start dev server (usually http://localhost:5173)
2. Run the Backend (Rust)
bash
Copy
Edit
cargo run      # From the project root (starts Rust server, e.g., http://localhost:8080)
💡 Tip: Use two terminal windows/tabs to run both simultaneously.

📂 Project Structure
csharp
Copy
Edit
.
├── frontend/          # React app (Vite/CRA)
│   ├── src/           # React components
│   ├── public/        # Static assets
│   └── package.json   # Frontend deps
├── src/               # Rust backend code
├── Cargo.toml         # Rust config
└── ...
🔌 API Connection
Configure your React app to talk to the Rust backend:

Example (Frontend Axios Call)
js
Copy
Edit
axios.get("http://localhost:8080/api/data")
  .then(response => console.log(response.data));
Proxy Setup (Optional)
To avoid CORS, add this to frontend/vite.config.js:

js
Copy
Edit
export default defineConfig({
  server: {
    proxy: {
      '/api': 'http://localhost:8080' // Rust backend port
    }
  }
});
🛠 Build for Production
Build React:
bash
Copy
Edit
cd frontend
npm run build
Serve with Rust (e.g., using actix-web-static-files):
rust
Copy
Edit
// Example: Embed the frontend build in Rust
serve_static_files!("frontend/dist");
❓ Troubleshooting
Port Conflicts?
Change ports in:

React: frontend/vite.config.js (e.g., server.port)

Rust: src/main.rs (e.g., .bind("127.0.0.1:8000"))

Missing Dependencies?
Run npm install (frontend)

Run cargo build (backend)
