"""Main FastAPI application."""

from fastapi import FastAPI

app = FastAPI(
    title="Super Pixeled API",
    description="Control your LED panel",
    version="0.1.0",
)


@app.get("/health")
async def health_check() -> dict[str, str]:
    """Health check endpoint."""
    return {"status": "ok"}
