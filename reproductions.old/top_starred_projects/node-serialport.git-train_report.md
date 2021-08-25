# Train report for javascript / file:///tmp/top-repos-quality-repos-0taj7mmw/node-serialport.git HEAD 863c8c038992b80530877d92cb832df84a9a646b

### Classification report

PPCR: 0.838

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.970| 0.914| 0.973| 0.944| 17052| 18094| 0.942 |
| `␣` | 0.913| 0.978| 0.897| 0.944| 0.905| 6987| 7624| 0.916 |
| `'` | 1.000| 1.000| 0.975| 1.000| 0.987| 2693| 2763| 0.975 |
| `⏎␣⁺␣⁺` | 0.954| 0.920| 0.812| 0.937| 0.877| 1166| 1322| 0.882 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 243| 1885| 0.129 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 60| 1261| 0.048 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 745| 0.043 |
| `weighted avg` | 0.951| 0.961| 0.805| 0.956| 0.827| 28233| 33694| 0.838 |
| `micro avg` | 0.961| 0.961| 0.805| 0.961| 0.876| 28233| 33694| 0.838 |
| `macro avg` | 0.549| 0.553| 0.514| 0.551| 0.530| 28233| 33694| 0.838 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1042 |16532 |474 |0 |0 |46 |0 |0 |
|637 |145 |6836 |0 |0 |6 |0 |0 |
|70 |0 |0 |2693 |0 |0 |0 |0 |
|1642 |134 |109 |0 |0 |0 |0 |0 |
|156 |66 |27 |0 |0 |1073 |0 |0 |
|1201 |47 |13 |0 |0 |0 |0 |0 |
|713 |0 |32 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/website/pages/en/index.js | 285 |
| packages/website/core/Footer.js | 115 |
| packages/website/pages/en/help.js | 89 |
| packages/website/pages/en/users.js | 69 |
| packages/serialport/test-manual/many-writes.js | 54 |
| packages/stream/stream.js | 52 |
| packages/bindings/lib/bindings-test.js | 51 |
| packages/website/languages.js | 38 |
| packages/binding-mock/binding-mock.js | 35 |
| packages/terminal/lib/terminal.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2693}, "macro avg": {"f1-score": 0.5505993872817173, "precision": 0.549025306510268, "recall": 0.5525905166120251, "support": 28233}, "micro avg": {"f1-score": 0.9610739205893812, "precision": 0.9610739205893812, "recall": 0.9610739205893812, "support": 28233}, "weighted avg": {"f1-score": 0.9555312443823379, "precision": 0.9505971376690047, "recall": 0.9610739205893812, "support": 28233}, "\u2205": {"f1-score": 0.9731575229573818, "precision": 0.9768376270385252, "recall": 0.969505043396669, "support": 17052}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 243}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9367088607594937, "precision": 0.9537777777777777, "recall": 0.9202401372212693, "support": 1166}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "\u2423": {"f1-score": 0.9443293272551458, "precision": 0.9125617407555734, "recall": 0.9783884356662373, "support": 6987}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9871700879765396, "precision": 1.0, "recall": 0.9746652189648932, "support": 2763}, "macro avg": {"f1-score": 0.5304134815640479, "precision": 0.549025306510268, "recall": 0.5138042084249527, "support": 33694}, "micro avg": {"f1-score": 0.8763221212072279, "precision": 0.9610739205893812, "recall": 0.8053065827743812, "support": 33694}, "weighted avg": {"f1-score": 0.8270743495749583, "precision": 0.8504827256300168, "recall": 0.8053065827743812, "support": 33694}, "\u2205": {"f1-score": 0.9442001256496658, "precision": 0.9768376270385252, "recall": 0.9136730407870012, "support": 18094}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1885}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 745}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8769922353902738, "precision": 0.9537777777777777, "recall": 0.8116490166414524, "support": 1322}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1261}, "\u2423": {"f1-score": 0.9045319219318557, "precision": 0.9125617407555734, "recall": 0.8966421825813221, "support": 7624}},
  "ppcr": 0.8379236659345878
}
```
</details>
