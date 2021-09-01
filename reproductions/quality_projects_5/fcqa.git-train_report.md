# Train report for javascript / file:///tmp/top-repos-quality-repos-axerpbcy/fcqa.git HEAD a3b72620e7447076e3a3bd70015021d404e51661

### Classification report

PPCR: 0.998

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 0.994| 0.994| 0.995| 0.995| 9167| 9167| 1.000 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 7924| 7924| 1.000 |
| `␣` | 0.993| 0.997| 0.997| 0.995| 0.995| 6993| 6993| 1.000 |
| `⏎` | 1.000| 1.000| 0.961| 1.000| 0.980| 1280| 1332| 0.961 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 1.000| 1.000| 1.000| 1.000| 1.000| 1269| 1269| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.981| 0.981| 0.990| 0.990| 1269| 1269| 1.000 |
| `micro avg` | 0.996| 0.996| 0.995| 0.996| 0.995| 27902| 27954| 0.998 |
| `weighted avg` | 0.996| 0.996| 0.995| 0.996| 0.995| 27902| 27954| 0.998 |
| `macro avg` | 0.998| 0.995| 0.989| 0.997| 0.993| 27902| 27954| 0.998 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|0 |9115 |0 |52 |0 |0 |0 |
|0 |0 |7924 |0 |0 |0 |0 |
|0 |24 |0 |6969 |0 |0 |0 |
|52 |0 |0 |0 |1280 |0 |0 |
|0 |0 |0 |0 |0 |1269 |0 |
|0 |24 |0 |0 |0 |0 |1245 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| FC-4894/route-data.js | 3 |
| FC-5871-Forms/route-data.js | 3 |
| FC-4806/route-data.js | 3 |
| FC-4831/route-data.js | 3 |
| FC-4847/route-data.js | 3 |
| FC-5390-Forms/route-data.js | 3 |
| FC-4773/route-data.js | 3 |
| FC-5365-Forms/route-data.js | 3 |
| FC-RoutingPOC-Forms/route-data.js | 3 |
| FC-5875-Forms/route-data.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 7924}, "macro avg": {"f1-score": 0.9965957958283006, "precision": 0.9978925314347619, "recall": 0.9953304910029889, "support": 27902}, "micro avg": {"f1-score": 0.9964160275249087, "precision": 0.9964160275249087, "recall": 0.9964160275249087, "support": 27902}, "weighted avg": {"f1-score": 0.996414251828852, "precision": 0.9964227089108423, "recall": 0.9964160275249087, "support": 27902}, "\u2205": {"f1-score": 0.994544462629569, "precision": 0.9947615409800283, "recall": 0.9943274790007636, "support": 9167}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1280}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1269}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9904534606205251, "precision": 1.0, "recall": 0.9810874704491725, "support": 1269}, "\u2423": {"f1-score": 0.9945768517197089, "precision": 0.992593647628543, "recall": 0.9965679965679966, "support": 6993}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 7924}, "macro avg": {"f1-score": 0.9932777764306486, "precision": 0.9978925314347619, "recall": 0.9888239844964825, "support": 27954}, "micro avg": {"f1-score": 0.9954883987396161, "precision": 0.9964160275249087, "recall": 0.9945624955283681, "support": 27954}, "weighted avg": {"f1-score": 0.9954723060564712, "precision": 0.9964293633837848, "recall": 0.9945624955283681, "support": 27954}, "\u2205": {"f1-score": 0.994544462629569, "precision": 0.9947615409800283, "recall": 0.9943274790007636, "support": 9167}, "\u23ce": {"f1-score": 0.9800918836140887, "precision": 1.0, "recall": 0.960960960960961, "support": 1332}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1269}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9904534606205251, "precision": 1.0, "recall": 0.9810874704491725, "support": 1269}, "\u2423": {"f1-score": 0.9945768517197089, "precision": 0.992593647628543, "recall": 0.9965679965679966, "support": 6993}},
  "ppcr": 0.9981398011018101
}
```
</details>
