# Alternative Backends

Ein "backend" ist ein Programm, das `mdbook` aufruft, wenn der Render Prozess gestartet wird.

Über den Gerätetreiber `stdin` wird bei Aufruf einen JSON Stream
Deines Buches und der Konfiguration an das "backend" übergeben.  Es
steht dem "backend" dann völlig frei, wie es diese Eingabe
verarbeite. Es gibt bereits einige alternative backend auf GitHub, die
als grobe Beispiele zeigen, wie diese Aufgabe gelöst werden kann.

- [mdbook-linkcheck] - ein einfaches Programm, das prüft, das im Buch
  keine Links vorhanden sind, die nicht aufgelöst werden können.
- [mdbook-epub] - ein EPUB Renderer
- [mdbook-test] - der Buch Inhalt wird mit dem Programm [rust-skeptic]
  überprüft, um sicherzustellen, dass Quellcode kompiliert und auch
  korrect aufgerufen werden kann (es ist vergleichbar mit `rustdoc
  --test`).

Diese Seite wird beispielhaft und schrittweise zeigen, wie Du vorgehen
musst, ein alternatives, eigenes "backend" zu erstellen. Diese
Programm wird letztlich nur die Worte in einem Buch zählen. Wir
verwenden hierzu Rust, aber es gibt keinen Grund, es alternativ auch
in Python oder Ruby umzusetzen.

## Einrichtung

Als Erstes erstellen wir ein neues Programm, in dem `mdbook` als
Abhängigkeit definiert wird.

```shell
$ cargo new --bin mdbook-wordcount
$ cd mdbook-wordcount
$ cargo add mdbook
```

Wenn unser `mdbook-wordcount` aufgerufen wird, erhält `mdbook` einen JSON
stream des [`RenderContext`] über `stdin` unseres Plugins. Der Bequemlichkeit halber existiert ein
[`RenderContext::from_json()`] Konstruktor, der einen `RenderContext` laden kann.

Mehr ist für unser "backend" nicht notwendig, um den Quelltext unses Buches zu laden.

```rust
// src/main.rs
extern crate mdbook;

use std::io;
use mdbook::renderer::RenderContext;

fn main() {
	let mut stdin = io::stdin();
	let ctx = RenderContext::from_json(&mut stdin).unwrap();
}
```

> **Anmerkung:** Unser `RenderContext` enthält ein `version`
  Feld. Damit können Backends prüfen, ob sie mit der sie aufrufenden
  Version von `mdbook` kompatibel sind. Der Inhalt von `version` wird direkt mit dem korrespondierenden Feld von `mdbook`'s
  `Cargo.toml` verglichen.

  Es wird empfohlen das hierzu für das backend das Crate [`semver`]
  verwendet wird für die Prüfung zu verwenden. Eine Warn-Nachricht kann bei erkannten
  Kompatibilitätsproblemen ausgegeben werden.


## Überprüfung des Buch-Quellen

Da unser backend nun über eine Kopie der Buch-Quellen verfügt, können wird fortfahren und zählen wieviele Wörter jedes Kapitel besitzt.

Weil `RenderContext` ein [`Book`] Feld besitzt (`book`), und `Book` die
[`Book::iter()`] Methode unterstützt, können wir über alle Einträge in
`Book` iterieren. Dieser Schritt ist also genauso einfach wie der vorherige.

```rust

fn main() {
	let mut stdin = io::stdin();
	let ctx = RenderContext::from_json(&mut stdin).unwrap();

	for item in ctx.book.iter() {
		if let BookItem::Chapter(ref ch) = *item {
			let num_words = count_words(ch);
			println!("{}: {}", ch.name, num_words);
		}
	}
}

fn count_words(ch: &Chapter) -> usize {
	ch.content.split_whitespace().count()
}
```


## Aktivierung des Backend

An dieser Stelle haben wir bereits eine lauffähige basis. Lass sie uns
nun auch anwenden. Zuerst installieren wir das Programm.

```shell
$ cargo install --path .
```

Es folgt ein `cd` in das Quellverzeichnis eines Buches, dessen Wortzahl wir ermitteln wollen.
Hier wird die `book.toml` Datei um unser "backend" ergänzt.

```diff
  [book]
  title = "mdBook Documentation"
  description = "Create book from markdown files. Like Gitbook but implemented in Rust"
  authors = ["Mathieu David", "Michael-F-Bryan"]

+ [output.html]

+ [output.wordcount]
```

Wird nun das Buch in den Arbeitsspeicher geladen, wird `mdbook` in der
`book.toml** Datei alle `output.*** parsen um festzustellen, welche
backends angewendet werden sollen. Sind keine Einträge enthalten wird
der default HTML Renderer verwendet.

> **Anmerkung**: Willst Du für die Bearbeitung ein eigenes Backend dem
>Renderprozess anfügen, musst Du sicherstellen, dass es neben dem von
>Dir anzufügenden backend auch ein HTML backend Eintrag
>existiert. `[output.html]` benötigt aber keine Schlüsseleinträge,
>kann also leer bleiben.

Starte nun den Erstellungsprozess wie gewohnt. Es sollte alles *einfach funktionieren*.

```shell
$ mdbook build
...
2018-01-16 07:31:15 [INFO] (mdbook::renderer): Invoking the "mdbook-wordcount" renderer
mdBook: 126
Command Line Tool: 224
init: 283
build: 145
watch: 146
serve: 292
test: 139
Format: 30
SUMMARY.md: 259
Configuration: 784
Theme: 304
index.hbs: 447
Syntax highlighting: 314
MathJax Support: 153
Rust code specific features: 148
For Developers: 788
Alternative Backends: 710
Contributors: 85
```

Wir mussten deshalb einen vollständigen Namen und Pfad zu unserer
`wordcount` Programmdatei angeben, weil `mdbook` standardmäßig
versucht, den Programmnamen aus der impliziten Namenskonvention
abzuleiten. Die ausführbare Datei für das backend `foo` heißt
typischerweise `mdbook-foo`, mit dem zugehörigen `[output.foo]`
Eintrag in der `boot.toml` Datei. Um `mdbook` anzuweisen einen
explizites backend aufzurufen, können wir den Schlüssel `command` in
der `book.toml` Datei hinterlegen. Für diesen Schlüssel können auch
erforderliche Kommandozeilen Argument oder das zu verwendende
Interpreter Skript ergänzt werden.

```diff
  [book]
  title = "mdBook Documentation"
  description = "Create book from markdown files. Like Gitbook but implemented in Rust"
  authors = ["Mathieu David", "Michael-F-Bryan"]

  [output.html]

  [output.wordcount]
+ command = "python /path/to/wordcount.py"
```


## Konfiguration

Nehmen wir uns nun vor, wir wollen **keine Berechnung ** der
verwendeten Anzahl von Worten für ein bestimmtes Kapitel. Vielleicht
weil in diesem Kapitel dynamisch Text oder Quellcode produziert wird.
Der klassische Weg dies umzusetzen besteht darin, in der `book.toml`
Datei einen geeigneten Schlüssel für `[output.foo]` zu erzeugen.

Als grobe Erläuterung können wir die Konfigurationsstruktur `Config`
wie eine geschachtelte HashMap Tabelle betrachten. Diese erlaubt uns
die Methode `get()` zu verwenden, die den Inhalt aus dem
Konfigurationsschlüssel extrahiert. Mit der `get_deserialized()`
Methode können wir bequem für uns interessanten Wert auslesen und
automatisch in deinen beliebigen Typ (hier: `T`) deserialisieren.

Um das das zu implementieren, erzeugen wir unsere eigene
serialisierbare Version der Structur `WordcountConfig`. Sie wird verwendet, um alle
Konfigurationen für unser backend zu kapseln.

Zunächst fügen wir die Abhängigkeiten zu den crates `serde` und
`serde_derive` in unser `Cargo.toml` bei. Sie werden die
Serialisierungs- und De-Serialisierungaufgaben transparent abarbeiten.

```
$ cargo add serde serde_derive
```

Nun können wir die Konfigurationsstruktur erstellen:

```rust
extern crate serde;
#[macro_use]
extern crate serde_derive;

...

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WordcountConfig {
  pub ignores: Vec<String>,
}
```

Bleibt die Implementierung der De-Serialisierungs-Anweisung in
`WordcountConfig` für unseren `RenderContext`. Um anschließend mit
einer Prüfung sicherzustellen, dass wir die gewünschten Kapitel ignorieren.

```diff
  fn main() {
	  let mut stdin = io::stdin();
	  let ctx = RenderContext::from_json(&mut stdin).unwrap();
+     let cfg: WordcountConfig = ctx.config
+         .get_deserialized("output.wordcount")
+         .unwrap_or_default();

	  for item in ctx.book.iter() {
		  if let BookItem::Chapter(ref ch) = *item {
+             if cfg.ignores.contains(&ch.name) {
+                 continue;
+             }
+
			  let num_words = count_words(ch);
			  println!("{}: {}", ch.name, num_words);
		  }
	  }
  }
```


## Ausgabe und Signallisierung eines Fehlers

Es ist sehr bequem, die Anzahl der gezählten Worte im Terminal
auszugeben, wenn wir das Buch interaktiv erstellen.
Aber diese Ausgabe auch in eine Datei vorzunehmen ist umso besser.

Prima, dass wir `mdbook` anweisen können, dass es einem "backend" über das
`destination` Feld in [`RenderContext`] mitteilt, wohin es die produzierte Ausgabe umleiten soll.
Wir setzen als Ausgabedatei `wordcounts.txt`.

```diff
+ use std::fs::{self, File};
+ use std::io::{self, Write};
- use std::io;
  use mdbook::renderer::RenderContext;
  use mdbook::book::{BookItem, Chapter};

  fn main() {
	...

+     let _ = fs::create_dir_all(&ctx.destination);
+     let mut f = File::create(ctx.destination.join("wordcounts.txt")).unwrap();
+
	  for item in ctx.book.iter() {
		  if let BookItem::Chapter(ref ch) = *item {
			  ...

			  let num_words = count_words(ch);
			  println!("{}: {}", ch.name, num_words);
+             writeln!(f, "{}: {}", ch.name, num_words).unwrap();
		  }
	  }
  }
```

> **Note:** Es gibtkeine Garantie dafür, dass das Zielverzeichnis
> existiert und/oder leer ist. vielleicht hat `mdbook` den vorherigen
> Inhalt erhalten um dem backend ein caching zu ermögliche. Deshalb
> ist es eine Gute Idee, immer `fs::create_dir_all()` zu verwenden.

Leider bleibt immer die Möglichkeit, das ein Fehler auftritt, während
wir ein Buch erstellen (Hast du bemerkt, wie viels `unwrap()`'s wir
bereits im Quellcode verwendet haben?). `mdbook` interpretiert aus
gutem Grund alle nicht-null Rückgabewerte (`exit codes`) als Render-Fehler.

Als Beispiel, wollten wir sicherstellen, das alle Kapitel eine
*gerade* Wortzahl besitzen und einen Fehler ausgeben, wenn eine
ungerade Wortzahl erkannt werden. Dann könntest du den folgenden Code
verwenden:

```diff
+ use std::process;
  ...

  fn main() {
	  ...

	  for item in ctx.book.iter() {
		  if let BookItem::Chapter(ref ch) = *item {
			  ...

			  let num_words = count_words(ch);
			  println!("{}: {}", ch.name, num_words);
			  writeln!(f, "{}: {}", ch.name, num_words).unwrap();

+             if cfg.deny_odds && num_words % 2 == 1 {
+               eprintln!("{} has an odd number of words!", ch.name);
+               process::exit(1);
			  }
		  }
	  }
  }

  #[derive(Debug, Default, Serialize, Deserialize)]
  #[serde(default, rename_all = "kebab-case")]
  pub struct WordcountConfig {
	  pub ignores: Vec<String>,
+     pub deny_odds: bool,
  }
```

Reinstallieren wir nun unser Backend und bauen das Buch erneut erhalten wir

```shell
$ cargo install --path . --force
$ mdbook build /path/to/book
...
2018-01-16 21:21:39 [INFO] (mdbook::renderer): Invoking the "wordcount" renderer
mdBook: 126
Command Line Tool: 224
init: 283
init has an odd number of words!
2018-01-16 21:21:39 [ERROR] (mdbook::renderer): Renderer exited with non-zero return code.
2018-01-16 21:21:39 [ERROR] (mdbook::utils): Error: Rendering failed
2018-01-16 21:21:39 [ERROR] (mdbook::utils):    Caused By: The "mdbook-wordcount" renderer failed
```

Wahrscheinlich hast du bereits bemerkt, die Ausgabe eines Plugin
Sub-Prozesses wird direkt an den Anwender weitergeleitet.  Es wird
aber empfohlen, für Plugins eine "Regel der Stille"
einzuhalten. D.h. nur dann Ausgaben zu produzieren, wenn sie notwendig
sind (z.B. Warnungen sind sinnvoll oder Fehler treten auf).

Alle Umgebungsvariablen werden an ein Backend weitergeleitet, was Dir
erlaubt die in Rust übliche Ausgabe von Logs über die variable
`RUST_LOG` zu steuern.

## Behandlung von fehlenden Backends

Wenn Du ein backend aktivierst, das nicht installiert ist, wird `mdbook` eine Fehler ausgeben.

```text
The command `mdbook-wordcount` wasn't found, is the "wordcount" backend installed?
If you want to ignore this error when the "wordcount" backend is not installed,
set `optional = true` in the `[output.wordcount]` section of the book.toml configuration file.
```

Dieses Verhalten kannst Du abändern, indem Du den Namen des backends
in der `book.toml` über den Schlüsselwert `optional` machst.

```diff
  [book]
  title = "mdBook Documentation"
  description = "Create book from markdown files. Like Gitbook but implemented in Rust"
  authors = ["Mathieu David", "Michael-F-Bryan"]

  [output.html]

  [output.wordcount]
  command = "python /path/to/wordcount.py"
+ optional = true
```

Das stuft die Fehlermeldung in ein Warnung herab. Die Meldung sieht dann so aus:

```text
The command was not found, but was marked as optional.
	Command: wordcount
```


## Zusammengefaßt

Auch wenn dieses Beispiel etwas konstruiert ist, so war es hoffentlich
hilfreich zu zeigen, wie man ein ein alternatives Backend für `mdbook`
erstellen kann. Wenn Du das Gefühl hast, dass etwas fehlt, zögere
nicht einen Fehler im [issue tracker] zu melden. Es hilft die
Anleitung verbessern.

Die bestehenden Backends, sollten als gute Beispiele dienen. So haben
Entwickler es für die Produktion bereits umgesetzt (Sie wurden am
Anfang dieses Kapitels erwähnt).
Schau Dir deren Quellcode an, stelle bei Unklarheiten Fragen...

[mdbook-linkcheck]: https://github.com/Michael-F-Bryan/mdbook-linkcheck
[mdbook-epub]: https://github.com/Michael-F-Bryan/mdbook-epub
[mdbook-test]: https://github.com/Michael-F-Bryan/mdbook-test
[rust-skeptic]: https://github.com/budziq/rust-skeptic
[`RenderContext`]: https://docs.rs/mdbook/*/mdbook/renderer/struct.RenderContext.html
[`RenderContext::from_json()`]: https://docs.rs/mdbook/*/mdbook/renderer/struct.RenderContext.html#method.from_json
[`semver`]: https://crates.io/crates/semver
[`Book`]: https://docs.rs/mdbook/*/mdbook/book/struct.Book.html
[`Book::iter()`]: https://docs.rs/mdbook/*/mdbook/book/struct.Book.html#method.iter
[`Config`]: https://docs.rs/mdbook/*/mdbook/config/struct.Config.html
[issue tracker]: https://github.com/rust-lang/mdBook/issues
