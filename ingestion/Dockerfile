# Use a small Node.js image
FROM node:16-slim AS builder

# Set working directory
WORKDIR /app

# Copy only package.json first (for caching)
COPY package*.json ./

# Install ALL dependencies (so build works)
RUN npm install

# Copy rest of the code
COPY . .

# Build frontend
RUN npm run build

# Production image
FROM node:16-alpine
WORKDIR /app
COPY --from=builder /app ./

# Expose port
EXPOSE 3000

# Start app
CMD ["npm", "start"]
