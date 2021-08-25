# Train report for javascript / file:///tmp/top-repos-quality-repos-gspqo8he/nwa.git HEAD 4eff6032326d9aa0065ce04abc05148ce8a87fa9

### Classification report

PPCR: 0.800

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.999| 0.939| 0.992| 0.961| 13250| 14094| 0.940 |
| `"` | 0.968| 1.000| 1.000| 0.984| 0.984| 3404| 3404| 1.000 |
| `␣` | 0.974| 0.850| 0.227| 0.908| 0.368| 974| 3645| 0.267 |
| `⏎␣⁻␣⁻` | 0.966| 0.748| 0.619| 0.843| 0.754| 230| 278| 0.827 |
| `⏎` | 0.973| 1.000| 0.238| 0.986| 0.382| 181| 761| 0.238 |
| `'` | 1.000| 0.082| 0.082| 0.152| 0.152| 122| 122| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 146| 0.034 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 272| 0.015 |
| `macro avg` | 0.733| 0.585| 0.388| 0.608| 0.450| 18170| 22722| 0.800 |
| `micro avg` | 0.981| 0.981| 0.785| 0.981| 0.872| 18170| 22722| 0.800 |
| `weighted avg` | 0.981| 0.981| 0.785| 0.978| 0.826| 18170| 22722| 0.800 |

### Confusion matrix

|refusal|  ∅| "| ␣| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|844 |13232 |0 |18 |0 |0 |0 |0 |0 |
|0 |0 |3404 |0 |0 |0 |0 |0 |0 |
|2671 |140 |0 |828 |0 |6 |0 |0 |0 |
|580 |0 |0 |0 |181 |0 |0 |0 |0 |
|48 |56 |0 |2 |0 |172 |0 |0 |0 |
|268 |2 |0 |2 |0 |0 |0 |0 |0 |
|0 |0 |112 |0 |0 |0 |0 |10 |0 |
|141 |0 |0 |0 |5 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| site/frontend/src/Components/DeviceForm.js | 53 |
| site/frontend/src/Components/AlarmCPPir.js | 34 |
| site/frontend/src/Components/AlarmCPLidar.js | 34 |
| site/frontend/src/Components/AlarmSNLidar.js | 34 |
| site/frontend/src/Components/AlarmCPUltra.js | 34 |
| site/frontend/src/serviceWorker.js | 32 |
| site/backend/api/controllers/component_controller.js | 29 |
| site/backend/api/models/atomic_component.js | 20 |
| site/frontend/src/Components/ServerRasOs.js | 11 |
| site/frontend/src/Components/PartsTable.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9838150289017341, "precision": 0.9681456200227532, "recall": 1.0, "support": 3404}, "\u0027": {"f1-score": 0.1515151515151515, "precision": 1.0, "recall": 0.08196721311475409, "support": 122}, "macro avg": {"f1-score": 0.6080802802419161, "precision": 0.7333663211310103, "recall": 0.5848171848637195, "support": 18170}, "micro avg": {"f1-score": 0.9811227297743533, "precision": 0.9811227297743533, "recall": 0.9811227297743533, "support": 18170}, "weighted avg": {"f1-score": 0.9778131505121452, "precision": 0.9807041077955095, "recall": 0.9811227297743533, "support": 18170}, "\u2205": {"f1-score": 0.9919040479760121, "precision": 0.9852568875651526, "recall": 0.9986415094339622, "support": 13250}, "\u23ce": {"f1-score": 0.9863760217983651, "precision": 0.9731182795698925, "recall": 1.0, "support": 181}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8431372549019608, "precision": 0.9662921348314607, "recall": 0.7478260869565218, "support": 230}, "\u2423": {"f1-score": 0.9078947368421053, "precision": 0.9741176470588235, "recall": 0.8501026694045175, "support": 974}},
  "cl_report_full": {"\"": {"f1-score": 0.9838150289017341, "precision": 0.9681456200227532, "recall": 1.0, "support": 3404}, "\u0027": {"f1-score": 0.1515151515151515, "precision": 1.0, "recall": 0.08196721311475409, "support": 122}, "macro avg": {"f1-score": 0.4502341765653936, "precision": 0.7333663211310103, "recall": 0.388064613268068, "support": 22722}, "micro avg": {"f1-score": 0.8719064853761127, "precision": 0.9811227297743533, "recall": 0.7845700202446968, "support": 22722}, "weighted avg": {"f1-score": 0.8257229255297449, "precision": 0.962222045227795, "recall": 0.7845700202446968, "support": 22722}, "\u2205": {"f1-score": 0.9614881557913095, "precision": 0.9852568875651526, "recall": 0.9388392223641265, "support": 14094}, "\u23ce": {"f1-score": 0.382259767687434, "precision": 0.9731182795698925, "recall": 0.2378449408672799, "support": 761}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 272}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7543859649122807, "precision": 0.9662921348314607, "recall": 0.6187050359712231, "support": 278}, "\u2423": {"f1-score": 0.36840934371523915, "precision": 0.9741176470588235, "recall": 0.2271604938271605, "support": 3645}},
  "ppcr": 0.799665522401197
}
```
</details>
