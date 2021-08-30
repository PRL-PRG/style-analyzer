# Train report for javascript / file:///tmp/top-repos-quality-repos-thr8jp55/opensource.builders.git HEAD 28b6a2c6d42127b16e790913df0140981ecf3c64

### Classification report

PPCR: 0.163

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.858| 1.000| 0.923| 693| 808| 0.858 |
| `∅` | 1.000| 1.000| 0.057| 1.000| 0.108| 183| 3212| 0.057 |
| `␣` | 1.000| 1.000| 0.115| 1.000| 0.206| 152| 1321| 0.115 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 571| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 200| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 185| 0.000 |
| `micro avg` | 1.000| 1.000| 0.163| 1.000| 0.281| 1028| 6297| 0.163 |
| `weighted avg` | 1.000| 1.000| 0.163| 1.000| 0.217| 1028| 6297| 0.163 |
| `macro avg` | 0.500| 0.500| 0.172| 0.500| 0.206| 1028| 6297| 0.163 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|3029 |183 |0 |0 |0 |0 |0 |
|1169 |0 |152 |0 |0 |0 |0 |
|115 |0 |0 |693 |0 |0 |0 |
|571 |0 |0 |0 |0 |0 |0 |
|200 |0 |0 |0 |0 |0 |0 |
|185 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 693}, "macro avg": {"f1-score": 0.5, "precision": 0.5, "recall": 0.5, "support": 1028}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1028}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1028}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 183}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 152}},
  "cl_report_full": {"\"": {"f1-score": 0.9233844103930713, "precision": 1.0, "recall": 0.8576732673267327, "support": 808}, "macro avg": {"f1-score": 0.20626192352370606, "precision": 0.5, "recall": 0.1716185767649178, "support": 6297}, "micro avg": {"f1-score": 0.2806825938566553, "precision": 1.0, "recall": 0.16325234238526282, "support": 6297}, "weighted avg": {"f1-score": 0.21676928477581414, "precision": 0.8481816738129268, "recall": 0.16325234238526282, "support": 6297}, "\u2205": {"f1-score": 0.10780559646539029, "precision": 1.0, "recall": 0.05697384806973848, "support": 3212}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 571}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 185}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 200}, "\u2423": {"f1-score": 0.20638153428377462, "precision": 1.0, "recall": 0.11506434519303559, "support": 1321}},
  "ppcr": 0.16325234238526282
}
```
</details>
