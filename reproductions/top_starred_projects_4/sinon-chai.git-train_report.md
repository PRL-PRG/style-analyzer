# Train report for javascript / file:///tmp/top-repos-quality-repos-errj3mnf/sinon-chai.git HEAD 74a1bc0d5b9518341f255bf771c15bddff97664e

### Classification report

PPCR: 0.968

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 1.000| 1.000| 0.991| 0.991| 11162| 11162| 1.000 |
| `␣` | 0.992| 0.924| 0.924| 0.957| 0.957| 2171| 2172| 1.000 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 870| 870| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.981| 0.998| 0.998| 0.990| 0.990| 620| 620| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.972| 0.972| 0.986| 0.986| 616| 616| 1.000 |
| `⏎⏎` | 0.932| 0.910| 0.468| 0.921| 0.623| 211| 410| 0.515 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 348| 0.098 |
| `macro avg` | 0.841| 0.829| 0.766| 0.835| 0.792| 15684| 16198| 0.968 |
| `weighted avg` | 0.983| 0.985| 0.953| 0.983| 0.956| 15684| 16198| 0.968 |
| `micro avg` | 0.985| 0.985| 0.953| 0.985| 0.969| 15684| 16198| 0.968 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |11157 |5 |0 |0 |0 |0 |0 |
|1 |152 |2007 |0 |12 |0 |0 |0 |
|0 |0 |0 |870 |0 |0 |0 |0 |
|0 |1 |0 |0 |619 |0 |0 |0 |
|0 |17 |0 |0 |0 |599 |0 |0 |
|199 |19 |0 |0 |0 |0 |192 |0 |
|314 |9 |11 |0 |0 |0 |14 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/sinon-chai.js | 97 |
| test/messages.js | 40 |
| test/throwing.js | 34 |
| test/callArguments.js | 23 |
| test/callOrder.js | 11 |
| test/callContext.js | 8 |
| test/returning.js | 7 |
| test/common.js | 6 |
| test/callingWithNew.js | 5 |
| test/regressions.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 870}, "macro avg": {"f1-score": 0.8349351410914595, "precision": 0.8410964434316825, "recall": 0.829250446739099, "support": 15684}, "micro avg": {"f1-score": 0.9846977811782709, "precision": 0.9846977811782709, "recall": 0.9846977811782709, "support": 15684}, "weighted avg": {"f1-score": 0.9834502935452956, "precision": 0.9826615810269245, "recall": 0.9846977811782709, "support": 15684}, "\u2205": {"f1-score": 0.9909845894213261, "precision": 0.982562747688243, "recall": 0.9995520516036552, "support": 11162}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u23ce": {"f1-score": 0.9208633093525179, "precision": 0.9320388349514563, "recall": 0.909952606635071, "support": 211}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9896083133493206, "precision": 0.9809825673534073, "recall": 0.9983870967741936, "support": 620}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9860082304526749, "precision": 1.0, "recall": 0.9724025974025974, "support": 616}, "\u2423": {"f1-score": 0.9570815450643777, "precision": 0.9920909540286703, "recall": 0.924458774758176, "support": 2171}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 870}, "macro avg": {"f1-score": 0.7924044505001454, "precision": 0.8410964434316825, "recall": 0.7660953682683639, "support": 16198}, "micro avg": {"f1-score": 0.9688225330907723, "precision": 0.9846977811782709, "recall": 0.9534510433386838, "support": 16198}, "weighted avg": {"f1-score": 0.9560549577114343, "precision": 0.9629912369388598, "recall": 0.9534510433386838, "support": 16198}, "\u2205": {"f1-score": 0.9909845894213261, "precision": 0.982562747688243, "recall": 0.9995520516036552, "support": 11162}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 348}, "\u23ce\u23ce": {"f1-score": 0.6233766233766234, "precision": 0.9320388349514563, "recall": 0.4682926829268293, "support": 410}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9896083133493206, "precision": 0.9809825673534073, "recall": 0.9983870967741936, "support": 620}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9860082304526749, "precision": 1.0, "recall": 0.9724025974025974, "support": 616}, "\u2423": {"f1-score": 0.9568533969010727, "precision": 0.9920909540286703, "recall": 0.9240331491712708, "support": 2172}},
  "ppcr": 0.968267687368811
}
```
</details>
