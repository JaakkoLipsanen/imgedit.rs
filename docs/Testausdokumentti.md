# Testausdokumentti

## Yksikkötestit
Ohjelmassa on kattavat yksikkötestit, jotka testaavat, että ohjelman jokainen osa-alue toimii kuten pitäisi.
Jokainen filtteri ollaan myös testattu yksikkötesteissä toimimaan suunnitellusti. Jokaista filtteriä kohden on noin 2 yksikkötestiä, joten tietysti testejä voisi olla enemmän varmistaakseen että kaikki reunaehdotkin toimivat halutusti.

## Suorituskykytestaus
Suorituskykytestit voi ajaa `cargo bench` komennolla (huom, vaatii nightly version Rustista!). Suorituskykytestit ajavat blur, harmaasävy, hue shift ja unsharp mask filtterit sekä full HD (1920x1080) että 4K (3840x2160) resoluutioisella kuvalla. 

Tässä tulokset minun MacBook Prollani:

```
test filters::implementations::tests::benchmark_blur_fullhd         ... bench: 180,319,354 ns/iter (+/- 33,134,404)
test filters::implementations::tests::benchmark_blur_uhd            ... bench: 726,914,300 ns/iter (+/- 143,616,930)
test filters::implementations::tests::benchmark_greyscale_fullhd    ... bench:   8,717,016 ns/iter (+/- 1,416,293)
test filters::implementations::tests::benchmark_greyscale_uhd       ... bench:  40,815,811 ns/iter (+/- 3,467,157)
test filters::implementations::tests::benchmark_hue_shift_fullhd    ... bench: 117,217,253 ns/iter (+/- 46,640,744)
test filters::implementations::tests::benchmark_hue_shift_uhd       ... bench: 479,540,156 ns/iter (+/- 32,474,803)
test filters::implementations::tests::benchmark_unsharp_mask_fullhd ... bench: 174,686,831 ns/iter (+/- 25,000,113)
test filters::implementations::tests::benchmark_unsharp_mask_uhd    ... bench: 772,186,627 ns/iter (+/- 230,133,547)
``` 

Taulukossa: 


| Filtteri     | Full HD | 4K     | Kuinka monta kertaa 4K on hitaampi kuin Full HD? |
|--------------|---------|--------|--------------------------------------------------|
| Blur         | 180ms   | 726ms  | 4.0333x                                          |
| Harmaasävy   | 8.7ms   | 40.8ms | 4.6895x                                          |
| Hue shift    | 117ms   | 479ms  | 4.094x                                           |
| Unsharp mask | 174ms   | 772ms  | 4.4367x                                          |

Täten huomataan, että algoritmit näyttävät skaalautuvan lineaarisesti kuten oli oletuskin, sillä 4K:ssa on neljä kertaa enemmän pikseleitä kuin full HD:ssa. Filttereiden ajoaika on myös yleisesti ottaen melko nopea, sillä sekunnissa pystyy ajaa filtteristä riippuen ~5-100 filtteriä full HD kuvalle.

## Miten ajaa testit?
Testit voidaan ajaa käyttämällä `cargo test` komentoa projektin juuripolussa. Tämä vaatii, että Rust ollaan asennettu ([ohjeet](https://www.rust-lang.org/tools/install)).

## Koodikattavuus
Yksikkötestien koodikattavuus on näkyvissä [CodeCovissa](https://codecov.io/gh/JaakkoLipsanen/imgedit.rs)
