# Test report for javascript / file:///tmp/top-repos-quality-repos-t71h_dym/objc.git HEAD 14168534948813a7e1b5e656c879b1ed032f3306

### Classification report

PPCR: 0.896

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.846| 0.922| 0.898| 0.882| 0.871| 1305| 1340| 0.974 |
| `␣` | 0.812| 0.714| 0.597| 0.760| 0.688| 611| 730| 0.837 |
| `'` | 0.882| 0.989| 0.989| 0.932| 0.932| 188| 188| 1.000 |
| `⏎␣⁺␣⁺` | 0.914| 0.944| 0.944| 0.929| 0.929| 90| 90| 1.000 |
| `⏎␣⁻␣⁻` | 1.000| 0.667| 0.667| 0.800| 0.800| 90| 90| 1.000 |
| `⏎⏎` | 0.708| 0.436| 0.215| 0.540| 0.330| 39| 79| 0.494 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 103| 0.233 |
| `macro avg` | 0.737| 0.667| 0.616| 0.692| 0.650| 2347| 2620| 0.896 |
| `micro avg` | 0.847| 0.847| 0.758| 0.847| 0.800| 2347| 2620| 0.896 |
| `weighted avg` | 0.838| 0.847| 0.758| 0.838| 0.774| 2347| 2620| 0.896 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|35 |1203 |95 |0 |0 |0 |7 |0 |
|119 |150 |436 |23 |0 |1 |1 |0 |
|0 |0 |2 |186 |0 |0 |0 |0 |
|79 |18 |0 |0 |0 |6 |0 |0 |
|40 |22 |0 |0 |0 |17 |0 |0 |
|0 |1 |2 |2 |0 |0 |85 |0 |
|0 |28 |2 |0 |0 |0 |0 |60 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9323308270676691, "precision": 0.8815165876777251, "recall": 0.9893617021276596, "support": 188}, "macro avg": {"f1-score": 0.6918350322485817, "precision": 0.7373911485901229, "recall": 0.6673990882354787, "support": 2347}, "micro avg": {"f1-score": 0.8466126970600767, "precision": 0.8466126970600767, "recall": 0.8466126970600767, "support": 2347}, "weighted avg": {"f1-score": 0.8382714274154137, "precision": 0.8375411619201543, "recall": 0.8466126970600767, "support": 2347}, "\u2205": {"f1-score": 0.8822882288228823, "precision": 0.8459915611814346, "recall": 0.9218390804597701, "support": 1305}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.5396825396825398, "precision": 0.7083333333333334, "recall": 0.4358974358974359, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9289617486338798, "precision": 0.9139784946236559, "recall": 0.9444444444444444, "support": 90}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 90}, "\u2423": {"f1-score": 0.759581881533101, "precision": 0.8119180633147114, "recall": 0.7135842880523732, "support": 611}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9323308270676691, "precision": 0.8815165876777251, "recall": 0.9893617021276596, "support": 188}, "macro avg": {"f1-score": 0.6501053561100276, "precision": 0.7373911485901229, "recall": 0.6158120220941352, "support": 2620}, "micro avg": {"f1-score": 0.8000805315079526, "precision": 0.8466126970600767, "recall": 0.7583969465648855, "support": 2620}, "weighted avg": {"f1-score": 0.7735351729272779, "precision": 0.8092635093647847, "recall": 0.7583969465648855, "support": 2620}, "\u2205": {"f1-score": 0.8711078928312818, "precision": 0.8459915611814346, "recall": 0.8977611940298508, "support": 1340}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u23ce\u23ce": {"f1-score": 0.3300970873786408, "precision": 0.7083333333333334, "recall": 0.21518987341772153, "support": 79}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9289617486338798, "precision": 0.9139784946236559, "recall": 0.9444444444444444, "support": 90}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 90}, "\u2423": {"f1-score": 0.6882399368587214, "precision": 0.8119180633147114, "recall": 0.5972602739726027, "support": 730}},
  "ppcr": 0.8958015267175573
}
```
</details>