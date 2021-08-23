# Train report for javascript / file:///tmp/top-repos-quality-repos-7i878ydz/namoxpanel.git HEAD 9ae8e2bc9911358184a0cddf62d27e5a951003aa

### Classification report

PPCR: 0.662

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.994| 0.969| 0.983| 0.970| 9397| 9638| 0.975 |
| `␣` | 0.979| 0.941| 0.547| 0.960| 0.702| 3028| 5202| 0.582 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.966| 0.933| 0.899| 0.949| 0.931| 551| 572| 0.963 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.978| 0.945| 0.919| 0.961| 0.948| 526| 541| 0.972 |
| `⏎` | 0.958| 0.784| 0.193| 0.863| 0.322| 320| 1299| 0.246 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 186| 0.038 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3167| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 293| 0.000 |
| `weighted avg` | 0.972| 0.973| 0.644| 0.972| 0.692| 13829| 20898| 0.662 |
| `macro avg` | 0.607| 0.575| 0.441| 0.589| 0.484| 13829| 20898| 0.662 |
| `micro avg` | 0.973| 0.973| 0.644| 0.973| 0.775| 13829| 20898| 0.662 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|241 |9343 |42 |0 |0 |8 |4 |0 |0 |
|2174 |165 |2848 |0 |0 |8 |7 |0 |0 |
|3167 |0 |0 |0 |0 |0 |0 |0 |0 |
|979 |52 |15 |0 |251 |2 |0 |0 |0 |
|21 |27 |3 |0 |7 |514 |0 |0 |0 |
|15 |28 |0 |0 |1 |0 |497 |0 |0 |
|293 |0 |0 |0 |0 |0 |0 |0 |0 |
|179 |4 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| namoxpanel/static/mptt/draggable-admin.js | 91 |
| namoxpanel/static/admin/js/core.js | 53 |
| namoxpanel/static/admin/js/SelectFilter2.js | 39 |
| namoxpanel/static/admin/js/admin/DateTimeShortcuts.js | 33 |
| namoxpanel/static/admin/js/actions.js | 25 |
| namoxpanel/static/admin/js/SelectBox.js | 21 |
| namoxpanel/static/admin/js/inlines.js | 19 |
| namoxpanel/static/versatileimagefield/js/versatileimagefield.js | 19 |
| namoxpanel/static/admin/js/calendar.js | 15 |
| webpack.config.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5894110380931405, "precision": 0.6066501485966638, "recall": 0.5746124489703353, "support": 13829}, "micro avg": {"f1-score": 0.9728107599971075, "precision": 0.9728107599971075, "recall": 0.9728107599971075, "support": 13829}, "weighted avg": {"f1-score": 0.972173224650376, "precision": 0.9723353987288224, "recall": 0.9728107599971075, "support": 13829}, "\u2205": {"f1-score": 0.9826461926798487, "precision": 0.9713067886474686, "recall": 0.9942534851548367, "support": 9397}, "\u23ce": {"f1-score": 0.8625429553264604, "precision": 0.9580152671755725, "recall": 0.784375, "support": 320}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9492151431209602, "precision": 0.9661654135338346, "recall": 0.9328493647912885, "support": 551}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9613152804642167, "precision": 0.9783464566929134, "recall": 0.9448669201520913, "support": 526}, "\u2423": {"f1-score": 0.9595687331536388, "precision": 0.9793672627235214, "recall": 0.940554821664465, "support": 3028}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 293}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3167}, "macro avg": {"f1-score": 0.48412606128898805, "precision": 0.6066501485966638, "recall": 0.44092122697425773, "support": 20898}, "micro avg": {"f1-score": 0.7747861894203356, "precision": 0.9728107599971075, "recall": 0.6437458129964589, "support": 20898}, "weighted avg": {"f1-score": 0.6923536854881712, "precision": 0.8030681027536295, "recall": 0.6437458129964589, "support": 20898}, "\u2205": {"f1-score": 0.9703484447214, "precision": 0.9713067886474686, "recall": 0.9693919900394272, "support": 9638}, "\u23ce": {"f1-score": 0.3215887251761691, "precision": 0.9580152671755725, "recall": 0.19322555812163203, "support": 1299}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 186}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9311594202898551, "precision": 0.9661654135338346, "recall": 0.8986013986013986, "support": 572}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9475691134413728, "precision": 0.9783464566929134, "recall": 0.9186691312384473, "support": 541}, "\u2423": {"f1-score": 0.7023427866831072, "precision": 0.9793672627235214, "recall": 0.5474817377931565, "support": 5202}},
  "ppcr": 0.6617379653555364
}
```
</details>
