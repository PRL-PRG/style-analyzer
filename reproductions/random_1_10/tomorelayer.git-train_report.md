# Train report for javascript / file:///tmp/top-repos-quality-repos-nf7bqfvp/tomorelayer.git HEAD 09c237ae1d8207b6310cdcd2a5c0002f1e839898

### Classification report

PPCR: 0.556

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 1.000| 0.916| 0.976| 0.935| 6869| 7500| 0.916 |
| `␣` | 1.000| 0.820| 0.195| 0.901| 0.326| 1031| 4339| 0.238 |
| `⏎␣⁻␣⁻` | 0.927| 0.818| 0.764| 0.869| 0.838| 468| 501| 0.934 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 58| 1138| 0.051 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 546| 0.055 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 797| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 283| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 113| 0.000 |
| `weighted avg` | 0.948| 0.957| 0.532| 0.951| 0.581| 8456| 15217| 0.556 |
| `micro avg` | 0.957| 0.957| 0.532| 0.957| 0.684| 8456| 15217| 0.556 |
| `macro avg` | 0.360| 0.330| 0.234| 0.343| 0.262| 8456| 15217| 0.556 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|631 |6868 |0 |0 |0 |0 |1 |0 |0 |
|3308 |157 |845 |0 |0 |0 |29 |0 |0 |
|1080 |58 |0 |0 |0 |0 |0 |0 |0 |
|797 |0 |0 |0 |0 |0 |0 |0 |0 |
|516 |30 |0 |0 |0 |0 |0 |0 |0 |
|33 |85 |0 |0 |0 |0 |383 |0 |0 |
|283 |0 |0 |0 |0 |0 |0 |0 |0 |
|113 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/relayer-contract.test.js | 65 |
| src/service/backend.js | 63 |
| src/component/route/Dashboard/actions.js | 58 |
| src/component/route/Dashboard/ConfigForms/forms.js | 26 |
| src/service/relayer_contract.js | 25 |
| src/component/route/Register/forms.js | 17 |
| src/service/wallet.js | 12 |
| src/__tests__/volume_chart_calculation.spec.js | 12 |
| src/service/frontend.js | 11 |
| src/component/shared/actions.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.34334864321124103, "precision": 0.3601893383082602, "recall": 0.3297278894116983, "support": 8456}, "micro avg": {"f1-score": 0.9574266792809839, "precision": 0.9574266792809839, "recall": 0.9574266792809839, "support": 8456}, "weighted avg": {"f1-score": 0.9511660800237474, "precision": 0.9483311493735721, "recall": 0.9574266792809839, "support": 8456}, "\u2205": {"f1-score": 0.9764697519016136, "precision": 0.9541539316476799, "recall": 0.999854418401514, "support": 6869}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8694665153234961, "precision": 0.927360774818402, "recall": 0.8183760683760684, "support": 468}, "\u2423": {"f1-score": 0.9008528784648189, "precision": 1.0, "recall": 0.8195926285160039, "support": 1031}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 113}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 797}, "macro avg": {"f1-score": 0.2623283003611715, "precision": 0.3601893383082602, "recall": 0.23436871553045097, "support": 15217}, "micro avg": {"f1-score": 0.6839859755839987, "precision": 0.9574266792809839, "recall": 0.5320365380824078, "support": 15217}, "weighted avg": {"f1-score": 0.5811605146973471, "precision": 0.7859474426984043, "recall": 0.5320365380824078, "support": 15217}, "\u2205": {"f1-score": 0.9345489182201661, "precision": 0.9541539316476799, "recall": 0.9157333333333333, "support": 7500}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1138}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 283}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 546}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8380743982494528, "precision": 0.927360774818402, "recall": 0.7644710578842315, "support": 501}, "\u2423": {"f1-score": 0.32600308641975306, "precision": 1.0, "recall": 0.19474533302604286, "support": 4339}},
  "ppcr": 0.5556942892817244
}
```
</details>
