# Train report for javascript / file:///tmp/top-repos-quality-repos-8wxup_of/code_for_fun.git HEAD 9abbd7cf8d332319f83bc1640693178af90ac9de

### Classification report

PPCR: 0.877

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 0.970| 0.951| 0.981| 0.971| 40113| 40926| 0.980 |
| `␣` | 0.924| 0.988| 0.937| 0.955| 0.930| 18008| 18988| 0.948 |
| `'` | 1.000| 1.000| 0.832| 1.000| 0.908| 1833| 2203| 0.832 |
| `⏎␣⁺␣⁺` | 0.930| 0.943| 0.915| 0.936| 0.922| 1568| 1616| 0.970 |
| `⏎␣⁻␣⁻` | 0.953| 0.927| 0.715| 0.940| 0.817| 1228| 1591| 0.772 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.950| 0.709| 0.532| 0.812| 0.682| 540| 720| 0.750 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 140| 757| 0.185 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 106| 4828| 0.022 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 808| 0.001 |
| `macro avg` | 0.639| 0.615| 0.543| 0.625| 0.581| 63537| 72437| 0.877 |
| `micro avg` | 0.969| 0.969| 0.850| 0.969| 0.905| 63537| 72437| 0.877 |
| `weighted avg` | 0.966| 0.969| 0.850| 0.967| 0.865| 63537| 72437| 0.877 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|813 |38926 |1184 |0 |0 |1 |0 |0 |0 |2 |
|980 |192 |17794 |0 |0 |21 |1 |0 |0 |0 |
|4722 |19 |74 |0 |0 |11 |2 |0 |0 |0 |
|370 |0 |0 |0 |1833 |0 |0 |0 |0 |0 |
|48 |2 |87 |0 |0 |1479 |0 |0 |0 |0 |
|363 |40 |32 |0 |0 |0 |1138 |0 |0 |18 |
|807 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|617 |18 |43 |0 |0 |79 |0 |0 |0 |0 |
|180 |56 |48 |0 |0 |0 |53 |0 |0 |383 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| node/js_basics/src/libraries/p5.sound.js | 539 |
| node/js_basics/src/libraries/p5.dom.js | 353 |
| algos/problems/search/dfs.js | 87 |
| algos/problems/search/bfs.js | 78 |
| algos/data_structures/trees/binary_search_tree.js | 54 |
| flutter/courses/maximillian/01_Exercise/first_exercise/functions/index.js | 41 |
| algos/data_structures/linked_lists/doubly_linked_list.js | 27 |
| algos/data_structures/hash_tables/first_recurring_element.js | 27 |
| algos/problems/memoization/fibonacci.js | 24 |
| algos/data_structures/linked_lists/linked_list.js | 19 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1833}, "macro avg": {"f1-score": 0.6249067496284781, "precision": 0.6387204774611442, "recall": 0.615303793480994, "support": 63537}, "micro avg": {"f1-score": 0.9687741001306326, "precision": 0.9687741001306326, "recall": 0.9687741001306326, "support": 63537}, "weighted avg": {"f1-score": 0.966940240377542, "precision": 0.9661735320024598, "recall": 0.9687741001306326, "support": 63537}, "\u2205": {"f1-score": 0.9809238212836731, "precision": 0.9916694265406466, "recall": 0.9704085957170991, "support": 40113}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.936372269705603, "precision": 0.9296040226272785, "recall": 0.9432397959183674, "support": 1568}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.939719240297275, "precision": 0.9530988274706867, "recall": 0.9267100977198697, "support": 1228}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8123011664899258, "precision": 0.9503722084367245, "recall": 0.7092592592592593, "support": 540}, "\u2423": {"f1-score": 0.9548442488798261, "precision": 0.9237398120749624, "recall": 0.9881163927143491, "support": 18008}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9083250743310208, "precision": 1.0, "recall": 0.8320472083522469, "support": 2203}, "macro avg": {"f1-score": 0.5812641625568453, "precision": 0.6387204774611442, "recall": 0.542526369788269, "support": 72437}, "micro avg": {"f1-score": 0.9053642608145674, "precision": 0.9687741001306326, "recall": 0.84974529591231, "support": 72437}, "weighted avg": {"f1-score": 0.8654030279114838, "precision": 0.8839532673554656, "recall": 0.84974529591231, "support": 72437}, "\u2205": {"f1-score": 0.9709774379825141, "precision": 0.9916694265406466, "recall": 0.9511313101695743, "support": 40926}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4828}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 808}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9223573433115062, "precision": 0.9296040226272785, "recall": 0.9152227722772277, "support": 1616}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 757}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8172351885098743, "precision": 0.9530988274706867, "recall": 0.7152734129478315, "support": 1591}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.6821015138023152, "precision": 0.9503722084367245, "recall": 0.5319444444444444, "support": 720}, "\u2423": {"f1-score": 0.9303809050743771, "precision": 0.9237398120749624, "recall": 0.9371181799030966, "support": 18988}},
  "ppcr": 0.8771346135262366
}
```
</details>
