# Test report for javascript / file:///tmp/top-repos-quality-repos-a4ywgfc9/npm-check-updates.git HEAD cf0a52ca7d569422072954903081f6f8b4f8a39c

### Classification report

PPCR: 0.948

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.984| 0.976| 0.986| 0.982| 3242| 3268| 0.992 |
| `␣` | 0.861| 0.980| 0.931| 0.917| 0.895| 1506| 1585| 0.950 |
| `'` | 0.989| 0.988| 0.977| 0.988| 0.983| 1126| 1138| 0.989 |
| `⏎␣⁻␣⁻` | 0.959| 0.949| 0.869| 0.954| 0.912| 273| 298| 0.916 |
| `⏎␣⁺␣⁺` | 0.723| 0.504| 0.434| 0.594| 0.542| 254| 295| 0.861 |
| `⏎` | 0.916| 0.432| 0.229| 0.587| 0.366| 176| 332| 0.530 |
| `⏎⏎` | 0.712| 0.795| 0.645| 0.751| 0.677| 112| 138| 0.812 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 33| 1.000 |
| `weighted avg` | 0.937| 0.942| 0.893| 0.936| 0.902| 6722| 7087| 0.948 |
| `micro avg` | 0.942| 0.942| 0.893| 0.942| 0.917| 6722| 7087| 0.948 |
| `macro avg` | 0.769| 0.704| 0.633| 0.722| 0.670| 6722| 7087| 0.948 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|26 |3191 |48 |0 |0 |0 |3 |0 |0 |
|79 |4 |1476 |0 |1 |16 |8 |1 |0 |
|12 |8 |6 |1112 |0 |0 |0 |0 |0 |
|156 |6 |59 |0 |76 |0 |0 |35 |0 |
|41 |10 |104 |12 |0 |128 |0 |0 |0 |
|25 |6 |7 |0 |1 |0 |259 |0 |0 |
|26 |4 |14 |0 |5 |0 |0 |89 |0 |
|0 |0 |0 |0 |0 |33 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9884444444444446, "precision": 0.9893238434163701, "recall": 0.9875666074600356, "support": 1126}, "macro avg": {"f1-score": 0.7221644251874677, "precision": 0.7685980962090553, "recall": 0.7038789067574607, "support": 6722}, "micro avg": {"f1-score": 0.9418327878607557, "precision": 0.9418327878607557, "recall": 0.9418327878607557, "support": 6722}, "weighted avg": {"f1-score": 0.9356979103845716, "precision": 0.9373950946553115, "recall": 0.9418327878607557, "support": 6722}, "\u2205": {"f1-score": 0.986246329779014, "precision": 0.9882316506658408, "recall": 0.9842689697717458, "support": 3242}, "\u23ce": {"f1-score": 0.5868725868725869, "precision": 0.9156626506024096, "recall": 0.4318181818181818, "support": 176}, "\u23ce\u23ce": {"f1-score": 0.7510548523206751, "precision": 0.712, "recall": 0.7946428571428571, "support": 112}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5939675174013921, "precision": 0.7231638418079096, "recall": 0.5039370078740157, "support": 254}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9539594843462247, "precision": 0.9592592592592593, "recall": 0.9487179487179487, "support": 273}, "\u2423": {"f1-score": 0.9167701863354039, "precision": 0.8611435239206534, "recall": 0.9800796812749004, "support": 1506}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9832007073386384, "precision": 0.9893238434163701, "recall": 0.9771528998242531, "support": 1138}, "macro avg": {"f1-score": 0.6697165871903267, "precision": 0.7685980962090553, "recall": 0.6327112991220216, "support": 7087}, "micro avg": {"f1-score": 0.9169382286914332, "precision": 0.9418327878607557, "recall": 0.89332580781713, "support": 7087}, "weighted avg": {"f1-score": 0.9022278001141562, "precision": 0.9343520031311734, "recall": 0.89332580781713, "support": 7087}, "\u2205": {"f1-score": 0.9822995228567031, "precision": 0.9882316506658408, "recall": 0.976438188494492, "support": 3268}, "\u23ce": {"f1-score": 0.3662650602409638, "precision": 0.9156626506024096, "recall": 0.2289156626506024, "support": 332}, "\u23ce\u23ce": {"f1-score": 0.6768060836501901, "precision": 0.712, "recall": 0.644927536231884, "support": 138}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5423728813559322, "precision": 0.7231638418079096, "recall": 0.43389830508474575, "support": 295}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9119718309859155, "precision": 0.9592592592592593, "recall": 0.8691275167785235, "support": 298}, "\u2423": {"f1-score": 0.8948166110942709, "precision": 0.8611435239206534, "recall": 0.931230283911672, "support": 1585}},
  "ppcr": 0.9484972484831381
}
```
</details>
