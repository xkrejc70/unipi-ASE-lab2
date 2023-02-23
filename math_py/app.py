from flask import Flask, render_template, request, make_response, jsonify
import requests
import os
import signal

app = Flask(__name__, instance_relative_config=True)

def kill_server():
    os.kill(os.getpid(), signal.SIGINT)

@app.route('/add')
def add():
    a = request.args.get('a', type=float)
    b = request.args.get('b', type=float)
    if a and b:
        return make_response(jsonify(s=a+b), 200)
    else:
        return make_response('Invalid input\n', 400)

@app.route('/sub')
def sub():
    a = request.args.get('a', 0, type=float)
    b = request.args.get('b', 0, type=float)
    return make_response(jsonify(s=a-b), 200)

@app.route('/mul')
def mul():
    a = request.args.get('a', 0, type=float)
    b = request.args.get('b', 0, type=float)
    return make_response(jsonify(s=a*b), 200)

@app.route('/div')
def div():
    a = request.args.get('a', 0, type=float)
    b = request.args.get('b', 0, type=float)
    if b == 0:
        return make_response('Cannot divide by zero\n', 400)
    else:
        return make_response(jsonify(s=a/b), 200)

@app.route('/mod')
def mod():
    a = request.args.get('a', 0, type=int)
    b = request.args.get('b', 0, type=int)
    if b == 0:
        return make_response('Cannot mod by zero\n', 400)
    else:
        return make_response(jsonify(s=a%b), 200)

@app.route('/crash')
def crash():
    print("Crashing")
    kill_server()

def create_app():
    return app

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=5000)