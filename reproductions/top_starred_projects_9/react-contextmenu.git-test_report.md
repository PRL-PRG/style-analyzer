# Test report for javascript / file:///tmp/top-repos-quality-repos-b1wc6w99/react-contextmenu.git HEAD d9018dbfbd6e21423cb2b753b3762adf5a6d77b0

### Classification report

PPCR: 0.922

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.824| 0.963| 0.963| 0.888| 0.888| 1265| 1265| 1.000 |
| `␣` | 0.872| 0.729| 0.713| 0.794| 0.784| 652| 666| 0.979 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.912| 0.712| 0.712| 0.800| 0.800| 73| 73| 1.000 |
| `'` | 1.000| 0.812| 0.800| 0.896| 0.889| 69| 70| 0.986 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 77| 0.429 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 93| 0.258 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 75| 0.280 |
| `weighted avg` | 0.817| 0.843| 0.777| 0.824| 0.762| 2137| 2319| 0.922 |
| `micro avg` | 0.843| 0.843| 0.777| 0.843| 0.808| 2137| 2319| 0.922 |
| `macro avg` | 0.515| 0.459| 0.455| 0.482| 0.480| 2137| 2319| 0.922 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |1218 |47 |0 |0 |0 |0 |0 |
|14 |174 |475 |0 |0 |0 |3 |0 |
|1 |12 |1 |56 |0 |0 |0 |0 |
|69 |9 |13 |0 |0 |0 |2 |0 |
|44 |25 |8 |0 |0 |0 |0 |0 |
|0 |21 |0 |0 |0 |0 |52 |0 |
|54 |20 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.896, "precision": 1.0, "recall": 0.8115942028985508, "support": 69}, "macro avg": {"f1-score": 0.4824865565273729, "precision": 0.515338535220945, "recall": 0.4593280610265961, "support": 2137}, "micro avg": {"f1-score": 0.8427702386523164, "precision": 0.8427702386523164, "recall": 0.8427702386523164, "support": 2137}, "weighted avg": {"f1-score": 0.8239094625839729, "precision": 0.8168545053085485, "recall": 0.8427702386523164, "support": 2137}, "\u2205": {"f1-score": 0.8877551020408163, "precision": 0.8235294117647058, "recall": 0.9628458498023715, "support": 1265}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8, "precision": 0.9122807017543859, "recall": 0.7123287671232876, "support": 73}, "\u2423": {"f1-score": 0.7936507936507936, "precision": 0.8715596330275229, "recall": 0.7285276073619632, "support": 652}},
  "cl_report_full": {"\u0027": {"f1-score": 0.888888888888889, "precision": 1.0, "recall": 0.8, "support": 70}, "macro avg": {"f1-score": 0.48015994727095357, "precision": 0.515338535220945, "recall": 0.4554839757341247, "support": 2319}, "micro avg": {"f1-score": 0.8083482944344704, "precision": 0.8427702386523164, "recall": 0.7766278568348426, "support": 2319}, "weighted avg": {"f1-score": 0.7615753352832498, "precision": 0.7584389446773407, "recall": 0.7766278568348426, "support": 2319}, "\u2205": {"f1-score": 0.8877551020408163, "precision": 0.8235294117647058, "recall": 0.9628458498023715, "support": 1265}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 93}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8, "precision": 0.9122807017543859, "recall": 0.7123287671232876, "support": 73}, "\u2423": {"f1-score": 0.7844756399669693, "precision": 0.8715596330275229, "recall": 0.7132132132132132, "support": 666}},
  "ppcr": 0.9215178956446745
}
```
</details>
