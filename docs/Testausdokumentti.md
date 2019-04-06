# Testausdokumentti

## Yksikkötestit
Ohjelmassa on kattavat yksikkötestit, jotka testaavat, että ohjelman jokainen osa-alue toimii kuten pitäisi.
Jokainen filtteri ollaan myös testattu yksikkötesteissä toimimaan suunnitellusti. Jokaista filtteriä kohden on noin 2 yksikkötestiä, joten tietysti testejä voisi olla enemmän varmistaakseen että kaikki reunaehdotkin toimivat halutusti.

## Suorituskykytestaus
En näe suorituskokotestausta hyödyllisenä tälle ohjelmalle.

## Miten ajaa testit?
Testit voidaan ajaa käyttämällä `cargo test` komentoa projektin juuripolussa. Tämä vaatii, että Rust ollaan asennettu ([ohjeet](https://www.rust-lang.org/tools/install)).

## Koodikattavuus
Yksikkötestien koodikattavuus on näkyvissä [CodeCovissa](https://codecov.io/gh/JaakkoLipsanen/imgedit.rs)
