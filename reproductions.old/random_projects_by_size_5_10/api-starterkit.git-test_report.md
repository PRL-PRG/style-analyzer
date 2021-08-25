# Test report for javascript / file:///tmp/top-repos-quality-repos-brm_wqmi/api-starterkit.git HEAD 0547bc907530afc90f960c82c41caecc63d23966

### Classification report

PPCR: 0.608

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.941| 1.000| 0.807| 0.969| 0.869| 222| 275| 0.807 |
| `␣` | 1.000| 0.757| 0.286| 0.862| 0.444| 37| 98| 0.378 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 38| 0.079 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 23| 0.087 |
| `macro avg` | 0.485| 0.439| 0.273| 0.458| 0.328| 264| 434| 0.608 |
| `weighted avg` | 0.931| 0.947| 0.576| 0.936| 0.651| 264| 434| 0.608 |
| `micro avg` | 0.947| 0.947| 0.576| 0.947| 0.716| 264| 434| 0.608 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|53 |222 |0 |0 |0 |
|61 |9 |28 |0 |0 |
|35 |3 |0 |0 |0 |
|21 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "macro avg": {"f1-score": 0.45774269398723544, "precision": 0.48516949152542377, "recall": 0.4391891891891892, "support": 264}, "micro avg": {"f1-score": 0.946969696969697, "precision": 0.946969696969697, "recall": 0.946969696969697, "support": 264}, "weighted avg": {"f1-score": 0.9359503669547337, "precision": 0.9311761684643041, "recall": 0.946969696969697, "support": 264}, "\u2205": {"f1-score": 0.9694323144104803, "precision": 0.940677966101695, "recall": 1.0, "support": 222}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.8615384615384616, "precision": 1.0, "recall": 0.7567567567567568, "support": 37}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "macro avg": {"f1-score": 0.32833224614046536, "precision": 0.48516949152542377, "recall": 0.27324675324675324, "support": 434}, "micro avg": {"f1-score": 0.7163323782234957, "precision": 0.946969696969697, "recall": 0.576036866359447, "support": 434}, "weighted avg": {"f1-score": 0.6509189034282148, "precision": 0.8218581582441615, "recall": 0.576036866359447, "support": 434}, "\u2205": {"f1-score": 0.8688845401174169, "precision": 0.940677966101695, "recall": 0.8072727272727273, "support": 275}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.4444444444444445, "precision": 1.0, "recall": 0.2857142857142857, "support": 98}},
  "ppcr": 0.6082949308755761
}
```
</details>
