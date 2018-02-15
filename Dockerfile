FROM rustlang/rust:nightly

RUN git clone https://github.com/atsushi130/testy.io.git
WORKDIR testy.io

CMD cargo run --release
