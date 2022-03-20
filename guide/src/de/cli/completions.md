# Das Autovervollständigungs Kommando

Die Kommando für die automatische Vervollständigung wird verwendet, um
für gängige Kommandozeilen Shell die auto-completion Konfigurationen
zu erzeugen. Das bedeutet, dass `mdbook` Aufrufe in der von Dir
verwendeten Shell mit den gültigen Optionen automatisch
vervollständigt werden können.

Hierzu muss die auto-complete Taste (üblicherweise die TAB-Taste)
aktiviert werden. Eine eindeutig erweiterbare Eingabe wird dann mit
dem vollständigen Optionsbezeichner ergänzt, oder eine Auswahl der
verfügbaren Optionen angeboten.

Zunächst ist dazu die Vervollständigung im Betriebssystem zu installieren:

```bash
mdbook completions bash > ~/.local/share/bash-completion/completions/mdbook
```

Das Kommando erzeugt die Anweisungen für das
Vervollständigungs-Skript.  Der Aufwurf von `mdbook completions
--help` listet die Namen der unterstützten Shell-Programme.

Der Installationsort für das erzeugte Vervollständigungs-Skript
unterscheidet sich je nach verwendetem Betriebssystem und der
gewählten Shell. In der Programmdokumentation deines OS werden die
Systemvorgaben für gültige Suchpfade angegeben (vgl. `man
<shell-name>`).
