## Umgebungs Variablen


Alle Konfigurationswerte können über auf der Kommandozeile durch das
Setzen korrespondierenden Umgebungsvariablen überschrieben werden.
Weil einige Betriebssysteme die Syntax von Umgebungsvariablen auf
alphanumerisch Zeichen oder `_` begrenzen, müssen die
Konfigurationsschlüssel ein wenig anders als in der üblichen Form
`foo.bar.baz` behandelt werden.

Variables die mit `MDBOOK_` beginnen werden im Konfigurationssystem
verwendet.  Ihr Schlüssel wird durch das Löschen des Prefixes
bestimmt. Im Ergebnis wird eine Zeichenfolge im `kebab-case` erzeugt.
Doppelte Unterstreichungen (`__`) trennen geschachtelte Schlüssel,
während einfache Untersteihungen (`_`) durch einen Bindestrich (`-`)
ausgetauscht werden.

Ein Beispiel:

- `MDBOOK_foo` -> `foo`
- `MDBOOK_FOO` -> `foo`
- `MDBOOK_FOO__BAR` -> `foo.bar`
- `MDBOOK_FOO_BAR` -> `foo-bar`
- `MDBOOK_FOO_bar__baz` -> `foo-bar.baz`

So kannst Du beispielsweise mit der Ugebungsvariable
`MDBOOK_BOOK__TITLE` den Titel Deines Buches verändern, ohne hierzu
einen Eintrag in der `book.toml` Datei anzupassen.

> **Anmerkung_** Um das Verändern komplexerer Konfigurationseinträge
> zu vereinfachen wird eine Umgebungsvariable zunächst im JSON Format
> geparst. Als Standardwert wird auf eine Zeichenfolge gewechselt,
> wenn das nicht möglich ist.
>
> Das bedeutet, du kannst alle in einer `book.toml` Datei angegebenen
> Metadaten für die Erzeugung des Buches mit folgendem Aufruf
> überscheiben (so es für Deinen Anwendungsfall Sinn macht):
>
> ```shell
> $ export MDBOOK_BOOK="{'title': 'My Awesome Book', authors: ['Michael-F-Bryan']}"
> $ mdbook build
> ```

Der letzte Fall ist dann relevant, wenn du `mdBook` über ein Skript
oder innerhalb einer CI aufrufen willst, und die Anpassung der
`book.toml` vor dem Aufruf nur umständlich oder gar nicht möglich ist.
