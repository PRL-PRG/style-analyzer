# Train report for javascript / file:///tmp/top-repos-quality-repos-cu3ifep7/htmx.git HEAD 7e7a7100b8557ea36538c3efdbd5b1c9129f0fc9

### Classification report

PPCR: 0.906

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.992| 0.974| 0.993| 0.984| 428387| 436219| 0.982 |
| `␣` | 0.946| 0.988| 0.945| 0.967| 0.945| 169508| 177261| 0.956 |
| `"` | 0.979| 0.999| 0.736| 0.989| 0.840| 53505| 72648| 0.736 |
| `⏎` | 0.973| 0.833| 0.582| 0.897| 0.729| 34861| 49859| 0.699 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.970| 0.969| 0.935| 0.969| 0.952| 24933| 25839| 0.965 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.972| 0.988| 0.924| 0.980| 0.947| 22958| 24552| 0.935 |
| `'` | 0.994| 0.908| 0.375| 0.949| 0.545| 12715| 30763| 0.413 |
| `⏎⏎` | 0.967| 0.863| 0.432| 0.912| 0.598| 6598| 13161| 0.501 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 333| 431| 0.773 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 323| 437| 0.739 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 316| 456| 0.693 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 179| 645| 0.278 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 146| 202| 0.723 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 43| 49| 0.878 |
| `⏎⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 161| 0.137 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 49| 0.000 |
| `micro avg` | 0.979| 0.979| 0.887| 0.979| 0.931| 754827| 832732| 0.906 |
| `macro avg` | 0.487| 0.471| 0.369| 0.478| 0.409| 754827| 832732| 0.906 |
| `weighted avg` | 0.977| 0.979| 0.887| 0.978| 0.920| 754827| 832732| 0.906 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ␣␣| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁻| ⏎⏎⏎| ⏎⇥⁺| ⏎⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7832 |424838 |3185 |0 |1 |0 |190 |173 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7753 |1831 |167486 |0 |2 |0 |155 |30 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|19143 |0 |0 |53433 |0 |72 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|14998 |693 |5022 |0 |29030 |0 |46 |6 |64 |0 |0 |0 |0 |0 |0 |0 |0 |
|18048 |0 |0 |1164 |0 |11551 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|906 |45 |724 |0 |0 |0 |24164 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1594 |71 |110 |0 |90 |0 |4 |22683 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6563 |113 |163 |0 |630 |0 |0 |0 |5692 |0 |0 |0 |0 |0 |0 |0 |0 |
|466 |0 |11 |0 |58 |0 |0 |110 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|98 |0 |331 |0 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|114 |0 |0 |0 |10 |0 |313 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|140 |0 |0 |0 |0 |0 |0 |316 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|49 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|56 |2 |14 |0 |6 |0 |0 |0 |124 |0 |0 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |43 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|139 |0 |0 |0 |3 |0 |0 |19 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| www/test/1.3.4/test/lib/_hyperscript.js | 763 |
| test/lib/_hyperscript.js | 763 |
| www/test/1.3.3/src/htmx.js | 364 |
| www/test/1.2.1/src/htmx.js | 325 |
| www/test/1.2.0/src/htmx.js | 322 |
| www/test/1.1.0/src/htmx.js | 315 |
| www/test/0.0.8/test/lib/_hyperscript.js | 294 |
| www/test/0.0.7/test/lib/_hyperscript.js | 294 |
| www/test/1.2.0/test/lib/_hyperscript.js | 289 |
| www/test/1.1.0/test/lib/_hyperscript.js | 289 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9885663539990008, "precision": 0.9786801472608385, "recall": 0.9986543313708999, "support": 53505}, "\u0027": {"f1-score": 0.9492152189990961, "precision": 0.9938053858728383, "recall": 0.9084545812033031, "support": 12715}, "macro avg": {"f1-score": 0.47849553391976984, "precision": 0.48714695141517034, "recall": 0.4712185023404615, "support": 754827}, "micro avg": {"f1-score": 0.9788693303233721, "precision": 0.9788693303233721, "recall": 0.9788693303233721, "support": 754827}, "weighted avg": {"f1-score": 0.9777231884471259, "precision": 0.9774231009261257, "recall": 0.9788693303233721, "support": 754827}, "\u2205": {"f1-score": 0.9926353419472418, "precision": 0.9935569571999542, "recall": 0.9917154348754748, "support": 428387}, "\u23ce": {"f1-score": 0.8974973334776088, "precision": 0.9731813610459269, "recall": 0.8327357218668426, "support": 34861}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.912033327992309, "precision": 0.9673691366417403, "recall": 0.8626856623219157, "support": 6598}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 323}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 179}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.969468405215647, "precision": 0.9697796684994181, "recall": 0.9691573416756909, "support": 24933}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9799330381250674, "precision": 0.9719758323691992, "recall": 0.9880216046693963, "support": 22958}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 316}, "\u2423": {"f1-score": 0.9665795229603467, "precision": 0.94600273375281, "recall": 0.9880713594638602, "support": 169508}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 333}},
  "cl_report_full": {"\"": {"f1-score": 0.8398443946716964, "precision": 0.9786801472608385, "recall": 0.7355054509415263, "support": 72648}, "\u0027": {"f1-score": 0.5450384560939934, "precision": 0.9938053858728383, "recall": 0.3754835354159217, "support": 30763}, "macro avg": {"f1-score": 0.40873439829627506, "precision": 0.48714695141517034, "recall": 0.368971089148749, "support": 832732}, "micro avg": {"f1-score": 0.930834066639413, "precision": 0.9788693303233721, "recall": 0.8872926703909542, "support": 832732}, "weighted avg": {"f1-score": 0.9204675705360985, "precision": 0.9762383161722993, "recall": 0.8872926703909542, "support": 832732}, "\u2205": {"f1-score": 0.983635328057494, "precision": 0.9935569571999542, "recall": 0.9739098938835768, "support": 436219}, "\u23ce": {"f1-score": 0.7285823639398161, "precision": 0.9731813610459269, "recall": 0.5822419222206623, "support": 49859}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u23ce": {"f1-score": 0.5977421895510633, "precision": 0.9673691366417403, "recall": 0.4324899323759593, "support": 13161}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 202}, "\u23ce\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 161}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 437}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 645}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.952163291039483, "precision": 0.9697796684994181, "recall": 0.9351755098881536, "support": 25839}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9473156674810499, "precision": 0.9719758323691992, "recall": 0.9238758553274682, "support": 24552}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 456}, "\u2423": {"f1-score": 0.9454286819058049, "precision": 0.94600273375281, "recall": 0.944855326326716, "support": 177261}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 431}},
  "ppcr": 0.9064464917884746
}
```
</details>