# Train report for javascript / file:///tmp/top-repos-quality-repos-ksif969j/puer.git HEAD c5ea5fc9d6f823ccbc66dfcc901a084d22f1d2ef

### Classification report

PPCR: 0.809

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.977| 0.946| 0.980| 0.964| 2757| 2847| 0.968 |
| `␣` | 0.944| 0.990| 0.864| 0.966| 0.903| 1353| 1549| 0.873 |
| `⏎␣⁻␣⁻` | 1.000| 0.778| 0.661| 0.875| 0.796| 158| 186| 0.849 |
| `"` | 1.000| 0.038| 0.015| 0.073| 0.030| 106| 261| 0.406 |
| `'` | 0.389| 1.000| 0.657| 0.560| 0.489| 65| 99| 0.657 |
| `⏎` | 0.727| 0.667| 0.025| 0.696| 0.049| 12| 316| 0.038 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 198| 0.051 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 63| 0.048 |
| `micro avg` | 0.948| 0.948| 0.767| 0.948| 0.848| 4464| 5519| 0.809 |
| `macro avg` | 0.630| 0.556| 0.396| 0.519| 0.404| 4464| 5519| 0.809 |
| `weighted avg` | 0.960| 0.948| 0.767| 0.941| 0.791| 4464| 5519| 0.809 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|90 |2693 |64 |0 |0 |0 |0 |0 |0 |
|196 |14 |1339 |0 |0 |0 |0 |0 |0 |
|304 |0 |2 |8 |0 |2 |0 |0 |0 |
|34 |0 |0 |0 |65 |0 |0 |0 |0 |
|60 |1 |0 |2 |0 |0 |0 |0 |0 |
|188 |0 |9 |1 |0 |0 |0 |0 |0 |
|28 |31 |4 |0 |0 |0 |0 |123 |0 |
|155 |0 |0 |0 |102 |0 |0 |0 |4 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/helper/util.js | 42 |
| lib/connect-puer.js | 38 |
| setup.js | 30 |
| lib/cli.js | 28 |
| lib/helper/parser.js | 23 |
| test/connect.js | 20 |
| lib/index.js | 19 |
| lib/helper/index.js | 8 |
| test/less.js | 8 |
| lib/helper/colorify.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.07272727272727272, "precision": 1.0, "recall": 0.03773584905660377, "support": 106}, "\u0027": {"f1-score": 0.5603448275862069, "precision": 0.38922155688622756, "recall": 1.0, "support": 65}, "macro avg": {"f1-score": 0.5188240833786679, "precision": 0.6304984453532019, "recall": 0.5561653142710163, "support": 4464}, "micro avg": {"f1-score": 0.9480286738351255, "precision": 0.9480286738351255, "recall": 0.9480286738351255, "support": 4464}, "weighted avg": {"f1-score": 0.9409073131586583, "precision": 0.96020288338208, "recall": 0.9480286738351255, "support": 4464}, "\u2205": {"f1-score": 0.9799854439592431, "precision": 0.9832055494706097, "recall": 0.9767863619876678, "support": 2757}, "\u23ce": {"f1-score": 0.6956521739130435, "precision": 0.7272727272727273, "recall": 0.6666666666666666, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8754448398576512, "precision": 1.0, "recall": 0.7784810126582279, "support": 158}, "\u2423": {"f1-score": 0.9664381089859256, "precision": 0.9442877291960508, "recall": 0.9896526237989652, "support": 1353}},
  "cl_report_full": {"\"": {"f1-score": 0.03018867924528302, "precision": 1.0, "recall": 0.01532567049808429, "support": 261}, "\u0027": {"f1-score": 0.4887218045112781, "precision": 0.38922155688622756, "recall": 0.6565656565656566, "support": 99}, "macro avg": {"f1-score": 0.4038435088797622, "precision": 0.6304984453532019, "recall": 0.3961043427874741, "support": 5519}, "micro avg": {"f1-score": 0.8478413302614446, "precision": 0.9480286738351255, "recall": 0.7668055807211451, "support": 5519}, "weighted avg": {"f1-score": 0.7905397226146583, "precision": 0.9018371095882274, "recall": 0.7668055807211451, "support": 5519}, "\u2205": {"f1-score": 0.9641962047977086, "precision": 0.9832055494706097, "recall": 0.9459079733052336, "support": 2847}, "\u23ce": {"f1-score": 0.04892966360856269, "precision": 0.7272727272727273, "recall": 0.02531645569620253, "support": 316}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 198}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7961165048543689, "precision": 1.0, "recall": 0.6612903225806451, "support": 186}, "\u2423": {"f1-score": 0.9025952140208966, "precision": 0.9442877291960508, "recall": 0.8644286636539703, "support": 1549}},
  "ppcr": 0.8088421815546295
}
```
</details>
