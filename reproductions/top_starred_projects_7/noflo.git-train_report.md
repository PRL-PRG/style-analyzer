# Train report for javascript / file:///tmp/top-repos-quality-repos-ogwqqd8r/noflo.git HEAD 6187566f761a463af95dd186de55dc488e2f03b8

### Classification report

PPCR: 0.859

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.996| 0.987| 0.993| 0.989| 59211| 59765| 0.991 |
| `␣` | 0.960| 0.964| 0.841| 0.962| 0.896| 16838| 19309| 0.872 |
| `'` | 1.000| 1.000| 0.995| 1.000| 0.998| 8537| 8578| 0.995 |
| `⏎` | 0.959| 0.874| 0.204| 0.914| 0.336| 1621| 6956| 0.233 |
| `⏎␣⁻␣⁻` | 0.920| 0.950| 0.373| 0.935| 0.530| 1311| 3344| 0.392 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 261| 3479| 0.075 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 71| 0.394 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 781| 0.031 |
| `macro avg` | 0.604| 0.598| 0.425| 0.601| 0.469| 87831| 102283| 0.859 |
| `weighted avg` | 0.980| 0.984| 0.845| 0.982| 0.871| 87831| 102283| 0.859 |
| `micro avg` | 0.984| 0.984| 0.845| 0.984| 0.909| 87831| 102283| 0.859 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|554 |58975 |236 |0 |0 |0 |0 |0 |0 |
|2471 |506 |16239 |0 |37 |0 |56 |0 |0 |
|41 |0 |0 |8537 |0 |0 |0 |0 |0 |
|5335 |33 |147 |0 |1417 |0 |24 |0 |0 |
|3218 |27 |233 |0 |1 |0 |0 |0 |0 |
|2033 |1 |64 |0 |0 |0 |1246 |0 |0 |
|757 |0 |1 |0 |23 |0 |0 |0 |0 |
|43 |0 |0 |0 |0 |0 |28 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| spec/Component.js | 344 |
| spec/Scoping.js | 170 |
| spec/NetworkLifecycle.js | 95 |
| spec/AsCallback.js | 82 |
| spec/Network.js | 82 |
| src/lib/Component.js | 60 |
| spec/LegacyNetwork.js | 54 |
| src/lib/BaseNetwork.js | 54 |
| spec/Subgraph.js | 38 |
| src/lib/InternalSocket.js | 36 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 8537}, "macro avg": {"f1-score": 0.6006117624770763, "precision": 0.6036491788486275, "recall": 0.5981264053907518, "support": 87831}, "micro avg": {"f1-score": 0.9838667440880783, "precision": 0.9838667440880783, "recall": 0.9838667440880783, "support": 87831}, "weighted avg": {"f1-score": 0.9820616332543637, "precision": 0.9803483998590711, "recall": 0.9838667440880783, "support": 87831}, "\u2205": {"f1-score": 0.9932380655646593, "precision": 0.9904773101340231, "recall": 0.9960142541081893, "support": 59211}, "\u23ce": {"f1-score": 0.914488544691836, "precision": 0.9587280108254398, "recall": 0.8741517581739667, "support": 1621}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 261}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9350844277673547, "precision": 0.9202363367799113, "recall": 0.950419527078566, "support": 1311}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.9620830617927603, "precision": 0.9597517730496454, "recall": 0.9644257037652928, "support": 16838}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9976044405492258, "precision": 1.0, "recall": 0.9952203310795057, "support": 8578}, "macro avg": {"f1-score": 0.46864424164804336, "precision": 0.6036491788486275, "recall": 0.4249156700343041, "support": 102283}, "micro avg": {"f1-score": 0.9090756072672186, "precision": 0.9838667440880783, "recall": 0.8448520281962789, "support": 102283}, "weighted avg": {"f1-score": 0.8707571793419864, "precision": 0.939079864488422, "recall": 0.8448520281962789, "support": 102283}, "\u2205": {"f1-score": 0.988625981711048, "precision": 0.9904773101340231, "recall": 0.9867815611143645, "support": 59765}, "\u23ce": {"f1-score": 0.3360208679155798, "precision": 0.9587280108254398, "recall": 0.20370902817711328, "support": 6956}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 781}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3479}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5304384844614729, "precision": 0.9202363367799113, "recall": 0.37260765550239233, "support": 3344}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 71}, "\u2423": {"f1-score": 0.8964641585470204, "precision": 0.9597517730496454, "recall": 0.8410067844010565, "support": 19309}},
  "ppcr": 0.8587057477782232
}
```
</details>
