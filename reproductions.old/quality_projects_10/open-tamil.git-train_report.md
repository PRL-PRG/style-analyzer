# Train report for javascript / file:///tmp/top-repos-quality-repos-av7k39rr/open-tamil.git HEAD 9f8c5889c87ec0cee94367cf3772ab59852a142f

### Classification report

PPCR: 0.868

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 0.996| 0.983| 0.988| 0.981| 271939| 275779| 0.986 |
| `␣` | 0.983| 0.964| 0.942| 0.973| 0.962| 121537| 124323| 0.978 |
| `⏎⇥⁻` | 0.974| 0.947| 0.915| 0.960| 0.944| 16096| 16649| 0.967 |
| `⏎⇥⁺` | 0.986| 0.944| 0.828| 0.964| 0.900| 15037| 17135| 0.878 |
| `⏎` | 0.934| 0.851| 0.415| 0.891| 0.574| 11235| 23051| 0.487 |
| `⏎⏎` | 0.937| 0.930| 0.598| 0.934| 0.730| 10628| 16538| 0.643 |
| `'` | 0.998| 0.986| 0.165| 0.992| 0.284| 6786| 40463| 0.168 |
| `"` | 0.960| 0.994| 0.214| 0.976| 0.350| 2340| 10865| 0.215 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 172| 323| 0.533 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 99| 102| 0.971 |
| `macro avg` | 0.775| 0.761| 0.506| 0.768| 0.573| 455869| 525228| 0.868 |
| `micro avg` | 0.978| 0.978| 0.849| 0.978| 0.909| 455869| 525228| 0.868 |
| `weighted avg` | 0.978| 0.978| 0.849| 0.978| 0.879| 455869| 525228| 0.868 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| "| ⏎⏎⇥⁻| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3840 |270973 |634 |0 |5 |113 |214 |0 |0 |0 |0 |
|2786 |3812 |117155 |0 |343 |54 |33 |140 |0 |0 |0 |
|33677 |0 |0 |6688 |0 |0 |0 |0 |98 |0 |0 |
|11816 |418 |776 |0 |9560 |25 |24 |432 |0 |0 |0 |
|2098 |306 |490 |0 |51 |14190 |0 |0 |0 |0 |0 |
|553 |796 |50 |0 |5 |3 |15238 |4 |0 |0 |0 |
|5910 |428 |30 |0 |270 |11 |0 |9889 |0 |0 |0 |
|8525 |0 |0 |14 |0 |0 |0 |0 |2326 |0 |0 |
|151 |0 |44 |0 |1 |2 |38 |87 |0 |0 |0 |
|3 |8 |0 |0 |0 |0 |90 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| webspell/static/tinymce/js/tinymce/classes/dom/Sizzle.js | 1549 |
| webspell/static/tinymce/tests/js/qunit/qunit.js | 1445 |
| webapp/opentamilapp/static/js/jquery.ime.js | 1009 |
| webspell/static/tinymce/js/tinymce/plugins/table/plugin.js | 280 |
| webspell/static/tinymce/js/tinymce/classes/Formatter.js | 165 |
| webspell/static/tinymce/tests/tinymce/html/Schema.js | 148 |
| webspell/static/tinymce/tests/plugins/lists.js | 140 |
| webspell/static/tinymce/tests/tinymce/html/SaxParser.js | 120 |
| webspell/static/tinymce/tests/tinymce/html/DomParser.js | 119 |
| webspell/static/tinymce/tests/plugins/table.js | 115 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9764903442485307, "precision": 0.9595709570957096, "recall": 0.994017094017094, "support": 2340}, "\u0027": {"f1-score": 0.9916963226571768, "precision": 0.9979110713219934, "recall": 0.9855585027998821, "support": 6786}, "macro avg": {"f1-score": 0.7678154192402674, "precision": 0.7750823055984426, "recall": 0.7611714591003922, "support": 455869}, "micro avg": {"f1-score": 0.978392915508622, "precision": 0.978392915508622, "recall": 0.978392915508622, "support": 455869}, "weighted avg": {"f1-score": 0.977922334447052, "precision": 0.9777362545000087, "recall": 0.978392915508622, "support": 455869}, "\u2205": {"f1-score": 0.9877269082160822, "precision": 0.9791574071062835, "recall": 0.9964477327635978, "support": 271939}, "\u23ce": {"f1-score": 0.8905449464368886, "precision": 0.9340498290180752, "recall": 0.8509123275478415, "support": 11235}, "\u23ce\u21e5\u207a": {"f1-score": 0.9641583149312043, "precision": 0.9855535491040422, "recall": 0.9436722750548646, "support": 15037}, "\u23ce\u21e5\u207b": {"f1-score": 0.9603882393722623, "precision": 0.9744835965978129, "recall": 0.9466948310139165, "support": 16096}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 99}, "\u23ce\u23ce": {"f1-score": 0.9337613899249326, "precision": 0.9370795034587321, "recall": 0.9304666917576214, "support": 10628}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "\u2423": {"f1-score": 0.9733877266155968, "precision": 0.9830171422817778, "recall": 0.9639451360491044, "support": 121537}},
  "cl_report_full": {"\"": {"f1-score": 0.35006396267589734, "precision": 0.9595709570957096, "recall": 0.2140819144040497, "support": 10865}, "\u0027": {"f1-score": 0.28360012721297573, "precision": 0.9979110713219934, "recall": 0.1652868052294689, "support": 40463}, "macro avg": {"f1-score": 0.5725198830658622, "precision": 0.7750823055984426, "recall": 0.5060353951744927, "support": 525228}, "micro avg": {"f1-score": 0.9092250817197484, "precision": 0.978392915508622, "recall": 0.8491912083894995, "support": 525228}, "weighted avg": {"f1-score": 0.8793547207134058, "precision": 0.9770742340137213, "recall": 0.8491912083894995, "support": 525228}, "\u2205": {"f1-score": 0.9808622312314487, "precision": 0.9791574071062835, "recall": 0.9825730022953162, "support": 275779}, "\u23ce": {"f1-score": 0.5744156702517575, "precision": 0.9340498290180752, "recall": 0.41473254956401023, "support": 23051}, "\u23ce\u21e5\u207a": {"f1-score": 0.9000095138426412, "precision": 0.9855535491040422, "recall": 0.8281295593813831, "support": 17135}, "\u23ce\u21e5\u207b": {"f1-score": 0.9439385492163787, "precision": 0.9744835965978129, "recall": 0.9152501651750856, "support": 16649}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 102}, "\u23ce\u23ce": {"f1-score": 0.7300579528256617, "precision": 0.9370795034587321, "recall": 0.5979562220341033, "support": 16538}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 323}, "\u2423": {"f1-score": 0.9622508234018612, "precision": 0.9830171422817778, "recall": 0.9423437336615107, "support": 124323}},
  "ppcr": 0.867944968661229
}
```
</details>
