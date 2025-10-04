FROM node:18

WORKDIR /app

# Copy package.json first if you have dependencies
COPY package.json package-lock.json* ./
RUN npm install || true

# Copy the rest of the producer code
COPY . .

CMD ["node", "index.js", "trades_data.csv"]
