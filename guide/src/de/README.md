# Einführung

**mdBook** ist ein Kommandozeilen Werkzeug und Rust crate, um Bücher
aus Markdown Quellcode zu erstellen. Die Ausgabe ähnelt Tools wie
Gitbook. mdBook ist ideal für die Erstellung von Produkt- oder
API-Dokumentation, Tutorials, Kursmaterialien. Es erstellt eine
saubere, leicht navigierbare Ausgabe. Da es mit
[Rust](https://www.rust-lang.org) erstellt wurde, eignet es sich dank
hoher Geschwindigkeit und Einfachheit als Werkzeug, um
Veröffentlichungen direkt auf gehostete Webseiten auszugeben.  Zudem
erleichtern dies Autmatisierungswerkzeuge wie beispielsweise [GitHub
Pages](https://pages.github.com). Dieser Leitfaden dient einerseits
als mdBook Dokumentation. Andererseits ist es ein hervorragendes
Beispiel, wie eine gerenderte Ausgaben aussehen kann, die mit
mdBook erzeugt wurde.

mdBook beinhaltet sowohl die Unterstützung für das Pre-Processing
deiner Markdown Quellen, als auch die unterstützung alternative
Renderer, die in anderer Formate als HTML ausgeben können. Seine
Fähigkeiten ermöglichen es auch andere Funktionen umzusetzen. Hierzu
zählt z.B. die Validierung des Quellmaterials. Eine
[Suche](https://crates.io/search?q=mdbook&sort=relevance) in Rust's
[crates.io](https://crates.io) ist eine großartige Möglichkeit, solche
Erweiterungen zu entdecken.

## API Dokumentationen

Neben diesem Buch kannst Du die mit Rustdoc erzeugte [API Dokumentation]
(https://docs.rs/mdbook/*/mdbook/) online lesen.  Wenn du mdBook als crate
verwenden, oder die vorliegenden Texte neu rendern möchtest, wird dir dieser
Link mit weiterführenden Erläuterungen bei der Einarbeitung helfen.

## Markdown

mdBook's [Parser](https://github.com/raphlinus/pulldown-cmark) liegt
die [CommonMark](https://commonmark.org/) Specification zu grunde. Du
kannst dir mit dem [Tutorial](https://commonmark.org/help/tutorial/)
einen schnellen Überblick verschaffen, Common Mark online
[ausprobieren](https://spec.commonmark.org/dingus/). Für eine
ausführliche Beschreibung der Sprache lies bitte im [Markdown
Guide](https://www.markdownguide.org).

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
[PR](https://github.com/rust-lang/mdBook/pulls) ein.

## License

mdBook, mit dem vollständigen Quellcode wird unter der [Mozilla Public
License v2.0](https://www.mozilla.org/MPL/2.0/) freigegeben.
