# Määrittelydokumentti

Projektini on komentorivillä toimiva kuvanmanipulointi ohjelma.

#### Mitä syötteitä ohjelma saa ja miten näitä käytetään?

Yksinkertaisesti ohjelmaa käytetään näin:
`imgedit -input kuva.png -output muokattu.png -filter1 <filter1_parametri> -filter2 <filter2 parametri> ... -filterN <filterN parametri>`

Ohjelmalle annetaan muokattavan kuvan tiedostopolku `-input` argumenttina ja polku, johon muokattu kuva tallennetaan `-output` parametrina.
Tämän jälkeen ohjelmalle annetaan lista filttereitä, jotka kuvaan suoritetaan. Esimerkkeinä filttereistä on:

- `-brightness X`, joka tekee jokaisesta pikselistä X% kirkkaamman
- `-sharpen X`, joka naiivisti terävöittää kuvaa. Terävöittämisen määrä riippuu X:n arvosta.
- `-blur X`, joka sumentaa kuvaa. Sumentamisen määrä riippuu X:n arvosta. Mahdollisesti teen myös erilaisia blur filttereitä, kuten `-box-blur X`, `-gaussian-blur X`.
- `-unsharp-mask X`, joka suorittaa kuvalle monimutkaisemman terävöittämisen käyttäen [unsharp masking](https://en.wikipedia.org/wiki/Unsharp_masking) tekniikkaa.
- `-invert-color`, joka kääntää jokaisen pikselin värin vastakohdakseen.
- `-hue-shift X`, joka muuttaa/"kääntää" kuvan pikselien värisävyä.
- `-grayscale`, joka muuttaa kuvan mustavalkoiseksi.
- `-edges`, joka suorittaa kuvaan reunantunnistus filtterin, jolloin lopputuloksena on mustavalkoinen kuva, jossa reunat on valkoisena.
- `-denoise`, joka poistaa häiriöitä ("noisea") kuvasta. Tähän on monta eri algoritmia, joista eräs yksinkertainen on [mediaani filtteri](https://en.wikipedia.org/wiki/Median_filter)

(Jos oikein innostun, niin voin toteuttaa myös komentoja kuten `-crop <x>x<y>x<width>x<height>`, joka leikkaa kuvaa, `-vignette X`, joka luo kuvaan vignette efektin ja muita ")

Toteutan mahdollisesti muitakin filttereitä jos tulee mieleen, enkä välttämättä toteuta kaikki ylläolevia.

#### Mitä algoritmeja ja tietorakenteita toteutat työssäsi?

En ole vielä _ihan_ täysin varma algoistani. Moni filtteri, kuten `-brightness`, `-sharpen`, `-blur` ja `-edges` voidaan ymmärtääkseni toteuttaa käyttäen pelkästään [kernel matriisia](<https://en.wikipedia.org/wiki/Kernel_(image_processing)>), joka kerrotaan jokaiselle kuvan pikselille (ja sen läheisille pikseleille).

`-hue-shift` filtterin aion toteuttaa muuttamalla kuvan pikselien arvot RGB-värimallista HSL-värimalliin (Hue-Saturation-Lightness), muuttamalla pikselien hue-arvoa, ja muuntamalla värimallin takaisin RGB:hen.

`-denoise` algoritmin toteutan luultavasti käyttämällä "mediaani filtteriä", joka laskee jokaisen pikselin arvoksi mediaanin kyseistä pikseliä lähimmistä arvoista.

Aion tallentaa kuvan datan `Image` luokkaan, joka on periaatteessa 2-ulotteinen taulukko.
Aion tehdä ainakin `Matrix` datastruktuurin, jolla on `Image apply(Matrix, Image)` filttereitä varten, jotka käyttävät kernel matriisia.
Aion myös implementoida `List`-datastruktuurin, johon tallennan filtterit samalla kun luen niitä komentorivi argumenteista.


#### Mitä ongelmaa ratkaiset ja miksi valitsit kyseiset algoritmit/tietorakenteet?

Tosiaankin kyseessä on kuvamanipulointi algoritmi. Valitsemani algoritmit ja tietorakenteet ovat yksinkertaisia ja tehokkaita tapoja toteuttaa kuvanmanipulointi filttereitä.

#### Tavoitteena olevat aika- ja tilavaativuudet (m.m. O-analyysit)

Ohjelmassa tulee olemaan paljon erilaisia algoritmeja. Tilavaativuudeltaan jotkut algoritmit ovat `O(1)`, sillä kuvan pikselidataa voidaan suoraan muokata. Tähän kuuluu esimerkiksi `-grayscale`, `-invert-color` ja `-hue-shift`. Algoritmeissa, jotka eivät prosessoi pelkästään yksittäistä pikseliä vaan myös läheisiä pikseliä pitää olla huolellinen, että läheisien pikselien arvoja lukiessaan ei lue jo prosessoituja pikseliarvoja, vaan alkuperäisiä pikseliarvoja. Tämän vuoksi nämä algoritmit saattavat joutua tekemään kopion kuvasta, jolloin tilavaatimus on `O(n)`, jossa `n` on kuvassa olevien pikselin lukumäärä.

Aikavaativuudeltaan olen melko varma, että kaikki algoritmit ovat `O(n)`.

#### Lähteet
https://en.wikipedia.org/wiki/Kernel_(image_processing)
https://en.wikipedia.org/wiki/HSL_and_HSV
http://setosa.io/ev/image-kernels/

