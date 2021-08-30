# Train report for javascript / file:///tmp/top-repos-quality-repos-uau4cre6/ackee.git HEAD 9a6e51d337ab7c9affdc6752e1acbe9405d0f633

### Classification report

PPCR: 0.892

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.993| 0.958| 0.989| 0.971| 26738| 27707| 0.965 |
| `␣` | 0.969| 0.979| 0.908| 0.974| 0.938| 18903| 20370| 0.928 |
| `'` | 1.000| 1.000| 0.973| 1.000| 0.986| 6036| 6203| 0.973 |
| `⏎` | 0.955| 0.949| 0.511| 0.952| 0.666| 1837| 3415| 0.538 |
| `⏎⇥⁺` | 0.890| 0.790| 0.715| 0.837| 0.793| 1430| 1581| 0.904 |
| `⏎⇥⁻` | 0.892| 0.693| 0.355| 0.780| 0.508| 783| 1530| 0.512 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 1762| 0.037 |
| `weighted avg` | 0.975| 0.977| 0.871| 0.976| 0.902| 55793| 62568| 0.892 |
| `macro avg` | 0.813| 0.772| 0.631| 0.790| 0.695| 55793| 62568| 0.892 |
| `micro avg` | 0.977| 0.977| 0.871| 0.977| 0.921| 55793| 62568| 0.892 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|969 |26546 |184 |0 |8 |0 |0 |0 |
|1467 |120 |18504 |0 |74 |139 |66 |0 |
|167 |0 |0 |6036 |0 |0 |0 |0 |
|1578 |6 |87 |0 |1744 |0 |0 |0 |
|151 |52 |248 |0 |0 |1130 |0 |0 |
|747 |228 |12 |0 |0 |0 |543 |0 |
|1696 |0 |66 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/ui/scripts/components/Filter.js | 65 |
| src/ui/scripts/components/routes/RouteSettings.js | 62 |
| src/ui/scripts/constants/route.js | 40 |
| src/database/actions.js | 29 |
| src/ui/scripts/components/Header.js | 29 |
| src/ui/scripts/components/presentations/PresentationCounterList.js | 22 |
| src/ui/scripts/components/presentations/PresentationIconList.js | 20 |
| src/aggregations/aggregateTopRecords.js | 19 |
| src/ui/scripts/components/Modals.js | 18 |
| src/database/durations.js | 18 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6036}, "macro avg": {"f1-score": 0.7903426785005279, "precision": 0.8129807033109521, "recall": 0.7721116867866062, "support": 55793}, "micro avg": {"f1-score": 0.9768788199236463, "precision": 0.9768788199236463, "recall": 0.9768788199236463, "support": 55793}, "weighted avg": {"f1-score": 0.9757730364134678, "precision": 0.9752016536865628, "recall": 0.9768788199236463, "support": 55793}, "\u2205": {"f1-score": 0.988861985472155, "precision": 0.9849361828435738, "recall": 0.9928192086169496, "support": 26738}, "\u23ce": {"f1-score": 0.9522249522249523, "precision": 0.9550930996714129, "recall": 0.9493739793140991, "support": 1837}, "\u23ce\u21e5\u207a": {"f1-score": 0.8373471656168952, "precision": 0.8904649330181245, "recall": 0.7902097902097902, "support": 1430}, "\u23ce\u21e5\u207b": {"f1-score": 0.7801724137931033, "precision": 0.8916256157635468, "recall": 0.6934865900383141, "support": 783}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u2423": {"f1-score": 0.9737922323965897, "precision": 0.9687450918800062, "recall": 0.9788922393270909, "support": 18903}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9863550943704551, "precision": 1.0, "recall": 0.9730775431242947, "support": 6203}, "macro avg": {"f1-score": 0.6945006263589516, "precision": 0.8129807033109521, "recall": 0.6314138688861821, "support": 62568}, "micro avg": {"f1-score": 0.9209621412458495, "precision": 0.9768788199236463, "recall": 0.871100242935686, "support": 62568}, "weighted avg": {"f1-score": 0.901950036656281, "precision": 0.9471234421148251, "recall": 0.871100242935686, "support": 62568}, "\u2205": {"f1-score": 0.9713313452496387, "precision": 0.9849361828435738, "recall": 0.9580972317464901, "support": 27707}, "\u23ce": {"f1-score": 0.6655218469757679, "precision": 0.9550930996714129, "recall": 0.5106881405563689, "support": 3415}, "\u23ce\u21e5\u207a": {"f1-score": 0.7929824561403509, "precision": 0.8904649330181245, "recall": 0.7147375079063883, "support": 1581}, "\u23ce\u21e5\u207b": {"f1-score": 0.5077138849929874, "precision": 0.8916256157635468, "recall": 0.35490196078431374, "support": 1530}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1762}, "\u2423": {"f1-score": 0.9375997567834612, "precision": 0.9687450918800062, "recall": 0.9083946980854197, "support": 20370}},
  "ppcr": 0.8917178110216085
}
```
</details>
