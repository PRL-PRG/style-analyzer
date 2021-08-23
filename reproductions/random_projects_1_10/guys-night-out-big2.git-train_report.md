# Train report for javascript / file:///tmp/top-repos-quality-repos-mhwba1tl/guys-night-out-big2.git HEAD dcc31501fa91e7e7c1159559399f18f8dd5389a6

### Classification report

PPCR: 0.536

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.999| 0.773| 0.994| 0.868| 2871| 3710| 0.774 |
| `␣` | 0.987| 0.984| 0.289| 0.986| 0.447| 554| 1889| 0.293 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 166| 332| 0.500 |
| `⏎` | 0.964| 0.991| 0.258| 0.977| 0.408| 109| 418| 0.261 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 223| 0.094 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 222| 0.018 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 154| 0.013 |
| `micro avg` | 0.989| 0.989| 0.531| 0.989| 0.691| 3727| 6948| 0.536 |
| `weighted avg` | 0.982| 0.989| 0.531| 0.985| 0.641| 3727| 6948| 0.536 |
| `macro avg` | 0.563| 0.568| 0.260| 0.565| 0.341| 3727| 6948| 0.536 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|839 |2867 |4 |0 |0 |0 |0 |0 |
|1335 |9 |545 |0 |0 |0 |0 |0 |
|309 |1 |0 |108 |0 |0 |0 |0 |
|166 |0 |0 |0 |166 |0 |0 |0 |
|218 |0 |3 |1 |0 |0 |0 |0 |
|202 |20 |0 |1 |0 |0 |0 |0 |
|152 |0 |0 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/modals/stats/stats.controller.js | 24 |
| src/modals/editScore/editScore.controller.js | 4 |
| src/modals/editPlayers/editPlayers.controller.js | 3 |
| src/modules/game/game.controller.js | 3 |
| src/directives/navBar/navBar.controller.js | 2 |
| src/index.js | 2 |
| src/modals/newGame/newGame.controller.js | 2 |
| src/modals/data/data.controller.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 166}, "macro avg": {"f1-score": 0.5652877755218139, "precision": 0.56303557352163, "recall": 0.5675981368480315, "support": 3727}, "micro avg": {"f1-score": 0.9889991950630534, "precision": 0.9889991950630534, "recall": 0.9889991950630534, "support": 3727}, "weighted avg": {"f1-score": 0.9854027635595572, "precision": 0.9818489467625078, "recall": 0.9889991950630534, "support": 3727}, "\u2205": {"f1-score": 0.9941054091539527, "precision": 0.9896444597859855, "recall": 0.9986067572274469, "support": 2871}, "\u23ce": {"f1-score": 0.9773755656108598, "precision": 0.9642857142857143, "recall": 0.9908256880733946, "support": 109}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.9855334538878843, "precision": 0.9873188405797102, "recall": 0.983754512635379, "support": 554}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 332}, "macro avg": {"f1-score": 0.3412313648287495, "precision": 0.56303557352163, "recall": 0.2599517037871081, "support": 6948}, "micro avg": {"f1-score": 0.6905854800936768, "precision": 0.9889991950630534, "recall": 0.5305123776626367, "support": 6948}, "weighted avg": {"f1-score": 0.6411898402486328, "precision": 0.9026623005515986, "recall": 0.5305123776626367, "support": 6948}, "\u2205": {"f1-score": 0.8678674133494778, "precision": 0.9896444597859855, "recall": 0.7727762803234501, "support": 3710}, "\u23ce": {"f1-score": 0.4075471698113208, "precision": 0.9642857142857143, "recall": 0.2583732057416268, "support": 418}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 222}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 223}, "\u2423": {"f1-score": 0.44653830397378125, "precision": 0.9873188405797102, "recall": 0.2885124404446797, "support": 1889}},
  "ppcr": 0.5364133563615429
}
```
</details>
