# Das Kommando watch

The `watch` command is useful when you want your book to be rendered on every
file change. You could repeatedly issue `mdbook build` every time a file is
changed. But using `mdbook watch` once will watch your files and will trigger a
build automatically whenever you modify a file.

#### Verzeichnisangabe

Dem `watch` Kommando kannst Du mit dem Aufruf ein Verzeichnis als Argument
übergeben. Es wird als neue Wurzel (`root`) anstelle Deines
aktuellen Verzeichnisses für die Ausgabe verwendet.

```bash
mdbook watch path/to/book
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

#### Angabe von Ausschluß Kriterien

Das Kommando `watch` wird keinen automatischen `build` von Dateien
starten, die in der Konfigurationsdatei `.gitignore` im
Wurzelverzeichnis deines Buchs angegeben sind. Die Datei `.gitignore`
wertet solche Musterwerte aus, wie sie in der Dokumentation
[gitignore](https://git-scm.com/docs/gitignore) beschrieben sind. Dies
ist sehr hilfreich, um die von manchen Editoren temporär erzeugte
Dateimuster auszuschließen.

-------------------

> **Anmerkung:** Nur die `.gitignore` Datei im Wurzelverzeichnis des
Buchs wird ausgewertet. Globale Werte aus `$HOME/.gitignore` oder
`.gitignore` Dateien in übergeordneten Verzeichnissen werden
ignoriert._
