FROM node as builder

WORKDIR /usr/src/app

COPY package.json /usr/src/app
RUN npm install

COPY . /usr/src/app
RUN npm run build


FROM nginx

COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

EXPOSE 80
RUN chown nginx.nginx /usr/share/nginx/html/ -R
