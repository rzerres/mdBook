# Das Kommando build

Mit dem build Kommando erzeugst du die gerenderte Ausgabe Deines Buchs:

```bash
mdbook build
```

Es wird die `SUMMARY.md` Datei überprüfen, um daraus die Struktur
abzuleiten und die erforderlichen Quelldateien auswerten.

Die gerenderte Ausgabe wird die gleiche Verzeichnisstruktur besitzen,
die Du in der Quelldatei vorgegeben hast. Bei großen, umfangreichen
Büchern bleibt daher die Struktur erhalten wenn du sie mehrfach
renderst.

#### Verzeichnisangabe

Dem `build` Kommando kannst Du mit dem Aufruf ein Verzeichnis als Argument
übergeben. Es wird als neue Wurzel (`root`) anstelle Deines
aktuellen Verzeichnisses für die Ausgabe verwendet.

```bash
mdbook build path/to/book
```

#### --open

Wenn du beim Aufruf von mdbook den Parameter `--open` (`-o`) angibst,
wird nach Abschluss des Render-Prozesses der Einstiegspunkt in
Dein Buch im default Browser geöffnet.

#### --dest-dir

Mit dem Parameter `--dest-dir` (`-d`) kannst Du beim Aufruf von mdbook
das Ausgabeverzeinis verändern.  Dabei werden sowohl relative wie
absolute Pfadangaben ausgewertet. Wird der Parameter nicht angegeben,
wendet mdbook den Unterlassungswert an, der in der Datei `book.toml`
dem Schlüsselwert `build.build-dir` zugewiesen wurde. Fehlt dieser,
wird in das Verzeichnis `./book` ausgegeben.

-------------------

> **Anmerkung:** Das Kommando build kopiert zunächst alle Dateien aus
> dem Quellverzeichnis in das build Verzeichnis (Dateien mit der
> Endung `.md` werden übergangen).
