# float-torrent
Projektna naloga za predmet Programiranje II na FMF, ki implementira ponudnik zaporedij in centralni strežnik, tako da lahko ponudnik komunicira z drugimi preko njega.

- [O endpointih](doc/endpoint.md)
- [O komunikaciji](doc/komunikacija.md)

## Lokalno implementirana zaporedja
Lokalno so implementrirana naslednja zaporedja:
| kratko ime zaporedja | opis                 |
|----------------------|----------------------|
| `const`              | Konstantno zaporedje | 
| `drop`               | Naprej zamaknjeno zaporedje |
| `sum`                | Vsota dveh zaporedij |
| `arithmetic`         | Aritmetično zaporedje |
| `geometric`          | Geometrijsko zaporedje |
| `linear_rec_h`       | Linearno rekurzivno zaporedje |
| `lin_com`            | Linearna kombinacija zaporedij |
| `round`              | Zaporedje zaokroženo na nekaj decimalk |
| `power_mod`          | Veriga, ki jo dobimo, če z nekim številom začnemo, ga potenciramo s `p` v nekem kolobarju `Z_M` in ta postopek nadaljujemo
| `simple_power_mod`          | (Najverjetneje) Po velikosti urejeni `M` za katere vse verige `power_mod`, pri potenciranjem s praštevilom, elementov `Z_M` po nekaj časa dosežejo število 0 ali 1.

Več o njihovem delovanju in signaturi, si lahko preberete na
endpointu `GET /sequence/` ponudnika.