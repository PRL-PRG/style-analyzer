# Test report for javascript / file:///tmp/top-repos-quality-repos-06dic4vx/hogwarts-london-web-career-042219.git HEAD f9d7c49cd83a18be963c26bfdbfefaa82010d1ce

### Classification report

PPCR: 0.765

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.898| 1.000| 0.992| 0.946| 0.943| 388| 391| 0.992 |
| `⏎` | 0.972| 1.000| 0.575| 0.986| 0.723| 103| 179| 0.575 |
| `␣` | 0.909| 0.674| 0.488| 0.774| 0.635| 89| 123| 0.724 |
| `'` | 1.000| 0.978| 0.489| 0.989| 0.657| 46| 92| 0.500 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 32| 0.594 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 31| 0.129 |
| `micro avg` | 0.918| 0.918| 0.703| 0.918| 0.796| 649| 848| 0.765 |
| `weighted avg` | 0.887| 0.918| 0.703| 0.898| 0.751| 649| 848| 0.765 |
| `macro avg` | 0.630| 0.609| 0.424| 0.616| 0.493| 649| 848| 0.765 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|3 |388 |0 |0 |0 |0 |0 |
|34 |29 |60 |0 |0 |0 |0 |
|76 |0 |0 |103 |0 |0 |0 |
|46 |1 |0 |0 |45 |0 |0 |
|13 |13 |6 |0 |0 |0 |0 |
|27 |1 |0 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.989010989010989, "precision": 1.0, "recall": 0.9782608695652174, "support": 46}, "macro avg": {"f1-score": 0.6158653223045124, "precision": 0.6298228617411007, "recall": 0.6087363621560006, "support": 649}, "micro avg": {"f1-score": 0.9183359013867488, "precision": 0.9183359013867488, "recall": 0.9183359013867488, "support": 649}, "weighted avg": {"f1-score": 0.8984587830605757, "precision": 0.8867110601709549, "recall": 0.9183359013867488, "support": 649}, "\u2205": {"f1-score": 0.9463414634146341, "precision": 0.8981481481481481, "recall": 1.0, "support": 388}, "\u23ce": {"f1-score": 0.985645933014354, "precision": 0.9716981132075472, "recall": 1.0, "support": 103}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.7741935483870969, "precision": 0.9090909090909091, "recall": 0.6741573033707865, "support": 89}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6569343065693432, "precision": 1.0, "recall": 0.4891304347826087, "support": 92}, "macro avg": {"f1-score": 0.492925636347681, "precision": 0.6298228617411007, "recall": 0.4241136121622829, "support": 848}, "micro avg": {"f1-score": 0.7962591850367402, "precision": 0.9183359013867488, "recall": 0.7028301886792453, "support": 848}, "weighted avg": {"f1-score": 0.7506914709073084, "precision": 0.8595849882172862, "recall": 0.7028301886792453, "support": 848}, "\u2205": {"f1-score": 0.9428918590522479, "precision": 0.8981481481481481, "recall": 0.9923273657289002, "support": 391}, "\u23ce": {"f1-score": 0.7228070175438597, "precision": 0.9716981132075472, "recall": 0.5754189944134078, "support": 179}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u2423": {"f1-score": 0.6349206349206349, "precision": 0.9090909090909091, "recall": 0.4878048780487805, "support": 123}},
  "ppcr": 0.7653301886792453
}
```
</details>
