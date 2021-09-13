# Editor

Wir haben schon beschrieben, wie Rust Quellcode Dateien so eingebunden
werden können, dass der Anwender diese aus dem gerenderten Buch heraus
interaktiv ausführt kann.

Diese Funktion kann dadurch weiter ergänzt werden, dass der Anwender
diese `snippets` auch editiert. Um Quellcode Blöcke für das editieren
freizugeben, ergänzt Du folgende Zeilen im ****book.toml***:

```toml
[output.html.playground]
editable = true
```

Nun wird die Einbindungsanweisung zusätzlich mit dem Attribut `editable` ergänzt:

<pre><code class="language-markdown">```rust,editable
fn main() {
	let number = 5;
	print!("{}", number);
}
```
</code></pre>

Und voila ... der angezeigte Quellcode kann ist direkt für den playground veränderbar:

```rust,editable
fn main() {
	let number = 5;
	print!("{}", number);
}
```

> **Anmerkung:** Beim Editieren wird nun im playground auch die Option
> `Undo Changes` angeboten.

## Editor Anpassungen

Standardmäß9g wird [Ace](https://ace.c9.io/) als Editor
aufgerufen. Aber wenn Du willst kannst Du dies anpassen. Ergänze in
der ***book.toml*** Datei den Parameter mit einem Pfad zu dem von Dir
präferierten Editor.

```toml
[output.html.playground]
editable = true
editor = "/pfad/zu/dienem/editor"
```

> **Anmerkung:** Bitte beachte, dass die Editierfunktion erst dann
korrekt arbeiten kann, wenn Du im `theme` Ordner die Datei `book.js`
überschreibst. Dies ist notwendig, da der Default einige Bindungen zu
Editor `Ace` definiert.
