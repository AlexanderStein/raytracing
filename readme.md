# Benchmarking multithreading
Command for running benchmark test:
```
hyperfine --setup 'cargo build --release' --max-runs 3 --export-json time.json --parameter-list threads 1,2,4,8,16 'cargo run --release -- --threads={threads}'
```

## Benchmark identification
Hyperfine does not (yet?) support adding some kind of identification to distinguish multiple runes.

For that reason this can (optionally) be inserted manually into the JSON file.

At top-level insert the following snippet, while replacing `<identification>`, e.g. using a git commit SHA1.

```JSON
  "env": {
    "id": "<identification>"
  },
```

# Plotting results

## Installation

`Bokeh` is required and can be installed using `pip`

```
pip install -r requirements.txt
```

## Execution
`./plot.py <JSON file>`
