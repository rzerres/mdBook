# Für Entwickler

Auch wenn `mdbook` hauptsächlich ein Kommandzeilen Tool ist kannst Du
die darunterliegende Bibliothek importieren und dein Buch damit direkt
pflegen.  Es existiert ein ziemlich flexibler Plugin Mechanismus, über
den Du eigene angepaßte Wege ('custom tooling ') und Nutzer (oftmals
als *backends* bezeichnet) definieren kannst. Dies kann hilfreich
werden, wenn du weitergehende Analysen für den Code in deinem Buch
vernehmen möchtest, oder in ein anderes Format rendern möchtest.

Das Kapitel *Für Entwickler* existiert, um Dir die erweiterte
Verwendung von `mdbook` zu zeigen.

Entwickler werden sich in der Regel über einer der beiden Wege in den `build` Prozess von `mdbook` einklinken

- [Preprocessors](preprocessors.md)
- [Alternative Backends](backends.md)


## The Build Prozess

Der Prozess der zum Rendern eines Buch Projekts führt vollzieht sich in mehreren Schritten.

1. Laden des Buchs
	- Prüfung der `book.toml` Datei. Existiert diese nicht, wird auf
	  die default Werte aus `Config` zurückgegriffen.
	- Laden der Buch Kapitel in den Arbeitsspeicher
	- Ermitteln, welcher Pre-Prozessoren und welche Backends verwendet werden sollen.
2. Ablauf des Pre-Prozessors
3. Aufruf des probaten Backend


## Verwendung von `mdbook` als Bibliothek

Die ausführbare Datei `mdbook` ist nur ein Wrapper um das `mdbook`
crate.  Es stellt die Funktionalität als Kommandzeilen Tool zur
Verfügung. Daher ist es relativ einfach Dein eigenes Programm zu
erstellen, das auf der Basis des `mdbook` crates geschrieben wird.
Hier kannst Du deine eigenen Funktionen abbilden (z.B. ein neuer
Pre-Prozessor) oder den Build-Prozess anpassen.

Der einfachste Weg herauszufinden wie du das `mdbook` crate sinnvoll
verwenden kannst ist es, sich mit der [API Dokumentation] vertraut zu
machen. Auf der obersten Ebene erläutert die Dokumentation wie der
[`MDBook`] Typ geladen wird und damit das Buch erstellt werden
kann. Hingegen erläutert das Modul [Konfiguration], wie innerhalb von
`mdbook` eine Adaption und Konfiguration von Parametern verarbeitet
wird.

[`MDBook`]: https://docs.rs/mdbook/*/mdbook/book/struct.MDBook.html
[API Dokumentation]: https://docs.rs/mdbook/*/mdbook/
[Konfiguration]: https://docs.rs/mdbook/*/mdbook/config/index.html
