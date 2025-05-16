from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import subprocess

app = FastAPI()

class BatchRequest(BaseModel):
    a: int
    b: int

@app.post("/submit-batch")
def submit_batch(data: BatchRequest):
    # Simula execução local da prova SP1 (em produção, use subprocess com segurança)
    try:
        output = subprocess.check_output(["cargo", "run", "-p", "host"], cwd="../consus-zkvm", text=True)
        return {"status": "ok", "output": output}
    except subprocess.CalledProcessError as e:
        raise HTTPException(status_code=500, detail=f"Execution failed: {e.output}")
