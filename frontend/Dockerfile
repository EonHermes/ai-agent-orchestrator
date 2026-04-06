# Build stage
FROM node:20-alpine as builder

WORKDIR /app

# Copy manifests
COPY frontend/package*.json ./
COPY frontend/tsconfig*.json ./
COPY frontend/vite.config.ts ./
COPY frontend/tailwind.config.js ./
COPY frontend/index.html ./
COPY frontend/src ./src
COPY frontend/public ./public

# Install dependencies
RUN npm ci

# Build
RUN npm run build

# Run stage
FROM nginx:alpine

# Copy built assets
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx config
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]