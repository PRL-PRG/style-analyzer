# Test report for javascript / file:///tmp/top-repos-quality-repos-moexuz3b/celluloid.git HEAD f54bfe86c60324c7f57667cb494bdd1cbb4e17c8

### Classification report

PPCR: 0.973

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 1.000| 1.000| 0.990| 0.990| 97| 97| 1.000 |
| `␣` | 1.000| 0.906| 0.829| 0.951| 0.906| 32| 35| 0.914 |
| `"` | 0.938| 1.000| 1.000| 0.968| 0.968| 30| 30| 1.000 |
| `⏎` | 0.778| 0.875| 0.778| 0.824| 0.778| 8| 9| 0.889 |
| `⏎␣⁺␣⁺` | 1.000| 0.600| 0.600| 0.750| 0.750| 5| 5| 1.000 |
| `⏎␣⁻␣⁻` | 1.000| 1.000| 1.000| 1.000| 1.000| 5| 5| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `micro avg` | 0.966| 0.966| 0.940| 0.966| 0.953| 177| 182| 0.973 |
| `weighted avg` | 0.968| 0.966| 0.940| 0.965| 0.948| 177| 182| 0.973 |
| `macro avg` | 0.814| 0.769| 0.744| 0.783| 0.770| 177| 182| 0.973 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |97 |0 |0 |0 |0 |0 |0 |
|3 |1 |29 |0 |2 |0 |0 |0 |
|0 |0 |0 |30 |0 |0 |0 |0 |
|1 |1 |0 |0 |7 |0 |0 |0 |
|0 |0 |0 |0 |0 |5 |0 |0 |
|0 |0 |0 |2 |0 |0 |3 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.967741935483871, "precision": 0.9375, "recall": 1.0, "support": 30}, "macro avg": {"f1-score": 0.7831267053924389, "precision": 0.8135822510822511, "recall": 0.7687499999999999, "support": 177}, "micro avg": {"f1-score": 0.9661016949152542, "precision": 0.9661016949152542, "recall": 0.9661016949152542, "support": 177}, "weighted avg": {"f1-score": 0.9650108867144812, "precision": 0.9682916738001485, "recall": 0.9661016949152542, "support": 177}, "\u2205": {"f1-score": 0.989795918367347, "precision": 0.9797979797979798, "recall": 1.0, "support": 97}, "\u23ce": {"f1-score": 0.823529411764706, "precision": 0.7777777777777778, "recall": 0.875, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7499999999999999, "precision": 1.0, "recall": 0.6, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5}, "\u2423": {"f1-score": 0.9508196721311475, "precision": 1.0, "recall": 0.90625, "support": 32}},
  "cl_report_full": {"\"": {"f1-score": 0.967741935483871, "precision": 0.9375, "recall": 1.0, "support": 30}, "macro avg": {"f1-score": 0.7702236616612852, "precision": 0.8135822510822511, "recall": 0.7437641723356008, "support": 182}, "micro avg": {"f1-score": 0.9526462395543176, "precision": 0.9661016949152542, "recall": 0.9395604395604396, "support": 182}, "weighted avg": {"f1-score": 0.9478638030008176, "precision": 0.962447274947275, "recall": 0.9395604395604396, "support": 182}, "\u2205": {"f1-score": 0.989795918367347, "precision": 0.9797979797979798, "recall": 1.0, "support": 97}, "\u23ce": {"f1-score": 0.7777777777777778, "precision": 0.7777777777777778, "recall": 0.7777777777777778, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7499999999999999, "precision": 1.0, "recall": 0.6, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5}, "\u2423": {"f1-score": 0.90625, "precision": 1.0, "recall": 0.8285714285714286, "support": 35}},
  "ppcr": 0.9725274725274725
}
```
</details>
