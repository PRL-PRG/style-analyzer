# Train report for javascript / file:///tmp/top-repos-quality-repos-bcykcysz/rage_mkdocs.git HEAD 9a195d4a4ca39b5ce0f47662e2821b0db18adf97

### Classification report

PPCR: 0.100

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 1.000| 0.183| 0.988| 0.308| 292| 1595| 0.183 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 181| 0.033 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 105| 0.010 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 866| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 152| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 104| 0.000 |
| `micro avg` | 0.977| 0.977| 0.097| 0.977| 0.177| 299| 3003| 0.100 |
| `weighted avg` | 0.954| 0.977| 0.097| 0.965| 0.164| 299| 3003| 0.100 |
| `macro avg` | 0.163| 0.167| 0.031| 0.165| 0.051| 299| 3003| 0.100 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1303 |292 |0 |0 |0 |0 |0 |
|866 |0 |0 |0 |0 |0 |0 |
|175 |6 |0 |0 |0 |0 |0 |
|152 |0 |0 |0 |0 |0 |0 |
|104 |1 |0 |0 |0 |0 |0 |
|104 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| site/search/text.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.164692611393119, "precision": 0.16276477146042365, "recall": 0.16666666666666666, "support": 299}, "micro avg": {"f1-score": 0.9765886287625418, "precision": 0.9765886287625418, "recall": 0.9765886287625418, "support": 299}, "weighted avg": {"f1-score": 0.9650215891663695, "precision": 0.9537253498283017, "recall": 0.9765886287625418, "support": 299}, "\u2205": {"f1-score": 0.988155668358714, "precision": 0.9765886287625418, "recall": 1.0, "support": 292}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "macro avg": {"f1-score": 0.05139035550862372, "precision": 0.16276477146042365, "recall": 0.03051201671891327, "support": 3003}, "micro avg": {"f1-score": 0.17686250757116898, "precision": 0.9765886287625418, "recall": 0.09723609723609723, "support": 3003}, "weighted avg": {"f1-score": 0.16377146260989975, "precision": 0.5187009200387127, "recall": 0.09723609723609723, "support": 3003}, "\u2205": {"f1-score": 0.3083421330517423, "precision": 0.9765886287625418, "recall": 0.18307210031347962, "support": 1595}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 181}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 866}},
  "ppcr": 0.09956709956709957
}
```
</details>
