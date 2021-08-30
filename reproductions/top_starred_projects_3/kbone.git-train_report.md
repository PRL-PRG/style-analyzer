# Train report for javascript / file:///tmp/top-repos-quality-repos-nlskh9c9/kbone.git HEAD 03e72e770dff9b83be6ad9c4813dc08e80dbd797

### Classification report

PPCR: 0.919

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.985| 0.967| 0.989| 0.980| 98980| 100772| 0.982 |
| `␣` | 0.940| 0.989| 0.952| 0.964| 0.946| 48267| 50153| 0.962 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 23533| 23533| 1.000 |
| `⏎` | 0.926| 0.923| 0.603| 0.924| 0.730| 7960| 12181| 0.653 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.923| 0.939| 0.801| 0.931| 0.858| 4849| 5685| 0.853 |
| `⏎⏎` | 0.982| 0.619| 0.340| 0.759| 0.505| 2323| 4227| 0.550 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.909| 0.616| 0.098| 0.735| 0.177| 896| 5644| 0.159 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 307| 589| 0.521 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 82| 578| 0.142 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 69| 229| 0.301 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 227| 0.123 |
| `weighted avg` | 0.973| 0.975| 0.896| 0.973| 0.916| 187294| 203818| 0.919 |
| `micro avg` | 0.975| 0.975| 0.896| 0.975| 0.934| 187294| 203818| 0.919 |
| `macro avg` | 0.607| 0.552| 0.433| 0.573| 0.472| 187294| 203818| 0.919 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1792 |97447 |1325 |0 |51 |121 |36 |0 |0 |0 |0 |0 |
|1886 |286 |47723 |0 |211 |39 |7 |1 |0 |0 |0 |0 |
|0 |0 |0 |23533 |0 |0 |0 |0 |0 |0 |0 |0 |
|4221 |75 |507 |0 |7345 |8 |0 |25 |0 |0 |0 |0 |
|836 |147 |116 |0 |31 |4555 |0 |0 |0 |0 |0 |0 |
|4748 |72 |272 |0 |0 |0 |552 |0 |0 |0 |0 |0 |
|1904 |2 |645 |0 |239 |0 |0 |1437 |0 |0 |0 |0 |
|282 |30 |67 |0 |2 |208 |0 |0 |0 |0 |0 |0 |
|496 |19 |51 |0 |0 |0 |12 |0 |0 |0 |0 |0 |
|160 |0 |11 |0 |54 |4 |0 |0 |0 |0 |0 |0 |
|199 |0 |28 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/miniprogram-element/test/tool.test.js | 203 |
| packages/miniprogram-render/src/node/element.js | 102 |
| .eslintrc.js | 98 |
| packages/miniprogram-element/test/utils.js | 93 |
| packages/kbone-ui/.eslintrc.js | 93 |
| packages/miniprogram-element/test/index.test.js | 92 |
| packages/miniprogram-element/src/util/component.js | 84 |
| packages/miniprogram-render/src/bom/location.js | 79 |
| packages/miniprogram-render/test/tree/query-selector.test.js | 69 |
| packages/kbone-ui/src/components/wx-picker/index.js | 59 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 23533}, "macro avg": {"f1-score": 0.5729149455484753, "precision": 0.6067736952861089, "recall": 0.5518197353269364, "support": 187294}, "micro avg": {"f1-score": 0.9748950847330934, "precision": 0.9748950847330934, "recall": 0.9748950847330934, "support": 187294}, "weighted avg": {"f1-score": 0.9730619740611469, "precision": 0.9728604191769709, "recall": 0.9748950847330934, "support": 187294}, "\u2205": {"f1-score": 0.9890184615696902, "precision": 0.9935663451538571, "recall": 0.9845120226308345, "support": 98980}, "\u23ce": {"f1-score": 0.9243062983703517, "precision": 0.9258792386234715, "recall": 0.9227386934673367, "support": 7960}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u23ce": {"f1-score": 0.7591125198098256, "precision": 0.9822282980177717, "recall": 0.618596642272923, "support": 2323}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 307}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9311120196238757, "precision": 0.922998986828774, "recall": 0.9393689420499072, "support": 4849}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7345309381237525, "precision": 0.9093904448105437, "recall": 0.6160714285714286, "support": 896}, "\u2423": {"f1-score": 0.963984163535733, "precision": 0.9404473347127796, "recall": 0.9887293596038701, "support": 48267}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 23533}, "macro avg": {"f1-score": 0.47235743305444894, "precision": 0.6067736952861089, "recall": 0.4327757227221061, "support": 203818}, "micro avg": {"f1-score": 0.933706968847798, "precision": 0.9748950847330934, "recall": 0.8958580694541208, "support": 203818}, "weighted avg": {"f1-score": 0.9157579485031018, "precision": 0.9647469106934639, "recall": 0.8958580694541208, "support": 203818}, "\u2205": {"f1-score": 0.9801056072416393, "precision": 0.9935663451538571, "recall": 0.9670047235343151, "support": 100772}, "\u23ce": {"f1-score": 0.7303370786516854, "precision": 0.9258792386234715, "recall": 0.6029882604055496, "support": 12181}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 229}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 227}, "\u23ce\u23ce": {"f1-score": 0.5050966608084358, "precision": 0.9822282980177717, "recall": 0.33995741660752304, "support": 4227}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 589}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8578154425612051, "precision": 0.922998986828774, "recall": 0.8012313104661389, "support": 5685}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 578}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.17661174212126063, "precision": 0.9093904448105437, "recall": 0.09780297661233169, "support": 5644}, "\u2423": {"f1-score": 0.9459652322147121, "precision": 0.9404473347127796, "recall": 0.951548262317309, "support": 50153}},
  "ppcr": 0.9189276707650943
}
```
</details>
