# Test report for javascript / file:///tmp/top-repos-quality-repos-d6c3j4rh/sound_of_sundholm.git HEAD bdd0df84422f79d4f010f393fd6ab36f4fe2a233

### Classification report

PPCR: 0.500

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.931| 1.000| 0.791| 0.964| 0.855| 579| 732| 0.791 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 311| 0.055 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 92| 0.109 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 58| 0.155 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 52| 0.135 |
| `weighted avg` | 0.867| 0.931| 0.465| 0.898| 0.503| 622| 1245| 0.500 |
| `micro avg` | 0.931| 0.931| 0.465| 0.931| 0.620| 622| 1245| 0.500 |
| `macro avg` | 0.186| 0.200| 0.158| 0.193| 0.171| 622| 1245| 0.500 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|153 |579 |0 |0 |0 |0 |
|294 |17 |0 |0 |0 |0 |
|82 |10 |0 |0 |0 |0 |
|49 |9 |0 |0 |0 |0 |
|45 |7 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "macro avg": {"f1-score": 0.19283930058284762, "precision": 0.18617363344051446, "recall": 0.2, "support": 622}, "micro avg": {"f1-score": 0.9308681672025724, "precision": 0.9308681672025724, "recall": 0.9308681672025724, "support": 622}, "weighted avg": {"f1-score": 0.8975398314909065, "precision": 0.8665155447110762, "recall": 0.9308681672025724, "support": 622}, "\u2205": {"f1-score": 0.9641965029142381, "precision": 0.9308681672025724, "recall": 1.0, "support": 579}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "macro avg": {"f1-score": 0.17104874446085672, "precision": 0.18617363344051446, "recall": 0.1581967213114754, "support": 1245}, "micro avg": {"f1-score": 0.6202463845741831, "precision": 0.9308681672025724, "recall": 0.4650602409638554, "support": 1245}, "weighted avg": {"f1-score": 0.5028420921499884, "precision": 0.5473056211986209, "recall": 0.4650602409638554, "support": 1245}, "\u2205": {"f1-score": 0.8552437223042836, "precision": 0.9308681672025724, "recall": 0.7909836065573771, "support": 732}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 311}},
  "ppcr": 0.4995983935742972
}
```
</details>
