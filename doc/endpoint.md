# Veljavni endpointi

Noben HTTP request ne sme vsebovati več kot 8 headerjev. 
Pomembno je tudi, da se vsak endpoint konča z znakom `/`.

## Ponudnik

### ```GET /ping/```
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

### ```POST /sequence/<ime zaporedja>/```
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

### ```GET /ping/```
Vrne response, ki pove, da je centrali strežnik še online: 
```json
{
  "name": "<ime strežnika>",
  "ip": "<ip strežnika>",
  "port": <port strežnika>
}
```

### ```GET /generator/```
Get request **ne sme** imeti telesa. 
Vrne response, ki vsebuje podatke o vseh registriranih ponudnikih, ki jih ponuja ta generator.
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

### ```POST /generator/```
Telo POST requesta naj bo oblike:
```json
{
    "name": "<ime ponudnika>",
    "ip": "<ip ponudnika>",
    "port": <port ponundika>
}
```
Vrne response o statusu (200 OK) če je registracija uspešna.