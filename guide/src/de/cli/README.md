# Kommandozeilen Tool

mdBook kann sowohl as Kommandozeilen Tool als auch als [Rust
crate](https://crates.io/crates/mdbook) verwendet werden. Lass uns
zunächst seine Fähigkeiten als Kommandozeilen Tool betrachten.
capabilities first.

## Installation aus Binärdateien

Vorkompilierte Binärdeien (`binaries`) werden für alle namhaften
Pattformen im Rahmen der gegebenen Ressourcen bereitgestellt
(`best-effort`). Bitte besuche [die Release
Seite](https://github.com/rust-lang/mdBook/releases) um die passende
Version für Deine Zeilplattform herunterzuladen.

## Installation aus dem Quellcode

mdBook kann selbstverständlich aus dem Quellcode als lauffähige Version erzeugt werden.

### Voraussetzungen

mdBook wurde vollständig in der Programmiersprache
**[Rust](https://www.rust-lang.org/)** implementiert und benötigt für
die Kompilation **Cargo**. Wenn Du nicht bereits Rust heruntergeladen hast,
 [installiere](https://www.rust-lang.org/tools/install) bitte die notwendige Toolchain.

### Installation als Crates.io Version

Die Installation von mdBook ist vergleichsweise einfach, sich Rust und
Cargo bereits ablauffähig auf deinem Rechner befinden. Du musst nur
den folgenden Code Schnipsel in einem Terminal ausführen:

```bash
cargo install mdbook
```

Dies wird den Quellcode der aktuellen Releaseversion von
[Crates.io](https://crates.io/) herunterladen und anschließend
Kompilieren. Bitte stelle sicher, das das sich Cargo's `bin` Verzeichnis in in deiner
`PATH` Environment-Variable befindet.

Mit eine Aufruf von `mdbook help` kannst du überprüfen, ob das Programm funktioniert. Glückwunsch, du hast mdBook erfolgreich installiert!

### Installation der Git Version

Die **[git version](https://github.com/rust-lang/mdBook)** enthält die
aktuelle Entwicklungsversion mit den letzten Fehlerkorrekturen und
neuesten Funktionen, wie sie nach Freigabe auf **Crates.io**
veröffentlich wird.  Wenn du nicht bis dahin abwarten möchtest, kannst
Du sie herunterladen und selber kompilieren.  Öffne ein Terminal und
wechsle ind das gewünschte Zielverzeichnis. Über das git Kommando
`clone` replizierst du das Repository und übersetzt es anschließend
mit Cargo.

```bash
git clone --depth=1 https://github.com/rust-lang/mdBook.git
cd mdBook
cargo build --release
```

Die auführbare Binärdatei `mdbook` wird im Ordner `./target/release`
erstellt. Ergänze diesen bitte zu deiner `PATH` Environment-Variable
befindet, oder kopiere das erzeugte Programm in einen aktiven
Systempfad.
