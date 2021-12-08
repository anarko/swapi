# Belo Backend challenge

Setup inicial:
    #cargo run

El sever de la api corre en http://localhost:8099
El endpoint para perdir la estimacion del swap es /spot_swap ( method GET )
    En el body debe indicarse par a swapear y cantidad en formato json
    *USDT-BTC para convertir USDT en BTC o BTC-USDT para convertir BTC en USDT
    *ej del body {"pair":"USDT-ETH","quantity":100} convierte 100 usdt en eth
    *ej de respuesta de estimacion :
    {'swap_uuid': '886546a5-2cf9-4cc3-8b18-c144e4803b16', 'pair': 'USDT-BTC', 'book': 'BTC-USDT', 'side': 'buy', 'quantity': 100.0, 'estimated_price': 48666.5, 'estimated_qty': 0.0020506917, 'time_satmp': 1638936634}

El endponint para ejecutar un swap estimado con anterioridad es /spot_swap ( mehtod POST )
    En el body debe indicarse el swap_uuid entregado por la estimación
    *ej de la respuesta a ejecutar el swap :
    {'swap_uuid': '886546a52cf94cc38b18c144e4803b16', 'pair': 'USDT-BTC', 'book': 'BTC-USDT', 'side': 'buy', 'quantity': '100', 'price': '48665', 'fee': '-0.00000205486', 'swapi_fee': '-0.00000205486', 'fee_currency': 'BTC', 'time_satmp': '1638936634'}

Para correr el test de intergacion :
Setup inicial ( requiere python3 y los modulos requests y json instalados ):
    #python3 test.py


