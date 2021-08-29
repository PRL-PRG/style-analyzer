# Train report for javascript / file:///tmp/top-repos-quality-repos-11ksql4x/js-ipfs-http-client.git HEAD 995abb41b83c8345b16cba67151e9ccb9cbea4de

### Classification report

PPCR: 0.784

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.996| 0.915| 0.985| 0.943| 20513| 22342| 0.918 |
| `␣` | 0.984| 0.959| 0.721| 0.971| 0.833| 8973| 11928| 0.752 |
| `'` | 1.000| 1.000| 0.892| 1.000| 0.943| 3750| 4206| 0.892 |
| `⏎␣⁺␣⁺` | 0.900| 0.952| 0.933| 0.925| 0.916| 1684| 1719| 0.980 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 304| 2176| 0.140 |
| `⏎⏎` | 0.922| 0.971| 0.157| 0.946| 0.268| 207| 1280| 0.162 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 62| 1645| 0.038 |
| `weighted avg` | 0.965| 0.975| 0.764| 0.970| 0.814| 35493| 45296| 0.784 |
| `macro avg` | 0.683| 0.697| 0.517| 0.690| 0.558| 35493| 45296| 0.784 |
| `micro avg` | 0.975| 0.975| 0.764| 0.975| 0.857| 35493| 45296| 0.784 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1829 |20440 |65 |0 |0 |0 |8 |0 |
|2955 |207 |8604 |0 |0 |1 |161 |0 |
|456 |0 |0 |3750 |0 |0 |0 |0 |
|1872 |235 |48 |0 |0 |16 |5 |0 |
|1073 |6 |0 |0 |0 |201 |0 |0 |
|35 |58 |23 |0 |0 |0 |1603 |0 |
|1583 |58 |0 |0 |0 |0 |4 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/files-mfs.spec.js | 84 |
| test/ping.spec.js | 42 |
| test/interface.spec.js | 40 |
| examples/upload-file-via-browser/src/App.js | 35 |
| test/node/swarm.js | 34 |
| examples/name-api/index.js | 22 |
| src/utils/send-request.js | 15 |
| test/request-api.spec.js | 15 |
| test/get.spec.js | 15 |
| examples/bundle-webpack/src/App.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3750}, "macro avg": {"f1-score": 0.6896116617048159, "precision": 0.6828088325625444, "recall": 0.6968903773302533, "support": 35493}, "micro avg": {"f1-score": 0.9747837601780632, "precision": 0.9747837601780632, "recall": 0.9747837601780632, "support": 35493}, "weighted avg": {"f1-score": 0.9697508861777051, "precision": 0.9650384883874277, "recall": 0.9747837601780632, "support": 35493}, "\u2205": {"f1-score": 0.9846568875400439, "precision": 0.9731479718148924, "recall": 0.99644128113879, "support": 20513}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 304}, "\u23ce\u23ce": {"f1-score": 0.9458823529411765, "precision": 0.9220183486238532, "recall": 0.9710144927536232, "support": 207}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9252525252525252, "precision": 0.9000561482313307, "recall": 0.9519002375296912, "support": 1684}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u2423": {"f1-score": 0.971489866199966, "precision": 0.9844393592677345, "recall": 0.958876629889669, "support": 8973}},
  "cl_report_full": {"\u0027": {"f1-score": 0.942684766214178, "precision": 1.0, "recall": 0.891583452211127, "support": 4206}, "macro avg": {"f1-score": 0.5575347085326823, "precision": 0.6828088325625444, "recall": 0.5167614904600544, "support": 45296}, "micro avg": {"f1-score": 0.8565027417098863, "precision": 0.9747837601780632, "recall": 0.7638202048746026, "support": 45296}, "weighted avg": {"f1-score": 0.8143130688715488, "precision": 0.8923049423499217, "recall": 0.7638202048746026, "support": 45296}, "\u2205": {"f1-score": 0.943108937387533, "precision": 0.9731479718148924, "recall": 0.9148688568615164, "support": 22342}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2176}, "\u23ce\u23ce": {"f1-score": 0.2683578104138852, "precision": 0.9220183486238532, "recall": 0.15703125, "support": 1280}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.916, "precision": 0.9000561482313307, "recall": 0.9325189063408958, "support": 1719}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1645}, "\u2423": {"f1-score": 0.8325914457131797, "precision": 0.9844393592677345, "recall": 0.721327967806841, "support": 11928}},
  "ppcr": 0.7835791239844578
}
```
</details>
