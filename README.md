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

Več o njihovem delovanju in signaturi, si lahko preberete na
endpointu `GET /sequence/` ponudnika.