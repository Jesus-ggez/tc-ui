from fastapi.responses import HTMLResponse
from fastapi import FastAPI

#.?
from src.components import Index

#<·
app: FastAPI = FastAPI()


@app.get('/', response_class=HTMLResponse)
def index() -> str:
    html: str = Index().as_tag()

    with open('_.txt', 'w') as doc:
        doc.write(html)

    return html

if __name__ == '__main__':
    from uvicorn import run

    run(
        app=app,
        port=8000,
    )

