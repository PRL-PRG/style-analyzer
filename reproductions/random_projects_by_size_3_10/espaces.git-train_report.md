# Train report for javascript / file:///tmp/top-repos-quality-repos-jcwyl908/espaces.git HEAD 3f73fd3a10e27d6aac7307f120e0616f02b84639

### Classification report

PPCR: 0.654

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 0.994| 0.902| 0.989| 0.941| 7746| 8540| 0.907 |
| `␣` | 0.956| 0.954| 0.376| 0.955| 0.540| 1355| 3437| 0.394 |
| `⏎` | 0.949| 0.902| 0.219| 0.925| 0.356| 184| 758| 0.243 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 189| 0.148 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 190| 0.121 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 146| 0.055 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 134| 0.052 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 280| 0.018 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 383| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 248| 0.000 |
| `weighted avg` | 0.971| 0.979| 0.640| 0.975| 0.710| 9356| 14305| 0.654 |
| `micro avg` | 0.979| 0.979| 0.640| 0.979| 0.774| 9356| 14305| 0.654 |
| `macro avg` | 0.289| 0.285| 0.150| 0.287| 0.184| 9356| 14305| 0.654 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|794 |7699 |47 |0 |0 |0 |0 |0 |0 |0 |0 |
|2082 |62 |1293 |0 |0 |0 |0 |0 |0 |0 |0 |
|574 |17 |1 |166 |0 |0 |0 |0 |0 |0 |0 |
|383 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|275 |0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |
|248 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|167 |16 |5 |2 |0 |0 |0 |0 |0 |0 |0 |
|161 |27 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|138 |1 |7 |0 |0 |0 |0 |0 |0 |0 |0 |
|127 |6 |0 |1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| installation/binaural_espaces/src/jquery.knob.js | 110 |
| installation/binaural_espaces/src/binaural-fir.js | 34 |
| installation/binaural_espaces/src/auralizr.js | 17 |
| installation/ihp/ihp_sergio/website/server/static/lib/auralizr.js | 15 |
| installation/binaural_espaces/main_cart.js | 8 |
| installation/binaural_espaces/src/kdt.js | 5 |
| installation/ecm_berlin/server/osc_server/index.js | 4 |
| installation/binaural_espaces/src/binaural-net.js | 3 |
| installation/binaural_espaces/main_BinauralNet.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2868438501588164, "precision": 0.2887746225508231, "recall": 0.28503498076606737, "support": 9356}, "micro avg": {"f1-score": 0.9788371098760152, "precision": 0.9788371098760154, "recall": 0.9788371098760154, "support": 9356}, "weighted avg": {"f1-score": 0.9750512873578122, "precision": 0.9713338770923422, "recall": 0.9788371098760154, "support": 9356}, "\u2205": {"f1-score": 0.9886991139077951, "precision": 0.9835206949412366, "recall": 0.9939323521817712, "support": 7746}, "\u23ce": {"f1-score": 0.924791086350975, "precision": 0.9485714285714286, "recall": 0.9021739130434783, "support": 184}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.9549483013293943, "precision": 0.9556541019955654, "recall": 0.9542435424354243, "support": 1355}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 248}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 383}, "macro avg": {"f1-score": 0.1836454136373653, "precision": 0.2887746225508231, "recall": 0.1496719784291979, "support": 14305}, "micro avg": {"f1-score": 0.7741008410464477, "precision": 0.9788371098760154, "recall": 0.6401957357567284, "support": 14305}, "weighted avg": {"f1-score": 0.7101838500553292, "precision": 0.8670302010635486, "recall": 0.6401957357567284, "support": 14305}, "\u2205": {"f1-score": 0.9407380254154448, "precision": 0.9835206949412366, "recall": 0.9015222482435598, "support": 8540}, "\u23ce": {"f1-score": 0.35584137191854237, "precision": 0.9485714285714286, "recall": 0.21899736147757257, "support": 758}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 280}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 134}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 189}, "\u2423": {"f1-score": 0.539874739039666, "precision": 0.9556541019955654, "recall": 0.37620017457084665, "support": 3437}},
  "ppcr": 0.6540370499825235
}
```
</details>
