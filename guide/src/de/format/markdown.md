## Markdown

mdBook's [Parser](https://github.com/raphlinus/pulldown-cmark) liegt
die [CommonMark](https://commonmark.org/) Specification zu grunde. Du
kannst dir mit dem [Tutorial](https://commonmark.org/help/tutorial/)
einen schnellen Überblick verschaffen, Common Mark online
[ausprobieren](https://spec.commonmark.org/dingus/). Für eine
ausführliche Beschreibung der Sprache lies bitte im [Markdown
Guide](https://www.markdownguide.org).

## Text und Abschnitte

Das Rendern von Text erfolgt relativ vorhersehbar:

```markdown
Hier ist eine Textzeile.

Dies ist eine neue Zeile.
```

Wir so aussehen, wie Du es wahrscheinlich erwartest:

Hier ist eine Textzeile.

Dies ist eine neue Zeile.

## Überschriften

Überschriften verwenden als Markierung das `#` Zeichen und sollten in
einer eigenen Zeile plaziert werden. Mehrere `#` Markierungen
kennzeichnen kleinere Überschriften:

```markdown
### Eine Überschrift

Ein wenig Text.

#### Eine kleiner Überschrift

Und noch etwas Text.
```

## Listen

Listen können strukturiert oder unstrukturiert definiert werden.
Strukturierte Listen werden automatisch sortiert:

```markdown
* Milch
* Eier
* Butter

1. Karotten
1. Radieschen
1. Sellerie
```

* Milch
* Eier
* Butter

1. Karotten
1. Radieschen
1. Sellerie

## Verweise

Ein Verweise auf eine URL oder lokale Datei kann leicht hergestellt werden:

```markdown
Verwende [mdBook](https://github.com/rust-lang/mdBook).

Lies über [mdBook](mdBook.md).

Einfach nur eine url: <https://www.rust-lang.org>.
```

Verwende [mdBook](https://github.com/rust-lang/mdBook).

Lies über [mdBook](mdbook.md).

Einfach nur eine url: <https://www.rust-lang.org>.

----

Relative Verweise, die mit den Zeichen `.md` enden werden in die
Erweiterung `.html` konvertiert.  Es wird empfohlen wenn möglich `.md`
Verweise zu verwenden.  Das ist hilfreich, wenn die Markdown Datei
außerhalb von mbBook dargestellt werden soll, z.B. auf GitHub oder
Gitlab. Dies Portale rendern Markdown automatisch.

Verweise auf `README.md` werden zu `index.html` konvertiert.  Zwar
rendern manche Dienste eine README Datei automatisch (z.B. GitHub),
hingegen erwarten Web-Server den Namen `index.html` als Root-Datei.

Du  kannst  auf  individuelle   Überschriften  mit  dem  `#`  Fragment
verweisen.  Zum Beispiel würde `mdbook.md#text-and-paragraphs` auf den
obigen    Abschnitt   [Text    und   Abschnitte](#text-and-paragraphs)
verweisen.  Die  ID wird  dadurch  erzeugt,  dass die  Überschrift  in
Kleinschreibung  konvertiert  wird  und darin  enthaltene  Leerzeichen
durch  Bindestriche ersetzt  werden.  Du kannst  auf jede  Überschrift
klicken und Du wirst den Inhalt der URL in einem Browser-Fenster sehen.

## Bilder

Um Bilder einzubinden musst Du nur einen Verweis auf deren Dateinamen
einfügen. Die Syntax ergibt sich in Anlehnung auf den Abschnitt
_Verweise_. Der nachfolgende markdown Code bindet das Rust Logo (
SVG Datei) ein, das sich auf der gleichen Verzeichnisstruktur im Ordner
`images` befindet:

```markdown
![Das Rust Logo](images/rust-logo-blk.svg)
```

Folgender HTML Code wird durch mbBook generiert:

```html
<p><img src="images/rust-logo-blk.svg" alt="Das Rust Logo" /></p>
```

Und diew wird natürlich das Bild so darstellen:

![Das Rust Logo](images/rust-logo-blk.svg)

## Erweiterungen

mdBook hat eine Vielzahl von Erweiterungen zur `CommonMark` Standard Definition.

### Durchstreichungen

Text kann mit einer horizontal durchgestrichenen Linie dargestellt
werden, wenn der Text zwischen zwei Tilde Zeichen gestellt wird:

```text
Ein Beispiel mit ~~durchgestrichem Text~~.
```

Das Beispiel wird so gerendert:

> Ein Bewispiel mit ~~durchgestrichenem Text~~.

Dies folgt der [GitHub Erweiterung Durchstreichen][strikethrough].

### Fußnoten

Eine Fußnote erzeugt einen kleinen nummerierten Verweis im Text, der
den Leser zum Fußnoten Text am Ende des Eintrags führt, wenn der
Verweis angeklickt wird.  Die Bezeichnung der Fußnote wird ähnlich zur
Link-Referenz mit einem vorangestellten Caret markiert.
Der Text der Fußnote wird wie eine Link-Referenz definiert, bei der der Text der Bezeichnung folgt. Hier ein Beispiel:

```text
Die ist ein Fußnoten Beispiel[^Fußnote].

[^Fußnote]: Dieser Text ist der Fußnoten-Inhalt, der am Ende des Abschnitts gerendert wird.
```

Das Beispiel wird so gerendert:

> Diest ist ein Fußnoten Beispiel[^Fußnote].
>
> [^Fußnote]:  Dieser Text ist der Fußnoten-Inhalt, der am Ende des Abschnitts gerendert wird.

Fußnoten werden automatisch, entsprechend ihrer Position im Quelltext nummeriert.

### Tabellen

Tabellen können durch Verwendung von senkrechten Strichen (pipes) und
Bindestrichen für die Zeichnung von Zeilen und Spalten von geschrieben
werden. Sie werden zu HTML Tabellen mit passender Gestalt umgewandelt.
Hier ein Beispiel:

```text
| Überschrift1 | Überschrift2 |
|--------------|--------------|
| abc          | def          |
```

Dieses Beispiel wir so abgebildet:

| Überschrift1 | Überschrift2 |
|--------------|--------------|
| abc          | def          |

Für weitere Details zur genauen Verwendung der unterstützten Syntax wird auf
die [GitHub Erweiterung Tabellen][tables] verweisen.

### Aufgaben Listen

Aufgaben-Listen können als Checkliste der erledigten Aufgaben
verwendet werden:

```md
- [x] Abgeschlossene Aufgabe
- [ ] Unvollständige Aufgabe
```

Dies sieht dann so aus:

> - [x] Abgeschlossene Aufgabe
> - [ ] Unvollständige Aufgabe

Vergleiche für weiterführende Details die Spezifikation zur [Erweiterung Aufgaben Listen].

### Intelligente Zeichensetzung

Einige ASCII Zeichensequenzen werden automatisch in schicke Unicode Zeichen übersetzt.

| ASCII Sequenz  | Unicode |
|----------------|---------|
| `--`           | –       |
| `---`          | —       |
| `...`          | …       |
| `"`            | “ oder ”, abhängig vom Kontext |
| `'`            | ‘ oder ’, abhängig vom Kontext |

Daher, keine Grund die Unicode Zeichen selber zu finden!

Diese Funktion ist als Standardwert deaktiviert. Um es zu aktivieren, vgl. die
[`output.html.curly-quotes`] Konfigurations-Option.

[strikethrough]: https://github.github.com/gfm/#strikethrough-extension-
[tables]: https://github.github.com/gfm/#tables-extension-
[Erweiterung Aufgaben Listen]: https://github.github.com/gfm/#task-list-items-extension-
[`output.html.curly-quotes`]: configuration/renderers.md#html-render-optionen
