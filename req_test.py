import requests

refresh_token = 19732
consumer_key = 19381793
grant_type = "refresh_token"

resp = requests.post('https://api.tdameritrade.com/v1/oauth2/token', data={
    refresh_token: refresh_token,
    consumer_key: "OD8T1O14POUWY00BJJPGQBIPWPQ8PNWZ",
    grant_type: grant_type,
})

print(resp)
print(resp.text)