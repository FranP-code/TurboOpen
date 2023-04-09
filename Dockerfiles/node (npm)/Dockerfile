FROM mhart/alpine-node:14

RUN apk add curl

COPY [".", "/usr/src"]

WORKDIR "/usr/src"

RUN npm install

ARG NODE_ENV

CMD if [ "$NODE_ENV" = "develop" ] ; then npm run dev ; else npm run start ; fi