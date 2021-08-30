# Test report for javascript / file:///tmp/top-repos-quality-repos-66ypy08t/g-desktop-suite.git HEAD e9622884dfa7948cbc6a079c06d73e0b8544cf8c

### Classification report

PPCR: 0.632

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.920| 0.972| 0.866| 0.945| 0.892| 247| 277| 0.892 |
| `␣` | 0.833| 0.735| 0.333| 0.781| 0.476| 68| 150| 0.453 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 22| 44| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 32| 0.188 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 20| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 20| 0.000 |
| `micro avg` | 0.910| 0.910| 0.575| 0.910| 0.704| 343| 543| 0.632 |
| `weighted avg` | 0.892| 0.910| 0.575| 0.899| 0.641| 343| 543| 0.632 |
| `macro avg` | 0.459| 0.451| 0.283| 0.454| 0.339| 343| 543| 0.632 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|30 |240 |7 |0 |0 |0 |0 |
|82 |18 |50 |0 |0 |0 |0 |
|22 |0 |0 |22 |0 |0 |0 |
|26 |3 |3 |0 |0 |0 |0 |
|20 |0 |0 |0 |0 |0 |0 |
|20 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 22}, "macro avg": {"f1-score": 0.4543553149606299, "precision": 0.45881226053639845, "recall": 0.4511590061125665, "support": 343}, "micro avg": {"f1-score": 0.9096209912536443, "precision": 0.9096209912536443, "recall": 0.9096209912536443, "support": 343}, "weighted avg": {"f1-score": 0.8994484745529256, "precision": 0.8915250829395798, "recall": 0.9096209912536443, "support": 343}, "\u2205": {"f1-score": 0.9448818897637794, "precision": 0.9195402298850575, "recall": 0.97165991902834, "support": 247}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.78125, "precision": 0.8333333333333334, "recall": 0.7352941176470589, "support": 68}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 44}, "macro avg": {"f1-score": 0.33917507523455476, "precision": 0.45881226053639845, "recall": 0.2832932210188528, "support": 543}, "micro avg": {"f1-score": 0.7042889390519188, "precision": 0.9096209912536443, "recall": 0.574585635359116, "support": 543}, "weighted avg": {"f1-score": 0.6406988052123502, "precision": 0.7803179441586757, "recall": 0.574585635359116, "support": 543}, "\u2205": {"f1-score": 0.8921933085501857, "precision": 0.9195402298850575, "recall": 0.8664259927797834, "support": 277}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u2423": {"f1-score": 0.47619047619047616, "precision": 0.8333333333333334, "recall": 0.3333333333333333, "support": 150}},
  "ppcr": 0.6316758747697975
}
```
</details>
