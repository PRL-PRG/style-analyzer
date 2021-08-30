# Test report for javascript / file:///tmp/top-repos-quality-repos-otox_8nd/hydra.git HEAD f8181b840b1ba664f40f5b38ee9c8dd1dce7f776

### Classification report

PPCR: 0.786

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.982| 0.939| 0.978| 0.956| 5780| 6043| 0.956 |
| `␣` | 0.948| 0.953| 0.787| 0.950| 0.860| 2153| 2606| 0.826 |
| `⏎` | 0.802| 0.715| 0.280| 0.756| 0.416| 193| 492| 0.392 |
| `⏎⏎` | 0.921| 0.927| 0.473| 0.924| 0.625| 151| 296| 0.510 |
| `"` | 0.820| 0.936| 0.171| 0.874| 0.282| 78| 428| 0.182 |
| `'` | 0.880| 0.611| 0.122| 0.721| 0.215| 72| 360| 0.200 |
| `⏎␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 181| 0.133 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 98| 0.031 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 96| 0.021 |
| `⏎␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 161| 0.000 |
| `macro avg` | 0.534| 0.512| 0.277| 0.520| 0.335| 8456| 10761| 0.786 |
| `micro avg` | 0.960| 0.960| 0.755| 0.960| 0.845| 8456| 10761| 0.786 |
| `weighted avg` | 0.957| 0.960| 0.755| 0.958| 0.800| 8456| 10761| 0.786 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|263 |5675 |92 |0 |13 |0 |0 |0 |0 |0 |0 |
|453 |90 |2051 |0 |12 |0 |0 |0 |0 |0 |0 |
|350 |1 |0 |73 |0 |4 |0 |0 |0 |0 |0 |
|299 |33 |10 |0 |138 |0 |12 |0 |0 |0 |0 |
|288 |4 |8 |16 |0 |44 |0 |0 |0 |0 |0 |
|145 |5 |2 |0 |4 |0 |140 |0 |0 |0 |0 |
|95 |0 |1 |0 |2 |0 |0 |0 |0 |0 |0 |
|157 |19 |0 |0 |3 |2 |0 |0 |0 |0 |0 |
|94 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|161 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.874251497005988, "precision": 0.8202247191011236, "recall": 0.9358974358974359, "support": 78}, "\u0027": {"f1-score": 0.7213114754098361, "precision": 0.88, "recall": 0.6111111111111112, "support": 72}, "macro avg": {"f1-score": 0.5203706384701375, "precision": 0.534496519146494, "recall": 0.5123644926898896, "support": 8456}, "micro avg": {"f1-score": 0.9603831598864712, "precision": 0.9603831598864712, "recall": 0.9603831598864712, "support": 8456}, "weighted avg": {"f1-score": 0.9581864051165182, "precision": 0.9566147438035273, "recall": 0.9603831598864712, "support": 8456}, "\u2205": {"f1-score": 0.9776897234903954, "precision": 0.9735803739921084, "recall": 0.9818339100346021, "support": 5780}, "\u23ce": {"f1-score": 0.7561643835616437, "precision": 0.8023255813953488, "recall": 0.7150259067357513, "support": 193}, "\u23ce\u23ce": {"f1-score": 0.9240924092409241, "precision": 0.9210526315789473, "recall": 0.9271523178807947, "support": 151}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9501968959925873, "precision": 0.9477818853974121, "recall": 0.9526242452392011, "support": 2153}},
  "cl_report_full": {"\"": {"f1-score": 0.28239845261121854, "precision": 0.8202247191011236, "recall": 0.1705607476635514, "support": 428}, "\u0027": {"f1-score": 0.21463414634146344, "precision": 0.88, "recall": 0.12222222222222222, "support": 360}, "macro avg": {"f1-score": 0.33536843181385, "precision": 0.534496519146494, "recall": 0.2772376773154913, "support": 10761}, "micro avg": {"f1-score": 0.8451891554352917, "precision": 0.9603831598864712, "recall": 0.7546696403679956, "support": 10761}, "weighted avg": {"f1-score": 0.7997385842357703, "precision": 0.9003343312098439, "recall": 0.7546696403679956, "support": 10761}, "\u2205": {"f1-score": 0.9560309973045821, "precision": 0.9735803739921084, "recall": 0.939103094489492, "support": 6043}, "\u23ce": {"f1-score": 0.4156626506024097, "precision": 0.8023255813953488, "recall": 0.2804878048780488, "support": 492}, "\u23ce\u23ce": {"f1-score": 0.6249999999999999, "precision": 0.9210526315789473, "recall": 0.47297297297297297, "support": 296}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 181}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 161}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u2423": {"f1-score": 0.859958071278826, "precision": 0.9477818853974121, "recall": 0.7870299309286263, "support": 2606}},
  "ppcr": 0.7858005761546325
}
```
</details>