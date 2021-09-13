# index.hbs

Das Handlebars-Template hat den Namen `index.hbs`. Es kommt beim
Rendern des Buches zur Anwendung. Die Markdown Quellen werden zu HTML
Dateien konvertiert und dann an dieses Template übergeben.

Wenn Du Anpassungen am Template oder dem Style deines Buches vornehmen
möchtest, wirst Du wahrscheinlich hier Veränderungen einarbeiten
müssen. Dazu solltest Du Folgendes wissen.

## Daten

Viele Metadaten werden für das Hanlebars-Template mit ihrem "context"
zugänglich gemacht. Im Handlebars-Template kannst Du auf diesen
"context" wie folgt zugreifen:

```handlebars
{{name_of_property}}
```

Eine Liste der veröffentlichten Eigenschaften:

- ***language*** Die Sprache Deines Buches in der Form `en`, wie sie
  im `book.toml` definiert wurde. Wurde kein Wert hinterlegt wird der
  Standardwert `en` verwendet. Verwende beispielsweise <code
  class="language-html">\<html lang="{{ language }}"></code>.
- ***title*** Der Titel der aktuellen Seite. Dies ist identisch mit
  `{{ book_title }} - {{ chapter_title }}` es sei denn `book_title`
  wurde nicht definiert. In diesem Fall wird der Default
  `chapter_title` gesetzt.
- ***book_title*** Titel des Buchs, wie er in `book.toml` definiert wurde.
- ***chapter_title*** Titel des aktuellen Kapitels, wie es in
  `SUMMARY.md` angegeben wurde.
- ***path*** Der relative Pfad zur original Markdown Datei, ausgehend
  vom aktuellen Quell-Verzeichnis directory
- ***content*** Dies ist das gerenderte Markdown.
- ***path_to_root*** Ein Pfad, der ausschließlich `../`
  Angaben enthält und ausgehend von der aktuellen Datei auf das
  Wurzelverzeichnis deines Buchs verweist. Weil die Original
  Dateistruktur gepflegt wird, ist es hilfreich bei relativen
  Pfadangaben den Wert dieser Variable voranzustellen
  (`path_to_root`).
- ***chapters*** Enthält ein Array von Verzeichnissen in der Form

  ```json
  {"section": "1.2.1", "name": "name of this chapter", "path":
  "dir/markdown.md"}
  ```

  Es enthält alle Kapitel des Buches. Der Inhalt wird beispielsweise
  für die Erstellung des Inhaltsverzeichnisses herangezogen.

## Regler und Hilfsmittel

Zusätzlich zu den abrufbaren Eigenschaften stehen Dir folgend
Handlebars-Hilfsmittel zur Verfügung:

### 1. Inhaltsverzeichnis

Dieser Helper wird mit

```handlebars
{{#toc}}{{/toc}}
```

aktiviert und wird - abhängig von der Struktur deines Buchs - eine
Ausgabe mit folgendem Inhalt produzieren

```html
<ul class="chapter">
	<li><a href="link/zur/datei.html">Ein Kapitel</a></li>
	<li>
		<ul class="section">
			<li><a href="link/zur/anderen_datei.html">Ein anderes Kapitel</a></li>
		</ul>
	</li>
</ul>
```

Wenn Du ein Inhaltsverzeichnis mit einer anderen Struktur erzeugen
möchtest, kannst Du dies umsetzen, da wahrscheinlich alle
erforderlichen Inhalte zu den Kapitel Eigenschaften abgerufen werden
können. Einzige derzeitige Einschränkung: Du musst dies in JavaScript
implementieren, da kein Handlebars-Helper derzeit dafür geschrieben wurde.

```html
<script>
var chapters = {{chapters}};
// Verarbeitung startet hier
</script>
```

### 2. vorheriger / nächster

Die Helper für `previous` und `next` werten die Eigenschaften `link` und `name` aus.
Die verweisen auf das jeweils vorherige und nachfolgende Kapitel.

Du verwendest den Helper mit

```handlebars
{{#previous}}
	<a href="{{link}}" class="nav-chapters previous">
		<i class="fa fa-angle-left"></i>
	</a>
{{/previous}}
```

Der innere HTML Code wird nur gerendert, wenn das vorherige, bzw. nachfolgende Kapitel existiert.
Natürlich kannst die diesen inneren HTML Code nach Deinen Vorstellungen umschreiben.

------

*Wenn du weitere Eigenschaften für sinnvoll hälst und benötigst,
erstelle auf GitHub bitte eine [neues Thema](https://github.com/rust-lang/mdBook/issues).*
