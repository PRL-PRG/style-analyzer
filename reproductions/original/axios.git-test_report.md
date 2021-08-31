# Test report for javascript / file:///tmp/top-repos-quality-repos-sph9fln0/axios.git HEAD 5bc9ea24dda14e74def0b8ae9cdb3fa1a0c77773

### Classification report

PPCR: 0.966

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.997| 0.997| 0.995| 0.995| 5049| 5049| 1.000 |
| `␣` | 0.989| 0.980| 0.974| 0.984| 0.982| 1860| 1870| 0.995 |
| `'` | 0.991| 0.997| 0.996| 0.994| 0.993| 899| 900| 0.999 |
| `⏎␣⁺␣⁺` | 0.959| 0.989| 0.984| 0.974| 0.971| 378| 380| 0.995 |
| `⏎␣⁻␣⁻` | 0.958| 0.995| 0.992| 0.976| 0.975| 368| 369| 0.997 |
| `⏎⏎` | 0.772| 0.970| 0.801| 0.860| 0.787| 133| 161| 0.826 |
| `⏎` | 1.000| 0.143| 0.027| 0.250| 0.053| 63| 331| 0.190 |
| `"` | 0.500| 0.111| 0.100| 0.182| 0.167| 9| 10| 0.900 |
| `macro avg` | 0.895| 0.773| 0.734| 0.777| 0.740| 8759| 9070| 0.966 |
| `micro avg` | 0.985| 0.985| 0.951| 0.985| 0.968| 8759| 9070| 0.966 |
| `weighted avg` | 0.986| 0.985| 0.951| 0.983| 0.951| 8759| 9070| 0.966 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |5032 |16 |0 |0 |0 |1 |0 |0 |
|10 |7 |1822 |0 |0 |16 |15 |0 |0 |
|1 |2 |0 |896 |0 |0 |0 |0 |1 |
|268 |16 |0 |0 |9 |0 |0 |38 |0 |
|2 |1 |3 |0 |0 |374 |0 |0 |0 |
|1 |1 |1 |0 |0 |0 |366 |0 |0 |
|28 |4 |0 |0 |0 |0 |0 |129 |0 |
|1 |0 |0 |8 |0 |0 |0 |0 |1 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.1818181818181818, "precision": 0.5, "recall": 0.1111111111111111, "support": 9}, "\u0027": {"f1-score": 0.9938990571270105, "precision": 0.9911504424778761, "recall": 0.996662958843159, "support": 899}, "macro avg": {"f1-score": 0.7769076912400383, "precision": 0.8954643073942393, "recall": 0.772592765094612, "support": 8759}, "micro avg": {"f1-score": 0.9851581230734102, "precision": 0.9851581230734102, "recall": 0.9851581230734102, "support": 8759}, "weighted avg": {"f1-score": 0.9828174550724295, "precision": 0.9857574780283209, "recall": 0.9851581230734102, "support": 8759}, "\u2205": {"f1-score": 0.995253164556962, "precision": 0.9938771479360063, "recall": 0.9966329966329966, "support": 5049}, "\u23ce": {"f1-score": 0.25, "precision": 1.0, "recall": 0.14285714285714285, "support": 63}, "\u23ce\u23ce": {"f1-score": 0.86, "precision": 0.7724550898203593, "recall": 0.9699248120300752, "support": 133}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9739583333333334, "precision": 0.958974358974359, "recall": 0.9894179894179894, "support": 378}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.976, "precision": 0.9581151832460733, "recall": 0.9945652173913043, "support": 368}, "\u2423": {"f1-score": 0.9843327930848191, "precision": 0.98914223669924, "recall": 0.9795698924731183, "support": 1860}},
  "cl_report_full": {"\"": {"f1-score": 0.16666666666666669, "precision": 0.5, "recall": 0.1, "support": 10}, "\u0027": {"f1-score": 0.9933481152993349, "precision": 0.9911504424778761, "recall": 0.9955555555555555, "support": 900}, "macro avg": {"f1-score": 0.7403255617782396, "precision": 0.8954643073942393, "recall": 0.7338791395445996, "support": 9070}, "micro avg": {"f1-score": 0.9679735262774131, "precision": 0.9851581230734102, "recall": 0.9513781697905181, "support": 9070}, "weighted avg": {"f1-score": 0.9514253374451433, "precision": 0.9854616460061408, "recall": 0.9513781697905181, "support": 9070}, "\u2205": {"f1-score": 0.995253164556962, "precision": 0.9938771479360063, "recall": 0.9966329966329966, "support": 5049}, "\u23ce": {"f1-score": 0.05294117647058823, "precision": 1.0, "recall": 0.027190332326283987, "support": 331}, "\u23ce\u23ce": {"f1-score": 0.7865853658536586, "precision": 0.7724550898203593, "recall": 0.8012422360248447, "support": 161}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9714285714285715, "precision": 0.958974358974359, "recall": 0.9842105263157894, "support": 380}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9747003994673767, "precision": 0.9581151832460733, "recall": 0.991869918699187, "support": 369}, "\u2423": {"f1-score": 0.9816810344827586, "precision": 0.98914223669924, "recall": 0.9743315508021391, "support": 1870}},
  "ppcr": 0.9657111356119074
}
```
</details>