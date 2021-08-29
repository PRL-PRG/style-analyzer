# Train report for javascript / file:///tmp/top-repos-quality-repos-a0qleu_g/pipedream.git HEAD 2ad7b3c7c44cb0262e31151a182044fe578026fc

### Classification report

PPCR: 0.894

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 0.998| 0.948| 0.988| 0.963| 75728| 79735| 0.950 |
| `␣` | 0.989| 0.972| 0.887| 0.981| 0.935| 38180| 41857| 0.912 |
| `"` | 0.973| 1.000| 0.839| 0.986| 0.901| 12663| 15098| 0.839 |
| `⏎` | 0.949| 0.974| 0.662| 0.961| 0.780| 10465| 15387| 0.680 |
| `⏎␣⁻␣⁻` | 0.986| 0.940| 0.858| 0.962| 0.918| 7090| 7767| 0.913 |
| `⏎␣⁺␣⁺` | 0.975| 0.900| 0.779| 0.936| 0.866| 6907| 7982| 0.865 |
| `⏎⏎` | 1.000| 0.767| 0.337| 0.868| 0.504| 601| 1369| 0.439 |
| `'` | 1.000| 0.011| 0.005| 0.022| 0.010| 360| 808| 0.446 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 85| 94| 0.904 |
| `weighted avg` | 0.979| 0.979| 0.875| 0.978| 0.919| 152079| 170097| 0.894 |
| `micro avg` | 0.979| 0.979| 0.875| 0.979| 0.924| 152079| 170097| 0.894 |
| `macro avg` | 0.872| 0.729| 0.590| 0.745| 0.653| 152079| 170097| 0.894 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4007 |75573 |143 |0 |0 |10 |2 |0 |0 |0 |
|3677 |458 |37130 |429 |0 |143 |20 |0 |0 |0 |
|4922 |196 |71 |10192 |0 |4 |2 |0 |0 |0 |
|2435 |0 |0 |0 |12663 |0 |0 |0 |0 |0 |
|1075 |579 |110 |4 |0 |6214 |0 |0 |0 |0 |
|677 |283 |78 |66 |0 |0 |6663 |0 |0 |0 |
|768 |88 |0 |52 |0 |0 |0 |461 |0 |0 |
|448 |0 |0 |0 |356 |0 |0 |0 |4 |0 |
|9 |14 |2 |0 |0 |0 |69 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| components/shopify/shopify.app.js | 93 |
| docs/docs/.vuepress/theme/util/index.js | 84 |
| components/google_drive/google_drive.app.js | 63 |
| components/google_sheets/sources/new-updates/new-updates.js | 53 |
| components/google_sheets/sources/new-row-added/new-row-added.js | 41 |
| components/jotform/jotform.app.js | 38 |
| components/procore/procore.app.js | 37 |
| components/trello/events.js | 35 |
| components/slack/actions/send-custom-message/send-custom-message.js | 31 |
| components/aws/sources/new-scheduled-tasks/new-scheduled-tasks.js | 31 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9861381512343276, "precision": 0.9726553498732622, "recall": 1.0, "support": 12663}, "\u0027": {"f1-score": 0.02197802197802198, "precision": 1.0, "recall": 0.011111111111111112, "support": 360}, "macro avg": {"f1-score": 0.7450073678776036, "precision": 0.8723592351257117, "recall": 0.7291080321054683, "support": 152079}, "micro avg": {"f1-score": 0.97909639069168, "precision": 0.97909639069168, "recall": 0.97909639069168, "support": 152079}, "weighted avg": {"f1-score": 0.9775241987706738, "precision": 0.9787340754960794, "recall": 0.97909639069168, "support": 152079}, "\u2205": {"f1-score": 0.9884056265081513, "precision": 0.9790390071381379, "recall": 0.9979532009296429, "support": 75728}, "\u23ce": {"f1-score": 0.961146737080347, "precision": 0.9487107884203667, "recall": 0.9739130434782609, "support": 10465}, "\u23ce\u23ce": {"f1-score": 0.8681732580037664, "precision": 1.0, "recall": 0.7670549084858569, "support": 601}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9359843349902094, "precision": 0.9753570867995605, "recall": 0.8996670044882004, "support": 6907}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9624440271558573, "precision": 0.9862344582593251, "recall": 0.9397743300423131, "support": 7090}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 85}, "\u2423": {"f1-score": 0.9807961539477508, "precision": 0.9892364256407524, "recall": 0.9724986904138292, "support": 38180}},
  "cl_report_full": {"\"": {"f1-score": 0.9007362094106768, "precision": 0.9726553498732622, "recall": 0.8387203603126242, "support": 15098}, "\u0027": {"f1-score": 0.009852216748768473, "precision": 1.0, "recall": 0.0049504950495049506, "support": 808}, "macro avg": {"f1-score": 0.6529457373136641, "precision": 0.8723592351257117, "recall": 0.59044690101751, "support": 170097}, "micro avg": {"f1-score": 0.924339491458085, "precision": 0.97909639069168, "recall": 0.8753828697743052, "support": 170097}, "weighted avg": {"f1-score": 0.9188207884990597, "precision": 0.9781212542264083, "recall": 0.8753828697743052, "support": 170097}, "\u2205": {"f1-score": 0.9631673527649974, "precision": 0.9790390071381379, "recall": 0.9478020944378253, "support": 79735}, "\u23ce": {"f1-score": 0.7800995024875622, "precision": 0.9487107884203667, "recall": 0.6623773315136154, "support": 15387}, "\u23ce\u23ce": {"f1-score": 0.5038251366120218, "precision": 1.0, "recall": 0.33674214755295834, "support": 1369}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8658816972061589, "precision": 0.9753570867995605, "recall": 0.7785016286644951, "support": 7982}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9175790126007024, "precision": 0.9862344582593251, "recall": 0.857860177674778, "support": 7767}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u2423": {"f1-score": 0.9353705079920898, "precision": 0.9892364256407524, "recall": 0.8870678739517882, "support": 41857}},
  "ppcr": 0.8940722058590098
}
```
</details>
