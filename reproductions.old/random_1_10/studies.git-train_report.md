# Train report for javascript / file:///tmp/top-repos-quality-repos-ks4hzi1_/studies.git HEAD cdb8206b5bd94bbbb5a310e0bb00302f988e180c

### Classification report

PPCR: 0.418

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.999| 0.679| 0.985| 0.799| 7040| 10364| 0.679 |
| `␣` | 0.991| 0.808| 0.178| 0.890| 0.302| 944| 4276| 0.221 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 396| 0.043 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 166| 0.042 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 1202| 0.002 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 443| 0.005 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 179| 0.011 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1240| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 584| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 307| 0.000 |
| `weighted avg` | 0.970| 0.973| 0.407| 0.970| 0.500| 8014| 19157| 0.418 |
| `micro avg` | 0.973| 0.973| 0.407| 0.973| 0.574| 8014| 19157| 0.418 |
| `macro avg` | 0.196| 0.181| 0.086| 0.188| 0.110| 8014| 19157| 0.418 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3324 |7034 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|3332 |181 |763 |0 |0 |0 |0 |0 |0 |0 |0 |
|1200 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1240 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|584 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|441 |1 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|379 |17 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|177 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|307 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|159 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Web_Development/react_native/confusion/src/redux/ActionCreators.js | 34 |
| Web_Development/react_native/confusion/src/components/ReservationComponent.js | 27 |
| Web_Development/react_native/confusion/src/components/MainComponent.js | 19 |
| Web_Development/react_native/confusion/src/components/DishdetailComponent.js | 19 |
| Web_Development/react_native/confusion/src/components/AboutComponent.js | 16 |
| Web_Development/react_native/confusion/src/components/MenuComponent.js | 12 |
| Web_Development/react_native/confusion/rascunho.js | 11 |
| Web_Development/react_native/confusion/rascunho_about.js | 10 |
| Web_Development/react_native/confusion/src/components/HomeComponent.js | 10 |
| Web_Development/react_native/confusion/src/redux/configureStore.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.18751932378909975, "precision": 0.19619195823502836, "recall": 0.18074104391371343, "support": 8014}, "micro avg": {"f1-score": 0.9729223858248066, "precision": 0.9729223858248066, "recall": 0.9729223858248066, "support": 8014}, "weighted avg": {"f1-score": 0.9700523876540144, "precision": 0.9697194960773867, "recall": 0.9729223858248066, "support": 8014}, "\u2205": {"f1-score": 0.9848781853822458, "precision": 0.9710104914411927, "recall": 0.9991477272727273, "support": 7040}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.8903150525087515, "precision": 0.990909090909091, "recall": 0.8082627118644068, "support": 944}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 584}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1240}, "macro avg": {"f1-score": 0.11013727770841741, "precision": 0.19619195823502836, "recall": 0.08571332766982492, "support": 19157}, "micro avg": {"f1-score": 0.5739207243016452, "precision": 0.9729223858248066, "recall": 0.40700527222425226, "support": 19157}, "weighted avg": {"f1-score": 0.4997394247159067, "precision": 0.7464989302095211, "recall": 0.40700527222425226, "support": 19157}, "\u2205": {"f1-score": 0.7989550204452521, "precision": 0.9710104914411927, "recall": 0.6786954843689695, "support": 10364}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1202}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 307}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 179}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 443}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 396}, "\u2423": {"f1-score": 0.30241775663892195, "precision": 0.990909090909091, "recall": 0.1784377923292797, "support": 4276}},
  "ppcr": 0.4183327243305319
}
```
</details>
