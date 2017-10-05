FROM alpine:3.6

RUN apk update && apk add bash ruby rust cargo
RUN gem install bundler --no-document
RUN adduser -D dave
