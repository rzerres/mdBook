# Themen

Der Renderer nutzt das [handlebars](http://handlebarsjs.com/)
Template, um deine Markdown Dateien zu übersetzen. Wenn Du `mdbook`
installierst, wird Handelbars und ein Standard-Thema ausgerollt.

Dieses Thema kann von Dir vollständig auf deine Bedürfnisse angepasst
werden. Wenn Du möchtest, erstellst Du im `src` Verzeichnis deines
Buch Wurzelverzeichnisses (`root`) einen eigenen Unterordner `theme`.
Hier erstellst Du selektiv die Dateien, die Du anpassen
möchtest. `mdbook` wird dann diese Dateien anstelle der im
Standard-Thema definierten verwenden.

Die folgende Liste beschreibt die veränderbaren Dateien:

- **_index.hbs_** das Handlebars Template.
- **_head.hbs_** wird an die HTML Abschnitt `<head>` angehängt.
- **_header.hbs_** mit Inhalt, der am Anfang jeder Buchseite angehängt wird.
- **_book.css_** beschreibt den Style, der in der Ausgabe verwendet
  wird. Wenn Du das Design deines Buches verändern möchtest, ist das
  die Datei die Du anpassen solltest. Willst Du auch das Layout
  signifikant verändern, sollten die Anpassungen mit der Datei `index.hbs`
  abgestimmt sein.
- **_book.js_** wird sicher am häufigsten angepasst, um lokale
  genutzet Funktionen umzusetzen. Beispiele sind u.a. Anpassungen an
  der Seitenleiste oder Themen-Anpassungen.
- **_highlight.js_** ist die JavaScript Datei, die für die
  syntaktische Hervorhebung von Quellcode-Dateien (`snippets`)
  verwendet wird. In der Regel wird Du diese nicht anpassen müssen.
- **_highlight.css_** ist das Thema, das für die syntaktische Hervorhebung zuständig ist.
- **_favicon.svg_** und **_favicon.png_** definiert das anzuwendende
  Favoriten-Symbol. Die SVG Variante wird in [neueren Browsern] angewendet.

Generell git, wenn Du ein Thema anpassen willst, musst Du nicht alle
Dateien überschreiben. Wenn Du lediglich Anpassungen am StyleSheet
vornimmst reicht es, auch nur die Datei anzupassen, die dafür
zuständig ist. Weil angepasste Dateien vorrangig ausgewertet werden,
wirst Du so automatisch von Fehlerbehebungen und neuen Funktion der
Upstream-Version profitieren.

> **Anmerkung:** Wenn Du Dateien überschreibst, ist es möglich das
> Du dadurch unbeabsichtigt Funktionen beeinträchtigst. Daher solltest Du
> zunächst die Themen-Standard-Datei als Ausgangs-Template verwenden
> und nur die Teile anpassen, die für Dich wichtig sind.
>
> Mit dem Befehl `mdbook init --theme` kannst du das gesamte
> Standard-Thema in Deine Buch-Quelle kopieren. Lösche dann einfach
> die Dateien, die Du nicht anpassen willst.
>
> Wenn du alle Default-Themen vollständig ersetzen willst, stelle
> sicher, dass Du in der `boot.toml` Konfiguration den Parameter für
> [`output.html.preferred-dark-theme`] anpasst. Dieser zeigt per ohne
> Anpassung auf das Default-Thema `navy`.

[`output.html.preferred-dark-theme`]: ../config.md#html-renderer-options
[neueren Browsern]: https://caniuse.com/#feat=link-icon-svg
