# Markdown in mdBook

## Code Zeilen verbergen

Wenn Du mit mdBook in einer Quellcode Datei einzelne Zeilen verbergen
möchtest, verwendest Du dazu eine einfache Syntax. Du markierst die
relevanten Zeilen in dem Du Zeichen `#` vorangestellt. Dies entspricht
der Notation, die auch [Rustdoc][rustdoc-hide] selbst verwendet.

[rustdoc-hide]: https://doc.rust-lang.org/stable/rustdoc/documentation-tests.html#hiding-portions-of-the-example

```bash
# fn main() {
	let x = 5;
	let y = 6;

	println!("{}", x + y);
# }
```

Dies wird dann im gerendten HTML wie folgt übersetzt.

```rust
# fn main() {
	let x = 5;
	let y = 7;

	println!("{}", x + y);
# }
```

## Dateien einfügen

Mit der folgenden Syntax können vollständige Dateien in mdbook eingefügt werden:

```hbs
\{{#include file.rs}}
```

Der Pfad zur Datei muss in relativer From zur aktiven Quelldatei angegeben werden.

mdBook wird inkludierte Dateien immer als Markdown Quellen interpretieren.

Oftmals soll die Quellcode Sequenze oder ein Beispiel in mdbook
eingefügt werden, deren Inhalte aber nicht interpretiert werden. Du
möchtest lediglich die `raw` Zeilen im Buch angeben. Um das zu
erreichen umrahmst du die Einfüge-Anweisung mit der
Zeichenfolge

	```` ``` ````

schließt sie also mit vier backquotes ein. Das sieht dann so aus:

````hbs
```
\{{#include file.rs}}
```
````

## Teile einer Datei einbinden

Du möchest nur einzelne Zeilen oder einen bestimmten Teil einer Datei
einbinden (z.B: Zeilen aus einem Beispiel erläutern). mdbook
unterstützt hierzu vier verschiedene Modi:

```hbs
\{{#include file.rs:2}}
\{{#include file.rs::10}}
\{{#include file.rs:2:}}
\{{#include file.rs:2:10}}
```

Das erste Kommando wird nur die zweite Zeile aus der Datei `file.rs`
einbinden. Das zweite Kommando wird alle Zeilen bis zur 10ten Zeil des
berücksichtigen. Die Zeilen 11 bis zum Ende der Datei werden
ignoriert.  Die dritte Zeile wird alle Zeilen ab Zeile 2 einbinden,
d.h. nur Zeile 1 wird ausgelassen. Im letzten Kommando wird der
Bereich von Zeile 2 bis 10 aus der Datei `files.rs` eingebunden.

Um zu verhindern, das unbeabsichtigt durch das Einbinden anderer
Dateien das Rendern Deines Buches korrumpiert wird, wurde die Syntax
mit `Ankern` anstelle von Zeilennummern eingeführt. In der zu
importierenden Datei kennzeichnest Du nun Bereiche mit den `Ankern`,
die sich aus einem Marker und einem regulären Ausdruck
zusammensetzt. Sie werden als Paar korrespondierender Markerzeilen
eingetragen. Die Zeile die den Anker startet, verwendet
"ANCHOR:<Marker>" (z.B "ANCHOR:\s*[\w_-]+"). Die Zeile die den Anker
abschließt verwendet "ANCHOR_END:<Marker>" (hier:
"ANCHOR_END:\s*[\w_-]+"). Anker können in jeder zu kommentierenden
Datei eingefügt werden.

Hier ein Beispiel, die mit drei korrespondierende Ankern arbeitet:

```rs
/* ANCHOR: all */

// ANCHOR: component
struct Paddle {
	hello: f32,
}
// ANCHOR_END: component

////////// ANCHOR: system
impl System for MySystem { ... }
////////// ANCHOR_END: system

/* ANCHOR_END: all */
```

In mdbook bindest Du diese Ankerzeilen nun wie folgt ein:

````hbs
Import einer Komponente:
```rust,no_run,noplayground
\{{#include file.rs:component}}
```

Import eines Systems:
```rust,no_run,noplayground
\{{#include file.rs:system}}
```

Import der ganzen Datei:
```rust,no_run,noplayground
\{{#include file.rs:all}}
```
````

> **Anmerkung:** Die Zeilen mit einer Anker-Anweisungen werden beim
Rendern ignoriert.

## Einfügen einer Datei, mit selektiv ausgewählten Zeilen

Für die Einbindung von Rust Quellcode aus externen Dateien hilft die
Anweisung `rustdoc_include`, wenn die anzuzeigenden Inhalte selektiv
auswählbar sein sollen. Die Kennzeichnung der Zeilen oder Anker in
der Quellcode Datei entspricht der Syntax, din in der `include`
Anweisung bereits beschriebenen wurde.

Im Unterschied zu `include` werden Zeilen die **nicht** in den Bereich
der ausgewählten Zeilen fallen dennoch angezeigt, jedoch mit einem
beginnenden `#` annotiert. Das gilt ebenso für Anker-Kennzeichnungen.
Das ermöglicht es dem Anwender, den Auszug (`sippet`) des Beispiels in
der vollständigen Form anzuzeigen. rustdoc wird das vollständige
Beispiel verwenden, wenn du den Renderaufruf mit `mdbook test`
startest.

Betrachten wir das folgende Beispiel, das Rust Quellquode aus der
Datei `file.rs` verwendet:

```rust
fn main() {
	let x = add_one(2);
	assert_eq!(x, 3);
}

fn add_one(num: i32) -> i32 {
	num + 1
}
```

Wenn ein snippet annotiert werden soll, das anfangs nur die Zeile 2 anzeigen
soll, lautet die Syntax:

````hbs
Um die `add_one` Function aufzurufen, übergeben wird ein `i32` und binden den Rückgabewert an `x`:

```rust
\{{#rustdoc_include file.rs:2}}
```
````

Dies hätte den gleichen Effekt, als ob wir die Quellcode Zeilen
manuell eingefügt hätten, um anschießend alle Zeilen bis auf Zeile 2
mit `#` auszukommentieren.

````hbs
Um die `add_one` Function aufzurufen, übergeben wird ein `i32` und binden den Rückgabewert an `x`:

```rust
# fn main() {
	let x = add_one(2);
#     assert_eq!(x, 3);
# }
#
# fn add_one(num: i32) -> i32 {
#     num + 1
#}
```
````

Das vollständige Beispiel mit `rustdoc_include` sieht so aus
(Aktiviere das "Auge" Icon um den Rest der Datei zu sehen):

```rust
# fn main() {
	let x = add_one(2);
#     assert_eq!(x, 3);
# }
#
# fn add_one(num: i32) -> i32 {
#     num + 1
#}
```

## Einfügen von ausführbaren Rust Dateien

Verwendest Du die folgenden Syntax, kannst Du Rust Quellcode Dateien
einbinden, die der Anwender interaktiv ausführen kann:

```hbs
\{{#playground file.rs}}
```

Der Pfad zur Rust Quellcode Datei muss in relativer Form zur aktiven
Markdown Datei angegeben werden. Wenn das `play` Icon angeklickt wird, wird das Code snippet zum
[Rust Playground] gesendet. Es wird dort kompiliert und ausführt. Das
Ergebnis wird wieder an den aufrufenden Prozess zurückgesendet und
unterhalb des Codes ausgegeben.


Im gerenderten mdbook sieht unser Beispiel dann so aus:

{{#playground example.rs}}
[Rust Playground]: https://play.rust-lang.org/
