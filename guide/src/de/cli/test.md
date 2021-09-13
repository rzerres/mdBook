# Das Kommando test

Wenn du ein Buch bearbeitest wirst Du sicherlich auch automatische Tests durchführen wollen. Z.B. verwendet
[The Rust Programming Book](https://doc.rust-lang.org/stable/book/) einige Quellcode Beispiele, die leicht veraltern können.
Es ist daher sehr wichtig, die Funktionalität dieser Beispiele über Test-Läufe zeitnah zu überprüfen.

#### Tests von Quellcode Blöcken aussetzen

rustdoc überprüft keinen Quellcode, der sich in Blöcken befindet, der mit dem Attribut `ignore` gekennzeichnet sind:

	```rust,ignore
	fn main() {}
	```

rustdoc überprüft ebenfalls keinen Quellcode, der sich in Blöcken
befindet, die mit einem anderen Sprachattribut als Rust gekennzeichnet
sind:

	```markdown
	**Foo**: _bar_
	```

rustdoc *testet* Quellcode, der sich in Blöcken befindet, die mit keinem Sprachattribut gekennzeichnet sind:

	```
	This is going to cause an error!
	```

#### Verzeichnisangabe

Dem `test` Kommando kannst Du mit dem Aufruf ein Verzeichnis als Argument
übergeben. Es wird als neue Wurzel (`root`) anstelle Deines
aktuellen Verzeichnisses für die Ausgabe verwendet.

```bash
mdbook test path/to/book
```

#### --library-path

Mit dem Parameter `--library-path` (`-L`) kannst Du beim Aufruf von
mdbook dem Pfad ergänzen, der für die Suche von Bibliotheken
ausgewertet wird. `rustdoc` wird diesen Pfad beim Erstellen und Testen
von Beispielen anwenden. Werden mehrere Pfadnamen benötigt, können
diese mit nachfolgenden Optionen (`-L foo -L bar`) angegeben
werden. Alternativ kann die Syntax von Komma separierten Listen
verwendet werden (`-L foo,bar`).

#### --dest-dir

Mit dem Parameter `--dest-dir` (`-d`) kannst Du beim Aufruf von mdbook
das Ausgabeverzeinis verändern.  Dabei werden sowohl relative wie
absolute Pfadangaben ausgewertet. Wird der Parameter nicht angegeben,
wendet mdbook den Unterlassungswert an, der in der Datei `book.toml`
dem Schlüsselwert `build.build-dir` zugewiesen wurde. Fehlt dieser,
wird in das Verzeichnis `./book` ausgegeben.
