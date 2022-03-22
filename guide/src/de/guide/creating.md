# Ein Buch erstellen

Wurde das CLI tool `mdbook` installiert, kannst Du es für die
Erstellung und das Rendern eines Buches verwenden.

## Ein Buch initzialisieren

Das `mdbook init` Kommando erstellt ein neues Verzeichnis, in dem ein
leeres Buch angelegt wird, das Dir als Ausgangspunkt dient. Als
Parameter kannst Du den Namen mit übergeben, der für die
Verzeichnis-Benennung verwendet wird:

```sh
mdbook init my-first-book
```

Du wirst ein paar Fragen beantworten müssen, bevor das initiale Buch
erzeugt wird. Nachdem die Fragen beantwortet wurden, und der
Programmaufruf endet, kannst Du in das so neu erstelle
Unterverzeichnis wechseln:

```sh
cd my-first-book
```

Es existieren eine Vielzahl von Wegen, wie Du dein Buch rendern
kannst.  Am einfachsten verwendest du das `serve` Kommando, das das
Buch kompilieren wird und es anschließend in einem Web Browser öffnet:

```sh
mdbook serve --open
```

Dabei wird mit der Option `--open` angegeben, dass der default Browser
verwendet werden soll, dem die Buchquelle übergeben wird.  Du kannst
anschließend den server weiterlaufen lassen, auch wenn Du den Inhalt
der Buchdateien editierst. `mdbook` wird automatisch neuen Inhalt des
Buches kompilieren *und* den Browser anweisen, den neu gerenderten
Inhalt am Bildschirm auszugeben.

Für Informationen zu weiteren `mdbook` Kommandos lies bitte im [CLI
Guide](../cli/index.html).


## Die Anatomie eines Buchs

Ein Buch wird aus dem Inhalt verschiedener Dateien erstellt, die das Layout und
die für das Buch maßgeblichen Einstellungen definieren.

### `book.toml`

Im Root-Verzeichnis Deines Buchs befindet sich die `book.toml` Datei,
die Einstellungen beinhaltet, die beschreiben, wie dein Buch erstellt
werden soll. Diese Einstellungen folgen der [TOML markup
language](https://toml.io/).  Deren Default-Einstellungen genügen
gewöhnlich um Dir einen Einstieg zu ermöglichen. Wenn Du daran
interessiert bist neue Funktionen und Optionen auszutesten die mdBook
bereitstelle, lies bitte das Kapitel
[Konfiguration](../format/configuration/index.html). Es enthält alle wichtigen Details.

Ein sehr rudimentäres `book.toml` kann auf folgende Einträge reduziert werden:

```toml
[book]
title = "My First Book"
```

### `SUMMARY.md`

Der nächste wichtige Bestandteil Deines Buchs ist die
Zusammenfassungs-Datei. Sie befindet sich im Ordern `src/SUMMARY.md`.
Diese Datei enthält eine Liste aller Kapitel, die im Buch verwendet
werden. Bevor ein Kapitel eingesehen werden kann, muss es in dieser
Liste eingetragen werden.

Nachfolgend eine Beispiel mit einer Zusammenfassungs-Datei die nur aus
wenigen Kapiteln besteht.

```md
# Summary

[Einleitung](README.md)

- [Mein erstes Kapitel](my-first-chapter.md)
- [Verschachteltes Beispiel](nested/README.md)
	- [Unterkapitel](nested/sub-chapter.md)
```

Versuche nun die Datei `src/SUMMARY.md` im Editor Deiner Wahl zu
öffnen und füge neue Kapitel-Einträge hinzu.  Wenn die zugehörigen
Kapitel Dateien noch nicht existieren, wird `mdbook` diese automatisch
für Dich erzeugen.

Weiter Details bezüglich Fromatierung-Optionen für die
Zusammenfassungs-Datei findest Du in der Dokumentation zum [Kapitel
Zusammenfassung](../format/summary.md).

### Die Quell Dateien

Den Inhalt Deines Buchs befindet sich im Unterverzeichnis `src`. Jedes
Kapitel ist eine eigenständige Markdown Datei.  Üblicherweise startet
jedes neue Kapitel mit der Überschriftsebene 1, die die
Kapitel-Überschrift repräsentiert.

```md
# Mein erstes Kapitel

Ergänze hier den gewünschten Inhalt.
```

Das genau Layout der Datei bestimmst Du selbst. Die Organisation der
Dateien im Unterverzeichnis `src` wird sich mit denen der erzeugten
HTML Dateien decken. Daher bedenke bitte, dass das Datei-Layout teil
der URL eines jeden Kapitels ist.

Während das `mdbook serve` Kommando aktiv ist, kannst Du alle Kapitel
Dateien öffnen und in diesen editieren.  Immer wenn du neuen Inhalt
speicherst oder änderst wird `mdbook` Dein Buch neu erzeugen und den
Browser mit diesem neuen Inhalt aktualisieren.

Weiter Details bezüglich der Formatierung-Optionen zum Inhalt deiner
Kapitel findest Du in der Dokumentation zum [Kapitel
Markdown](../format/markdown.md).

Alle anderen Dateien im `src` Unterverzeichnis werden in der Ausgabe
berücksichtigt. Auf Bilder oder andere statische Inhalte kann einfach
aus dem `src` Verzeichnis verwiesen werden.

## Die Veröffentlichung eines Buchs

Ist Dein Buch fertig, wirst Du es sicher für andere auf einem Server
veröffentlichen wollen.  Im ersten Schritt wird Du hierzu mit dem
`mdbook build` beginnen. Der Aufruf erfolgt im gleichen Verzeichnis,
in dem Du die `book.toml` Datei abgelegt hast:

```sh
mdbook build
```

Dieser Aufruf erzeicht eine neues Unterverzeichnis `book`, der den
HTML Inhalt deines Buchs beinhaltet.  Du kannst dann anschließend den
Inhalt diese Verzeichnisses auf jeden Web-Server transferieren um das
Buch dort zu veröffentlichen.

Weiter Details bezüglich der Veröffentlichungs- und Freigabe-Optionen
findest Du im [Kapitel Continuous
Integraten](../continuous-integration.md).
