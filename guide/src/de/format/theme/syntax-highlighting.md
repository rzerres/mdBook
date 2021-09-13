# Syntax Hervorhebungen

Um die Quellcode Datein syntaktisch im Editor hervorzuheben kannst Du
z.B. [Highlight.js](https://highlightjs.org) in Verbindung mit einem
angepassten Thema verwenden.

Die automatische Erkennung der Quellcode Sprache wurde abgeschaltet.
Das kannst Du wie folgt anpassen:

<pre><code class="language-markdown">
```rust
fn main() {
	// Some code
}
```
</code></pre>

## Angepasste Themen

Die Anpassungen für die Syntax Hervorhebund erfolgt analgo zu den
bereits beschriebenen Überschreibungen im Themen Ordner.

- ***highlight.js*** Normalerweise ist es nicht notwendig diese Datei
  anzupassen, es sei denn, Du möchtest eine aktuellere oder Entwicklungs-Version
  verwenden.

- ***highlight.css*** Diese cascading style sheet beschreibt die
  aktive Beschreibung, die bei der syntaktischen Hervorhebung von
  Quellcode Anwendung findet.

Wenn Du also ein angepasstes Thema für `highlight.js` verwenden
möchtest, lade zunächst eine `heiglight.css` von der Webseite herunter
und speichere diese dann bitte im Ordner `theme` deines Buchs. Nach
den für Dich passenden Änderungen werden dies beim Editieren
anstelle der Standardwerte verwendet.

## Quellcode Zeile ausblenden

Wie schon für `rustdoc` beschrieben kannst Du auch in `mdBook` Quellcode
Zeilen ausblenden, indem Du als erstes Zeichen der Zeile ein `#`
einfügst und sie damit auskommentierst.

```bash
# fn main() {
	let x = 5;
	let y = 6;

	println!("{}", x + y);
# }
```

Im Buch erhäst Du dann

```rust
# fn main() {
	let x = 5;
	let y = 7;

	println!("{}", x + y);
# }
```

>**Anmerkung**: Derzeit funktioniert das nur für `rust` kommentierten
>Quellcode. Ursächlich hierfür sind semantische Kollisionen mit
>anderen Programmiersprachen. Wir arbeiten daran, dies in Zukunft zu
>verbessern. Über neue, konfigurierbare Parameter im `book.toml`
>könnte eine entsprechend angepasste Steuerung erfolgen und jeder
>würde davon profitieren.


## Verbesserungen für das Standard Thema

Wenn Du der Meinung bist, das jetzige Standard Thema passt nicht für
die für Dich relevante Sprache bist Du herzlich eingeladen an einer
Verbesserung mitzuarbeiten. Bitte erstelle in github eine [neues
Thema](https://github.com/rust-lang/mdBook/issues) und beschreibe
deinen Lösungsvorschlag.

Natürlich ist auch ein `PR` mit den gewünschten Veränderungen willkommen.

Insgesamt sollte das Thema hell und nüchtern erscheinen. Bitte
verwende keine zu auffälligen Farben.
