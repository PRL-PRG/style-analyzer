# Train report for javascript / file:///tmp/top-repos-quality-repos-qmg5n5d_/ssbc.git HEAD 09a64ec201bf974add80db5408237d57fffa9135

### Classification report

PPCR: 0.501

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.996| 0.757| 0.986| 0.853| 2244| 2950| 0.761 |
| `␣` | 0.960| 0.887| 0.175| 0.922| 0.296| 274| 1387| 0.198 |
| `'` | 1.000| 1.000| 0.413| 1.000| 0.585| 169| 409| 0.413 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 162| 0.130 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 330| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 167| 0.000 |
| `weighted avg` | 0.969| 0.977| 0.490| 0.973| 0.586| 2708| 5405| 0.501 |
| `macro avg` | 0.490| 0.480| 0.224| 0.485| 0.289| 2708| 5405| 0.501 |
| `micro avg` | 0.977| 0.977| 0.490| 0.977| 0.652| 2708| 5405| 0.501 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|706 |2234 |10 |0 |0 |0 |0 |
|1113 |31 |243 |0 |0 |0 |0 |
|240 |0 |0 |169 |0 |0 |0 |
|330 |0 |0 |0 |0 |0 |0 |
|167 |0 |0 |0 |0 |0 |0 |
|141 |21 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| web/plugins/filters.js | 13 |
| spider/apis.js | 12 |
| spider/spider.js | 11 |
| spider/metadata.js | 7 |
| spider/ecosystem.config.js | 4 |
| spider/partialDump.js | 4 |
| spider/dump.js | 3 |
| spider/task.js | 3 |
| web/ecosystem.config.js | 2 |
| spider/reduce.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 169}, "macro avg": {"f1-score": 0.48475243405059804, "precision": 0.4896211919491618, "recall": 0.48040083098047887, "support": 2708}, "micro avg": {"f1-score": 0.9771048744460856, "precision": 0.9771048744460856, "recall": 0.9771048744460856, "support": 2708}, "weighted avg": {"f1-score": 0.9730319531658057, "precision": 0.9693963593248656, "recall": 0.9771048744460856, "support": 2708}, "\u2205": {"f1-score": 0.9863134657836644, "precision": 0.9772528433945756, "recall": 0.9955436720142602, "support": 2244}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.922201138519924, "precision": 0.9604743083003953, "recall": 0.8868613138686131, "support": 274}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5847750865051904, "precision": 1.0, "recall": 0.4132029339853301, "support": 409}, "macro avg": {"f1-score": 0.28907328289343653, "precision": 0.4896211919491618, "recall": 0.224281556537545, "support": 5405}, "micro avg": {"f1-score": 0.6522864538395168, "precision": 0.9771048744460856, "recall": 0.48954671600370026, "support": 5405}, "weighted avg": {"f1-score": 0.5860318048264689, "precision": 0.8555178082565488, "recall": 0.48954671600370026, "support": 5405}, "\u2205": {"f1-score": 0.8533231474407945, "precision": 0.9772528433945756, "recall": 0.7572881355932204, "support": 2950}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 330}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 167}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 162}, "\u2423": {"f1-score": 0.29634146341463413, "precision": 0.9604743083003953, "recall": 0.17519826964671953, "support": 1387}},
  "ppcr": 0.5010175763182239
}
```
</details>
