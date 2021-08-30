# Train report for javascript / file:///tmp/top-repos-quality-repos-66ypy08t/g-desktop-suite.git HEAD e9622884dfa7948cbc6a079c06d73e0b8544cf8c

### Classification report

PPCR: 0.248

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.998| 1.000| 0.311| 0.999| 0.475| 504| 1618| 0.311 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 174| 348| 0.500 |
| `␣` | 1.000| 1.000| 0.169| 1.000| 0.289| 147| 872| 0.169 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 118| 0.008 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 245| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 128| 0.000 |
| `micro avg` | 0.999| 0.999| 0.248| 0.999| 0.397| 826| 3329| 0.248 |
| `weighted avg` | 0.998| 0.999| 0.248| 0.998| 0.376| 826| 3329| 0.248 |
| `macro avg` | 0.500| 0.500| 0.163| 0.500| 0.238| 826| 3329| 0.248 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1114 |504 |0 |0 |0 |0 |0 |
|725 |0 |147 |0 |0 |0 |0 |
|174 |0 |0 |174 |0 |0 |0 |
|245 |0 |0 |0 |0 |0 |0 |
|128 |0 |0 |0 |0 |0 |0 |
|117 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/js/preload-view.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 174}, "macro avg": {"f1-score": 0.4998348199537496, "precision": 0.4996699669966997, "recall": 0.5, "support": 826}, "micro avg": {"f1-score": 0.9987893462469734, "precision": 0.9987893462469734, "recall": 0.9987893462469734, "support": 826}, "weighted avg": {"f1-score": 0.9981846192979887, "precision": 0.9975810898281111, "recall": 0.9987893462469734, "support": 826}, "\u2205": {"f1-score": 0.9990089197224975, "precision": 0.998019801980198, "recall": 1.0, "support": 504}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 147}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 348}, "macro avg": {"f1-score": 0.2383307722180029, "precision": 0.4996699669966997, "recall": 0.1633456092204292, "support": 3329}, "micro avg": {"f1-score": 0.3971119133574007, "precision": 0.9987893462469734, "recall": 0.2478221688194653, "support": 3329}, "weighted avg": {"f1-score": 0.376033020833716, "precision": 0.851545821449072, "recall": 0.2478221688194653, "support": 3329}, "\u2205": {"f1-score": 0.4747998115873763, "precision": 0.998019801980198, "recall": 0.311495673671199, "support": 1618}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u2423": {"f1-score": 0.28851815505397443, "precision": 1.0, "recall": 0.16857798165137614, "support": 872}},
  "ppcr": 0.24812255932712526
}
```
</details>
