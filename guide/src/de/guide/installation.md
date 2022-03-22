# Installation

Du kannst die Installation von mdBook über die Kommandozeile (CLI)
steuern.  Wähle aus der Auswahl die für Dich passende Methoden.  Wenn
Du mdBook über eine automatische Bereitstellung installieren willst,
lies bitte für weitere Details im Kapitel [continuous integration].

[continuous integration]: ../continuous-integration.md

## Vorkomplierte Binärdateien

Bereits für Deine Plattform übersetzte Binärdateien werden über den
Link [GitHub Releases page][releases] zum Download bereitgestellt.  Du
findest dort Archive, die die Programmversionen von `mdbook` für das
jeweilige Betriebssystem enthalten (Windows, macOS, or
Linux). Entpacke die jeweilige Version und kopiere sie im Dateisystem
an einen Ort, der im Suchpfad aufgelöst werden kann (vgl. `PATH`).

[releases]: https://github.com/rust-lang/mdBook/releases

## Erstellung aus den Quellen mittels Rust

Um eine ausführbare Programmversion von `mdbook` aus dem Quellcode zu
erzeugen, musst Du gegebenenfalls zunächst Rust und Cargo installieren.

Folge hierzu bitte den Angaben auf der [Rust Installationsseite].
mdBook benötigt derzeit als Minimalvoraussetzung die Rust Version 1.46.

Ist Rust verfügbar, erstellt und installiert der nachfolgende Aufruf die gewünschte Programmverson von `mdbook`:

```sh
cargo install mdbook
```

mdBook wird zunächst inklusive der Auflösung definierter
Abhängigkeiten von [crates.io] heruntergeladen. Anschießend startet
der Übersetzungsprozess und das Programm wird ins Cargo's
Programmverzeichnis kopiert (Standardwert: `~/.cargo/bin/`).

[Rust Installationsseite]: https://www.rust-lang.org/tools/install
[crates.io]: https://crates.io/

### Installation der aktuellsten master Version

Die auf crates.io veröffentlichte Programmversion wird immer leicht
hinter der auf GitHub verwalteten Quellcode-Version
abweichen. Benötigst Du die letzte verfügbare Version, kannst Du die
dort bereitgestellte mdBook Git-Version selber leicht erstellen. Cargo ***vereinfacht*** diesen Prozess sehr!

```sh
cargo install --git https://github.com/rust-lang/mdBook.git mdbook
```

Wie schon weiter oben erwähnt, stelle bitte sicher, dass das
Binäreverzeichnis für mit Cargo erzeugte Programme in der
Suchpfad-Variable Deines Betriebssystems (`PATH`) enthalten ist.

Wenn Du mithelfen möchtest Änderungen an mdBook selbst umzusetzten,
lese bitte für weiter Details im Kapitel [Contributing Guide].

[Contributing Guide]: https://github.com/rust-lang/mdBook/blob/master/CONTRIBUTING.md
