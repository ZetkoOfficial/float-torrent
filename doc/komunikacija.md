# Komunikacija in napake
Ponudnik lahko komunicira z drugimi ponudniki, centralnim strežnikom ali uporabnikom preko HTTP protokola. Med komunikacijo se lahko zgodijo napake, ki jih ta implementacija vrne kot JSON oblike s statusom ```400 Bad Request```.
```json
{
  "error_type": "<vrsta napake>",
  "message": "<opis napake>",
  "extra": <json dodatnih podatkov o napaki>/null
}
```

# Branje/pošiljanje HTTP requestov in responsov
## Pisanje HTTP
Naša implementacija piše takole(po vsaki vrstici je `\r\n`):
### `GET` Request

```http
GET {endpoint} HTTP/1.1
Host: {host}
# prazna vrstica
```
### `POST` Request
```http
POST {endpoint} HTTP/1.1
Host: {host}
Content-Type: application/json
Content-Length: {dolzina_body}
# prazna vrstica
{body}
```

### Response
```http
HTTP/1.1 {status}
Content-Type: text/html
Content-Length: {dolzina_body}
# prazna vrstica
{body}
```

## Branje HTTP 
Zahtevamo, da je headerjev najevč 8 in da je request brez `body` velik največ 16KB.

### Branje Requesta
Tukaj dodatno zahtevamo še, da je celoten request velik največ 16KB.

Trenutna implementacija ne loči med `GET` in `POST`, ampak zahteva v primeru, da uporabnik 
želi narediti `GET` request, da je `body` prazen.

### Branje Response
Tukaj dodatno zahtevano da ima response header `Content-Length`, lahko pa ima `body` poljubne velikosti (tudi večji od 16KB). 

# Zahteva po zaporedjih
Kadar od našo implementacije ponudnika zahtevamo zaporedje s specifično signaturo
najprej preveri, če to zaporedje implementria že lokalno. Če ga, potem uporabi lokalno implementacijo, če ne, pa preveri svoj lokalni register oddaljenih ponudnikov zaporedij in izmed njih izbere nakjučnega, ki se ujema s signaturo.

V primeru, da oddaljeni (remote) ponudnik vrne error, potem error preposreduje uporabniku in **ne poskusi znova**, to je na uporabniku/centralnemu strežniku
(da morda blacklista ali odregistrira ponudnika).

# Osvežitev notranjega registra
Kot omenjeno prej, se na vsake toliko časa ponudniku osveži notranji register oddaljenih ponudnikov zaporedij. To je storjeno, zato, ker si med threadi te ponudnike delimo in bi upočasnitev po vsaki zahtevi močno vplivala na vse threade(saj za nekaj časa `write` dostop blokira). Raje sem se zato odločil za manj pogosto, periodično posodabljanje.  