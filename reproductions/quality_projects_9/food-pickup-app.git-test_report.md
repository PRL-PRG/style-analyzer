# Test report for javascript / file:///tmp/top-repos-quality-repos-apw6u0y0/food-pickup-app.git HEAD 808fbde3c98308cae23ff425f423b69e6b533df9

### Classification report

PPCR: 0.817

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.844| 0.963| 0.956| 0.900| 0.897| 681| 686| 0.993 |
| `␣` | 0.795| 0.455| 0.433| 0.579| 0.561| 213| 224| 0.951 |
| `⏎␣⁺␣⁺` | 0.944| 0.829| 0.829| 0.883| 0.883| 41| 41| 1.000 |
| `⏎` | 0.298| 0.389| 0.179| 0.337| 0.224| 36| 78| 0.462 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 33| 0.273 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 72| 0.028 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 68| 0.000 |
| `micro avg` | 0.816| 0.816| 0.666| 0.816| 0.734| 982| 1202| 0.817 |
| `macro avg` | 0.412| 0.377| 0.343| 0.386| 0.366| 982| 1202| 0.817 |
| `weighted avg` | 0.808| 0.816| 0.666| 0.799| 0.661| 982| 1202| 0.817 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|5 |656 |25 |0 |0 |0 |0 |0 |
|11 |81 |97 |0 |33 |0 |2 |0 |
|70 |2 |0 |0 |0 |0 |0 |0 |
|42 |22 |0 |0 |14 |0 |0 |0 |
|68 |0 |0 |0 |0 |0 |0 |0 |
|0 |7 |0 |0 |0 |0 |34 |0 |
|24 |9 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3856333691582766, "precision": 0.4116673709079907, "recall": 0.3766922175820824, "support": 982}, "micro avg": {"f1-score": 0.8156822810590632, "precision": 0.8156822810590632, "recall": 0.8156822810590632, "support": 982}, "weighted avg": {"f1-score": 0.798888196145141, "precision": 0.808297242814401, "recall": 0.8156822810590632, "support": 982}, "\u2205": {"f1-score": 0.8998628257887518, "precision": 0.8442728442728443, "recall": 0.9632892804698973, "support": 681}, "\u23ce": {"f1-score": 0.3373493975903615, "precision": 0.2978723404255319, "recall": 0.3888888888888889, "support": 36}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.883116883116883, "precision": 0.9444444444444444, "recall": 0.8292682926829268, "support": 41}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.5791044776119403, "precision": 0.7950819672131147, "recall": 0.45539906103286387, "support": 213}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "macro avg": {"f1-score": 0.3663711354055132, "precision": 0.4116673709079907, "recall": 0.3425799154328807, "support": 1202}, "micro avg": {"f1-score": 0.7335164835164835, "precision": 0.8156822810590632, "recall": 0.6663893510815307, "support": 1202}, "weighted avg": {"f1-score": 0.6609578537264055, "precision": 0.6815522434295528, "recall": 0.6663893510815307, "support": 1202}, "\u2205": {"f1-score": 0.8967874231032126, "precision": 0.8442728442728443, "recall": 0.956268221574344, "support": 686}, "\u23ce": {"f1-score": 0.224, "precision": 0.2978723404255319, "recall": 0.1794871794871795, "support": 78}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.883116883116883, "precision": 0.9444444444444444, "recall": 0.8292682926829268, "support": 41}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u2423": {"f1-score": 0.5606936416184971, "precision": 0.7950819672131147, "recall": 0.4330357142857143, "support": 224}},
  "ppcr": 0.8169717138103162
}
```
</details>
