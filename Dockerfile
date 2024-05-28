FROM scratch
COPY musl-dist .
CMD ["/ip-locator"]
