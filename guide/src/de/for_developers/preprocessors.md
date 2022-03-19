# Pro-Prozessoren

Um es einfach auszudrücken ist ein *Pre-Prozessor* die Software, die
sofort nach dem Laden der Markdown Quellen deines Buchs gestartet
wird. Und zwar **bevor** der Render Prozess die eigentlichem Größen
und Positionierungen von Text und Bildern berechnet.
Deshalb ist es an dieser Stelle noch möglich, die Struktur des Buches anzupassen.
Mögliche Anwendungsfälle sind daher:

- Erstellung von angepassten Hilfstools (z.B `\{{#include /path/to/file.md}}`).
- Aktualisierung von Links. So der Eintrag `[Ein Kaptitel](some_chapter.md)` automatisch zu
  `[ein Kaptiel](some_chapter.html)` konvertiert, den der HTML renderer dann als Quelle bearbeitet.
- Austausch von Quell-Anweisungen mit LaTex-Style Ausdrücken (z.B die MathJax Befehle für `$$ \frac{1}{3} $$`).

## Das Einklinken in mdBook

Der Mechanismus, der in mdBook verwendet wird, um Plugins von
Drittherstellern einzufügen ist ziemlich simpel.  Füge eine neue
Deklaration in `book.toml` ein (z.B. `preprocessor.foo` für den neuen
`foo` Pre-Prozessor) und `mdbook` wird versuchen das `mdbook-foo`
Programm im Rahmen des build Prozesses aufzurufen.

Ebenso kann fest vorgegeben werden, welche Backends für welche
Pre-Prozessors zur Anwendung kommen sollen. Es macht beispielsweise
keinen Sinn, MathJax ein nicht für die HTML Ausgabe zuständigen
Renderer aufzurufen.

Die Zuordnung eines dedizierten Render backends erfolgt in `book.toml` über den
Schlüssel `renderer`.

```toml
[book]
title = "My Book"
authors = ["Michael-F-Bryan"]

[preprocessor.foo]
# Das Kommando kann auch manuell angegeben werden
command = "python3 /path/to/foo.py"
# Start den `foo` Pre-Prozessor ausschließlich für den HTML und EPUB Renderer
renderer = ["html", "epub"]
```

Im für Unix typischen Stil, werden alle Eingaben an das Plugin an den
Geratetreiber `stdin` in Form eines JSON streams übergeben und
es wird vom Gerätetreiber `stdout` gelesen, wenn `mdBook` eine Ausgabe
erwartet.

Der wohl einfachste Weg einen neuen Pre-Prozessor zu entwickeln, ist
es, eine eigene Implementierung des `Pre-Prozesser` traits zu erzeugen
(z.B. in `lib.rs`). Anschließend erstellst Du ein Shell Programm, das
eingaben in die richtige `Pre-Prozessor` Methode umleitet. Als
Hilfestellung existiert hierzu ein Beispiel im Unterverzeichnis
`examples/`. Dieses [no-op preprocessor] Beispiel kann leicht für
andere Pre-Prozessoren leicht angepasst werden.

<details> <summary>Biespiel Quellcode für einen no-op Pre-Prozessor</summary>

```rust
// nop-preprocessors.rs

{{#include ../../../../examples/nop-preprocessor.rs}}
```
</details>

## Hinweise für die Implementierung eines Pre-Prozessors

Wenn die `mdBook` als Bibliothek einliest, erhält der Pre-Prozessor
Zugriff auf die bestehende Infrastruktur für die Erstellung von
Büchern.

Ein von Dir angepasster Pre-Prozessor könnte zum Beispiel die Funktion
[`CmdPreprocessor::parse_input()`] nutzen, um eine De-Serialisierung
der Eingabe in einen JSON stream für `stdin` umzuwandeln. Nun kann
jedes Kapitel in deinem `Book` an Ort und Stelle mit
[`Book::for_each_mut()`] verändert, und mit dem `serde_json` crate an
den Gerätetreiber `stdout`ausgegeben werden.

Kapitel können entweder direkt aufgerufen werden (z.B. mit einer
Rekursiven Iteration über alle Kapitel), oder du verwendest die
Helper-Methode `Book::for_each_mut()`.

Das Kaptiel `chapter.content` ist nur eine Zeichenfolge die ein
Markdown Quelltext ist. Es ist zwar möglich hierzu "Reguläre
Ausdrücke" in Verbindung mit einem "Suche & Ersetze" Befehl zu
verwenden, sinnvoller ist es jedoch, die Eingabe in einer Computer
freundlichern Art und Weise zu bearbeiten. Das Crate
[`pulldown-cmark`][pc] stellt hierzu einen Event basierten Markdown
Parser zur Verfügung (in Produktions-Qualität), das dir ebenso
erlaubt, die Events mit [`pulldown-cmark-to-cmark`][pctc] wieder
zurück in Markdown Text zu übersetzen.

Der folgende code Block zeigt, wie alle in der Markdown-Quelle
enthaltenen Hervorhebungen gelöscht werden können, ohne versehentlich
dabei die Dokumentenstrutur zu zerstören.

```rust
fn remove_emphasis(
	num_removed_items: &mut usize,
	chapter: &mut Chapter,
) -> Result<String> {
	let mut buf = String::with_capacity(chapter.content.len());

	let events = Parser::new(&chapter.content).filter(|e| {
		let should_keep = match *e {
			Event::Start(Tag::Emphasis)
			| Event::Start(Tag::Strong)
			| Event::End(Tag::Emphasis)
			| Event::End(Tag::Strong) => false,
			_ => true,
		};
		if !should_keep {
			*num_removed_items += 1;
		}
		should_keep
	});

	cmark(events, &mut buf, None).map(|_| buf).map_err(|err| {
		Error::from(format!("Markdown serialization failed: {}", err))
	})
}
```

Für alle anderen Anwendungsfälle schau dir doch bitte ein auf GitHub verfügbares [komplettes Beispiel][example].

## Implementierung eines Pre-Prozessors mit einer anderen Programmiersprache

Dadurch das mdBook die Umleitung von stdin und stdout für die
Kommunikation mit Pre-Prozessoren unterstützt, macht dies deren
Implementierung in anderen Programmiersprachen sehr einfach.  Der
nachfolgend gezeigte Quellcode ist die Implementierung eines
einfachen Pre-Prozessors in `Python`. Dieser verändert den Inhalt des
ersten Kapitels. Das Beispiel folgt der in `preprocessor.foo.command`
angegebenen Konfiguration, die nun auf ein Python Skript verweist.

```python
import json
import sys


if __name__ == '__main__':
	if len(sys.argv) > 1: # we check if we received any argument
		if sys.argv[1] == "supports":
			# then we are good to return an exit status code of 0, since the other argument will just be the renderer's name
			sys.exit(0)

	# load both the context and the book representations from stdin
	context, book = json.load(sys.stdin)
	# and now, we can just modify the content of the first chapter
	book['sections'][0]['Chapter']['content'] = '# Hello'
	# we are done with the book's modification, we can just print it to stdout,
	print(json.dumps(book))
```

[preprocessor-docs]: https://docs.rs/mdbook/latest/mdbook/preprocess/trait.Preprocessor.html
[pc]: https://crates.io/crates/pulldown-cmark
[pctc]: https://crates.io/crates/pulldown-cmark-to-cmark
[example]: https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs
[no-op preprocessor]: https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs
[`CmdPreprocessor::parse_input()`]: https://docs.rs/mdbook/latest/mdbook/preprocess/trait.Preprocessor.html#method.parse_input
[`Book::for_each_mut()`]: https://docs.rs/mdbook/latest/mdbook/book/struct.Book.html#method.for_each_mut
