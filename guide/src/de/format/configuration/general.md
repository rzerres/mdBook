# Generelle Konfigurationen

Du kannst für Dein Buch Steuerungs-Parameter in der Datei
***book.toml*** hinterlegen.

Dies sieht dann z.B. so aus:

```toml
[book]
title = "Example book"
author = "John Doe"
description = "The example book covers examples."
language = "en"

[rust]
edition = "2018"

[build]
build-dir = "my-example-book"
create-missing = false

[preprocessor.index]

[preprocessor.links]

[output.html]
additional-css = ["custom.css"]

[output.html.search]
limit-results = 15

[language.en]
name = "English"
```

## Unterstützte Konfigurationsoptionen

Es ist wichtig anzumerken, dass jeder relativ beschriebenen Pfad in
der Konfiguration sich immer relativ auf das Wurzelverzeichnis
(`root`) in deinem Buch bezieht. Dort erstellst Du `book.toml`.

### Generelle Metadaten

Folgende Metadaten definieren übergreifende Angaben.

- **title:** Der Buchtitel
- **authors:** Der/die Autor(en) des Buchs
- **description:** Eine Beschreibung zum Buch, die als Meta Infomation
  im HTML `<head>` einer jeden Seite angezeigt wird.
- **src:** Als Standardwert wird das Quellverzeichnis als
  Unterverzeichnis `src` des Wurzelverzeichnisses gesucht.  Du kannst
  die aber mit einem Parameter in der Konfigurationsdatei
  überschreiben.
- **language:** Die Hauptsprache des Buchs, die als Sprachattribut
  verwendet wird, z.B. `<html lang="en">`. Auf dateien der hier
  angegebenen Sprache wird zurückgegriffen, wenn für eine ausgewählte
  Übersetzung noch keine Datei gefunden werden können. Für die
  Hauptsprache muss eine Eintrag [`[language]` table](#translations)
  hinterlegt werden.

**book.toml**
```toml
[book]
title = "Mein wunderbarer mdBook Leitfaden"
authors = ["John Doe", "Jane Doe"]
description = "Dieser Leitfaden beinhaltet auch Beispiele."
src = "my-src"  # Unsere Markdownquellen befinden sich in `root/my-src` und nicht `root/src`
language = "en"
```

### Rust Optionen

Werden Test- oder Beispielprogramme in der Sprache Rust in den
Markdownquellen eingebunden, werden folgende Optionen für die
Ausführung dieser Programme über die playground
Integration ausgewertet.

- **edition**: Die default Rust Edition die bei Code snippets angewendet wird. Default
  ist "2015". Individuelle Code Blöcke können mit `edition2015`
  order `edition2018` markiert werden:

  ~~~text
  ```rust,edition2015
  // This only works in 2015.
  let try = true;
  ```
  ~~~

### Build Optionen

Diese Optionen steuern den Build Prozess für dein Buch.

- **build-dir:** Das Verzeichnis, in das alle im Renderprozess
  erstellen Dateien ausgegeben werden. Als Standardwert wird `book/`
  im Wurzelverzeichnis deines Buchs verwendet.
- **create-missing:** Fehlen Dateien im Dateisystem, die Du in
  `SUMMARY.md` definiert hast, wird mdBook diese beim parsen
  automatisch für Dich erstellen. Dieser Standard kann über den
  Konfigurationsparameter `create-missing = false` verändert
  werden. Ist dieser gesetzt, wird der build Prozess mit einem Fehler
  abgebrochen, wenn fehlende Dateien erkannt werden.
- **use-default-preprocessors:** Der Standard Pre-Prozessor für (`links` &
  `index`) wird mit Angabe dieses Konfigurationsparameters auf `false` deaktiviert.

  Wenn Du die gleichen und/oder andere Pre-Prozessoren über Einträge
  eine speziellen Konfigurationstabelle definiert hast, haben dies Vorrang und werden
  ausgeführt.

  - Nur zur Klarstellung: Fehlt eine Pre-Prozessorangabe in der
	Konfiguration wird der Standardwert für `links` and `index` im build ausgeführt.
  - Der Konfigurationsparameter `use-default-preprocessors = false`
	deaktiviert explizit diese Standart Einstellung.
  - Wird z.B ein eine Tabelleneintrag `[preprocessor.links]`
	hinterlegt, hat dieser bei der Ausführung Vorrang. Egal welcher
	Wert für `use-default-preprocessors` eingetragen wurde.
