from typing import Annotated
from fastapi import FastAPI, Request, Form, HTTPException, Cookie
from fastapi.templating import Jinja2Templates
from fastapi.responses import HTMLResponse, RedirectResponse   #opcional pero buena practica 
from jose import JWTError, jwt
from datetime import datetime, timedelta, timezone


SECRETE_KEY = "c95e2beae4d2c4a8dddf272ea10d3a10a67bdb35e036584a018d00a346b56ae6" #clave secreta para firmar el token
TOKEN_SECONDS_EXP = 20 #tiempo de expiracion del token en segundos

db_users = { #simulacion de base de datos, esto no se hace en produccion
    "dylan": {
        "id": 0,
        "username": "dylan",
        "password": "12345#hash"
    },     
    "gabi": {
        "id": 1,
        "username": "gabi",
        "password": "1234#hash"
    },     
}

app = FastAPI()
jinja2_templates = Jinja2Templates(directory="templates")

def get_user(username: str, db:list): #simulacion de base de datos, esto no se hace en produccion
    if username in db_users:
        return db[username]
    return None

def authenticate_user(password: str, password_plane: str): #hashearlo despues, esto no se hace en produccion
    password_clean = password.split("#")[0]
    if password_plane == password_clean:
        return True
    return False

def create_token(data: list):
    data_token = data.copy()
    data_token["exp"] = datetime.now(timezone.utc) + timedelta(seconds=TOKEN_SECONDS_EXP) # desuso data_token["exp"] = datetime.utcnow() + timedelta(seconds=TOKEN_SECONDS_EXP) 
    token_jwt = jwt.encode(data_token, SECRETE_KEY, algorithm="HS256")
    return token_jwt

@app.get("/", response_class=HTMLResponse) #
def root(request: Request):
    return jinja2_templates.TemplateResponse("index.html", {"request": request})

@app.get("/users/dashboard", response_class=HTMLResponse) 
def dashboard(request: Request, access_token: Annotated[str | None, Cookie()] = None):
    if access_token is None:
        return RedirectResponse("/", status_code=302)
    try:
        data_user = jwt.decode(access_token, SECRETE_KEY, algorithms=["HS256"])
        if get_user(data_user["username"], db_users) is None:
            return RedirectResponse("/", status_code=302)
        return jinja2_templates.TemplateResponse("dashboard.html", {"request": request})
    except JWTError:
        return RedirectResponse("/", status_code=302)
            
    
@app.post("/users/login") 
def login(username: Annotated[str, Form()], password: Annotated[str, Form()]):
    user_data = get_user(username, db_users)
    if user_data is None:
        raise HTTPException(
            status_code=401,
            detail="username or password no authorization"
            )
    if not authenticate_user(user_data["password"], password):
        raise HTTPException(
            status_code=401,
            detail="username or password no authorization"
            ) 
    token = create_token({"username": user_data["username"]})   
    
    return RedirectResponse(
        "/users/dashboard",
        status_code=302,
        headers = {"set-cookie": f"access_token={token};  Max-Age={TOKEN_SECONDS_EXP}"}
        )
    
@app.post("/users/logout")
def logout():
    return RedirectResponse("/", status_code=302, headers={
        "set-cookie": "access_token=; Max-Age=0"
        })
