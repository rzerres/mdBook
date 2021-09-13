# Renderer

## Konfiguration der Renderer

### HTML Render Optionen

The HTML Renderer wertet eine Reihe von Optionen aus. Die Optionswerte
müssen im Tabelleneitrag `[output.html]` angegeben werden.

Die folgenden Konfigurationsoptionen sind verfügbar:

- **theme:** mdBook wird mit einem Standard-Thema installiert, das
  alle erforderlichen Ressourcen-Dateien mitliefert. Wird dieser
  Parameter gesetzt, wird mdBook selektiv alle Dateien des angegebenen
  Themas ersetzten, wenn sie im angegebenen Pfad gefunden werden können.
- **default-theme:** Definiert den Namen des Farb-Schemas, das
  standardmäßig im Auswahlmenü `Thema ändern` angeboten wird. Der default ist `light`.
- **preferred-dark-theme:** Der Name des Farb-Schemas für das Rendern
  die im dunklen Thema. Dieses Thema wird angewählt, wenn im Browser
  die dunkle Version für die Darstelleung der Seite ausgewählt wird
  (vgl. via the
  ['prefers-color-scheme'](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme)). Der
  default ist `navy`.
- **curly-quotes:** Konvertiert gerade in geschwungene
  Anführungszeichen. Hiervon sind Code-Blocks und Code-Beriche
  ausgenommen. Der Standardwert ist `false`.
- **mathjax-support:** Ergänzt die Unterstützung für
  [MathJax](mathjax.md). Der Standardwert ist `false`.
- **copy-fonts:** Kopiert die Datei fonts.css zusammen mit den
  jeweiligen Schriftarten Dateien in das Ausgabeverzeichnis und wendet
  sie im Standart Thema an.
- **google-analytics:** Wenn sie Google Analytics einsetzen, aktiviert
  diese Option die Nutzung unter Angabe Deiner ID in der Konfigurationsdatei.
- **additional-css:** Wenn nur einzelne Änderungen im Aussehen des zu
  rendernden Buches vornehmen, kannst Du die Namen weiterer
  Style-Sheets angeben. Diese werden ergänzend zu den vorhandenen Style-Sheets
  geladen. In Deinen Style-Sheets änderst nur die zu verändernden Abschnitte.
- **additional-js:** Wenn Du nur das Verhalten beim rendern einzelner
  Teile im Buch abändern möchtest, kannst du die Namen weiterer
  JavaScript Dateien angeben, die neben den Standard Dateien
  eingebunden werden.
- **no-section-label:** mdBook wird bei der Erstellung des
  Inhaltsverzeichnisses eine automatische Nummerierung erzeugen, den
  Beschreibungen vorangestellt werden (z.B. "1.", "2.1"). Um dieses Nummerierung
  auzublenden, aktiviere bitte diesen Parameter. Der Standartwerit ist `false.`
- **fold:** Die Abschnitte in einer Seite werden in einer aufklappbaren Untertabelle angeboten.
- **playground:** Eine Untertabelle zur Auswahl von unterschiedlichen playground Einstellungen.
- **search:** Eine Zntertablelle zur Konfiguration die im Browser angebotenen Suchfunktion.
  mdBook muss hierzu mit der Option `search` übersetzt worden sein. Dies ist der Standardwert.
- **git-repository-url:** Eine URL die den Pfad zum Repository des
  Buchs angibt. Wenn definiert, wird ein anklickbarers Symbol in der Menüzele des Buchs eingefügt.
- **git-repository-icon:** Die Symbol-Klasse FontAwesome wird für die definition des Repository Links verwendet.
  Der Standardwert ist `fa-github`.
- **redirect:**  Ein Unterverzeichnis  mit einer  Umleitungsdatei. Die
  Datei  wird erstellt,  wenn  Seiten verschoben  werden. Die Tabelle
  enthält Schlüssel-Wert  Paare, in der der Schlüssel angibt, wo die
  Umleitungsdatei erstellt werden muss (z.B. `/appendices/bibliography.html`).  Als Wert sind  alle gültigen
  URI's möglich zu denen der Browser umleiten soll (z.B. `https://rust-lang.org/`,
  `/overview.html`, or `../bibliography.html`).
- **input-404:** Der Name der Markdown Quelldatei, die ausgewertet wird, wenn eine Datei fehlt.
  Der Renderprozess erzeugt entsprechend konvertierte HTML-Datei. Standardwert ist `404.md`.
- **site-url:** Die URL, auf der das Buch gehostet wird. Die Angabe
  ist erforderlich, um die `input-404` Datei korrekt die
  Navigations-Links bei CSS, bzw. Skripten einbinden zu können. Das
  ist insbesonder bei Unterverzeichnissen wichtig. Standardwert ist
  `/`.

Unterstützte Konfigurations-Optionen für den Tabelleneintrag `[output.html.fold]`:

- **enable:** Aktiviert das Einklappen von Ebenen. Wenn deaktiviert werden alle Einträge gezeigt.
  Der Standardwert ist `false`.
- **level:** Je größer der angegebene Wert, umso höher die Anzahl der angezeigten Ebenen.
  Der Standardwert ist `0`, alle Ebenen werden angezeigt.

Unterstützte Konfigurations-Optionen für den Tabelleneintrag `[output.html.playground]`:

- **editable:** Erlaubt des verändern von Quell-Code Blöcken. Standardwert ist `false`.
- **copyable:** Display the copy button on code snippets. Standardwert ist `true`.
- **copy-js:** Kopiert JavaScript Dateien für die Nutzung des Editors in das Ausgabeverzeichnis.
  Standardwert ist `true`.
- **line-numbers** Anzeige der Zeilennummern im editierbaren
Quell-Code Abschnitten. Erfordert das beide Konfigurationsparameter
`editable` und `copy-js` auf `true` gesetzt sind. Standardwert ist `false`.

[Ace]: https://ace.c9.io/

Unterstützte Konfigurations-Optionen für den Tabelleneintrag `[output.html.search]`:

- **enable:** Aktiviert die Suchfunktion. Standardwert ist `true`.
- **limit-results:** Die maximale Anzahl von Such Resultaten. Standardwert ist `30`.
- **teaser-word-count:** Die Anzahl der Worte, die im Suchergebnis des Teaser berücksichtigt werden.
  Standardwert ist `30`.
- **use-boolean-and:** Definiert den logischen Verknüpfungsoperator,
  der zwischen den angegebenen Suchbegriffen angewendet wird. Wenn
  dieser auf `true` gesetzt ist müssen alle angegebenen Suchbegriffe
  im Ergebnis vorhanden sein. Standardwert ist `false`.
- **boost-title:** Faktor für die Verstärkung von Suchresultaten
  (boost factor), wenn sich ein gesuchtes Wort im Überschrift befindet. Standardwert ist `2`.
- **boost-hierarchy:** Faktor für die Verstärkung von Suchresultaten
  (boost factor), wenn sich ein gesuchtes Wort in der Hierarchie
  befindet. Die Hierarchie beinhaltet alle Titel von hierarchisch
  übergeordneten Dokumenten als auch deren Überschriften. Standardwert ist
  `1`.
- **boost-paragraph:** Faktor für die Verstärkung von Suchresultaten
  (boost factor), wenn sich ein gesuchtes Wort im Text befindet. Standardwert ist `1`.
- **expand:** Wenn aktiviert wird die Suche auch Resultate anfügen, in
  denen sich der Begriff als Prefix verwendet wird. Beispielsweise
  wird die Suche `micro` auch Begriffe wie `microwave` einbeziehen. Standardwert ist `true`.
- **heading-split-level:** Suchergebnisse werden mit Abschnitten des
  Dokuments verknüpft, die dem angegebenen Suchmuster
  entsprechen. Dokumente werden in Kapitelüberschriften aufgesplittet,
  die kleiner oder gleich der vorgegebenen Ebene sind.  Standardwert
  ist `3`. (`### This is a level 3 heading`)
- **copy-js:** Kopiert JavaScript Dateien für den Suchprozess in das
  Ausgabeverzeichnis. Standardwert ist `true`.

Nachfolgend noch einmal alle verfügbaren HTML Ausgabe Optionen in der
Datei **book.toml**:

```toml
[book]
title = "Example book"
authors = ["John Doe", "Jane Doe"]
description = "The example book covers examples."

[output.html]
theme = "my-theme"
default-theme = "light"
preferred-dark-theme = "navy"
curly-quotes = true
mathjax-support = false
copy-fonts = true
google-analytics = "UA-123456-7"
additional-css = ["custom.css", "custom2.css"]
additional-js = ["custom.js"]
no-section-label = false
git-repository-url = "https://github.com/rust-lang/mdBook"
git-repository-icon = "fa-github"
site-url = "/example-book/"
input-404 = "not-found.md"

[output.html.fold]
enable = false
level = 0

[output.html.playground]
editable = false
copy-js = true
line-numbers = false

[output.html.search]
enable = true
limit-results = 30
teaser-word-count = 30
use-boolean-and = true
boost-title = 2
boost-hierarchy = 1
boost-paragraph = 1
expand = true
heading-split-level = 3
copy-js = true

[output.html.redirect]
"/appendices/bibliography.html" = "https://rustc-dev-guide.rust-lang.org/appendix/bibliography.html"
"/other-installation-methods.html" = "../infra/other-installation-methods.html"
```

### Markdown Renderer

Renderer verwenden den angegebenen Pre-Prozessor, um den übergebenen
Eingabestream (Markdown Quelle) zu verarabeiten. Das ist für die
Fehlersuche in Pre-Prozessoren sehr hilfreich, insbesondere in
Verbindung mit `mdbook test`. Der Steam enhält den Markdown Code, den
`mdbook` an `rustdoc` übergibt.

Der Markdown Renderer ist in der Standardinstallation von `mdbook`
enthalten, aber standardmäßig deaktiviert.  Er wird mit einem leeren
Tabelleneintrag in `book.toml` wie folgt aktiviert:

```toml
[output.markdown]
```

Es gibt zur Zeit keine sonstigen Konfigurationsparameter.

Lies bitte im Abschnitt [Pre-Prozessor
Dokumentation](#configuring-preprocessors) nach, wie angegeben werden
kann, dass ein bestimmter Pre-Prozessor vor dem Renderlauf gestartet
wird.

### Eigene Renderer

Ein selbst geschriebener Renerer wird eingebunden, wenn du einen
Tabelleneintrag `[output.foo]` in der Datei `book.toml` erstellst.  In
Analogie zu den Ausführungen in der [Pre-Prozessor
Dokumentation](#configuring-preprocessors) wird `mdbook` damit angewiesen
den Markdown-Stream deines Buches an `mdbook-foo` zu übergeben, der dann den Renderjob übernimmt.
Lies bitte im Kaptitel [Alternative Backends] für weitere Details.

Dein eigener Renderer verfügt über den Zugriff auf alle Parameter im Tabelleneintrag (d.h.
alles unterhalb von `[output.foo]`). mdBook wird zwei Parameter überprüfen:

- **command:** Das Kommando für die Ausführung deines Renderers. Der Standardwert sucht nach einer ausführbaren Datei
  der `mdbook-` als Prefix vorangestellt wird (hier: `mdbook-foo`).
- **optional:** Wenn der Parameter auf `true` gesetzt wird, ist die
  Ausführung des Renderers optional.  Der Standardwert ist `false`,
  d.h. `mdBook` wird mit einer Fehlermeldung beendet, wenn der
  Renderer nicht ausgeführt werden kann.

[Alternative Backends]: ../for_developers/backends.md
