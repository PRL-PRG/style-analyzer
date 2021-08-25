# Train report for javascript / file:///tmp/top-repos-quality-repos-dtiwvh3g/api-sieg-down-v2.git HEAD 6a5fa6ff3c549b224d2cc33757d06184ddda6e5a

### Classification report

PPCR: 0.327

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 0.994| 0.425| 0.994| 0.596| 1179| 2756| 0.428 |
| `␣` | 0.982| 0.984| 0.250| 0.983| 0.399| 382| 1504| 0.254 |
| `'` | 1.000| 1.000| 0.553| 1.000| 0.712| 199| 360| 0.553 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 275| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 173| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 170| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 142| 0.000 |
| `weighted avg` | 0.993| 0.993| 0.325| 0.993| 0.464| 1760| 5380| 0.327 |
| `micro avg` | 0.993| 0.993| 0.325| 0.993| 0.489| 1760| 5380| 0.327 |
| `macro avg` | 0.425| 0.425| 0.175| 0.425| 0.244| 1760| 5380| 0.327 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1577 |1172 |7 |0 |0 |0 |0 |0 |
|1122 |6 |376 |0 |0 |0 |0 |0 |
|161 |0 |0 |199 |0 |0 |0 |0 |
|275 |0 |0 |0 |0 |0 |0 |0 |
|173 |0 |0 |0 |0 |0 |0 |0 |
|170 |0 |0 |0 |0 |0 |0 |0 |
|142 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| backend/src/util/functions.js | 3 |
| backend/src/services/LoopForTypeNFAndCNPJ.js | 2 |
| backend/src/services/CreateFolderToSaveXML.js | 2 |
| backend/src/services/LoopForCompetences.js | 2 |
| backend/src/controllers/LogSIEGController.js | 1 |
| backend/src/controllers/ProcessFrequencyController.js | 1 |
| backend/src/services/LoopForTypeEvents.js | 1 |
| backend/src/services/SaveNotes.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 199}, "macro avg": {"f1-score": 0.4253558643086707, "precision": 0.4252328369985859, "recall": 0.42547942268177275, "support": 1760}, "micro avg": {"f1-score": 0.9926136363636363, "precision": 0.9926136363636363, "recall": 0.9926136363636363, "support": 1760}, "weighted avg": {"f1-score": 0.9926168971529162, "precision": 0.9926211269226104, "recall": 0.9926136363636363, "support": 1760}, "\u2205": {"f1-score": 0.9944845142129826, "precision": 0.9949066213921901, "recall": 0.9940627650551315, "support": 1179}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9830065359477125, "precision": 0.9817232375979112, "recall": 0.9842931937172775, "support": 382}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7119856887298748, "precision": 1.0, "recall": 0.5527777777777778, "support": 360}, "macro avg": {"f1-score": 0.24376186671431674, "precision": 0.4252328369985859, "recall": 0.17543310986707214, "support": 5380}, "micro avg": {"f1-score": 0.4893557422969187, "precision": 0.9926136363636363, "recall": 0.32472118959107804, "support": 5380}, "weighted avg": {"f1-score": 0.4642739751118621, "precision": 0.8510175460788353, "recall": 0.32472118959107804, "support": 5380}, "\u2205": {"f1-score": 0.595831215048297, "precision": 0.9949066213921901, "recall": 0.42525399129172714, "support": 2756}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 275}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 173}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 170}, "\u2423": {"f1-score": 0.3985161632220456, "precision": 0.9817232375979112, "recall": 0.25, "support": 1504}},
  "ppcr": 0.3271375464684015
}
```
</details>
