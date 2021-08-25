# Test report for javascript / file:///tmp/top-repos-quality-repos-7r9y5l73/simulaciones.git HEAD 5e875fc1b6a1ab7f94e7287dff1a3607771a170c

### Classification report

PPCR: 0.785

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.881| 0.978| 0.861| 0.927| 0.871| 1285| 1460| 0.880 |
| `␣` | 0.881| 0.800| 0.670| 0.839| 0.761| 621| 742| 0.837 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 76| 172| 0.442 |
| `⏎` | 0.570| 0.803| 0.276| 0.667| 0.372| 66| 192| 0.344 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 46| 0.587 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 42| 0.214 |
| `weighted avg` | 0.824| 0.867| 0.681| 0.843| 0.719| 2084| 2654| 0.785 |
| `macro avg` | 0.389| 0.430| 0.301| 0.405| 0.334| 2084| 2654| 0.785 |
| `micro avg` | 0.867| 0.867| 0.681| 0.867| 0.763| 2084| 2654| 0.785 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|175 |1257 |27 |1 |0 |0 |0 |
|121 |85 |497 |39 |0 |0 |0 |
|126 |0 |13 |53 |0 |0 |0 |
|96 |54 |22 |0 |0 |0 |0 |
|19 |22 |5 |0 |0 |0 |0 |
|33 |9 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "macro avg": {"f1-score": 0.4054127304183314, "precision": 0.3886611837880969, "recall": 0.43026041349224114, "support": 2084}, "micro avg": {"f1-score": 0.8670825335892515, "precision": 0.8670825335892515, "recall": 0.8670825335892515, "support": 2084}, "weighted avg": {"f1-score": 0.8426535304378638, "precision": 0.8237803430418223, "recall": 0.8670825335892515, "support": 2084}, "\u2205": {"f1-score": 0.9269911504424779, "precision": 0.8808689558514365, "recall": 0.9782101167315175, "support": 1285}, "\u23ce": {"f1-score": 0.6666666666666666, "precision": 0.5698924731182796, "recall": 0.803030303030303, "support": 66}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.838818565400844, "precision": 0.8812056737588653, "recall": 0.8003220611916264, "support": 621}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "macro avg": {"f1-score": 0.3339720944137666, "precision": 0.3886611837880969, "recall": 0.30113531525516213, "support": 2654}, "micro avg": {"f1-score": 0.7627691008864501, "precision": 0.8670825335892515, "recall": 0.6808590806330068, "support": 2654}, "weighted avg": {"f1-score": 0.7187328034294446, "precision": 0.7721713038096779, "recall": 0.6808590806330068, "support": 2654}, "\u2205": {"f1-score": 0.8708001385521301, "precision": 0.8808689558514365, "recall": 0.8609589041095891, "support": 1460}, "\u23ce": {"f1-score": 0.37192982456140355, "precision": 0.5698924731182796, "recall": 0.2760416666666667, "support": 192}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u2423": {"f1-score": 0.7611026033690658, "precision": 0.8812056737588653, "recall": 0.6698113207547169, "support": 742}},
  "ppcr": 0.7852298417483045
}
```
</details>
