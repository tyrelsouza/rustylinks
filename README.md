# RustyLinks

Create a LinkTree page for free from a yaml template.

## Demo

See my implementation at [https://tyrel.dev/links/](https://tyrel.dev/links/).


## Acknowledgements

 - 100% inspired by [Ho0ber's Links](https://github.com/ho0ber/links) while helping him make it in Python.
 - My "moroccan-flower-dark.png" file is from [SubtlePatterns](https://www.toptal.com/designers/subtlepatterns/moroccan-flower-dark-pattern/)


## Installation

Install my-project with cargo

```bash
  cargo install
```
    
## Run Locally

Clone the project

```bash
  git clone https://gitea.tyrel.dev/tyrel/rustylinks
```

Go to the project directory

```bash
  cd rustylinks
```

Install dependencies

```bash
  cargo build
```

Run
```
  cargo run
```

Optionally install to get `rustylinks` on path.
```
   cargo install --path ./
   rustylinks
```


## Roadmap

- Add command line options for directories
- Don't hardcode `links.yaml`
- Add tests
- Add Github Action support
- Add SSH copying support
