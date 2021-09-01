# Train report for javascript / file:///tmp/top-repos-quality-repos-e_5gl3_3/reveal.js.git HEAD b18f12d964ef80bd9ffb061aae48ff4c15fb43ad

### Classification report

PPCR: 0.851

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.967| 0.876| 0.836| 0.919| 0.897| 26404| 27676| 0.954 |
| `␣` | 0.835| 0.958| 0.881| 0.892| 0.857| 18930| 20596| 0.919 |
| `'` | 1.000| 1.000| 0.979| 1.000| 0.989| 2554| 2609| 0.979 |
| `⏎` | 0.980| 0.844| 0.431| 0.907| 0.598| 1810| 3549| 0.510 |
| `⏎⏎` | 0.875| 0.911| 0.489| 0.893| 0.628| 985| 1833| 0.537 |
| `⏎⏎⇥⁻` | 0.850| 0.869| 0.441| 0.860| 0.581| 176| 347| 0.507 |
| `⏎⏎⇥⁺` | 0.674| 0.955| 0.325| 0.790| 0.438| 132| 388| 0.340 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 119| 1542| 0.077 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 1566| 0.016 |
| `weighted avg` | 0.915| 0.910| 0.774| 0.909| 0.810| 51135| 60106| 0.851 |
| `macro avg` | 0.687| 0.713| 0.487| 0.696| 0.554| 51135| 60106| 0.851 |
| `micro avg` | 0.910| 0.910| 0.774| 0.910| 0.836| 51135| 60106| 0.851 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1272 |23125 |3244 |0 |0 |31 |0 |0 |4 |0 |
|1666 |748 |18138 |0 |14 |29 |0 |0 |1 |0 |
|55 |0 |0 |2554 |0 |0 |0 |0 |0 |0 |
|1739 |22 |213 |0 |1528 |42 |0 |0 |2 |3 |
|848 |6 |60 |0 |17 |897 |0 |0 |2 |3 |
|1423 |2 |62 |0 |0 |0 |0 |0 |0 |55 |
|1541 |6 |1 |0 |0 |0 |0 |0 |18 |0 |
|171 |1 |1 |0 |0 |21 |0 |0 |153 |0 |
|256 |0 |1 |0 |0 |5 |0 |0 |0 |126 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/qunit-2.5.0.js | 3713 |
| js/reveal.js | 347 |
| plugin/markdown/markdown.js | 74 |
| gulpfile.js | 66 |
| plugin/multiplex/index.js | 58 |
| plugin/notes/notes.js | 53 |
| js/controllers/slidecontent.js | 49 |
| plugin/zoom-js/zoom.js | 43 |
| js/controllers/autoanimate.js | 41 |
| plugin/notes-server/index.js | 24 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2554}, "macro avg": {"f1-score": 0.6956418219231407, "precision": 0.6868095136603609, "recall": 0.7125220387345534, "support": 51135}, "micro avg": {"f1-score": 0.9097682604869464, "precision": 0.9097682604869464, "recall": 0.9097682604869464, "support": 51135}, "weighted avg": {"f1-score": 0.909258370327971, "precision": 0.9147116930397236, "recall": 0.9097682604869464, "support": 51135}, "\u2205": {"f1-score": 0.919227252852089, "precision": 0.9671685487243831, "recall": 0.8758142705650659, "support": 26404}, "\u23ce": {"f1-score": 0.9070940932027308, "precision": 0.9801154586273252, "recall": 0.8441988950276244, "support": 1810}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 119}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.8925373134328358, "precision": 0.8751219512195122, "recall": 0.9106598984771573, "support": 985}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.7899686520376177, "precision": 0.6737967914438503, "recall": 0.9545454545454546, "support": 132}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.8595505617977528, "precision": 0.85, "recall": 0.8693181818181818, "support": 176}, "\u2423": {"f1-score": 0.8923985239852399, "precision": 0.8350828729281768, "recall": 0.958161648177496, "support": 18930}},
  "cl_report_full": {"\u0027": {"f1-score": 0.989347278713926, "precision": 1.0, "recall": 0.9789191261019548, "support": 2609}, "macro avg": {"f1-score": 0.5542297424959254, "precision": 0.6868095136603609, "recall": 0.48674522634459017, "support": 60106}, "micro avg": {"f1-score": 0.8364002481099595, "precision": 0.9097682604869464, "recall": 0.7739826306857884, "support": 60106}, "weighted avg": {"f1-score": 0.8101696904827498, "precision": 0.868709198077708, "recall": 0.7739826306857884, "support": 60106}, "\u2205": {"f1-score": 0.8965610824642344, "precision": 0.9671685487243831, "recall": 0.8355614973262032, "support": 27676}, "\u23ce": {"f1-score": 0.5982772122161316, "precision": 0.9801154586273252, "recall": 0.43054381515919976, "support": 3549}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1542}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1566}, "\u23ce\u23ce": {"f1-score": 0.6277116864940517, "precision": 0.8751219512195122, "recall": 0.48936170212765956, "support": 1833}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.4382608695652174, "precision": 0.6737967914438503, "recall": 0.3247422680412371, "support": 388}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.5806451612903226, "precision": 0.85, "recall": 0.4409221902017291, "support": 347}, "\u2423": {"f1-score": 0.8572643917194442, "precision": 0.8350828729281768, "recall": 0.8806564381433288, "support": 20596}},
  "ppcr": 0.8507470136092903
}
```
</details>
