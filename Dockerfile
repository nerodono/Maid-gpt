FROM python:alpine

ENV PATH="$PATH:/root/.local/bin"
RUN python -m pip install --user poetry

WORKDIR /app
COPY . .

RUN apk add alpine-sdk linux-headers
RUN poetry install

CMD poetry run python main.py