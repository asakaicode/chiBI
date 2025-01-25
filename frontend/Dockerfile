FROM node:23.6.1-alpine3.20 AS deps

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY package.json package-lock.json ./
RUN npm install

FROM deps AS build
COPY . .
RUN npm run build

FROM build AS dev
EXPOSE 3000
CMD ["npm", "run", "dev"]