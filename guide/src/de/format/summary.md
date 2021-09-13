# Zusammenfassung

Eine Zusammenfassung (`SUMMARY.md`) wertet mdBook aus, um
festzustellen welche Kapitel eingebunden werden sollen, in welcher
Reihenfolge das erfolgen soll, wie deren Hierarchie aufgebaut wird und
an welcher Stelle die zugehörigen Quelldateien gefunden werden können.
Ohne dies Datei - kein Buch!

Auch wenn `SUMMARY.md` als Markdown Quelle erzeugt wird, ist deren Formatierung sehr strikt und ermöglicht ein einfache Prüfung (`parsen`).
Schauen wir uns an, wie Du `SUMMARY.md` richtig formatierst:

#### Die Struktur

1. ***Titel*** Es ist üblich, mit einem Titel zu
   beginnen. Normalierweise lautet diese Zeile <code
   class="language-markdown"># Zusammenfassung</code>. Aber das ist
   nicht zwingend. Der Parser ignoriert es, und wenn Du willst, kannst
   Du das auch.

   ```markdown
   # Summary
   ```

2. ***Vorspann Kapitel*** - Bevor Du mit den nummerierten
   Hauptkapiteln beginnst, kannst Du Elemente einfügen, die nicht mit
   nummeriert werden sollen. Das ist z.B. nützlich, wenn es um
   Vorworte, Einführungen, etc geht. Es gilt aber ein paar
   Einschränkungen zu beachten:

	* Diese können nicht verschachtelt werden, sollten sich alle im Wurzelverzeichnis befinden.
	* Du kannst diese nicht mehr einfügen, wenn du bereits nummerierbare Hauptkapitel eingefügt hast.

   ```markdown
   [Ein Vorspann Kapitel](relative/path/to/markdown.md)

   - [Erstes Kapitel](relative/path/to/markdown2.md)
   ```

3. ***Titel mit Vorspann:*** - Die Überschriften können als Titel für die
   folgenden nummerierten Kapiteln fungieren. Dies kann zur logischen
   Trennung verschiedener Abschnitte des Buches verwendet werden. Der
   Titel wird so gerendert, das er als Text mit der Maus nicht
   angeklickt werden kann. Er ist optional. Numerierte Kapitel können
   in beliebig tief verschachtelte Teile aufgesplittet werden.

   ```markdown
   # My Part Tile

   - [Erstes Kapitel](relative/path/to/markdown.md)
   ```

4. ***Nummerierte Kapitel*** - Die nummerierten Kapitel stellen den
   Hauptteil in einem Buch dar. Ihre Nummerierung erfolgt automatisch
   und fortlaufend. Sie können diese verschachteln und damit die
   gewünschte hierarchische Struktur abbilden (i.d.R. Kapitel, Sub-Kapitel, etc.).

   ```markdown
   # Titel der Kategorie

   - [Erstes Kapitel](relativer/pfad/zur/markdown.md)
   - [Zweites Kapitel](relativer/pfad/zur/markdown2.md)
	  - [Unter Kapitel](relativer/pfad/zur/markdown3.md)

   # Title der nächsten Kategorie

   - [weiteres Kapitel](relativer/pfad/zum/markdown2.md)
   ```

   Sie können entweder `-` oder `*` verwenden um damit ein nummeriertes Kapitel anzuzeigen.

5. ***Kapitel mit Nachsatz*** - Sie verhalten sich wie
   Vorspann-Kapitel und werden *nicht* nummeriert. Sie stehen immer
   hinter nummerierten Kapiteln.

   ```markdown
   - [Letztes Kapitel](relative/path/to/markdown.md)

   [Titel des Nachsatz Kapitels](relative/path/to/markdown2.md)
   ```

- ***Entwurfs-Kapitel*** - Es handelt sich um eine Sonderform von
  Kapitlen, die auf keine Datei verweisen.  Sie besitzen folglich
  keinen Inhalt. Sinnvoll sind Entwurfs-Kapitel deshalb, weil sie auf
  Inhalte verweisen, die sich in Vorbereitung befinden. Sie werden
  auch dann verwendet, wenn der Autor eine noch nicht finalisierte
  Strukur erarbeitet, die noch häufigen Veränderungen
  unterliegt. Entwurfs-Kapitel werden in der gerenderten HTML Ausgabe
  im Inhaltsverzeichnis **ohne** aktive Verweise angefügt. Sie werden
  wie normale Kapitel erstellt, ihnen fehlt jedoch der Pfad zu einer
  Datei.

  ```markdown
  - [Entwurfs-Kapitel]()
  ```

- ***Trennzeichen*** - Zwischen den Kapiteln können Trennzeichen
  eingefügt werden. In der gerenderten HTML Ausgabe werden solche
  Zeilen innerhalb des Inhaltsverzeichnisses ausgegeben. Ein
  Trennzeichen ist eine Linie, die ausschließlich Bindestriche
  enthält. Es werden immer drei Bindestriche gerendert: `---`.


### Beispiel

Der nächste Absatz zeigt die Markdown Quelldatei für `SUMMARY.md`
dieses Leitfadens mit dem gerenderten Inhaltsverzeichnis auf der
linken Seite.

```markdown
{{#include ../SUMMARY.md}}
```
