# Train report for javascript / file:///tmp/top-repos-quality-repos-16uny_w2/slate.git HEAD e3d77f11e042cbf6281455ed324a0e78986b37bb

### Classification report

PPCR: 0.738

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.982| 0.908| 0.985| 0.946| 20148| 21797| 0.924 |
| `␣` | 0.960| 0.986| 0.679| 0.973| 0.795| 16332| 23740| 0.688 |
| `"` | 0.978| 1.000| 0.923| 0.989| 0.950| 2332| 2526| 0.923 |
| `⏎` | 0.988| 0.881| 0.446| 0.932| 0.615| 1622| 3202| 0.507 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1313| 2035| 0.645 |
| `'` | 1.000| 0.212| 0.113| 0.350| 0.203| 66| 124| 0.532 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 43| 1456| 0.030 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 1864| 0.015 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.947| 0.947| 0.699| 0.947| 0.804| 41884| 56744| 0.738 |
| `weighted avg` | 0.944| 0.947| 0.699| 0.945| 0.773| 41884| 56744| 0.738 |
| `macro avg` | 0.410| 0.338| 0.256| 0.352| 0.292| 41884| 56744| 0.738 |

### Confusion matrix

|refusal|  ␣| ∅| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⏎⇥⁺| ⏎⏎⇥⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7408 |16109 |207 |0 |0 |16 |0 |0 |0 |0 |
|1649 |363 |19784 |0 |0 |1 |0 |0 |0 |0 |
|1580 |174 |14 |1429 |0 |5 |0 |0 |0 |0 |
|194 |0 |0 |0 |2332 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1413 |6 |0 |13 |0 |24 |0 |0 |0 |0 |
|722 |118 |7 |4 |0 |1184 |0 |0 |0 |0 |
|1836 |11 |13 |0 |0 |4 |0 |0 |0 |0 |
|58 |0 |0 |0 |52 |0 |0 |0 |0 |14 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| source/javascripts/lib/_jquery.js | 1932 |
| source/javascripts/lib/_lunr.js | 175 |
| source/javascripts/app/_lang.js | 36 |
| source/javascripts/lib/_energize.js | 32 |
| source/javascripts/lib/_jquery.highlight.js | 19 |
| source/javascripts/app/_search.js | 14 |
| source/javascripts/app/_toc.js | 7 |
| source/javascripts/all_nosearch.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9889737065309585, "precision": 0.9781879194630873, "recall": 1.0, "support": 2332}, "\u0027": {"f1-score": 0.35, "precision": 1.0, "recall": 0.21212121212121213, "support": 66}, "macro avg": {"f1-score": 0.35236972336341416, "precision": 0.4095292586662762, "recall": 0.3384509853641357, "support": 41884}, "micro avg": {"f1-score": 0.9470919682933817, "precision": 0.9470919682933817, "recall": 0.9470919682933817, "support": 41884}, "weighted avg": {"f1-score": 0.9448836063378135, "precision": 0.9438822726380433, "recall": 0.9470919682933817, "support": 41884}, "\u2205": {"f1-score": 0.9849401339207925, "precision": 0.9879650436953807, "recall": 0.9819336906889021, "support": 20148}, "\u23ce": {"f1-score": 0.9315514993481095, "precision": 0.9882434301521439, "recall": 0.8810110974106042, "support": 1622}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1313}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.9729713405611089, "precision": 0.959954710684703, "recall": 0.9863458241489101, "support": 16332}},
  "cl_report_full": {"\"": {"f1-score": 0.94989816700611, "precision": 0.9781879194630873, "recall": 0.9231987331749802, "support": 2526}, "\u0027": {"f1-score": 0.2028985507246377, "precision": 1.0, "recall": 0.11290322580645161, "support": 124}, "macro avg": {"f1-score": 0.2924069720364553, "precision": 0.4095292586662762, "recall": 0.2557160638852779, "support": 56744}, "micro avg": {"f1-score": 0.8043963174757675, "precision": 0.9470919682933817, "recall": 0.6990695051459185, "support": 56744}, "weighted avg": {"f1-score": 0.7734959141862883, "precision": 0.8826176694803677, "recall": 0.6990695051459185, "support": 56744}, "\u2205": {"f1-score": 0.9461049208550523, "precision": 0.9879650436953807, "recall": 0.9076478414460706, "support": 21797}, "\u23ce": {"f1-score": 0.6148881239242685, "precision": 0.9882434301521439, "recall": 0.44628357276702063, "support": 3202}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1456}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2035}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1864}, "\u2423": {"f1-score": 0.7950939019273957, "precision": 0.959954710684703, "recall": 0.6785593934288121, "support": 23740}},
  "ppcr": 0.738122092203581
}
```
</details>
