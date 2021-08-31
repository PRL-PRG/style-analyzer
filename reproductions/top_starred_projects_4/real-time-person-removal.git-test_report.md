# Test report for javascript / file:///tmp/top-repos-quality-repos-20nfz900/real-time-person-removal.git HEAD bf5d194cf2579fd62e7ceb5dd7b5548d1f64cbf0

### Classification report

PPCR: 0.408

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.959| 0.471| 0.966| 0.635| 221| 450| 0.491 |
| `␣` | 0.940| 0.959| 0.385| 0.949| 0.547| 147| 366| 0.402 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 87| 0.000 |
| `macro avg` | 0.637| 0.639| 0.285| 0.638| 0.394| 368| 903| 0.408 |
| `weighted avg` | 0.960| 0.959| 0.391| 0.959| 0.538| 368| 903| 0.408 |
| `micro avg` | 0.959| 0.959| 0.391| 0.959| 0.555| 368| 903| 0.408 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|229 |212 |9 |0 |
|219 |6 |141 |0 |
|87 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.6384421281915588, "precision": 0.6374923547400612, "recall": 0.6394865638563118, "support": 368}, "micro avg": {"f1-score": 0.9592391304347826, "precision": 0.9592391304347826, "recall": 0.9592391304347826, "support": 368}, "weighted avg": {"f1-score": 0.9593057193705902, "precision": 0.9595038891104907, "recall": 0.9592391304347826, "support": 368}, "\u2205": {"f1-score": 0.9658314350797267, "precision": 0.9724770642201835, "recall": 0.9592760180995475, "support": 221}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9494949494949495, "precision": 0.94, "recall": 0.9591836734693877, "support": 147}},
  "cl_report_full": {"macro avg": {"f1-score": 0.3937473889430441, "precision": 0.6374923547400612, "recall": 0.28545233758348515, "support": 903}, "micro avg": {"f1-score": 0.5554681353265145, "precision": 0.9592391304347826, "recall": 0.3909191583610188, "support": 903}, "weighted avg": {"f1-score": 0.5378205961560615, "precision": 0.8656197994452741, "recall": 0.3909191583610188, "support": 903}, "\u2205": {"f1-score": 0.6347305389221557, "precision": 0.9724770642201835, "recall": 0.4711111111111111, "support": 450}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}, "\u2423": {"f1-score": 0.5465116279069767, "precision": 0.94, "recall": 0.38524590163934425, "support": 366}},
  "ppcr": 0.40753045404208194
}
```
</details>
