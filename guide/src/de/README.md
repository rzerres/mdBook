# Einführung

**mdBook** ist ein Kommandozeilen Werkzeug, um Bücher aus Markdown
Quellcode zu erstellen. mdBook ist ideal für die Erstellung von
Produkt- oder API-Dokumentation, Tutorials, Kursmaterialien, die eine
einfache, klare und adaptierbare Darstellung benötigen.


* Die einfache [Markdown] Syntax hilft Dir, Dich auf den Inhalt zu konzentrieren
* Integrierte [Suchfunktion]-Unterstützung
* [Hervorhebung von Farb-Syntax], mit denen Code-Blöcke für viele unterschiedliche Sprachen hervorgehoben werden können
* [Themen]-Dateien erlauben die Anpassung von Formatierungsanweisungen der Ausgabe
* [Pre-Prozessoren] können Erweiterungen für eine angepasste Syntax und modifizierten Inhalt bereitstellen
* [Backends] können die Ausgabe in vielfältige Formate rendern
* In [Rust] mit Blick auf Geschwindigkeit, Sicherheit und Einfachheit geschrieben
* Automatische Tests der [Rust Quellcode Fragmente]

Dieser Leitfaden ist ein Beispiel dafür, welche Ausgaben mdBook
erzeugt. mdBook wird vom Projekt `Die Programmier-Sprache Rust`
selbst verwendet. Das Buch [The Rust Programming Language][trpl] ist
ein weiteres Beispiele in dem mdBook zur Anwendung kommt.

[Markdown]: format/markdown.md
[Suchfunktion]: guide/reading.md#search
[Hervorhebung von Farb-Syntax]: format/theme/syntax-highlighting.md
[Themen]: format/theme/index.html
[Pre-Prozessoren]: format/configuration/preprocessors.md
[Backends]: format/configuration/renderers.md
[Rust]: https://www.rust-lang.org/
[trpl]: https://doc.rust-lang.org/book/
[Rust Quellcode Fragmente]: cli/test.md

## Beteiligung

mdBook ist frei und open source Software. Du findes den Quellcode auf
[GitHub](https://github.com/rust-lang/mdBook). Probleme und
Funktionserweiterungen können auf [GitHub issue
tracker](https://github.com/rust-lang/mdBook/issues) eingestellt
werden. mdBook verlässt sich auf die Community, um Fehler zu
bereinigen und neue Funktionen einzubinden. Wenn du Dich beteiligen
möchtest lies bitte unsern Leitfaden
[CONTRIBUTING](https://github.com/rust-lang/mdBook/blob/master/CONTRIBUTING.md)
oder sende deine Verbesserungen als
[Pull Request](https://github.com/rust-lang/mdBook/pulls) ein.

## Lizenz

mdBook, mit dem vollständigen Quellcode wird unter der [Mozilla Public
License v2.0](https://www.mozilla.org/MPL/2.0/) freigegeben.
