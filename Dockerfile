FROM rust:1.75-alpine

# Install git for smolcase operations
RUN apk add --no-cache git musl-dev

# Set working directory
WORKDIR /app

# Copy everything
COPY . .

# Build smolcase
RUN cargo build --release

# Make binary available globally
RUN cp target/release/smolcase /usr/local/bin/

# Set working directory for user
WORKDIR /workspace

# Default command
CMD ["smolcase", "--help"]
