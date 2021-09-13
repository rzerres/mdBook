# Das Kommando init

Es existiert eine minimale Rahmenumgebung, die für alle mit Markdown erstellten Bücher
 identisch ist. Nur aus diesem Grund ergänzt mdBook das `init`
 Kommando.

Du verwendest `init` wie folgt:

```bash
mdbook init
```

Wenn Du `init` zum ersten mal anwendest, werden ein paar Dateien für Dich automatisch angelegt:

```bash
book-test/
├── book
├── book.toml
└── src
	└── en
		├── chapter_1.md
		└── SUMMARY.md
```

- Das Verzeichnis `src` ist der Ort, in dem du die Markdown Dateien
  für dein Buch einstellst. Es beinhaltet alle Quellcode Dateien, auch
  die verfügbaren Übersetzungen! Als Unterlassungswert (`default`)
  wird immer ein Verzeichnis für die englische Übersetzung erstellt,
  `src/en`.

- Die Datei `book.toml` entält die Konfigurationsanweisungen, wie Dein Buch gerendert werden soll.
  Bitte lies im Kapitel [Konfiguration](../format/config.md) die verfügbaren Details nach.

- Das Verzeichnis `book` wird die nach dem Render-Prozess erzeugten
  Ausgabedateien enthalten. Diese Ausgabe ist beinhaltet alle für den
  Leser erforderlichen Dateien und kann in dieser Form auf einen
  beliebigen Server hochgeladen werden.

- Die Datei `SUMMARY.md` ist wohl die zunächst wichtigste Quelle. Sie ist das Gerüst, in der Du die Struktur Deines Buches definierst.
  Sie wird im Detail im Kapitel [Zusammenfassung](../format/summary.md) besprochen.

#### Tip: Erzeuge die Kapitel aus SUMMARY.md

Wenn die Datei `SUMMARY.md` bereits zuvor erzeugt wurde, wird das
`init` Kommando diese zunächst prüfen und die darin beschriebenen, im
Verzeichnis noch fehlenden Dateien entsprechend in `SUMMARY.md`
definierten Pfade für Dich erzeugen. Das erleichtert es für Dich, die Zielstruktur zunächst zu überdenken. mdBook kann sie anschießend für Dich generieren.

#### <Verzeichnisname>

Dem Befehl `init` kannst Du bei Aufruf einen Verzeichnisnamen als
Argument übergeben. Dieses Verzeichnis ist dann anstelle des aktuellen
Verzeichnisses die Wurzel (`root`) für dein Buch.

```bash
mdbook init path/to/book
```

#### --theme

Übergibst Du mit dem Aufruf den Parameter `--theme`, wird ein
Unterverzeichnis `theme` angelegt in dem der gegebene Name als
Unterlassungswert verwendet wird. Hier nimmst Du dann bitte deine Veränderungen vor.

Das Thema wird selektiv überschrieben. Das bedeutet, das solche
Dateien, die du nicht verändern möchtest einfach von Dir gelöscht werden
können. Die `default` Werte werden dann auch in Deinem Thema
Anwendung finden.
