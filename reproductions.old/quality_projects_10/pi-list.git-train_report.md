# Train report for javascript / file:///tmp/top-repos-quality-repos-waamnoay/pi-list.git HEAD 0d9d9e3d5b482f03ef34a8ec02f4e20cb2deb998

### Classification report

PPCR: 0.818

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 0.995| 0.940| 0.987| 0.959| 68550| 72583| 0.944 |
| `␣` | 0.969| 0.982| 0.823| 0.975| 0.890| 28340| 33813| 0.838 |
| `'` | 0.981| 0.993| 0.948| 0.987| 0.964| 8634| 9044| 0.955 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.939| 0.693| 0.328| 0.798| 0.486| 2078| 4390| 0.473 |
| `"` | 0.968| 0.912| 0.887| 0.939| 0.926| 1926| 1979| 0.973 |
| `⏎` | 0.986| 0.605| 0.131| 0.750| 0.232| 1622| 7473| 0.217 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 249| 3947| 0.063 |
| `⏎⏎` | 0.962| 0.829| 0.068| 0.890| 0.128| 245| 2968| 0.083 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 164| 0.104 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 233| 0.034 |
| `macro avg` | 0.678| 0.601| 0.413| 0.633| 0.458| 111669| 136594| 0.818 |
| `micro avg` | 0.976| 0.976| 0.798| 0.976| 0.878| 111669| 136594| 0.818 |
| `weighted avg` | 0.973| 0.976| 0.798| 0.973| 0.838| 111669| 136594| 0.818 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4033 |68193 |351 |0 |4 |2 |0 |0 |0 |0 |0 |
|5473 |430 |27816 |0 |3 |91 |0 |0 |0 |0 |0 |
|410 |0 |0 |8576 |0 |0 |0 |0 |58 |0 |0 |
|5851 |432 |200 |0 |982 |0 |0 |8 |0 |0 |0 |
|2312 |366 |272 |0 |0 |1440 |0 |0 |0 |0 |0 |
|3698 |221 |28 |0 |0 |0 |0 |0 |0 |0 |0 |
|2723 |18 |17 |0 |7 |0 |0 |203 |0 |0 |0 |
|53 |0 |0 |170 |0 |0 |0 |0 |1756 |0 |0 |
|225 |6 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|147 |0 |17 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| apps/gui/app/components/LineChart.js | 72 |
| apps/listwebserver/api/pcap.js | 59 |
| apps/gui/app/headerRoutes/index.js | 49 |
| apps/gui/app/utils/api.js | 47 |
| apps/gui/app/components/common/PopUp.js | 44 |
| apps/listwebserver/util/analysis/index.js | 42 |
| apps/capture_probe/server.js | 39 |
| apps/listwebserver/controllers/workflow/index.js | 38 |
| apps/gui/app/pages/LiveStreamList.js | 36 |
| apps/gui/app/pages/simulator/VrxPage/generator.js | 36 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9390374331550801, "precision": 0.9680264608599779, "recall": 0.9117341640706127, "support": 1926}, "\u0027": {"f1-score": 0.9868814729574222, "precision": 0.9805625428767436, "recall": 0.9932823720176048, "support": 8634}, "macro avg": {"f1-score": 0.6326047899189826, "precision": 0.6783906288758162, "recall": 0.6008289734298641, "support": 111669}, "micro avg": {"f1-score": 0.9757945356365688, "precision": 0.9757945356365688, "recall": 0.9757945356365688, "support": 111669}, "weighted avg": {"f1-score": 0.9734391812478719, "precision": 0.9732536257598526, "recall": 0.9757945356365688, "support": 111669}, "\u2205": {"f1-score": 0.986759854141344, "precision": 0.9788562569976746, "recall": 0.9947921225382932, "support": 68550}, "\u23ce": {"f1-score": 0.7501909854851032, "precision": 0.9859437751004017, "recall": 0.6054254007398274, "support": 1622}, "\u23ce\u23ce": {"f1-score": 0.8903508771929824, "precision": 0.9620853080568721, "recall": 0.8285714285714286, "support": 245}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.7975630019385211, "precision": 0.9393346379647749, "recall": 0.6929740134744947, "support": 2078}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 249}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.9752642743193731, "precision": 0.9690973069017176, "recall": 0.9815102328863797, "support": 28340}},
  "cl_report_full": {"\"": {"f1-score": 0.925916161349855, "precision": 0.9680264608599779, "recall": 0.8873168266801414, "support": 1979}, "\u0027": {"f1-score": 0.9641371557054526, "precision": 0.9805625428767436, "recall": 0.9482529854046882, "support": 9044}, "macro avg": {"f1-score": 0.45845788398339254, "precision": 0.6783906288758162, "recall": 0.4125550355014428, "support": 136594}, "micro avg": {"f1-score": 0.877827142989491, "precision": 0.9757945356365688, "recall": 0.7977363573802656, "support": 136594}, "weighted avg": {"f1-score": 0.8381023612497656, "precision": 0.9440198620001803, "recall": 0.7977363573802656, "support": 136594}, "\u2205": {"f1-score": 0.9587835415363201, "precision": 0.9788562569976746, "recall": 0.9395175178760867, "support": 72583}, "\u23ce": {"f1-score": 0.2319045932223403, "precision": 0.9859437751004017, "recall": 0.13140639636023016, "support": 7473}, "\u23ce\u23ce": {"f1-score": 0.1277131173324945, "precision": 0.9620853080568721, "recall": 0.06839622641509434, "support": 2968}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.4862400810400134, "precision": 0.9393346379647749, "recall": 0.32801822323462415, "support": 4390}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3947}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 233}, "\u2423": {"f1-score": 0.8898841896474503, "precision": 0.9690973069017176, "recall": 0.8226421790435632, "support": 33813}},
  "ppcr": 0.8175249278884871
}
```
</details>
