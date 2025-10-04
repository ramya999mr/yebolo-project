# Use a small Node.js image
FROM node:16-alpine

# Set working directory
WORKDIR /app

# Copy only package.json first (for caching)
COPY package*.json ./

# Install only production dependencies (lighter)
RUN npm install --production

# Copy rest of the code
COPY . .

# Build frontend
RUN npm run build

# Expose port
EXPOSE 3000

# Start app
CMD ["npm", "start"]