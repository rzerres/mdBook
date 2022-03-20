# MathJax Unterstützung

mdBook unterstütz optional mathematische Ausdrücke via
[MathJax](https://www.mathjax.org/).

Um MathJax einzubinden musst Du den `mathjax-support` Schlüssel in deiner `book.toml`
Datei im Abschnitt `output.html` aktivieren.

```toml
[output.html]
mathjax-support = true
```

>**Anmerkung:** Die von MathJax üblicherweise verwendeten Trennzeichen
werden derzeit noch nicht unterstützt. Daher musst Du bis auf
Weiteres als Trennung `$$ ... $$` und `\[ ... \]` als Trennung
verwenden, damit zusätzliche backslash'es funktionieren. Wir hoffen,
diese Einschränkung bald in den Quellen überflüssig zu machen.

>**Anmerkung:** Wenn doppelte backslash'es in einem MathJax Block einsetzt wird,
z.B. im Kommando `\begin{cases} \frac 1 2 \\ \frac 3 4 \end{cases}`), musst Du
> für eine korrekte Funktion in mdBook _zwei weitere_ backslashes einfügen (z.B. `\begin{cases} \frac 1 2 \\\\ \frac 3 4
> \end{cases}`).

### Inline Gleichungen
Inline Gleichungen werden mit dem Ausdruck `\\(` und `\\)` getrennt. Um also die nachfolgend Gleichung zu rendern,
\\( \int x dx = \frac{x^2}{2} + C \\) würdest Du folgenden Code eingeben:

```
\\( \int x dx = \frac{x^2}{2} + C \\)
```

### Block Gleichungen
Block Gleichungen werden mit dem Ausdruck `\\[` und `\\]` getrennt. Um die folgende Block Gleichung zu rendern

\\[ \mu = \frac{1}{N} \sum_{i=0} x_i \\]


ist dieser Code einzugeben:

```bash
\\[ \mu = \frac{1}{N} \sum_{i=0} x_i \\]
```
