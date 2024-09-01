# Veljavni endpointi

Noben HTTP request ne sme vsebovati več kot 16 headerjev. 
Če se endpoint konča na znak `/`, je ta končni znak ignoriran (oboje je veljavno).

POZOR: V učilnici in primeru implemenacije sta različni specifikaciji, za to
na kateri endpoint se naj generator registrirar (na `/generator/` ali `/project/`), preferenco smo dali
`/project/`, lahko pa to spremenite pri zagonu z `--register-endpoint "/nek_čudni_endpoint"`.

## Ponudnik

### ```GET /ping```
Vrne response, ki pove, da je ponudnik še online: 
```json
{
  "name": "<ime ponudnika>",
  "ip": "<ip ponudnika>",
  "port": (port ponudnika)
}
```

### ```GET /sequence/```
Get request **ne sme** imeti telesa. 
Vrne response, ki vsebuje signature vseh zaporedij, ki jih ponuja ta ponudnik.
```json
[
  {
    "name": "<ime zaporedja 1>",
    "description": "<opis zaporedja 1>",
    "parameters": <število parametrov zaporedja 1>,
    "sequences": <število zaporedij, ki jih sprejme zaporedje 1>
  },
  ...
]
```

### ```POST /sequence/<ime zaporedja>```
Telo POST requesta naj bo oblike:
```json
{
  "range": {
    "from": <od>,
    "to":   <do>,
    "step": <korak>
  },
  "parameters": [<parameter 1>, ...],
  "sequences": [
    {"name": "<ime zaporedja 1>", "parameters": [<parametri>], "sequences": [<zaporedja>]},
    ...
    ]
}
```
Če je request veljaven in je mogoče zaporedje generirati vrne:
```json
[ <1. generiran element>, ... ]
```
## Centralni strežnik

### ```GET /ping```
Vrne response, ki pove, da je centralni strežnik še online: 
```json
{
  "name": "<ime strežnika>",
  "ip": "<ip strežnika>",
  "port": <port strežnika>
}
```

### ```GET /project/```
Get request **ne sme** imeti telesa. 
Vrne response, ki vsebuje podatke o vseh registriranih generatorjih, ki jih ponuja ta centralni strežnik.
```json
[
  {
    "name": "<ime ponudnika 1>",
    "ip": "<ip ponudnika 1>",
    "port": <port ponundika 1>
  },
  ...
]
```

### ```POST /project/```
Telo POST requesta naj bo oblike:
```json
{
    "name": "<ime ponudnika>",
    "ip": "<ip ponudnika>",
    "port": <port ponundika>
}
```
Vrne response o statusu (200 OK) če je registracija uspešna.