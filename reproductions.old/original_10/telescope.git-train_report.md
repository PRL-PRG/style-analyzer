# Train report for javascript / file:///tmp/top-repos-quality-repos-y5ks62f8/telescope.git HEAD 0ad3dfd523f6fda6fa5e7adaa6a72a5d8921f752

### Classification report

PPCR: 0.782

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.993| 0.860| 0.983| 0.913| 700| 808| 0.866 |
| `␣` | 0.964| 0.976| 0.814| 0.970| 0.883| 409| 490| 0.835 |
| `'` | 1.000| 1.000| 0.863| 1.000| 0.927| 360| 417| 0.863 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.987| 0.975| 0.975| 0.981| 0.981| 79| 79| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.986| 0.948| 0.924| 0.967| 0.954| 77| 79| 0.975 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 152| 0.086 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 71| 0.028 |
| `weighted avg` | 0.969| 0.978| 0.765| 0.974| 0.816| 1640| 2096| 0.782 |
| `macro avg` | 0.702| 0.699| 0.634| 0.700| 0.665| 1640| 2096| 0.782 |
| `micro avg` | 0.978| 0.978| 0.765| 0.978| 0.859| 1640| 2096| 0.782 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|108 |695 |5 |0 |0 |0 |0 |0 |
|81 |8 |399 |0 |0 |1 |1 |0 |
|57 |0 |0 |360 |0 |0 |0 |0 |
|139 |6 |7 |0 |0 |0 |0 |0 |
|0 |1 |1 |0 |0 |77 |0 |0 |
|2 |4 |0 |0 |0 |0 |73 |0 |
|69 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| webpack.mix.js | 15 |
| resources/js/base.js | 11 |
| resources/js/app.js | 7 |
| resources/js/routes.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 360}, "macro avg": {"f1-score": 0.7000613343375514, "precision": 0.7015462064786142, "recall": 0.698734679637468, "support": 1640}, "micro avg": {"f1-score": 0.9780487804878049, "precision": 0.9780487804878049, "recall": 0.9780487804878049, "support": 1640}, "weighted avg": {"f1-score": 0.9735582503315127, "precision": 0.9692075288942296, "recall": 0.9780487804878049, "support": 1640}, "\u2205": {"f1-score": 0.983026874115983, "precision": 0.9733893557422969, "recall": 0.9928571428571429, "support": 700}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.980891719745223, "precision": 0.9871794871794872, "recall": 0.9746835443037974, "support": 79}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9668874172185431, "precision": 0.9864864864864865, "recall": 0.948051948051948, "support": 77}, "\u2423": {"f1-score": 0.9696233292831105, "precision": 0.9637681159420289, "recall": 0.9755501222493888, "support": 409}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9266409266409266, "precision": 1.0, "recall": 0.8633093525179856, "support": 417}, "macro avg": {"f1-score": 0.6653994836776519, "precision": 0.7015462064786142, "recall": 0.6337825369814821, "support": 2096}, "micro avg": {"f1-score": 0.8586723768736617, "precision": 0.9780487804878049, "recall": 0.7652671755725191, "support": 2096}, "weighted avg": {"f1-score": 0.8157220255085252, "precision": 0.8738857767991327, "recall": 0.7652671755725191, "support": 2096}, "\u2205": {"f1-score": 0.9132720105124835, "precision": 0.9733893557422969, "recall": 0.8601485148514851, "support": 808}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 71}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.980891719745223, "precision": 0.9871794871794872, "recall": 0.9746835443037974, "support": 79}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.954248366013072, "precision": 0.9864864864864865, "recall": 0.9240506329113924, "support": 79}, "\u2423": {"f1-score": 0.8827433628318583, "precision": 0.9637681159420289, "recall": 0.8142857142857143, "support": 490}},
  "ppcr": 0.7824427480916031
}
```
</details>
