## Übersetzungen

> **Anmerkung** Dies ist WIP Code. Für die fortlaufende Diskussion
> lies bitte zum [PR 1306].

[PR 1306]: https://github.com/rust-lang/mdBook/pull/1306

Es ist möglich, dein Buch in mehreren Sprachen zu verfassen. Alle
Übersetzungen werden in einem Ausgabeverzeichnis gebündelt. So wird
dem Anwender angeboten, zwischen den verfügbaren Übersetzungen
auszuwählen. Die unterstützten Sprachen werden im Tabelleneintrag
`[language]` definiert:

```toml
[language.en]
name = "English"

[language.ja]
name = "日本語"
title = "本のサンプル"
description = "この本は実例です。"
authors = ["Ruin0x11"]

[language.de]
name = "Deutsch""
title = "mdBook Documentation"
description = "Erstelle ein Buch aus Markdown Dateien."
authors = ["Ralf Zerres"]
```

```

Jede Sprache muss den Parameter `name` definieren. Für eine
Sprachvariante muss also im Tabelleneintrag `[language]` der Wert
`book.language` als Schlüssel hinterlegt werden. Die erste definierte
Sprachvariante ist die Default-Sprache, deren Dateien verwendet
werden, wenn eine entsprechende Übersetzungsdatei nicht gefunden
werden kann.

Die Parameter `title` und `description` werden die definierten
Metadaten im Abschnitt `[book]` des Buches überschreiben, sofern sie angegeben werden.

Nach der Definition einer neuen Sprachvariante, wie `[language.ja]`, erzeuge bitte ein neues Unterverzeichnis
`src/ja` und erstelle die Datei `SUMMARY.md` neben den weiteren Übersetzungsdateien.

> **Anmerkung:** Es ist wesentlich, ober der Tabelleneintrag
> `[language]` definiert ist oder nicht. Fehlt `[language]`, wird
> mdBook das Verzeicnis `src` als alleiniges Quellverzeichnis
> ansehen. Hier wird dann `SUMMARY.md` erwartet:
>
> ```
> ├── book.toml
> └── src
>     ├── chapter
>     │   ├── 1.md
>     │   ├── 2.md
>     │   └── README.md
>     ├── README.md
>     └── SUMMARY.md
> ```
>
> Wird ein Tabelleneintrag `[language]` definiert, wir mdBook nun
> Unterverzeichnisse von `src` auswerten, in denen sich die
> Übersetzungen befinden, die mit dem angegebenen Schlüssen
> korrespondieren (hier: `ja`):
>
> ```
> ├── book.toml
> └── src
>     ├── en
>     │   ├── chapter
>     │   │   ├── 1.md
>     │   │   ├── 2.md
>     │   │   └── README.md
>     │   ├── README.md
>     │   └── SUMMARY.md
>     └── ja
>         ├── chapter
>         │   ├── 1.md
>         │   ├── 2.md
>         │   └── README.md
>         ├── README.md
>         └── SUMMARY.md
> ```

Wir der Tabelleneintrag `[language]` verwendet, kann an den `mdBook
build` Aufruf ein Parameter `-l <language id>` übergeben werden. Dies
weist `mdBook` an, nur die spezifizierte Sprachvariante zu rendern. In
unserem Beispiel wären als Schlüssel `<language id>` die Werte `en` oder
`ja` zulässig.

Ein paar Anmerkungen zu den Übersetzungen:

- In einer `SUMMARY.md` oder anderen übersetzten Markdown Datei kannst
  Du mit Links auf Dateien der Standard-Sprachvariante arbeiten
  (z.B. Bilder oder Quellcode-Listings). So werden Fallbacks angefügt,
  für die es zwar noch keine Übersetzungen gibt, jedoch in der
  Standard-Sprachvariante neu angefügt wurden.
- Jede Übersetzung kann ihre spezifische `SUMMARY.md` besitzen, die
  auch in der Struktur von anderen Sprachvarianten unterscheidet. Auch
  wenn die vorhandenen Übersetzungen inhaltlich nicht mehr aktuell
  sein sollten (out-of-sync), gilt dies nicht für die verlinkten Dateien.
- Jede Übersetzung kann in ihrer `SUMMARY.md` auf Dateien verweisen,
  die nur in der Sprachvariante Sinn ergeben.
