# mdBook-spezifische Funktionen

## Code Zeilen unterdrücken

Diese mdBook Funktion ermöglicht die Unterdrückung von Code Zeilen
durch voranstellen von `#` [wie auch in Rustdoc üblich][rustdoc-hide].
Das funktioniert derzeit nur mit Blöcken, die Rust Quellcode enthalten.

[rustdoc-hide]: https://doc.rust-lang.org/stable/rustdoc/documentation-tests.html#hiding-portions-of-the-example
```bash
# fn main() {
	let x = 5;
	let y = 6;

	println!("{}", x + y);
# }
```

Würde zu folgendem gerenderten Ausdruck führen.

```rust
# fn main() {
	let x = 5;
	let y = 6;

	println!("{}", x + y);
# }
```

Der Code Block wird mit einem Icon annotiert (<i class="fa fa-eye"></i>),
das die Sichtbarkeit von verborgenen Zeilen umschalten kann.

## Rust Playground

Blöcke mit Rust Code erhalten automatische eine Annotierung mit einem `play` Button
(<i class="fa fa-play"></i>). Wird er aktiviert erfolgt die Ausführung des enthaltenen
Quellcodes und die Ergebnis-Ausgabe unterhalb des Quellcode Blocks.
Dies funktioniert durch Übergabe des Quellcodes an den [Rust Playground].

```rust
println!("Hello, World!");
```

Existiert in diesem Code-Block keine `main` Funktion, wird der
Quellcode automatisch in eine solche eingebettet.

Möchtest die die Anzeige des play Button unterdrücken, kannst Du die `noplayground` Option wie folgt einbinden:

~~~markdown
```rust,noplayground
let mut name = String::new();
std::io::stdin().read_line(&mut name).expect("failed to read line");
println!("Hallo {}!", name);
```
~~~

## Rust Code Block Attribute

Zusätzliche Attribute können mit einem Komma, mit Leerstellen oder
Tab-Zeichen getrennt direkt hinter dem Sprach Attribut ergänzt werden.
Hier ein Beispiel:

~~~markdown
```rust,ignore
# Dieses Beispiel würde nicht getestet.
panic!("oops!");
```
~~~

Dies ist äusserst wichtig, wenn [`mdbook test`] verwendet wird, um
eingebundenen Rust Beispielcode zu testen.  Es werden die gleichen
Attribute wie bei [rustdoc Attribute] verwendet. Zusätzlich gelten
folgende Ergänzungen:

* `editable` — Schaltet den [editor] ein.
* `noplayground` — Deaktiviert den play Button, wird aber dennoch getestet.
* `mdbook-runnable` — Erzwingt die Anzeige des play Buttons.  Es ist
  erwünscht, dies mit dem `ignore` Attribut zu kombinieren, wenn
  Beispiele nicht getestet werden sollen, dem Anwender aber offen
  stehen soll, diesen Quellcode interaktive auszuführen.
* `ignore` — Es wird nicht getestet und der play Button wird nicht
  angezeigt. Dennoch wird die Rust Syntax hervorgehoben.
* `should_panic` — When Quellcode ausgeführt wird, soll panic ausgelöst werden.
* `no_run` — Der Quellcode wird kompiliert und getestet, aber nicht ausgeführt.
  Zusätzlich wird der play Button ausgeblendet.
* `compile_fail` — Das Kompilieren des Quellcodes soll scheitern.
* `edition2015`, `edition2018`, `edition2021` — Erzwingt die Nutzung der angegebenen Rust Edition.
  Um dies global anzugeben vergleiche  [`rust.edition`].

[`mdbook test`]: ../cli/test.md
[rustdoc Attribute]: https://doc.rust-lang.org/rustdoc/documentation-tests.html#attributes
[editor]: theme/editor.md
[`rust.edition`]: configuration/general.md#rust-options

## Einfügen von Dateien

Mit der folgenden Syntax können Dateien mit Rust Quellcode im Buch eingefügt werden:

```hbs
\{{#include file.rs}}
```

Der Pfad zur Datei wird relativ zur aktiven Quelldatei ausgewertet.

mdBook wird inkludierte Dateien als Markdown Dateien
interpretieren. Da ein include Kommando oft verwendet wird um
Quellcode Artifakte und Beispiele einzubinden, wird es oft wie folgt
realisiert:

````hbs
```
\{{#include file.rs}}
```
````

Diese Syntax wird verwendet, um den Inhalt der Datei anzuzeigen, die
Interpretation der Anweisungen aber zu verhindern.

## Einbindung von Teilen innerhalb einer Datei

Oftmals wird nur ein bestimmter Teil einer Datei benötigt (z.B. nur ein relevanter Zeilenblock).
mdBook unterstützt hierzu vier verschiedene Modi für deren Einbindung:

```hbs
\{{#include file.rs:2}}
\{{#include file.rs::10}}
\{{#include file.rs:2:}}
\{{#include file.rs:2:10}}
```

Das erste Kommando bindet nur die zweite Zeile aus der Datei
`files.rs` ein. Das zweite Kommando bindet alle Zeilen von Beginn der
Datei bis Zeile 10 ein. Code von Zeile 11 an bis zum Dateiende wird
ignoriert. Das dritte Kommando bindet Code ab Zeile 2 bis zum
Dateiende ein (die erste Zeile wird also ausgelassen). Im letzten
Kommando wird nur der Auszug von Zeile 2 bis 10 aus der Datei
`file.rs` eingebunden.

Damit nicht die Änderungen innerhalb der eingebundenen Dateien zu
Abbrüchen bei der Erstellung Deines Buchs führt, kannst Du auch mit
sogenannten `Anchors` arbeiten. Dann werden nicht explizite Zeilen der
Quellcode-Datei eingebunden, sondern vielmehr entsprechend der
gewählten Markierungen ausgewählt. Ein `Anchor` ist ein
gekennzeichneter Code-Block bestehend aus zwei zusammengehörigen
Anchor Anweisungen. Der Start-Block des Anchors ist mit dem regulären
Ausdruck `ANCHOR:\s*[\w_-]+` gekennzeichnet. Der Anchor endet mit dem
regulären Ausdruck `ANCHOR_END:\s*[\w_-]+`. Diese Syntax erlaubt es
Dir, auch mehrere Anchor-Paare in den zu kommentierten Quellcode
einzufügen.

Betrachten wir den folgenden Quellcode einer einzubindenden Datei:

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

Anschließend kannst Du diese Anchor wie folgt im Buch aufrufen:

````hbs
Here is a component:
```rust,no_run,noplayground
\{{#include file.rs:component}}
```

So wird das Anchor Paar system angesprochen:

```rust,no_run,noplayground
\{{#include file.rs:system}}
```

Zum Abschluss die gesammte Datei.

```rust,no_run,noplayground
\{{#include file.rs:all}}
```
````

Zeilen die andere Anchor Anweisungen innerhalb des gewählten Anchor
Paares enthalten werden ignoriert.

## Einbinden einer Datei mit Ausblenden aller ausser der der spezifizierten Zeilen

Mit `rustdoc_include` kann Quellcode von externen Rust Dateien im Buch
eingebunden werden. Dies kann sich auf Beispiel Quellcode beziehen,
für den zunächst nur ausgewählte Zeilen oder Anchor Blöcke
angezeigt werden sollen (vgl. `include`).

Zeilen die sich nicht mit den ausgewählten Zeilennummern oder Anchor
Blöcken übereinstimmen werden dennoch eingebunden, jedoch mit wird bei
dieser ein `#` Zeichen vorangestellt. So kann der Leser den
Code-Schnipsel expandieren, um den gesammten Quellcode-Block
anzusehen. Rustdoc wird bei Aufruf von `mdbook test` das vollständige
Beispiel berücksichtigen.

Hierzu betrachten wir das folgende Rust Beispiel, das in der Datei `file.rs`
codiert ist:

```rust
fn main() {
	let x = add_one(2);
	assert_eq!(x, 3);
}

fn add_one(num: i32) -> i32 {
	num + 1
}
```

Wir können diesen Schnipsel mit nur 2 anzuzeigenden Quellcodzeilen einbinden:

````hbs
To call the `add_one` function, we pass it an `i32` and bind the returned value to `x`:

```rust
\{{#rustdoc_include file.rs:2}}
```
````

Dies hat den gleichen Effekt, als hätten wir den Quellcode manuell
hinein kopiert und alle ausser Zeile 2 mit einem `#` Zeichen
auskommentiert:

````hbs
To call the `add_one` function, we pass it an `i32` and bind the returned value to `x`:

```rust
# fn main() {
	let x = add_one(2);
#     assert_eq!(x, 3);
# }
#
# fn add_one(num: i32) -> i32 {
#     num + 1
# }
```
````

Im gerenderten Buch sieht dies dann wie folgt aus. Aktiviere das
"expand" Icon um den Rest des Quellcodes anzuzeigen.

```rust
# fn main() {
	let x = add_one(2);
#     assert_eq!(x, 3);
# }
#
# fn add_one(num: i32) -> i32 {
#     num + 1
# }
```

## Einfügen eine Datei mit ausführbarem Rust Code

Die folgende Syntax ermöglicht die Einbindung einer Datei mit
ausführbaren Rust Code in Deinem Buch:

```hbs
\{{#playground file.rs}}
```
Der Pfad Angabe zur Rust Datei wird in relativer Form zur aktiven Quelldatei angegeben.

Wenn der play Butten aktiviert wird, wird die angesprochenen Quellcode
zum [Rust Playground] übertragen, dort kompiliert und ausgeführt. Das
Ergebnis wird zurückgeschickt und unterhalb der Code Zeilen ausgegeben.

Ein gerendertes Code Schnipsel sieht dann so aus:

{{#playground example.rs}}


Alle zusätzlichen Werte die nach dem Dateinamen angegeben werden, werden
als Attribute an den Code Block angefügt. Z.B. erzeugt

`\{{#playground example.rs editable}}`

den folgenden Code Block:

~~~markdown
```rust,editable
# Contents of example.rs here.
```
~~~

Das Attribut `editable` aktiviert den [editor] wie im Link [Rust
code block Attribute](#rust-code-block-attribute) beschrieben.

[Rust Playground]: https://play.rust-lang.org/

## Steuerungsseite \<title\>

Ein Kapitel kann ein Marke \<title\> setzen, die sich vom Eintrag im
Inhaltsverzeichnis (sidebar) unterscheidet. Hierzu verwendest Du die folgende Syntax:

```hbs
\{{#title My Title}}
```
