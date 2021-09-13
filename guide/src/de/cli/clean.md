# Das Kommando clean

Wird mdbook mit dem Kommando clean gestartet, werden alle erzeugten
Bücher neben den übrigen Erstellungsartefakten gelöscht.


```bash
mdbook clean
```

#### Verzeichnisangabe

Dem `clean` Kommando kannst Du mit dem Aufruf ein Verzeichnis als Argument
übergeben. Es wird als neue Wurzel (`root`) anstelle Deines
aktuellen Verzeichnisses für die Ausgabe verwendet.  root instead of

```bash
mdbook clean path/to/book
```

#### --dest-dir

Mit dem Parameter `--dest-dir` (`-d`) kannst Du beim Aufruf von mdbook
das Ausgabeverzeinis verändern. Existiert diesen, wird es samt Inhalt
von mdbook gelöscht. Wird der Parameter nicht angegeben, wendet
mdbook den Unterlassungswert an, der in der Datei `book.toml` dem
Schlüsselwert `build.build-dir` zugewiesen wurde. Fehlt dieser, wird
das Verzeichnis `./book` verwendet und gelöscht.

```bash
mdbook clean --dest-dir=path/to/book
```
