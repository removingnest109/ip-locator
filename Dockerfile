FROM scratch
COPY musl-dist .
EXPOSE 8000
CMD ["/ip-locator"]
