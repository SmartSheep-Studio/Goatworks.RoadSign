# Building Backend
FROM golang:alpine as roadsign-server

WORKDIR /source
COPY . .
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -buildvcs -o /dist ./pkg/cmd/main.go

# Runtime
FROM golang:alpine

RUN apk add zip
RUN apk add nodejs npm

COPY --from=roadsign-server /dist /roadsign/server

EXPOSE 81

CMD ["/roadsign/server"]
