# Build stage
FROM node:18-alpine AS builder

WORKDIR /usr/src/app

# Copy package files
COPY package.json package-lock.json ./

# Install dependencies
RUN npm ci

# Copy source
COPY . .

# Build
RUN npm run build

# Runtime stage
FROM nginx:alpine AS runtime

# Copy nginx config (we'll create this separately if needed)
# COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy built files
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

# Expose port
EXPOSE 80

# Run nginx
CMD ["nginx", "-g", "daemon off;"]