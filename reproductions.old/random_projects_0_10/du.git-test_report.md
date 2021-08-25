# Test report for javascript / file:///tmp/top-repos-quality-repos-kqsipvr1/du.git HEAD af7702b6e889717a6c334a7cad4aa3d098f73f2e

### Classification report

PPCR: 0.822

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.877| 1.000| 0.927| 0.934| 0.901| 356| 384| 0.927 |
| `␣` | 1.000| 0.494| 0.260| 0.662| 0.413| 89| 169| 0.527 |
| `⏎` | 0.895| 0.756| 0.694| 0.819| 0.782| 45| 49| 0.918 |
| `'` | 0.933| 1.000| 1.000| 0.966| 0.966| 28| 28| 1.000 |
| `micro avg` | 0.892| 0.892| 0.733| 0.892| 0.805| 518| 630| 0.822 |
| `weighted avg` | 0.903| 0.892| 0.733| 0.879| 0.764| 518| 630| 0.822 |
| `macro avg` | 0.926| 0.812| 0.720| 0.845| 0.765| 518| 630| 0.822 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|28 |356 |0 |0 |0 |
|80 |39 |44 |2 |4 |
|0 |0 |0 |28 |0 |
|4 |11 |0 |0 |34 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9655172413793104, "precision": 0.9333333333333333, "recall": 1.0, "support": 28}, "macro avg": {"f1-score": 0.8452079218127823, "precision": 0.9262293665197477, "recall": 0.8124843945068664, "support": 518}, "micro avg": {"f1-score": 0.8918918918918919, "precision": 0.8918918918918919, "recall": 0.8918918918918919, "support": 518}, "weighted avg": {"f1-score": 0.8792077039203825, "precision": 0.9026141442008696, "recall": 0.8918918918918919, "support": 518}, "\u2205": {"f1-score": 0.9343832020997376, "precision": 0.8768472906403941, "recall": 1.0, "support": 356}, "\u23ce": {"f1-score": 0.8192771084337349, "precision": 0.8947368421052632, "recall": 0.7555555555555555, "support": 45}, "\u2423": {"f1-score": 0.6616541353383459, "precision": 1.0, "recall": 0.4943820224719101, "support": 89}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9655172413793104, "precision": 0.9333333333333333, "recall": 1.0, "support": 28}, "macro avg": {"f1-score": 0.7653844498681307, "precision": 0.9262293665197477, "recall": 0.7203289784848851, "support": 630}, "micro avg": {"f1-score": 0.8048780487804879, "precision": 0.8918918918918919, "recall": 0.7333333333333333, "support": 630}, "weighted avg": {"f1-score": 0.7638746119473523, "precision": 0.9137853939720676, "recall": 0.7333333333333333, "support": 630}, "\u2205": {"f1-score": 0.9012658227848102, "precision": 0.8768472906403941, "recall": 0.9270833333333334, "support": 384}, "\u23ce": {"f1-score": 0.7816091954022989, "precision": 0.8947368421052632, "recall": 0.6938775510204082, "support": 49}, "\u2423": {"f1-score": 0.4131455399061033, "precision": 1.0, "recall": 0.2603550295857988, "support": 169}},
  "ppcr": 0.8222222222222222
}
```
</details>
