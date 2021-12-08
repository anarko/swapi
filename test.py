import requests
import json

if __name__ == '__main__':
    s = requests.Session()

    print("Pedimos precio de convertir 100 USDT en ETH")
    payload = '{"pair":"USDT-ETH","quantity":100}'
    r = s.get('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("-"*200)
    print("Ejecutamos el swap")
    payload = '{"swap_uuid":"'+swap_rta['swap_uuid']+'"}'
    r = s.post('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("="*200)
    print("Pedimos precio de convertir 0.5 ETH en USDT")
    payload = '{"pair":"ETH-USDT","quantity":0.5}'
    r = s.get('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("-"*200)
    print("Ejecutamos el swap")
    payload = '{"swap_uuid":"'+swap_rta['swap_uuid']+'"}'
    r = s.post('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("-*"*100)
    print("Pedimos precio de convertir 100 USDT en BTC")
    payload = '{"pair":"USDT-BTC","quantity":100}'
    r = s.get('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("-"*200)
    print("Ejecutamos el swap")
    payload = '{"swap_uuid":"'+swap_rta['swap_uuid']+'"}'
    r = s.post('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("="*200)
    print("Pedimos precio de convertir 0.05 BTC en USDT")
    payload = '{"pair":"BTC-USDT","quantity":0.05}'
    r = s.get('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("-"*200)
    print("Ejecutamos el swap")
    payload = '{"swap_uuid":"'+swap_rta['swap_uuid']+'"}'
    r = s.post('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("-*"*100)
    print("Pedimos precio de convertir 100 USDC en AAVE")
    payload = '{"pair":"USDC-AAVE","quantity":100}'
    r = s.get('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("-"*200)
    print("Ejecutamos el swap")
    try:
        payload = '{"swap_uuid":"'+swap_rta['swap_uuid']+'"}'
        r = s.post('http://localhost:8099/spot_swap', data=payload)
        swap_rta = json.loads(r.text)
        print(swap_rta)
    except :
        print("AAVE NO EXISTE EN LA TESTNET")
    print("="*200)
    print("Pedimos precio de convertir 0.05 AAVE en USDC")
    payload = '{"pair":"AAVE-USDC","quantity":0.05}'
    r = s.get('http://localhost:8099/spot_swap', data=payload)
    swap_rta = json.loads(r.text)
    print(swap_rta)
    print("-"*200)
    print("Ejecutamos el swap")
    try:
        payload = '{"swap_uuid":"'+swap_rta['swap_uuid']+'"}'
        r = s.post('http://localhost:8099/spot_swap', data=payload)
        swap_rta = json.loads(r.text)
        print(swap_rta)
    except :
        print("AAVE NO EXISTE EN LA TESTNET")
    print("-"*200)