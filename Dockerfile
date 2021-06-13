FROM ubuntu

WORKDIR /opt/SVGcolour

RUN apt update

#install rust stuff
RUN apt install cargo rustc make -y

RUN ls -ltr
COPY ./program ./
RUN cargo build

CMD ["cargo","run"]


