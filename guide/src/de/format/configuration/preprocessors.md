## Konfiguration von Pre-Prozessoren

In der Standardinstallation sind folgende Pre-Prozessoren verfügbar.

- `links`: Vervollständigt das Handlebars Hilfstool für `{{
  #playground }}`, `{{ #include }}`, und `{{ #rustdoc_include }}`
  in einem Kapitel und fügt den Inhalt der Datei ein.
- `index`: Wandelt alle Kapiteldataeien mit dem Namen `README.md` in
  die Datei `index.md`. Im Ergebnis werden somit alle `README.md`
  Dateien im erstellen Buch zu einem Index `index.html` gerendert.

**book.toml**
```toml
[build]
build-dir = "build"
create-missing = false

[preprocessor.links]

[preprocessor.index]
```

### Angepasste Pre-Prozessor Konfiguration

Wie auch bei Renderer erfordert die Verwendung explizit einzubindender
Pre-Prozessoren die Angabe eines entsprechenden Tabelleneintrags
(z.B. `[preprocessor.mathjax]`). In diesem Abschnitt kannst Du dann
zusätzliche, für diesen Pre-Prozessor anzuwendende Parameter ( key = value)
definieren.

Hier ein Beispiel:

```toml
[preprocessor.links]
# set the renderers this preprocessor will run for
renderers = ["html"]
some_extra_feature = true
```

#### Bindung einer PreProzessor Abhängikeite an einen Renderer

Du kannst erzwingen dass ein Pre-Prozessor einen bestimmten Renderer anwendet, indem Du die gewünschten Parameter bindest:

```toml
[preprocessor.mathjax]
renderers = ["html"]  # mathjax macht nur im HTML Renderer Sinn
```

### Eigene Kommandos

Wenn Du einen Tabelleneintrag `[preprocessor.foo]` definierst, wird
`mdBook` im Standardverhalten versuchen eine Binärdatei `mdbook-foo`
auszuführen. Wenn Du möchtest, dass auf ein anderer Programmname
(gegebenfalls auch mit Parametern) ausgeführt wird, kann dieses
Verhalten mit dem Parameter `command` überschrieben werden.

```toml
[preprocessor.random]
command = "python random.py"
```
