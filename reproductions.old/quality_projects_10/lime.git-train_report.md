# Train report for javascript / file:///tmp/top-repos-quality-repos-h854ovbm/lime.git HEAD 152bd3df2cf661bd6251e2a3533dbc410ee7d2fd

### Classification report

PPCR: 0.498

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.999| 0.908| 0.993| 0.946| 2766| 3043| 0.909 |
| `␣` | 0.965| 0.852| 0.065| 0.905| 0.122| 128| 1681| 0.076 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 125| 0.072 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 317| 0.009 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 137| 0.015 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 275| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 263| 0.000 |
| `macro avg` | 0.279| 0.264| 0.139| 0.271| 0.153| 2908| 5841| 0.498 |
| `micro avg` | 0.987| 0.987| 0.492| 0.987| 0.656| 2908| 5841| 0.498 |
| `weighted avg` | 0.982| 0.987| 0.492| 0.985| 0.528| 2908| 5841| 0.498 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|277 |2762 |4 |0 |0 |0 |0 |0 |
|1553 |19 |109 |0 |0 |0 |0 |0 |
|275 |0 |0 |0 |0 |0 |0 |0 |
|314 |3 |0 |0 |0 |0 |0 |0 |
|263 |0 |0 |0 |0 |0 |0 |0 |
|135 |2 |0 |0 |0 |0 |0 |0 |
|116 |9 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| templates/bin/node/http-server/bin/http-server | 18 |
| templates/bin/node/watch/cli.js | 6 |
| templates/bin/node/watch/main.js | 5 |
| templates/bin/node/http-server/lib/http-server.js | 3 |
| assets/docs-theme/resources/index.js | 3 |
| templates/bin/node/watch/cli-custom.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.27113011939467074, "precision": 0.2789707102940278, "recall": 0.26430233834314637, "support": 2908}, "micro avg": {"f1-score": 0.9872764786795049, "precision": 0.9872764786795049, "recall": 0.9872764786795049, "support": 2908}, "weighted avg": {"f1-score": 0.9846563644495185, "precision": 0.9823973258894149, "recall": 0.9872764786795049, "support": 2908}, "\u2205": {"f1-score": 0.993346520409998, "precision": 0.9881932021466905, "recall": 0.9985538684020245, "support": 2766}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.904564315352697, "precision": 0.9646017699115044, "recall": 0.8515625, "support": 128}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 275}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 263}, "macro avg": {"f1-score": 0.1525329460000357, "precision": 0.2789707102940278, "recall": 0.13892846760803432, "support": 5841}, "micro avg": {"f1-score": 0.6563035775517201, "precision": 0.9872764786795049, "recall": 0.4915254237288136, "support": 5841}, "weighted avg": {"f1-score": 0.5279231751454121, "precision": 0.792427236663862, "recall": 0.4915254237288136, "support": 5841}, "\u2205": {"f1-score": 0.946214457005824, "precision": 0.9881932021466905, "recall": 0.9076569175156096, "support": 3043}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 317}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 137}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 125}, "\u2423": {"f1-score": 0.12151616499442587, "precision": 0.9646017699115044, "recall": 0.06484235574063058, "support": 1681}},
  "ppcr": 0.4978599554870741
}
```
</details>
