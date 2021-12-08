# Belo Backend challenge

Setup inicial:
    En la base de datos ( swapi.db ) tabla swap_config puede configurarse el tiempo de validez de la estimación el fee de la api y un % tolerancia de variacion entre el precio estimado y el precio que se va a ejecutar, para ejecutar o no el swap. Esta configurdo con 30 segundos para ejecutar el swap, 0.1% de fee y 3% de variacion en el precio estimado.
    
   Para correr el servidor :
    
    #cargo run

El sever de la api corre en http://localhost:8099

El endpoint para perdir la estimacion del swap es /spot_swap ( method GET ), en el body debe indicarse par a swapear y cantidad en formato json.
Para convertir USDT en BTC debe enviarse el par USDT-BTC, para convertir BTC en USDT debe enviarse BTC-USDT, osea que se quiere convertir en que, la api detecta de que lado del book tiene que operar. 

Ej del body 
    
    {"pair":"USDT-ETH","quantity":100} convierte 100 usdt en eth

Ej de respuesta de estimacion :

    {'swap_uuid': '886546a5-2cf9-4cc3-8b18-c144e4803b16', 'pair': 'USDT-BTC', 'book': 'BTC-USDT', 'side': 'buy', 'quantity': 100.0, 'estimated_price': 48666.5, 'estimated_qty': 0.0020506917, 'time_satmp': 1638936634}

El endponint para ejecutar un swap estimado con anterioridad es /spot_swap ( mehtod POST ), en el body debe indicarse el swap_uuid entregado por la estimación

Ej del body

    {"swap_uuid":"0b244784-163f-4305-9796-41990adc0837"}

Ej de la respuesta a ejecutar el swap :

    {'swap_uuid': '886546a52cf94cc38b18c144e4803b16', 'pair': 'USDT-BTC', 'book': 'BTC-USDT', 'side': 'buy', 'quantity': '100', 'price': '48665', 'fee': '-0.00000205486', 'swapi_fee': '-0.00000205486', 'fee_currency': 'BTC', 'time_satmp': '1638936634'}

Para correr el test de intergacion :

Setup inicial ( requiere python3 y los modulos requests y json instalados ):

    #python3 test.py


