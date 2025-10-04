FROM node:18

WORKDIR /app

# Copy dependencies first
COPY package*.json ./
RUN npm install --only=production

# Copy rest of the frontend code
COPY . .

# Build Next.js app
RUN npm run build

# Expose port (default for Next.js)
EXPOSE 3000

CMD ["npm", "start"]