# Test report for javascript / file:///tmp/top-repos-quality-repos-qjprgsnd/reactor.js.git HEAD 8b27230d591d77df490bac2bb9762b2201315f9a

### Classification report

PPCR: 0.611

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.998| 1.000| 0.819| 0.999| 0.900| 2274| 2777| 0.819 |
| `␣` | 0.989| 0.997| 0.483| 0.993| 0.649| 620| 1279| 0.485 |
| `⏎` | 1.000| 0.500| 0.007| 0.667| 0.014| 6| 431| 0.014 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 135| 0.044 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 135| 0.000 |
| `weighted avg` | 0.994| 0.996| 0.609| 0.995| 0.701| 2906| 4757| 0.611 |
| `micro avg` | 0.996| 0.996| 0.609| 0.996| 0.756| 2906| 4757| 0.611 |
| `macro avg` | 0.597| 0.499| 0.262| 0.532| 0.313| 2906| 4757| 0.611 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|503 |2274 |0 |0 |0 |0 |
|659 |2 |618 |0 |0 |0 |
|425 |0 |3 |3 |0 |0 |
|135 |0 |0 |0 |0 |0 |
|129 |2 |4 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.5317118032763744, "precision": 0.5974088147497805, "recall": 0.49935483870967745, "support": 2906}, "micro avg": {"f1-score": 0.996214728148658, "precision": 0.996214728148658, "recall": 0.996214728148658, "support": 2906}, "weighted avg": {"f1-score": 0.9950171471983484, "precision": 0.9941717218530642, "recall": 0.996214728148658, "support": 2906}, "\u2205": {"f1-score": 0.9991212653778558, "precision": 0.9982440737489026, "recall": 1.0, "support": 2274}, "\u23ce": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.9927710843373493, "precision": 0.9888, "recall": 0.9967741935483871, "support": 620}},
  "cl_report_full": {"macro avg": {"f1-score": 0.31253756255062565, "precision": 0.5974088147497805, "recall": 0.2618039664850582, "support": 4757}, "micro avg": {"f1-score": 0.7555787550567662, "precision": 0.996214728148658, "recall": 0.6085768341391633, "support": 4757}, "weighted avg": {"f1-score": 0.7010110783732072, "precision": 0.939205169813055, "recall": 0.6085768341391633, "support": 4757}, "\u2205": {"f1-score": 0.8997032640949554, "precision": 0.9982440737489026, "recall": 0.8188692833993518, "support": 2777}, "\u23ce": {"f1-score": 0.013824884792626725, "precision": 1.0, "recall": 0.0069605568445475635, "support": 431}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "\u2423": {"f1-score": 0.6491596638655461, "precision": 0.9888, "recall": 0.4831899921813917, "support": 1279}},
  "ppcr": 0.6108892158923691
}
```
</details>
