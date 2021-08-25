# Train report for javascript / file:///tmp/top-repos-quality-repos-c866grbi/carlo.git HEAD 8f2cbfedf381818792017fe53651fe07f270bb96

### Classification report

PPCR: 0.819

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 0.980| 0.936| 0.982| 0.960| 11662| 12205| 0.956 |
| `␣` | 0.896| 0.944| 0.700| 0.920| 0.786| 3569| 4811| 0.742 |
| `'` | 1.000| 1.000| 0.924| 1.000| 0.961| 1454| 1573| 0.924 |
| `⏎␣⁻␣⁻` | 0.920| 0.816| 0.555| 0.864| 0.692| 309| 454| 0.681 |
| `⏎` | 0.975| 0.707| 0.101| 0.820| 0.183| 164| 1148| 0.143 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 541| 0.061 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 306| 0.095 |
| `weighted avg` | 0.962| 0.965| 0.790| 0.963| 0.833| 17220| 21038| 0.819 |
| `macro avg` | 0.682| 0.635| 0.460| 0.655| 0.512| 17220| 21038| 0.819 |
| `micro avg` | 0.965| 0.965| 0.790| 0.965| 0.869| 17220| 21038| 0.819 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|543 |11428 |234 |0 |0 |0 |0 |0 |
|1242 |178 |3369 |0 |0 |0 |22 |0 |
|119 |0 |0 |1454 |0 |0 |0 |0 |
|984 |2 |46 |0 |116 |0 |0 |0 |
|508 |3 |30 |0 |0 |0 |0 |0 |
|145 |4 |53 |0 |0 |0 |252 |0 |
|277 |0 |26 |0 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/color.js | 141 |
| lib/carlo.js | 102 |
| rpc/test.js | 91 |
| rpc/rpc.js | 73 |
| test/app.spec.js | 50 |
| lib/find_chrome.js | 33 |
| .eslintrc.js | 15 |
| lib/http_request.js | 14 |
| examples/terminal/main.js | 12 |
| lib/features/shortcuts.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1454}, "macro avg": {"f1-score": 0.6551154073969457, "precision": 0.682126509664906, "recall": 0.6352496827023604, "support": 17220}, "micro avg": {"f1-score": 0.9650987224157956, "precision": 0.9650987224157956, "recall": 0.9650987224157956, "support": 17220}, "weighted avg": {"f1-score": 0.9633417810821172, "precision": 0.9623613529075168, "recall": 0.9650987224157956, "support": 17220}, "\u2205": {"f1-score": 0.9819134768226145, "precision": 0.9839001291433491, "recall": 0.9799348310752872, "support": 11662}, "\u23ce": {"f1-score": 0.8197879858657244, "precision": 0.9747899159663865, "recall": 0.7073170731707317, "support": 164}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8644939965694682, "precision": 0.9197080291970803, "recall": 0.8155339805825242, "support": 309}, "\u2423": {"f1-score": 0.9196123925208135, "precision": 0.8964874933475253, "recall": 0.9439618940879798, "support": 3569}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9606871489924018, "precision": 1.0, "recall": 0.9243483788938335, "support": 1573}, "macro avg": {"f1-score": 0.5117081639448902, "precision": 0.682126509664906, "recall": 0.4595810764314309, "support": 21038}, "micro avg": {"f1-score": 0.8687856134664645, "precision": 0.9650987224157956, "recall": 0.7899515163038312, "support": 21038}, "weighted avg": {"f1-score": 0.8332416943225402, "precision": 0.9236195776915299, "recall": 0.7899515163038312, "support": 21038}, "\u2205": {"f1-score": 0.9595298068849706, "precision": 0.9839001291433491, "recall": 0.9363375665710775, "support": 12205}, "\u23ce": {"f1-score": 0.18310970797158643, "precision": 0.9747899159663865, "recall": 0.10104529616724739, "support": 1148}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 541}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6923076923076923, "precision": 0.9197080291970803, "recall": 0.5550660792951542, "support": 454}, "\u2423": {"f1-score": 0.7863227914575797, "precision": 0.8964874933475253, "recall": 0.7002702140927042, "support": 4811}},
  "ppcr": 0.8185188706150774
}
```
</details>
