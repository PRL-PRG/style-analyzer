# Train report for javascript / file:///tmp/top-repos-quality-repos-n9b4dd7w/booking-server.git HEAD 79f5916a3295e18ce614e96eb890a456673b2b77

### Classification report

PPCR: 0.572

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.999| 0.928| 0.991| 0.955| 21277| 22911| 0.929 |
| `␣` | 0.993| 0.830| 0.141| 0.904| 0.248| 1938| 11370| 0.170 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 2028| 0.009 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 283| 0.018 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 245| 0.016 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 862| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1727| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1177| 0.000 |
| `micro avg` | 0.984| 0.984| 0.563| 0.984| 0.717| 23242| 40603| 0.572 |
| `weighted avg` | 0.983| 0.984| 0.563| 0.983| 0.608| 23242| 40603| 0.572 |
| `macro avg` | 0.247| 0.229| 0.134| 0.237| 0.150| 23242| 40603| 0.572 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1634 |21266 |11 |0 |0 |0 |0 |0 |0 |
|9432 |330 |1608 |0 |0 |0 |0 |0 |0 |
|862 |0 |0 |0 |0 |0 |0 |0 |0 |
|2010 |18 |0 |0 |0 |0 |0 |0 |0 |
|278 |5 |0 |0 |0 |0 |0 |0 |0 |
|241 |4 |0 |0 |0 |0 |0 |0 |0 |
|1727 |0 |0 |0 |0 |0 |0 |0 |0 |
|1177 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/main/resources/public/web/appclient/libs/bootstrap/js/bootstrap.bundle.js | 303 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/MultiSelect.js | 24 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/Bs4Adapter.js | 9 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/build/rollup.config.js | 8 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/Bs4AdapterJs.js | 5 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/BsMultiSelect.js | 4 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/AddToJQueryPrototype.js | 3 |
| src/main/resources/public/web/appclient/js/core.js | 3 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/build/postcss.config.js | 3 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/Bs4AdapterCss.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2369443259392655, "precision": 0.24708693563064538, "recall": 0.22865054650648944, "support": 23242}, "micro avg": {"f1-score": 0.9841665949574047, "precision": 0.9841665949574047, "recall": 0.9841665949574047, "support": 23242}, "weighted avg": {"f1-score": 0.9829917035690008, "precision": 0.9831574365824897, "recall": 0.9841665949574047, "support": 23242}, "\u2205": {"f1-score": 0.9914219114219114, "precision": 0.983489802525089, "recall": 0.9994830098228134, "support": 21277}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9041326960922126, "precision": 0.9932056825200741, "recall": 0.8297213622291022, "support": 1938}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 862}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1727}, "macro avg": {"f1-score": 0.15032996265623544, "precision": 0.24708693563064538, "recall": 0.1337031537316188, "support": 40603}, "micro avg": {"f1-score": 0.7165478894196883, "precision": 0.9841665949574047, "recall": 0.5633573873851686, "support": 40603}, "weighted avg": {"f1-score": 0.6082357086507101, "precision": 0.8330784295718433, "recall": 0.5633573873851686, "support": 40603}, "\u2205": {"f1-score": 0.9550455831499529, "precision": 0.983489802525089, "recall": 0.9282004277421326, "support": 22911}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2028}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1177}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 283}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u2423": {"f1-score": 0.2475941180999307, "precision": 0.9932056825200741, "recall": 0.14142480211081795, "support": 11370}},
  "ppcr": 0.5724207570869148
}
```
</details>
