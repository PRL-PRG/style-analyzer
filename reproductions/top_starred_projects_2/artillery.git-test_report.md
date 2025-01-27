# Test report for javascript / file:///tmp/top-repos-quality-repos-zlgsufzr/artillery.git HEAD d3cb703f2720a3902a0d1af2454717b6e5a41074

### Classification report

PPCR: 0.910

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 0.978| 0.975| 0.970| 0.968| 5461| 5482| 0.996 |
| `␣` | 0.891| 0.938| 0.850| 0.914| 0.870| 2041| 2254| 0.906 |
| `'` | 1.000| 0.889| 0.873| 0.941| 0.932| 1033| 1052| 0.982 |
| `⏎␣⁻␣⁻` | 0.773| 0.882| 0.860| 0.824| 0.814| 271| 278| 0.975 |
| `⏎␣⁺␣⁺` | 0.990| 0.781| 0.622| 0.873| 0.764| 251| 315| 0.797 |
| `⏎` | 0.891| 0.386| 0.098| 0.538| 0.177| 127| 500| 0.254 |
| `⏎⏎` | 0.588| 0.488| 0.078| 0.533| 0.137| 41| 258| 0.159 |
| `micro avg` | 0.941| 0.941| 0.856| 0.941| 0.897| 9225| 10139| 0.910 |
| `macro avg` | 0.871| 0.763| 0.622| 0.799| 0.666| 9225| 10139| 0.910 |
| `weighted avg` | 0.942| 0.941| 0.856| 0.939| 0.872| 9225| 10139| 0.910 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|21 |5343 |97 |0 |0 |0 |21 |0 |
|213 |69 |1915 |0 |0 |1 |49 |7 |
|19 |55 |60 |918 |0 |0 |0 |0 |
|373 |31 |39 |0 |49 |1 |0 |7 |
|64 |37 |18 |0 |0 |196 |0 |0 |
|7 |24 |8 |0 |0 |0 |239 |0 |
|217 |2 |13 |0 |6 |0 |0 |20 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9410558687852384, "precision": 1.0, "recall": 0.888673765730881, "support": 1033}, "macro avg": {"f1-score": 0.7990597786902713, "precision": 0.8705717500095219, "recall": 0.7631083600856415, "support": 9225}, "micro avg": {"f1-score": 0.9409214092140922, "precision": 0.9409214092140922, "recall": 0.9409214092140922, "support": 9225}, "weighted avg": {"f1-score": 0.9392474709902267, "precision": 0.9423490811969323, "recall": 0.9409214092140922, "support": 9225}, "\u2205": {"f1-score": 0.9695155144256942, "precision": 0.9607984175508002, "recall": 0.9783922358542392, "support": 5461}, "\u23ce": {"f1-score": 0.5384615384615384, "precision": 0.8909090909090909, "recall": 0.3858267716535433, "support": 127}, "\u23ce\u23ce": {"f1-score": 0.5333333333333332, "precision": 0.5882352941176471, "recall": 0.4878048780487805, "support": 41}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8730512249443209, "precision": 0.98989898989899, "recall": 0.7808764940239044, "support": 251}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8241379310344827, "precision": 0.7734627831715211, "recall": 0.8819188191881919, "support": 271}, "\u2423": {"f1-score": 0.9138630398472919, "precision": 0.8906976744186047, "recall": 0.938265556099951, "support": 2041}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9319796954314721, "precision": 1.0, "recall": 0.8726235741444867, "support": 1052}, "macro avg": {"f1-score": 0.6659029926100037, "precision": 0.8705717500095219, "recall": 0.6220460580973738, "support": 10139}, "micro avg": {"f1-score": 0.8965089857467465, "precision": 0.9409214092140922, "recall": 0.8561002071210179, "support": 10139}, "weighted avg": {"f1-score": 0.8715014133532467, "precision": 0.9321224548801444, "recall": 0.8561002071210179, "support": 10139}, "\u2205": {"f1-score": 0.9676718283075251, "precision": 0.9607984175508002, "recall": 0.9746442904049617, "support": 5482}, "\u23ce": {"f1-score": 0.17657657657657658, "precision": 0.8909090909090909, "recall": 0.098, "support": 500}, "\u23ce\u23ce": {"f1-score": 0.136986301369863, "precision": 0.5882352941176471, "recall": 0.07751937984496124, "support": 258}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7641325536062378, "precision": 0.98989898989899, "recall": 0.6222222222222222, "support": 315}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8143100511073254, "precision": 0.7734627831715211, "recall": 0.8597122302158273, "support": 278}, "\u2423": {"f1-score": 0.8696639418710262, "precision": 0.8906976744186047, "recall": 0.849600709849157, "support": 2254}},
  "ppcr": 0.9098530427063813
}
```
</details>
