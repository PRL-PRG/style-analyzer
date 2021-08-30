# Test report for javascript / file:///tmp/top-repos-quality-repos-ymn6j1l9/takana.git HEAD 3e274d5990392590ce44b00f1c0acd680f98cf7b

### Classification report

PPCR: 0.855

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.925| 0.991| 0.964| 0.957| 0.944| 1401| 1441| 0.972 |
| `␣` | 0.965| 0.861| 0.735| 0.910| 0.835| 610| 714| 0.854 |
| `'` | 0.891| 0.930| 0.848| 0.910| 0.869| 299| 328| 0.912 |
| `⏎␣⁻␣⁻` | 0.833| 0.792| 0.792| 0.812| 0.812| 101| 101| 1.000 |
| `⏎⏎` | 0.696| 0.941| 0.640| 0.800| 0.667| 34| 50| 0.680 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 30| 0.867 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 160| 0.150 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 99| 0.051 |
| `weighted avg` | 0.903| 0.922| 0.788| 0.911| 0.806| 2500| 2923| 0.855 |
| `macro avg` | 0.539| 0.564| 0.497| 0.549| 0.516| 2500| 2923| 0.855 |
| `micro avg` | 0.922| 0.922| 0.788| 0.922| 0.850| 2500| 2923| 0.855 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|40 |1389 |6 |0 |0 |0 |6 |0 |0 |
|104 |75 |525 |0 |0 |0 |10 |0 |0 |
|29 |9 |12 |278 |0 |0 |0 |0 |0 |
|136 |6 |0 |4 |0 |0 |0 |14 |0 |
|94 |0 |0 |5 |0 |0 |0 |0 |0 |
|0 |21 |0 |0 |0 |0 |80 |0 |0 |
|16 |1 |1 |0 |0 |0 |0 |32 |0 |
|4 |1 |0 |25 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u0027": {"f1-score": 0.9099836333878887, "precision": 0.8910256410256411, "recall": 0.9297658862876255, "support": 299}, "macro avg": {"f1-score": 0.5486232690956825, "precision": 0.5387314568809123, "recall": 0.5643889990011332, "support": 2500}, "micro avg": {"f1-score": 0.9216, "precision": 0.9216, "recall": 0.9216, "support": 2500}, "weighted avg": {"f1-score": 0.9108064137803722, "precision": 0.9034115581895352, "recall": 0.9216, "support": 2500}, "\u2205": {"f1-score": 0.9569410954185326, "precision": 0.9247669773635153, "recall": 0.9914346895074947, "support": 1401}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.7999999999999999, "precision": 0.6956521739130435, "recall": 0.9411764705882353, "support": 34}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8121827411167514, "precision": 0.8333333333333334, "recall": 0.7920792079207921, "support": 101}, "\u2423": {"f1-score": 0.9098786828422877, "precision": 0.9650735294117647, "recall": 0.860655737704918, "support": 610}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u0027": {"f1-score": 0.86875, "precision": 0.8910256410256411, "recall": 0.8475609756097561, "support": 328}, "macro avg": {"f1-score": 0.515774044478913, "precision": 0.5387314568809123, "recall": 0.4973560312280475, "support": 2923}, "micro avg": {"f1-score": 0.8497141803429836, "precision": 0.9216, "recall": 0.7882312692439275, "support": 2923}, "weighted avg": {"f1-score": 0.8061819110292135, "precision": 0.8323152240846919, "recall": 0.7882312692439275, "support": 2923}, "\u2205": {"f1-score": 0.9439347604485219, "precision": 0.9247669773635153, "recall": 0.963913948646773, "support": 1441}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u23ce\u23ce": {"f1-score": 0.6666666666666666, "precision": 0.6956521739130435, "recall": 0.64, "support": 50}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 99}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8121827411167514, "precision": 0.8333333333333334, "recall": 0.7920792079207921, "support": 101}, "\u2423": {"f1-score": 0.834658187599364, "precision": 0.9650735294117647, "recall": 0.7352941176470589, "support": 714}},
  "ppcr": 0.8552856654122477
}
```
</details>