# Train report for javascript / file:///tmp/top-repos-quality-repos-4pnbbv7r/node-react-ecommerce.git HEAD 51896ab498b92f6ad81df5fdbc9e05f43c330b9c

### Classification report

PPCR: 0.559

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 1.000| 0.878| 0.989| 0.925| 9119| 10389| 0.878 |
| `␣` | 1.000| 0.573| 0.043| 0.729| 0.083| 354| 4668| 0.076 |
| `'` | 1.000| 1.000| 0.365| 1.000| 0.535| 282| 772| 0.365 |
| `"` | 1.000| 1.000| 0.397| 1.000| 0.568| 229| 577| 0.397 |
| `⏎` | 1.000| 1.000| 0.159| 1.000| 0.275| 127| 798| 0.159 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 391| 0.123 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 438| 0.016 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 147| 0.000 |
| `macro avg` | 0.622| 0.572| 0.230| 0.590| 0.298| 10166| 18180| 0.559 |
| `weighted avg` | 0.975| 0.980| 0.548| 0.975| 0.603| 10166| 18180| 0.559 |
| `micro avg` | 0.980| 0.980| 0.548| 0.980| 0.703| 10166| 18180| 0.559 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1270 |9119 |0 |0 |0 |0 |0 |0 |0 |
|4314 |151 |203 |0 |0 |0 |0 |0 |0 |
|671 |0 |0 |127 |0 |0 |0 |0 |0 |
|490 |0 |0 |0 |282 |0 |0 |0 |0 |
|348 |0 |0 |0 |0 |229 |0 |0 |0 |
|431 |7 |0 |0 |0 |0 |0 |0 |0 |
|343 |48 |0 |0 |0 |0 |0 |0 |0 |
|147 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/screens/ProductScreen.js | 23 |
| backend/routes/productRoute.js | 17 |
| frontend/src/App.js | 17 |
| frontend/src/screens/CartScreen.js | 13 |
| frontend/src/screens/OrderScreen.js | 12 |
| frontend/src/screens/ProductsScreen.js | 12 |
| frontend/src/screens/PlaceOrderScreen.js | 12 |
| frontend/src/screens/HomeScreen.js | 10 |
| frontend/src/components/Rating.js | 9 |
| frontend/src/components/CheckoutSteps.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 229}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 282}, "macro avg": {"f1-score": 0.5897169879458496, "precision": 0.6222386058981233, "recall": 0.571680790960452, "support": 10166}, "micro avg": {"f1-score": 0.9797363761558135, "precision": 0.9797363761558135, "recall": 0.9797363761558135, "support": 10166}, "weighted avg": {"f1-score": 0.9751310955334392, "precision": 0.9747738321345557, "recall": 0.9797363761558135, "support": 10166}, "\u2205": {"f1-score": 0.9888310561700282, "precision": 0.9779088471849866, "recall": 1.0, "support": 9119}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 127}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u2423": {"f1-score": 0.7289048473967683, "precision": 1.0, "recall": 0.5734463276836158, "support": 354}},
  "cl_report_full": {"\"": {"f1-score": 0.5682382133995038, "precision": 1.0, "recall": 0.3968804159445407, "support": 577}, "\u0027": {"f1-score": 0.5351043643263758, "precision": 1.0, "recall": 0.36528497409326427, "support": 772}, "macro avg": {"f1-score": 0.298302120426125, "precision": 0.6222386058981233, "recall": 0.23031951910193846, "support": 18180}, "micro avg": {"f1-score": 0.7027446553305581, "precision": 0.9797363761558135, "recall": 0.5478547854785478, "support": 18180}, "weighted avg": {"f1-score": 0.6028794928117173, "precision": 0.9336905947967451, "recall": 0.5478547854785478, "support": 18180}, "\u2205": {"f1-score": 0.9251293497007202, "precision": 0.9779088471849866, "recall": 0.8777553181249398, "support": 10389}, "\u23ce": {"f1-score": 0.27459459459459457, "precision": 1.0, "recall": 0.15914786967418545, "support": 798}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 147}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 438}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 391}, "\u2423": {"f1-score": 0.08335044138780538, "precision": 1.0, "recall": 0.043487574978577546, "support": 4668}},
  "ppcr": 0.5591859185918592
}
```
</details>
