# Train report for javascript / file:///tmp/top-repos-quality-repos-b943es_f/minecraft-server.git HEAD 9c9e2abbe25a4642cb509a8eb80dabb41b5f6968

### Classification report

PPCR: 0.860

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.971| 0.934| 0.980| 0.961| 41276| 42887| 0.962 |
| `␣` | 0.925| 0.988| 0.918| 0.956| 0.921| 17471| 18807| 0.929 |
| `'` | 0.963| 1.000| 0.811| 0.981| 0.880| 2333| 2878| 0.811 |
| `⏎⇥⁻` | 0.916| 0.949| 0.914| 0.932| 0.915| 1856| 1926| 0.964 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 253| 2709| 0.093 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 194| 2059| 0.094 |
| `"` | 1.000| 0.250| 0.122| 0.400| 0.218| 120| 245| 0.490 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 244| 0.156 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 1861| 0.006 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 90| 0.011 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 163| 0.000 |
| `macro avg` | 0.436| 0.378| 0.336| 0.386| 0.354| 63554| 73869| 0.860 |
| `weighted avg` | 0.960| 0.967| 0.832| 0.963| 0.851| 63554| 73869| 0.860 |
| `micro avg` | 0.967| 0.967| 0.832| 0.967| 0.894| 63554| 73869| 0.860 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⏎| ⏎⇥⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1611 |40064 |1182 |0 |0 |0 |0 |30 |0 |0 |0 |0 |
|1336 |109 |17260 |0 |0 |0 |0 |102 |0 |0 |0 |0 |
|545 |0 |0 |2333 |0 |0 |0 |0 |0 |0 |0 |0 |
|2456 |160 |64 |0 |0 |0 |0 |29 |0 |0 |0 |0 |
|1865 |67 |127 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1849 |0 |12 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|70 |87 |8 |0 |0 |0 |0 |1761 |0 |0 |0 |0 |
|206 |35 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|163 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|125 |0 |0 |90 |0 |0 |0 |0 |0 |0 |30 |0 |
|89 |0 |0 |0 |0 |0 |0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| 1.14.4 +/dynmap/js/leaflet.js | 486 |
| 1.14.4 +/dynmap/js/Scrollify/jquery.scrollify.js | 486 |
| 1.14.4 +/dynmap/js/map.js | 368 |
| 1.14.4 +/dynmap/js/markers.js | 168 |
| 1.14.4 +/dynmap/js/dynmaputils.js | 94 |
| 1.14.4 +/dynmap/js/chatbox.js | 87 |
| 1.14.4 +/dynmap/js/jquery.mousewheel.js | 62 |
| 1.14.4 +/dynmap/js/timeofdayclock.js | 53 |
| 1.14.4 +/dynmap/js/playermarkers.js | 51 |
| 1.14.4 +/dynmap/js/hdmap.js | 37 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.4, "precision": 1.0, "recall": 0.25, "support": 120}, "\u0027": {"f1-score": 0.9810765349032801, "precision": 0.9628559636813867, "recall": 1.0, "support": 2333}, "macro avg": {"f1-score": 0.3861973647313944, "precision": 0.43568014710807385, "recall": 0.3779431080312239, "support": 63554}, "micro avg": {"f1-score": 0.9668628253139063, "precision": 0.9668628253139063, "recall": 0.9668628253139063, "support": 63554}, "weighted avg": {"f1-score": 0.9628628958331266, "precision": 0.9604295075568504, "recall": 0.9668628253139063, "support": 63554}, "\u2205": {"f1-score": 0.9795838529059391, "precision": 0.9886974976555944, "recall": 0.9706366896017056, "support": 41276}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 253}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u23ce\u21e5\u207b": {"f1-score": 0.9319925906324424, "precision": 0.9157566302652106, "recall": 0.9488146551724138, "support": 1856}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u2423": {"f1-score": 0.9555180336036758, "precision": 0.9251715265866209, "recall": 0.9879228435693435, "support": 17471}},
  "cl_report_full": {"\"": {"f1-score": 0.2181818181818182, "precision": 1.0, "recall": 0.12244897959183673, "support": 245}, "\u0027": {"f1-score": 0.880211280890398, "precision": 0.9628559636813867, "recall": 0.810632383599722, "support": 2878}, "macro avg": {"f1-score": 0.3541401976925212, "precision": 0.43568014710807385, "recall": 0.3363028029239446, "support": 73869}, "micro avg": {"f1-score": 0.8942898932493105, "precision": 0.9668628253139063, "recall": 0.8318509794365702, "support": 73869}, "weighted avg": {"f1-score": 0.8512180878299258, "precision": 0.8742749626478751, "recall": 0.8318509794365702, "support": 73869}, "\u2205": {"f1-score": 0.960663717344651, "precision": 0.9886974976555944, "recall": 0.9341758574859514, "support": 42887}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2709}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2059}, "\u23ce\u21e5\u207b": {"f1-score": 0.9150428682774746, "precision": 0.9157566302652106, "recall": 0.9143302180685359, "support": 1926}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1861}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 244}, "\u2423": {"f1-score": 0.9214424899233911, "precision": 0.9251715265866209, "recall": 0.9177433934173446, "support": 18807}},
  "ppcr": 0.8603609091770567
}
```
</details>
