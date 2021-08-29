# Train report for javascript / file:///tmp/top-repos-quality-repos-g97dacqi/asar.git HEAD b0415c8ef29c22aeedfc5b4e179ef8969d5c43a6

### Classification report

PPCR: 0.362

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.967| 0.999| 0.529| 0.983| 0.684| 1640| 3098| 0.529 |
| `'` | 1.000| 1.000| 0.443| 1.000| 0.614| 264| 596| 0.443 |
| `␣` | 0.994| 0.981| 0.085| 0.987| 0.157| 160| 1837| 0.087 |
| `⏎` | 0.986| 0.902| 0.439| 0.942| 0.608| 153| 314| 0.487 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 227| 0.167 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 155| 0.006 |
| `weighted avg` | 0.957| 0.974| 0.353| 0.965| 0.476| 2256| 6227| 0.362 |
| `macro avg` | 0.658| 0.647| 0.249| 0.652| 0.344| 2256| 6227| 0.362 |
| `micro avg` | 0.974| 0.974| 0.353| 0.974| 0.518| 2256| 6227| 0.362 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1458 |1638 |0 |0 |2 |0 |0 |
|1677 |3 |157 |0 |0 |0 |0 |
|332 |0 |0 |264 |0 |0 |0 |
|161 |15 |0 |0 |138 |0 |0 |
|189 |37 |1 |0 |0 |0 |0 |
|154 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| bin/asar.js | 20 |
| test/cli-spec.js | 14 |
| test/api-spec.js | 11 |
| lib/asar.js | 4 |
| lib/crawlfs.js | 3 |
| lib/filesystem.js | 3 |
| lib/disk.js | 2 |
| test/util/compareDirectories.js | 1 |
| test/util/compareFiles.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 264}, "macro avg": {"f1-score": 0.6520007308560397, "precision": 0.6577212200917609, "recall": 0.6469985453531005, "support": 2256}, "micro avg": {"f1-score": 0.9738475177304965, "precision": 0.9738475177304965, "recall": 0.9738475177304965, "support": 2256}, "weighted avg": {"f1-score": 0.9652393591917705, "precision": 0.9572636309633775, "recall": 0.9738475177304965, "support": 2256}, "\u2205": {"f1-score": 0.9826034793041393, "precision": 0.9669421487603306, "recall": 0.998780487804878, "support": 1640}, "\u23ce": {"f1-score": 0.9419795221843005, "precision": 0.9857142857142858, "recall": 0.9019607843137255, "support": 153}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9874213836477987, "precision": 0.9936708860759493, "recall": 0.98125, "support": 160}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6139534883720931, "precision": 1.0, "recall": 0.4429530201342282, "support": 596}, "macro avg": {"f1-score": 0.34381931441636643, "precision": 0.6577212200917609, "recall": 0.2494395184190731, "support": 6227}, "micro avg": {"f1-score": 0.5179771307320524, "precision": 0.9738475177304965, "recall": 0.35281837160751567, "support": 6227}, "weighted avg": {"f1-score": 0.4759680158313698, "precision": 0.9196201188847452, "recall": 0.35281837160751567, "support": 6227}, "\u2205": {"f1-score": 0.6836393989983305, "precision": 0.9669421487603306, "recall": 0.5287282117495158, "support": 3098}, "\u23ce": {"f1-score": 0.6079295154185022, "precision": 0.9857142857142858, "recall": 0.4394904458598726, "support": 314}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 227}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u2423": {"f1-score": 0.1573934837092732, "precision": 0.9936708860759493, "recall": 0.085465432770822, "support": 1837}},
  "ppcr": 0.3622932391199615
}
```
</details>
