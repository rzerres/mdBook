# `mdbook` in Continuous Integration einbinden

Auch wenn die folgenden Beispiele die Travis CI verwenden, die
beschiebenen Prinzipen sollten leicht auf die Lösungen anderer
Anbieter übertragen werden können.

## Erstellung- und Testläufe für dein Buch sicherstellen

Nachfolgend findest Du beispielhaft eine einfache Travis CI Konfiguration `.travis.yml`.
Diese stellt sicher, dass `mdbook
build` und `mdbook test` erfolgreich ausgeführt werden können.

Der Schlüssel zu schnellen CI Ablaufzeiten ist die Caching-Installation von `mdbook`!
Dies verhindert, dass ein `mdbook` binary bei jedem CI Lauf erneut erstellt werden muss.

```yaml
language: rust
sudo: false

cache:
  - cargo

rust:
  - stable

before_script:
  - (test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update)
  - (test -x $HOME/.cargo/bin/mdbook || cargo install --vers "^0.3" mdbook)
  - cargo install-update -a

script:
  - mdbook build path/to/mybook && mdbook test path/to/mybook
```

## Bereitstellung Deines Buchs über GitHub Pages

Mit den folgenden Anweisungen kannst Du erreichen, dass dein Buch über
GitHub pages veröffentlicht wird, wenn es mit einem CI Lauf erfolgreich
erstellt werden konnte. Die Veröffentlichung erfolg ind einer `master` branch.

Zunächst musst Du dazu einen neuen GitHub "Persönlichen
Zugangs-Schlüssel" erzeugen, der die Berechtigung für dein
"public_repro" besitzt (verwende "repro" für en privates
Repository). Gehe anschließend zu deinen Repository Einstellungen für
Travis CI. Füge eine Umgebungsvariable mit dem Namen `GITHUB_TOKEN` ein,
der mit der tag `secure` markiert ist und *nicht* in den Log Dateien angefügt wird.

Dann füge das nachfolgende `snippet` in Deine `.travis.yml` Datei an
und aktualisieren den Pfad zu deinem Buch Verzeichnis:

```yaml
deploy:
  provider: pages
  skip-cleanup: true
  github-token: $GITHUB_TOKEN
  local-dir: path/to/mybook/book
  keep-history: false
  on:
	branch: master
```

Das wars!

Anmerkung: Travis verfügt über eine neues Konfigurationsformat
[dplv2](https://blog.travis-ci.com/2019-08-27-deployment-tooling-dpl-v2-preview-release),
das sich derzeit noch im Beta-Stadium befindet. Um dieses Format zu
nutzen aktualisiere die `.travis.yml` Datei bitte wie folgt:

```yaml
language: rust
os: linux
dist: xenial

cache:
  - cargo

rust:
  - stable

before_script:
  - (test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update)
  - (test -x $HOME/.cargo/bin/mdbook || cargo install --vers "^0.3" mdbook)
  - cargo install-update -a

script:
  - mdbook build && mdbook test # In case of custom book path: mdbook build path/to/mybook && mdbook test path/to/mybook

deploy:
  provider: pages
  strategy: git
  edge: true
  cleanup: false
  github-token: $GITHUB_TOKEN
  local-dir: book # In case of custom book path: path/to/mybook/book
  keep-history: false
  on:
	branch: main
  target_branch: gh-pages
```

### Manuelle Bereitstellung der GitHub Pages

Wenn Dein CI keine GitHub Pages unterstützt, oder Du die
Bereitstellung auf einer anderen Plattform mit GitHub Pages Unterstützung vornehmen willst:

 *Anmerkung: Du solltest wahrscheinlich anderes temporäres Verzeichnis verwenden*:


```console
$> git worktree add /tmp/book gh-pages
$> mdbook build
$> rm -rf /tmp/book/* # this won't delete the .git directory
$> cp -rp book/* /tmp/book/
$> cd /tmp/book
$> git add -A
$> git commit 'new book message'
$> git push origin gh-pages
$> cd -
```

Oder erstelle ein Makefile Datei mit folgenden Regeln:

```makefile
.PHONY: deploy
deploy: book
	@echo "====> deploying to github"
	git worktree add /tmp/book gh-pages
	rm -rf /tmp/book/*
	cp -rp book/* /tmp/book/
	cd /tmp/book && \
		git add -A && \
		git commit -m "deployed on $(shell date) by ${USER}" && \
		git push origin gh-pages
```

## Bereitstellung Deines Buchs über GitLab Pages

Erstelle bitte im Wurzelverzeichnis Deines repository's (`project root`), die Datei `.gitlab-ci.yml`.
Deren Inhalt sollte so aussehen:

```yml
stages:
	- deploy

pages:
  stage: deploy
  image: rust
  variables:
	CARGO_HOME: $CI_PROJECT_DIR/cargo
  before_script:
	- export PATH="$PATH:$CARGO_HOME/bin"
	- mdbook --version || cargo install mdbook
  script:
		- mdbook build -d public
  only:
	  - master
  artifacts:
	  paths:
		  - public
  cache:
	paths:
	- $CARGO_HOME/bin
```

Nach einem commit und upload dieser Datei via push this, wird die
GitLab CI aktiv werden, Dein Buch ist verfügbar!
