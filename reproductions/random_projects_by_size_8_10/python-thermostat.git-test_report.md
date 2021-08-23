# Test report for javascript / file:///tmp/top-repos-quality-repos-bsd4vjjv/python-thermostat.git HEAD 8136fd5274bb770f164a1b03f7a49c2eb70f919e

### Classification report

PPCR: 0.798

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.842| 0.941| 0.916| 0.889| 0.878| 12267| 12600| 0.974 |
| `␣` | 0.824| 0.629| 0.446| 0.713| 0.579| 5864| 8260| 0.710 |
| `⏎` | 0.895| 0.851| 0.518| 0.872| 0.656| 798| 1310| 0.609 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1560| 0.001 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.512| 0.484| 0.376| 0.495| 0.423| 18930| 23730| 0.798 |
| `micro avg` | 0.840| 0.840| 0.670| 0.840| 0.746| 18930| 23730| 0.798 |
| `weighted avg` | 0.839| 0.840| 0.670| 0.834| 0.704| 18930| 23730| 0.798 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|333 |11538 |728 |0 |1 |
|2396 |2099 |3686 |0 |79 |
|1559 |1 |0 |0 |0 |
|512 |57 |62 |0 |679 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.49479740440120584, "precision": 0.5121197090069526, "recall": 0.4840061266601978, "support": 18930}, "micro avg": {"f1-score": 0.840095087163233, "precision": 0.840095087163233, "recall": 0.840095087163233, "support": 18930}, "weighted avg": {"f1-score": 0.8336065250863729, "precision": 0.838765218152439, "recall": 0.840095087163233, "support": 18930}, "\u2205": {"f1-score": 0.8888375317772129, "precision": 0.8424972617743702, "recall": 0.9405722670579604, "support": 12267}, "\u23ce": {"f1-score": 0.8721901091843288, "precision": 0.8945981554677207, "recall": 0.8508771929824561, "support": 798}, "\u2423": {"f1-score": 0.7129593810444873, "precision": 0.823503127792672, "recall": 0.628581173260573, "support": 5864}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1560}, "macro avg": {"f1-score": 0.4225537349828966, "precision": 0.5121197090069526, "recall": 0.3760563739533852, "support": 23730}, "micro avg": {"f1-score": 0.7455696202531645, "precision": 0.840095087163233, "recall": 0.6701643489254109, "support": 23730}, "weighted avg": {"f1-score": 0.7036873048689536, "precision": 0.7833765241292563, "recall": 0.6701643489254109, "support": 23730}, "\u2205": {"f1-score": 0.8775812892184826, "precision": 0.8424972617743702, "recall": 0.9157142857142857, "support": 12600}, "\u23ce": {"f1-score": 0.6563557274045433, "precision": 0.8945981554677207, "recall": 0.518320610687023, "support": 1310}, "\u2423": {"f1-score": 0.5788316582914572, "precision": 0.823503127792672, "recall": 0.4462469733656174, "support": 8260}},
  "ppcr": 0.797724399494311
}
```
</details>
