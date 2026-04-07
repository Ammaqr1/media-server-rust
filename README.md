# 🎬 Rust Media Streaming Server

A **high-performance media streaming server** built with **Rust + Axum** that supports:

* 📤 Video Upload
* 📃 Video Listing
* 🎥 Video Streaming
* ⏩ Seek / Range Requests (like YouTube)
* 🚫 Duplicate File Prevention
* 🎞️ Format Validation (mp4, mkv, webm)
* ⚡ Chunked Streaming for Fast Playback

---

# 🚀 Demo

## Upload UI

<img width="1913" height="1003" alt="Upload UI" 
src="https://github.com/user-attachments/assets/5508c631-71c2-4374-82cb-8c8132ade7e7" />


## Streaming Demo

[Screencast From 2026-04-07 17-41-33.webm](https://github.com/user-attachments/assets/57287f8f-576f-40c9-a4b1-d14950be6d6f)



# ✨ Features

### 🎬 Real Media Streaming

* Instant playback
* Buffer while playing
* Seek video position
* Supports large files

### 📤 Upload Validation

* Reject duplicate filenames
* Reject invalid formats
* Reject empty filenames

### 📃 Video Listing

* Automatically lists uploaded videos
* Click to stream

---

# 🏗️ Built With

* Rust
* Axum
* Tokio
* Tower HTTP

---

# 📁 Project Structure

```
.
├── src/
│   ├── main.rs
│   ├── stream.rs
│   ├── upload.rs
│   └── list.rs
│
├── videos/
├── static/
│   └── index.html
│
└── Cargo.toml
```

---

# ⚙️ Setup

## 1. Clone Repo

```
git clone https://github.com/yourusername/rust-media-server.git
cd rust-media-server
```

## 2. Create Required Folders

```
mkdir videos
mkdir static
```

## 3. Install Dependencies

```
cargo build
```

## 4. Run Server

```
cargo run
```

---

# 🌐 Open in Browser

```
http://localhost:3000
```

---

# 📡 API Endpoints

## Upload Video

```
POST /upload
```

## List Videos

```
GET /videos
```

## Stream Video

```
GET /stream/:filename
```

---

# 🧠 How Streaming Works

This project uses:

* HTTP Range Requests
* Chunked Streaming
* Async File Reading

This allows:

* Instant playback
* Seek functionality
* Efficient memory usage

---

# 🔥 Example

Click video:

```
http://localhost:3000/stream/sample.mp4
```

---

# 🛠️ Future Improvements

* Video thumbnails
* Authentication
* Upload progress bar
* Delete videos
* Cloud storage (S3)
* HLS streaming

---

# 📸 Screenshots

Place your screenshots in:

```
docs/
```

Example:

```
docs/ui.png
```

---

# 🎥 Demo Video

Place video demo in:

```
docs/demo.mp4
```

---

# ⭐ Why This Project is Useful

This project demonstrates:

* Rust Backend Development
* Async Programming
* Media Streaming
* HTTP Range Requests
* File Upload Handling

Perfect for:

* Portfolio
* Resume
* Rust Backend Jobs

---
