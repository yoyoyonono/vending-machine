import hashlib
import hmac
import json
import random
import requests
import sys
import time
import websockets




def generate_sha512_hmac(data, secret):
    return hmac.new(secret.encode(), data.encode(), hashlib.sha512).hexdigest()


print(generate_sha512_hmac('14,5d76d323-d1f6-4a38-8231-0063f9581c98,NBQM,test1,test2',
      'a7e3512f5032480a83137793cb2021dc'))

with open('key.txt') as f:
    s = f.read().splitlines()
    pan = s[0].split(' ')[1]
    merchant_code = s[1].split(' ')[2]
    secret = s[2].split(' ')[2]
    username = s[3].split(' ')[1]
    password = s[4].split(' ')[1]

print(pan, merchant_code, secret)

amount = str(sys.argv[1])
remarks1 = 'test1'
remarks2 = 'test2'
prn = str(int(time.time()))

x = requests.post('https://merchantapi.fonepay.com/api/merchant/merchantDetailsForThirdParty/thirdPartyDynamicQrDownload',
                  json={
                      "amount": amount,
                      "remarks1": remarks1,
                      "remarks2": remarks2,
                      "prn": prn,
                      "merchantCode": merchant_code,
                      "dataValidation": generate_sha512_hmac(f'{amount},{prn},{merchant_code},{remarks1},{remarks2}', secret),
                      "username": username,
                      "password": password
                  })

response = x.json()

print(response['qrMessage'])
with open('qr.txt', 'w') as f:
    f.write(response['qrMessage'])
print(response['merchantWebSocketUrl'])
with open ('url.txt', 'w') as f:
    f.write(response['merchantWebSocketUrl'])

