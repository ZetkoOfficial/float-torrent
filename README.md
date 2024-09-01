# float-torrent
Projektna naloga za predmet Programiranje II na FMF, ki implementira ponudnik zaporedij in centralni strežnik, tako da lahko ponudnik komunicira z drugimi preko njega.

- [O endpointih](doc/endpoint.md)
- [O komunikaciji](doc/komunikacija.md)

## Lokalno implementirana zaporedja
Lokalno so implementrirana naslednja zaporedja, katerih implmentacie najdete v
[sequence_providers/implementations/](src/common/sequnce_providers/implementations/).
Svoje implementacije zaporedij lahko dodate direktno v definicji funkcije `ProviderManager::new`, lahko pa tudi kar med runtime-om, da jih dodate v `ProviderManager.local_providers`.

| kratko ime zaporedja | opis                 |
|----------------------|----------------------|
| `const`              | Konstantno zaporedje | 
| `drop`               | Naprej zamaknjeno zaporedje |
| `sum`                | Vsota dveh zaporedij |
| `prod`               | Produkt dveh zaporedij |
| `arithmetic`         | Aritmetično zaporedje |
| `geometric`          | Geometrijsko zaporedje |
| `linear_rec_h`       | Linearno rekurzivno zaporedje |
| `lin_com`            | Linearna kombinacija zaporedij |
| `round`              | Zaporedje zaokroženo na nekaj decimalk |
| `power_mod`          | Veriga, ki jo dobimo, če z nekim številom začnemo, ga potenciramo s $p$ v nekem kolobarju $\mathbb{Z}_M$ in ta postopek nadaljujemo
| `p_euler`            | (Najverjetneje) Po velikosti urejena števila $M$ za katere je $\varphi(M)$ potenca nekega praštevila. Alternativno elementi oblike $2^{\alpha} \prod f_{\delta_i}$, kjer so $f_{\delta_i}$ različna fermatova praštevila.

Več o njihovem delovanju in signaturi, si lahko preberete na
endpointu `GET /sequence/` ponudnika.