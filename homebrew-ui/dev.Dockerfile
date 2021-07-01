FROM node:14 as builder

WORKDIR /usr/src/app

COPY package.json /usr/src/app
RUN npm install

COPY . /usr/src/app

ENTRYPOINT [ "npm", "run" ]
CMD [ "dev" ]
