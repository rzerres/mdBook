# Lesen von Büchern

Dieses Kapitel gibt einen Überblick, wie sie als Anwender die von
mbBook erstellten Büchern nutzen können. Wir gehen im Weiteren davon
aus, dass sie ein in HTML gerendertes Buch lesen. Die Optionen und
Formatierungen bei anderen Ausgabeformaten, z.B. bei PDF Dokumenten,
werden sich unterscheiden.

Ein Buch wird in *Kapiteln* gegliedert. Jedes Kapitel ist eine
eigenständige Seite.  Kapitel können in hierarchisch in Unterkapitel
verschachtelt werden. Typischerweise wird dabei jedes Kapitel durch eine
Reihe von *Überschriften* untergliedert.

## Navigation

Es gibt verschiedene Arten, wie Du im Buch zwischen den Kapitel

Die **Übersichtsleiste** auf der linken Seite listet alle vorhandenen
Kapitel. Wenn Du eine Kapitel-Überschrift anklickst, wird die
zugehörige Seite geladen und angezeigt.

Die Übersichgtsleiste wird nur dann automatisch angezeigt, wenn die
Fenstergröße dies in sinnvoller Weise zulässt. Gerade auf Mobilen
Endgeräten würde dies zu viel Darstellungsfläche verschwenden. Deshalb
erscheint dann obern links auf der Seite ein Menü-Icon (symbolisiert
durch drei horizontale Striche). Duch Aktivierung des Icons wird die
Übersichtsleiste geöffnet, bzw. geschlossen.

Über die **Pfeile** unten auf der Seite kannst Du zum
vorherigen oder nächsten Kapitel zu wechseln.

Ebenso kannst Du dazu die **Linke und Rechte Pfeiltaste** auf Deiner Tastatur verwenden.

## Hauptmenü Zeile

Der Aufbau der Menüzeile oben auf der Seite stellt weitere Navigations-Icons zur Verfügung:
Die angezeigten Icons hängen dabei von den Einstellungen ab, die Du bei der Erzeugung des Buchs definieren kannst.

| Icon                              | Beschreibung                                                                                  |
|-----------------------------------|-----------------------------------------------------------------------------------------------|
| <i class="fa fa-bars"></i>        | Öffnet und schließt die Kapilelliste in der Übersichtsleiste.                                 |
| <i class="fa fa-paint-brush"></i> | Öffnet die Auswahlliste der verfügbaren Farb-Themen.                                          |
| <i class="fa fa-search"></i>      | Öffnet die Eingabezeile in der Du Suchbegriffe erfassen kannst.                               |
| <i class="fa fa-print"></i>       | Startet den Dialog im Browser, über den das vollständige Buch ausgedruckt werden kann.        |
| <i class="fa fa-github"></i>      | Öffnet einen Verweis auf die Web-Seite, die den Quellcode des Buchs bereitstellt.             |
| <i class="fa fa-edit"></i>        | Öffnet eine Seite, in der Du den Quell-Code der aktiv angezeigten Leseseite editieren kannst. |

Tippst Du auf die Menüzeile, wird auf der Seite nach oben geblättert.

## Suche

Jedes Buch hat ein eingebautes Such-System.  Wir das Suchsymbol in der
Menüzeile aktiviert (<i class="fa fa-search"></i>), der auf der
Tastatur der Buchstabe 'S' gedrückt, wird eine Eingabezeile geöffnet, in der Du einen Suchbegriff eingeben kannst.
Mit der Eingabe des Suchbegriffs wird das Buch in Echtzeit auf passende Kapitel und Unterkapitel durchsucht.

Wähle einfach in der Ergebnisliste einen angebotenen Treffer und die
Darstellung wechselt auf die entsprechende Seite.  Du kannst auch mit
den Pfeil-Tasten `nach oben`, bzw. `nach unten` auf das gewünschte
Resultat wechseln. Mit der Eingabezeile bestätigst Du die
hervorgehobene Auswahl.

Nachdem ein Suchergebnis geladen wurde, werden die passenden
Suchbegriffe im Text hervorgehoben.  Wenn Du auf das hervorgehobene
Wort drückst oder die `Esc` Taste verwendest, wird die Hervorhebung
deaktiviert.

## Quellcode Blöcke

Besonders häufig wird mdBook für die Dokumentation von
Programmen verwendet. Daher unterstützt es gerade die
Hervorhebung von Quellcode Blöcken und Beispielen. Quellcode Blöcke können dabei mit unterschiedlichen Symbole kommentiert werden. Diese , über die die :

books are often used for programming projects, and thus support highlighting code blocks and samples.
Code blocks may contain several different icons for interacting with them:

| Symbol                        | Description                                                                                                                                                                                                                                          |
|-------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <i class="fa fa-copy"></i>    | Kopiert den Quellcode Block in die Zwischenablage und kann damit in andere Programme übernommen werden.                                                                                                                                              |
| <i class="fa fa-play"></i>    | Für Rust Quellcode Beispiele wird dadurch die Übersetzung und die Ausgabe der Kompiliermeldungen gestartet. Diese erfolgt direkt unterhalb des Beispiel-Codes (vgl. [playground]).                                                                   |
| <i class="fa fa-eye"></i>     | Für Rust Quellcode und Beispiele werden "versteckte" Zeilen angezeigt, bzw ausgeblendet. Oftmals ist es sinnvoll solche Zeile auszublenden, deren Regionen im Quellcode nicht für die Kommentierung relevant sind (vgl [verteckte Quellcodezeilen]). |
| <i class="fa fa-history"></i> | [Editierbare Quellcode][editor] ermöglicht es getätigte Änderungen zurückzusetzen.                                                                                                                                                     |

Hier ein Beispiel:

```rust
println!("Hallo, mdBook!");
```

[editor]: ../format/theme/editor.md
[playground]: ../format/mdbook.md#rust-playground
[versteckte Quellcodezeilen]: ../format/mdbook.md#hiding-code-lines
