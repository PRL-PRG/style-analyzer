# Train report for javascript / file:///tmp/top-repos-quality-repos-j54yfxxz/shakespurrean-twitter-bot.git HEAD 64f7cda4d9f2453041b167f18584558bcd8c08b6

### Classification report

PPCR: 0.848

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.992| 0.938| 0.982| 0.955| 20111| 21268| 0.946 |
| `␣` | 0.964| 0.953| 0.787| 0.958| 0.867| 7040| 8517| 0.827 |
| `'` | 1.000| 1.000| 0.995| 1.000| 0.997| 6917| 6952| 0.995 |
| `⏎␣⁺␣⁺` | 0.931| 0.919| 0.795| 0.925| 0.858| 1209| 1397| 0.865 |
| `⏎` | 0.916| 0.898| 0.368| 0.907| 0.525| 1001| 2444| 0.410 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 195| 1122| 0.174 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 1149| 0.049 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 143| 0.084 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 121| 0.008 |
| `macro avg` | 0.532| 0.529| 0.431| 0.530| 0.467| 36542| 43113| 0.848 |
| `micro avg` | 0.974| 0.974| 0.825| 0.974| 0.893| 36542| 43113| 0.848 |
| `weighted avg` | 0.967| 0.974| 0.825| 0.970| 0.861| 36542| 43113| 0.848 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1157 |19946 |147 |0 |14 |4 |0 |0 |0 |0 |
|1477 |237 |6707 |0 |28 |68 |0 |0 |0 |0 |
|35 |0 |0 |6917 |0 |0 |0 |0 |0 |0 |
|1443 |34 |62 |0 |899 |6 |0 |0 |0 |0 |
|188 |67 |27 |0 |4 |1111 |0 |0 |0 |0 |
|1093 |13 |13 |0 |29 |1 |0 |0 |0 |0 |
|927 |185 |0 |0 |7 |3 |0 |0 |0 |0 |
|131 |12 |0 |0 |0 |0 |0 |0 |0 |0 |
|120 |1 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/src/tooltip.js | 120 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/Gruntfile.js | 83 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/tests/unit/tooltip.js | 83 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/src/carousel.js | 71 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/src/modal.js | 69 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/tests/unit/scrollspy.js | 67 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/src/scrollspy.js | 59 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/src/collapse.js | 59 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/src/dropdown.js | 42 |
| shakespurr_twitter_bot/static/lib/bootstrap-4.0.0-alpha.2/js/src/tab.js | 38 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6917}, "macro avg": {"f1-score": 0.5303400011591324, "precision": 0.5316771205991165, "recall": 0.5290597300333562, "support": 36542}, "micro avg": {"f1-score": 0.9736741284001971, "precision": 0.9736741284001971, "recall": 0.9736741284001971, "support": 36542}, "weighted avg": {"f1-score": 0.9700646296915132, "precision": 0.9665729244046031, "recall": 0.9736741284001971, "support": 36542}, "\u2205": {"f1-score": 0.9824163916662563, "precision": 0.9732129787753111, "recall": 0.9917955347819601, "support": 20111}, "\u23ce": {"f1-score": 0.9071644803229061, "precision": 0.9164118246687054, "recall": 0.8981018981018981, "support": 1001}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9250624479600332, "precision": 0.9312657166806371, "recall": 0.9189412737799835, "support": 1209}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 195}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9584166904829953, "precision": 0.964203565267395, "recall": 0.9526988636363637, "support": 7040}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9974763861850169, "precision": 1.0, "recall": 0.9949654775604143, "support": 6952}, "macro avg": {"f1-score": 0.46694266848928223, "precision": 0.5316771205991165, "recall": 0.43148949098325773, "support": 43113}, "micro avg": {"f1-score": 0.893352583014249, "precision": 0.9736741284001971, "recall": 0.825273119476724, "support": 43113}, "weighted avg": {"f1-score": 0.8608725228982281, "precision": 0.9139494839994639, "recall": 0.825273119476724, "support": 43113}, "\u2205": {"f1-score": 0.9551995785743362, "precision": 0.9732129787753111, "recall": 0.9378408877186384, "support": 21268}, "\u23ce": {"f1-score": 0.5249635036496351, "precision": 0.9164118246687054, "recall": 0.3678396072013093, "support": 2444}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1149}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 143}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8579150579150578, "precision": 0.9312657166806371, "recall": 0.7952755905511811, "support": 1397}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1122}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u2423": {"f1-score": 0.8669294900794933, "precision": 0.964203565267395, "recall": 0.7874838558177762, "support": 8517}},
  "ppcr": 0.847586574815021
}
```
</details>
