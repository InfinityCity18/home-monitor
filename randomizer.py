import requests
import json
import random
import time

url = 'http://10.21.37.100:8138/monitor'

headers = {
    'Content-Type': 'application/json', 
}


while True:
    data = {
        "l" : random.uniform(0.1, 100000.0),
        "m" : bool(random.getrandbits(1)),
        "h" : random.uniform(30.0, 84.0),
        "t" : random.uniform(20.0, 32.0)
    }
    response = requests.post(url, json=data, headers=headers)
    print(response.status_code)
    time.sleep(300)
