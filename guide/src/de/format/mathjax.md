# MathJax Unterstützung

mdBook verfügt über eine optionale Unterstützung für mathematische Gleichungen durch das Crate
[MathJax](https://www.mathjax.org/).

Um MathJax zu aktivieren musst Du den `mathjax-support` Schlüssel in deiner `book.toml`
in der Rubrik `output.html` ergänzen.

```toml
[output.html]
mathjax-support = true
```

>**Anmerkung:** Der gewöhnlich in MathJax verwendet Trenner wird
> derzeit noch nicht unterstützt. Derzeit musst du die relevanten
> Bereiche mit dem Trennsymbol `$$ ... $$` kennzeichnen. Ein
> zusätzlicher Backslash (`\[ ... \]`) ist als Trennsymbol
> notwendig. Wir hoffen diese Limitierung zeitnah auszuräumen.

>**Anmerkung:** Wenn sie zwei Backslashes in MathJax Blöcken verwenden
> (z.B: in Kommandos wie `\begin{cases} \frac 1 2 \\ \frac 3 4
> \end{cases}`) ist es leider nötig diese um zwei weitere Backslashes
> zu ergänzen: (hier: `\begin{cases} \frac 1 2 \\\\ \frac 3 4
> \end{cases}`).


### Inkludierte Gleichungen

Gleichungen, die in den Text inkludiert werden verwenden `\\(` und
`\\)` als Trennzeichen. Nachfolgendes Beispiel veranschaulicht eine inkludierte Gleichung
\\( \int x dx = \frac{x^2}{2} + C \\):

```
\\( \int x dx = \frac{x^2}{2} + C \\)
```

### GLeichungsblöcke

Ganz Blöcke werden mit den Trennzeichen `\\[` und `\\]` gekennzeichnet. Um die folgende GLeichung

\\[ \mu = \frac{1}{N} \sum_{i=0} x_i \\]

auszudürcken würden sie folgenen Code im Text anfügen:

```bash
\\[ \mu = \frac{1}{N} \sum_{i=0} x_i \\]
```
