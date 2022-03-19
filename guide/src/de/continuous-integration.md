# `mdbook` in Continuous Integration einbinden

Es gibt eine Vielzahl von Diensten, beispielsweise [GitHub Actions] oder [GitLab
CI/CD], die zum automatisierten Testen und Ausrollen deines Buchs verwendet werden können.

Die nachfolgenden Anmerkungen geben generelle Hinweise, wie Du einen
solchen Dienst für dein mdBook verwenden kannst.
Spezielle Formeln findest auf der Wiki Seite [Automated Deployment].

[GitHub Actions]: https://docs.github.com/en/actions
[GitLab CI/CD]: https://docs.gitlab.com/ee/ci/
[Automated Deployment]: https://github.com/rust-lang/mdBook/wiki/Automated-Deployment

## Installing mdBook

Es gibt einige Strategien um mbBook zu installieren.
Die für Dich passende Methode hängt von den Anforderungen und Präferenzen ab.

### Vorkompilierte Binärdateien

Die wahrscheinlich einfachste Methode ist die Verwendung von
vorkompilierten Binärdateien, wie sie auf [GitHub Releases
page][releases] gefunden werden können. Ein einfacher Ansatz ist es,
das bewünschte Programm mit dem beliebten CLI-Werkzeug `curl` herunterzuladen:

```sh
mkdir bin
curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.15/mdbook-v0.4.15-x86_64-unknown-linux-gnu.tar.gz | tar -xz --directory=bin
bin/mdbook build
```

Einige Überlegungen zu diesem Ansatz:

* Es ist relativ schnell und benötigt nicht notwendigerweise die Handhabung von Zwischenspeicherung.
* Rust muss nicht zuvor installiert werden.
* Wenn eine spezielle URL angegeben wird bedeutet die, Du musst diese manuell anpassen, wenn eine neuere Version verwendet werden soll.
  Das ist vielleicht von Nutzen, wenn Du bei einer speziellen Version verbleiben willst.
  Wie auch immer, einige Anwender bevorzugen den Einsatz einer aktuelleren Version, zum Zeitpunkt ihrer Verfügbarkeit.
* Du bist darauf angewiesen, dass das GitHub CDN verfügbar ist.

[releases]: https://github.com/rust-lang/mdBook/releases

### Erstellung aus dem Quellcode

Um die Version aus dem Quellcode selber zu bauen erzwingt die
vorherige Installation von Rust. Einige Dienste haben Rust bereits
vorinstalliert. Falls nicht, musst Du eine Anweisung ergänzen, die
diese umsetzt.

Wenn Rust installiert wurde, kannst Du mit dem Aufruf von `cargo
install` die mbBook Version erzeugen und installieren. Wir empfehlen Dir, dabei
der SemVer Spezifikation zu folgen. Die kann sicherstellen, dass du immer nur eine
**funktionsfähige** Version von mbBook verwendest. Hier ein Beispiel:

```sh
cargo install mdbook --no-default-features --features search --vers "^0.4" --locked
```

Die bindet einige empfohlene Optionen ein:

* `--no-default-features` — Deaktiviert Funktionen wie beispielsweise
  einen HTTP server, der mit `mdbook serve` aktiviert werden
  kann. Dies ist in einer CI Umgebung wahrscheinlich nicht
  notwendig. Sicher beschleunigt diese Abschaltung die Erstellungszeit
  signifikant.
* `--features search` — Wenn die Standard-Funktionen deaktiviert
  werden erfordert dies natürlich, dass die gewünschten Dienste
  manuell aktiviert werden müssen. Hierzu gehört beispielsweise die im
  code eingebaute [Suche] Funktion.
* `--vers "^0.4"` — Dies wird die aktuellste Version der `0.4` Series
  installieren. Jedoch verhindert diese Angabe auch, dass
  Nachfolge-Versionen wie beispielsweise `0.5.0` zur Anwendung kommen.
  Cargo wird deine mdBook Version automatisch aktualisieren, wenn die
  bereits installierte Version veraltet ist.
* `--locked` — Dies Angabe verwendet die Abhängigkeiten, die zur
  installierten Release-Version passen.  Ohne die `--locked` Option,
  würde die Nachfolge-Versionen mit all ihren Abhängigkeiten
  einbinden, was möglicherweise neben gewünschten Fehlerbehebungen bar
  auch (selten )dazu führen könnte, das der Erstellunglauf scheitert.

Wahrscheinlich wirst Du die Möglichkeiten des Zwischenspeicherns
überdenken, da das Erstellen von mdBook für denen Compile-Lauf etwas
langsam sein kann.

[Suche]: guide/reading.md#search

## Test Ausführung

Vielleicht willst du nach jeder Veränderung innerhalb deines Buchs
(z.b. nach pull requests oder commit pushes) Testläufe vornehmen. Dies
kann dazu verwendet werden, um Rust Quellcode-Beispiel im Buch zu prüfen.
Natürlich ist hierzu die Installation von Rust zwingend. Einige Dienste haben Rust bereits
vorinstalliert. Falls nicht, musst Du eine Anweisung ergänzen, die
diese umsetzt.

Abgesehen davon, dass Du sicherstellen mußt, dass die richtige
Version von Rust installiert ist, bleibt nicht viel außer einem Aufruf
von `mdbook test` aus dem Buch-Verzeichnis.

Vielleicht überlegst Du auch andere Test vorzunehmen. Als Beispiel kann mit
[mdbook-linkcheck] überprüft werden, ob fehlerhafte Verweise existieren.
Oder die möchtest eine Rechschreibprüfung und eigene Style-Checker zum ablauf bringen. Dies macht im CI Lauf durchaus Sinn.

[`mdbook test`]: cli/test.md
[mdbook-linkcheck]: https://github.com/Michael-F-Bryan/mdbook-linkcheck#continuous-integration

## Bereitstellung

Du wirst Dein Buch wahrscheinlich automatisch bereitstellen wollen.

Some may want to do this with every time a change is pushed, and others may want to only deploy when a specific release is tagged.

You'll also need to understand the specifics on how to push a change to your web service.
For example, [GitHub Pages] just requires committing the output onto a specific git branch.
Other services may require using something like SSH to connect to a remote server.

The basic outline is that you need to run `mdbook build` to generate the output, and then transfer the files (which are in the `book` directory) to the correct location.

You may then want to consider if you need to invalidate any caches on your web service.

See the [Automated Deployment] wiki page for examples of various different services.

[GitHub Pages]: https://docs.github.com/en/pages

### 404 handling

mdBook erstellt automatisch eine 404 Seite für alle ungültigen Verweise.
Der Standardwert für die Ausgabe ist eine Datei mit dem Namen
`404.html`, die sich im Root Verzeicnis des Buchs befindet.

Manche Dienste, wie z.B. [GitHub Pages] wird diese Seite automatisch
für ungültige Verweise verwenden.  Bei anderen kannst Du überlegen, ob
Du eine Web Server konfigurieren willst, den im Fehlerfall hilft dies
dem Leser zur aufrufenden Seite in deinem Buch zurück zu finden.

Wenn dein Buch nicht im document-root deiner Domäne bereitgestellt
wird, solltest du die [`output.html.site-url`] Seite so setzten, dass
eine 404 Seite korrekt gefunden werden kann. Daher ist es wichtig,
diesen Speicherort anzugeben, damit die statischen Dateien z.B. für
CSS aufgelöst werden können.

Ein Beispiel: Diese Anleitung soll unter <https://rust-lang.github.io/mdBook/> installiert werden,
dann ist die `site-url` Einstellung wie folgt vorzunehmen:

```toml
# book.toml
[output.html]
site-url = "/mdBook/"
```

Du kannst das Aussehen der 404 Seite in deinem Buch anpassen, indem du
die Datei `src/404.md` erzeugst.  Wenn Du einen anderen Dateinamen
verwenden willst, kannst Du in [`output.html.input-404`] auf diesen
Dateinamen verweisen.

[`output.html.site-url`]: format/configuration/renderers.md#html-renderer-options
[`output.html.input-404`]: format/configuration/renderers.md#html-renderer-options
