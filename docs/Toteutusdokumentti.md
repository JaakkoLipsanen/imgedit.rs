# Toteutusdokumentti

## Yleisrakenne

Ohjelma on melko yksinkertainen. Ohjelma lukee komentoriviltä prosessoitavan kuvan polun, polun mihin prosessoitu kuva tallennetaan sekä listan filttereitä.
Komentoriviargumenttien lukeminen tapahtuu `src/main.rs` tiedostossa. Filtterien lukeminen argumenteista tapahtuu `src/filters/parse.rs` tiedostossa.

Komentoriviargumenttien lukemisen jälkeen kaikki filtterit prosessoidaan syötteenä saatuun kuvaan.
Filtterien prosessointi tapahtuu `src/filters/mod.rs` tiedoston kautta, mutta varsinaiset filtterit ovat toteutettu `src/filters/implementations.rs` tiedostossa.
Tässä tiedostossa on kaikki sovelluksen tukemat filtterit, yksi funktio per filtteri. Jotkin filtterit ovat toteutettu kokonaan yhdessä funktiossa, kun taas toiset filtterit kutsuvat muita funktioita.

Jotkin filttereistä on toteutettu niin sanotun [kernelin](<https://en.wikipedia.org/wiki/Kernel_(image_processing)>) avulla. Kernel on matriisi, joka kerrotaan kuvan jokaisen pikselin ja kyseisen pikselin läheisten pikselien arvolla.
Tällä tekniikalla voi toteuttaa monta filtteriä hyvin yksinkertaisesti.

## Aika ja tilavaativuudet

Jokaisen ohjelman filtterin aikavaativuus on `O(n)`. Kerneliin perustuvat filtterit, kuten `unsharp-mask` ja `blur` käyttävät joko 3x3 tai 5x5 kokoista kerneliä, jolloinka aikavaativuuden voitaisiin ajatella olevan `O(n)*O(x)` missä `n` on kuvan koko ja `x` on matriisin koko. Mutta koska kernelinä käytetyn matriisin koko on aina jokaisessa filtterissä vakio, ei ne vaikuta algoritmien aikavaativuuteen. Koko ohjelman aikavaativuus on siten `O(n) * O(|f|)`, missä `n` on kuvan koko pikseleissä ja `|f|` on filtterien lukumäärä.

Jokaisen ohjelman filtterin tilavaativuus on myös `O(n)`, sillä jokainen filtteri tekee uuden kopion prosessoitavasta kuvasta. Täten koko ohjelman tilavaativuus on siten `O(n) * O(|f|)`, missä `n` on kuvan koko pikseleissä ja `|f|` on filtterien lukumäärä.

## Puutteet ja parannusehdotukset
Varsinaisia puutteita ohjelmassa ei mielestäni ole. Lisää filttereitä tietysti voisi aina olla.
