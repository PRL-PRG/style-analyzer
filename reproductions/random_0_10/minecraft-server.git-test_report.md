# Test report for javascript / file:///tmp/top-repos-quality-repos-b943es_f/minecraft-server.git HEAD 9c9e2abbe25a4642cb509a8eb80dabb41b5f6968

### Classification report

PPCR: 0.935

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.911| 0.907| 0.944| 0.942| 8112| 8144| 0.996 |
| `␣` | 0.770| 0.959| 0.936| 0.854| 0.845| 2773| 2840| 0.976 |
| `'` | 0.753| 0.949| 0.946| 0.840| 0.839| 534| 536| 0.996 |
| `⏎⇥⁺` | 0.775| 0.729| 0.695| 0.751| 0.733| 269| 282| 0.954 |
| `⏎⇥⁻` | 0.754| 0.904| 0.774| 0.822| 0.764| 197| 230| 0.857 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 193| 200| 0.965 |
| `⏎` | 0.930| 0.333| 0.074| 0.491| 0.136| 159| 721| 0.221 |
| `⏎⏎` | 0.312| 0.741| 0.127| 0.440| 0.180| 27| 158| 0.171 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 11| 1.000 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 2| 1.000 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5| 0.000 |
| `macro avg` | 0.479| 0.502| 0.405| 0.467| 0.404| 12277| 13129| 0.935 |
| `weighted avg` | 0.896| 0.896| 0.838| 0.890| 0.840| 12277| 13129| 0.935 |
| `micro avg` | 0.896| 0.896| 0.838| 0.896| 0.866| 12277| 13129| 0.935 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⏎| ⏎⇥⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|32 |7389 |705 |0 |0 |0 |2 |16 |0 |0 |0 |0 |
|67 |57 |2658 |0 |0 |31 |3 |24 |0 |0 |0 |0 |
|2 |21 |6 |507 |0 |0 |0 |0 |0 |0 |0 |0 |
|562 |33 |7 |1 |53 |10 |39 |16 |0 |0 |0 |0 |
|13 |16 |57 |0 |0 |196 |0 |0 |0 |0 |0 |0 |
|131 |1 |2 |0 |4 |0 |20 |0 |0 |0 |0 |0 |
|33 |15 |4 |0 |0 |0 |0 |178 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |11 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |2 |0 |0 |0 |0 |
|7 |9 |14 |165 |0 |5 |0 |0 |0 |0 |0 |0 |
|5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u0027": {"f1-score": 0.8400994200497099, "precision": 0.7533432392273403, "recall": 0.949438202247191, "support": 534}, "macro avg": {"f1-score": 0.46740620624593576, "precision": 0.47947432632097403, "recall": 0.5022810510459945, "support": 12277}, "micro avg": {"f1-score": 0.8960658141239717, "precision": 0.8960658141239717, "recall": 0.8960658141239717, "support": 12277}, "weighted avg": {"f1-score": 0.8901783625975729, "precision": 0.8958699244330126, "recall": 0.8960658141239717, "support": 12277}, "\u2205": {"f1-score": 0.9441001724908963, "precision": 0.9798435220792998, "recall": 0.9108727810650887, "support": 8112}, "\u23ce": {"f1-score": 0.49074074074074076, "precision": 0.9298245614035088, "recall": 0.3333333333333333, "support": 159}, "\u23ce\u21e5\u207a": {"f1-score": 0.7509578544061302, "precision": 0.7747035573122529, "recall": 0.7286245353159851, "support": 269}, "\u23ce\u21e5\u207b": {"f1-score": 0.8221709006928407, "precision": 0.7542372881355932, "recall": 0.9035532994923858, "support": 197}, "\u23ce\u23ce": {"f1-score": 0.43956043956043955, "precision": 0.3125, "recall": 0.7407407407407407, "support": 27}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.8538387407645358, "precision": 0.7697654213727194, "recall": 0.9585286693112153, "support": 2773}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 200}, "\u0027": {"f1-score": 0.838709677419355, "precision": 0.7533432392273403, "recall": 0.9458955223880597, "support": 536}, "macro avg": {"f1-score": 0.4035197824497773, "precision": 0.47947432632097403, "recall": 0.4052858660652401, "support": 13129}, "micro avg": {"f1-score": 0.8660159017554908, "precision": 0.8960658141239717, "recall": 0.8379160636758322, "support": 13129}, "weighted avg": {"f1-score": 0.8401807766129729, "precision": 0.8897471936128697, "recall": 0.8379160636758322, "support": 13129}, "\u2205": {"f1-score": 0.9421740516416959, "precision": 0.9798435220792998, "recall": 0.9072937131630648, "support": 8144}, "\u23ce": {"f1-score": 0.13624678663239073, "precision": 0.9298245614035088, "recall": 0.07350901525658807, "support": 721}, "\u23ce\u21e5\u207a": {"f1-score": 0.7327102803738318, "precision": 0.7747035573122529, "recall": 0.6950354609929078, "support": 282}, "\u23ce\u21e5\u207b": {"f1-score": 0.7639484978540771, "precision": 0.7542372881355932, "recall": 0.7739130434782608, "support": 230}, "\u23ce\u23ce": {"f1-score": 0.1801801801801802, "precision": 0.3125, "recall": 0.12658227848101267, "support": 158}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.8447481328460195, "precision": 0.7697654213727194, "recall": 0.9359154929577465, "support": 2840}},
  "ppcr": 0.9351054916596847
}
```
</details>