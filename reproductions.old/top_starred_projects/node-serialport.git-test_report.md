# Test report for javascript / file:///tmp/top-repos-quality-repos-0taj7mmw/node-serialport.git HEAD 863c8c038992b80530877d92cb832df84a9a646b

### Classification report

PPCR: 0.888

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.961| 0.888| 0.972| 0.933| 2254| 2437| 0.925 |
| `␣` | 0.907| 0.973| 0.864| 0.939| 0.885| 1031| 1161| 0.888 |
| `'` | 1.000| 0.979| 0.944| 0.989| 0.971| 328| 340| 0.965 |
| `⏎` | 0.748| 0.823| 0.585| 0.784| 0.656| 209| 294| 0.711 |
| `⏎␣⁻␣⁻` | 0.840| 0.937| 0.937| 0.886| 0.886| 174| 174| 1.000 |
| `⏎␣⁺␣⁺` | 0.965| 0.939| 0.799| 0.952| 0.874| 148| 174| 0.851 |
| `⏎⏎` | 1.000| 0.100| 0.039| 0.182| 0.075| 60| 154| 0.390 |
| `weighted avg` | 0.948| 0.944| 0.838| 0.940| 0.875| 4204| 4734| 0.888 |
| `micro avg` | 0.944| 0.944| 0.838| 0.944| 0.888| 4204| 4734| 0.888 |
| `macro avg` | 0.920| 0.816| 0.722| 0.815| 0.754| 4204| 4734| 0.888 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|183 |2165 |63 |0 |0 |0 |26 |0 |
|130 |12 |1003 |0 |6 |5 |5 |0 |
|12 |0 |0 |321 |7 |0 |0 |0 |
|85 |8 |29 |0 |172 |0 |0 |0 |
|26 |9 |0 |0 |0 |139 |0 |0 |
|0 |9 |2 |0 |0 |0 |163 |0 |
|94 |0 |9 |0 |45 |0 |0 |6 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9892141756548537, "precision": 1.0, "recall": 0.9786585365853658, "support": 328}, "macro avg": {"f1-score": 0.8146800591319195, "precision": 0.9204189220108395, "recall": 0.8158503405503972, "support": 4204}, "micro avg": {"f1-score": 0.9441008563273073, "precision": 0.9441008563273073, "recall": 0.9441008563273073, "support": 4204}, "weighted avg": {"f1-score": 0.9400002705855953, "precision": 0.947539857131631, "recall": 0.9441008563273073, "support": 4204}, "\u2205": {"f1-score": 0.9715054969710568, "precision": 0.9827507943713119, "recall": 0.9605146406388643, "support": 2254}, "\u23ce": {"f1-score": 0.7835990888382688, "precision": 0.7478260869565218, "recall": 0.8229665071770335, "support": 209}, "\u23ce\u23ce": {"f1-score": 0.18181818181818182, "precision": 1.0, "recall": 0.1, "support": 60}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.952054794520548, "precision": 0.9652777777777778, "recall": 0.9391891891891891, "support": 148}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8858695652173914, "precision": 0.8402061855670103, "recall": 0.9367816091954023, "support": 174}, "\u2423": {"f1-score": 0.9386991109031352, "precision": 0.906871609403255, "recall": 0.9728419010669254, "support": 1031}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9712556732223903, "precision": 1.0, "recall": 0.9441176470588235, "support": 340}, "macro avg": {"f1-score": 0.7544124502551627, "precision": 0.9204189220108395, "recall": 0.7222918095847659, "support": 4734}, "micro avg": {"f1-score": 0.8881181472365182, "precision": 0.9441008563273073, "recall": 0.8384030418250951, "support": 4734}, "weighted avg": {"f1-score": 0.8750646779712087, "precision": 0.9454703640868772, "recall": 0.8384030418250951, "support": 4734}, "\u2205": {"f1-score": 0.9331896551724137, "precision": 0.9827507943713119, "recall": 0.8883873615100534, "support": 2437}, "\u23ce": {"f1-score": 0.6564885496183207, "precision": 0.7478260869565218, "recall": 0.5850340136054422, "support": 294}, "\u23ce\u23ce": {"f1-score": 0.07500000000000001, "precision": 1.0, "recall": 0.03896103896103896, "support": 154}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8742138364779873, "precision": 0.9652777777777778, "recall": 0.7988505747126436, "support": 174}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8858695652173914, "precision": 0.8402061855670103, "recall": 0.9367816091954023, "support": 174}, "\u2423": {"f1-score": 0.8848698720776357, "precision": 0.906871609403255, "recall": 0.8639104220499569, "support": 1161}},
  "ppcr": 0.8880439374735952
}
```
</details>