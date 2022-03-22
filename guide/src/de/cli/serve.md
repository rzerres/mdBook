# Das Kommand serve

Das Kommando serve wird verwendet, um die Vorschau des Buchs
anzuzeigen. Als Unterlassungswert wird die URL HTTP `localhost:3000`
genutzt.

```bash
mdbook serve
```

Das `serve` Kommand überwacht da Quellverzeichnis deines Buchs `src`
auf Veränderungen. Werden Veränderungen festgestellt wird automatisch
ein rebuild ausgelöst und der Client darüber informiert. Das Beinhalte
die erneute Erstellung von manuell gelöschten Dateien, wenn diese
weiterhin in der `SUMMARY.md` Datei aufgeführt werden! Für den Refresh
wird als Trigger eine websocket Verbindung verwendet.

> **Anmerkung:** Das Kommando `serve` dient für den Test der Buch HTML
 Ausgabe. Es ersetzt keinen vollständigen WebServer für eine
 Webseite.

#### Verzeichnisangabe

Dem `build` Kommando kannst Du mit dem Aufruf ein Verzeichnis als Argument
übergeben. Es wird als neue Wurzel (`root`) anstelle Deines
aktuellen Verzeichnisses für die Ausgabe verwendet.

```bash
mdbook serve path/to/book
```

#### Server options

`serve` unterstützt vier Parameter: Den HTTP Port, Den WebSocket Port, den HTTP Hostnamen
der abgehört wird und der Hostname des WebBrowsers der sich zum WebSockets verbindet.

Als Beispeil: Nehmen wir an, du hast einen nginx server der SSL
Sitzungen terminiert der auf die öffentliche Address 192.168.1.100 auf
Port 80 hört und Anfragen über die proxy Funktionalität an die Addresse 172.0.0.1 auf port 8000
weiterleitet. Zur Verwendung dieser Umgebung startest Du:

```bash
mdbook serve path/to/book -p 8000 -n 127.0.0.1 --websocket-hostname 192.168.1.100
```

Soll ein live Reload ausgeführt werden, musst Du das WebSocket ebenso
über nginx von `192.168.1.100:<WS-PORT>` auf `127.0.0.1:<WS_PORT`
umleiten. Der Parameter `-w` erlaubt es diesen WebSocket Port zu definieren.

#### --open


Wenn du beim Aufruf von mdbook den Parameter `--open` (`-o`) angibst,
wird nach Abschluss des Render-Prozesses der Einstiegspunkt in
Dein Buch im default Browser geöffnet.

When you use the `--open` (`-o`) flag, mdbook will open the book in your
default web browser after starting the server.

#### --dest-dir


Mit dem Parameter `--dest-dir` (`-d`) kannst Du beim Aufruf von mdbook
das Ausgabeverzeinis verändern.  Dabei werden sowohl relative wie
absolute Pfadangaben ausgewertet. Wird der Parameter nicht angegeben,
wendet mdbook den Unterlassungswert an, der in der Datei `book.toml`
dem Schlüsselwert `build.build-dir` zugewiesen wurde. Fehlt dieser,
wird in das Verzeichnis `./book` ausgegeben.

#### Angabe von Ausschluss Kriterien

Das Kommando `serve` wird keinen automatischen `build` von Dateien
starten, die in der Konfigurationsdatei `.gitignore` im
Wurzelverzeichnis deines Buchs angegeben sind. Die Datei `.gitignore`
wertet solche Musterwerte aus, wie sie in der [gitignore
Dokumentation](https://git-scm.com/docs/gitignore) beschrieben
sind. Dies ist sehr hilfreich, um die von manchen Editoren temporär
erzeugte Dateimuster auszuschließen.

***Anmerkung:*** *Nur die `.gitignore` Datei im Wurzelverzeichnis des
Buchs wird ausgewertet. Globale Werte aus `$HOME/.gitignore` oder
`.gitignore` Dateien in übergeordneten Verzeichnissen werden
ignoriert.*
