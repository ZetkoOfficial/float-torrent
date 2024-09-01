# Komunikacija in napake
Ponudnik lahko komunicira z drugimi ponudniki, centralnim strežnikom ali uporabnikom preko HTTP protokola. Med komunikacijo se lahko zgodijo napake, ki jih ta implementacija vrne kot JSON oblike s statusom ```400 Bad Request```.
```json
{
  "error": "<vrsta napake>",
  "message": "<opis napake>",
  "extra": <json dodatnih podatkov o napaki>/null
}
```

Pri zagonu ponudnika se ta najprej poskusi registrirati z osrednjim strežnikom z endpointom podanim
kot argument funckije (zaradi neskladnosti specifikacij). Po defaultu je to endpoint `/project`.

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
Zahtevamo, da je headerjev najevč 16 v HTTP verziji 1.1.

### Branje Requesta
Trenutna implementacija ne loči med `GET` in `POST`, ampak zahteva v primeru, da uporabnik 
želi narediti `GET` request, da je `body` prazen. (Če uporabnik sledi specifikaciji v učilnici to ne bo povzorčilo težav)  

### Branje Response
Zahtevamo, da ima response header `Content-Length`, če ima nek body.

# Zahteva po zaporedjih
Kadar od naše implementacije ponudnika zahtevamo zaporedje s specifično signaturo
najprej preveri, če to zaporedje implementira že lokalno. Če ga, potem uporabi lokalno implementacijo, če ne, pa preveri svoj lokalni register oddaljenih ponudnikov zaporedij in izmed njih izbere nakjučnega, ki se ujema s signaturo.

V primeru, da oddaljeni (remote) ponudnik vrne error, potem error preposreduje uporabniku in **ne poskusi znova**, to je na uporabniku/centralnemu strežniku
(da morda blacklista ali odregistrira ponudnika).

# Osvežitev notranjega registra
Kot omenjeno prej, se na vsake toliko časa ponudniku osveži notranji register oddaljenih ponudnikov zaporedij. To je storjeno, zato, ker si med threadi te ponudnike delimo in bi upočasnitev po vsaki zahtevi močno vplivala na vse threade(saj za nekaj časa `write` dostop blokira). Raje sem se zato odločil za manj pogosto, periodično posodabljanje.  