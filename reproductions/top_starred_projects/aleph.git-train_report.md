# Train report for javascript / file:///tmp/top-repos-quality-repos-pdekxrhf/aleph.git HEAD 47ac45fa72607e1ab16c7c30690013a7d00be116

### Classification report

PPCR: 0.426

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.999| 0.633| 0.996| 0.773| 5009| 7905| 0.634 |
| `␣` | 0.995| 0.990| 0.162| 0.993| 0.279| 888| 5420| 0.164 |
| `'` | 1.000| 1.000| 0.907| 1.000| 0.951| 761| 839| 0.907 |
| `⏎` | 0.931| 0.946| 0.277| 0.939| 0.426| 242| 828| 0.292 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 369| 0.043 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 477| 0.021 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 436| 0.009 |
| `weighted avg` | 0.988| 0.992| 0.422| 0.990| 0.539| 6930| 16274| 0.426 |
| `micro avg` | 0.992| 0.992| 0.422| 0.992| 0.593| 6930| 16274| 0.426 |
| `macro avg` | 0.560| 0.562| 0.283| 0.561| 0.347| 6930| 16274| 0.426 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2896 |5006 |3 |0 |0 |0 |0 |0 |
|4532 |9 |879 |0 |0 |0 |0 |0 |
|586 |13 |0 |229 |0 |0 |0 |0 |
|78 |0 |0 |0 |761 |0 |0 |0 |
|467 |10 |0 |0 |0 |0 |0 |0 |
|432 |2 |1 |1 |0 |0 |0 |0 |
|353 |0 |0 |16 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| ui/src/components/MappingEditor/MappingList.js | 17 |
| ui/src/queries.js | 10 |
| ui/src/app/Query.js | 10 |
| ui/src/util/convertPathsToTree.js | 6 |
| ui/src/selectors.js | 4 |
| ui/src/actions/asyncActionCreator.js | 2 |
| ui/src/reducers/util.js | 2 |
| ui/src/actions/roleActions.js | 1 |
| ui/src/util/fetchCsvData.js | 1 |
| ui/src/app/error-toast-middleware.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 761}, "macro avg": {"f1-score": 0.5610717351655369, "precision": 0.559945466553147, "recall": 0.5622209906656993, "support": 6930}, "micro avg": {"f1-score": 0.9920634920634921, "precision": 0.9920634920634921, "recall": 0.9920634920634921, "support": 6930}, "weighted avg": {"f1-score": 0.989922318897681, "precision": 0.9878012842268007, "recall": 0.9920634920634921, "support": 6930}, "\u2205": {"f1-score": 0.9963180415961789, "precision": 0.9932539682539683, "recall": 0.999401078059493, "support": 5009}, "\u23ce": {"f1-score": 0.9385245901639343, "precision": 0.9308943089430894, "recall": 0.9462809917355371, "support": 242}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9926595143986449, "precision": 0.9954699886749717, "recall": 0.9898648648648649, "support": 888}},
  "cl_report_full": {"\u0027": {"f1-score": 0.95125, "precision": 1.0, "recall": 0.9070321811680572, "support": 839}, "macro avg": {"f1-score": 0.34714771981028375, "precision": 0.559945466553147, "recall": 0.28272134763927037, "support": 16274}, "micro avg": {"f1-score": 0.5925702465092225, "precision": 0.9920634920634921, "recall": 0.42245299250337964, "support": 16274}, "weighted avg": {"f1-score": 0.539316992998993, "precision": 0.9129224803656657, "recall": 0.42245299250337964, "support": 16274}, "\u2205": {"f1-score": 0.7734260332174585, "precision": 0.9932539682539683, "recall": 0.633270082226439, "support": 7905}, "\u23ce": {"f1-score": 0.4264432029795158, "precision": 0.9308943089430894, "recall": 0.27657004830917875, "support": 828}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 369}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 477}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 436}, "\u2423": {"f1-score": 0.2789148024750119, "precision": 0.9954699886749717, "recall": 0.1621771217712177, "support": 5420}},
  "ppcr": 0.42583261644340664
}
```
</details>
