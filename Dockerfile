# Step 1: Build the Rust docs
FROM rust:latest AS builder

# Set work directory
WORKDIR /usr/src/app

# Copy the repo into the container
COPY . .

# Generate the documentation
RUN cargo doc --no-deps

# Step 2: Serve the documentation using a simple server
FROM nginx:alpine

# Copy the generated docs to the nginx html folder
COPY --from=builder /usr/src/app/target/doc /usr/share/nginx/html

# Expose the default nginx port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
