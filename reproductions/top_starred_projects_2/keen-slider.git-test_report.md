# Test report for javascript / file:///tmp/top-repos-quality-repos-q8jykux0/keen-slider.git HEAD bfb2816b46d9f59874843f13c1982b4ad17bd2f3

### Classification report

PPCR: 0.422

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.827| 0.942| 0.644| 0.881| 0.724| 173| 253| 0.684 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 68| 0.515 |
| `␣` | 0.909| 0.526| 0.083| 0.667| 0.152| 19| 121| 0.157 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 58| 0.017 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 23| 0.043 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 22| 0.045 |
| `micro avg` | 0.752| 0.752| 0.317| 0.752| 0.446| 230| 545| 0.422 |
| `macro avg` | 0.289| 0.245| 0.121| 0.258| 0.146| 230| 545| 0.422 |
| `weighted avg` | 0.697| 0.752| 0.317| 0.718| 0.370| 230| 545| 0.422 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|80 |163 |1 |9 |0 |0 |0 |
|102 |3 |10 |6 |0 |0 |0 |
|57 |1 |0 |0 |0 |0 |0 |
|22 |1 |0 |0 |0 |0 |0 |
|21 |1 |0 |0 |0 |0 |0 |
|33 |28 |0 |7 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "macro avg": {"f1-score": 0.25795795795795795, "precision": 0.28941701276726656, "recall": 0.2447520535442653, "support": 230}, "micro avg": {"f1-score": 0.7521739130434782, "precision": 0.7521739130434782, "recall": 0.7521739130434782, "support": 230}, "weighted avg": {"f1-score": 0.7177986682334508, "precision": 0.6974559097931423, "recall": 0.7521739130434782, "support": 230}, "\u2205": {"f1-score": 0.8810810810810811, "precision": 0.8274111675126904, "recall": 0.9421965317919075, "support": 173}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.6666666666666666, "precision": 0.9090909090909091, "recall": 0.5263157894736842, "support": 19}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "macro avg": {"f1-score": 0.145993265993266, "precision": 0.28941701276726656, "recall": 0.12115223380045514, "support": 545}, "micro avg": {"f1-score": 0.4464516129032258, "precision": 0.7521739130434782, "recall": 0.3174311926605505, "support": 545}, "weighted avg": {"f1-score": 0.369940876656473, "precision": 0.5859358263866251, "recall": 0.3174311926605505, "support": 545}, "\u2205": {"f1-score": 0.7244444444444446, "precision": 0.8274111675126904, "recall": 0.6442687747035574, "support": 253}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u2423": {"f1-score": 0.1515151515151515, "precision": 0.9090909090909091, "recall": 0.08264462809917356, "support": 121}},
  "ppcr": 0.42201834862385323
}
```
</details>
