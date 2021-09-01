# Train report for javascript / file:///tmp/top-repos-quality-repos-06dic4vx/hogwarts-london-web-career-042219.git HEAD f9d7c49cd83a18be963c26bfdbfefaa82010d1ce

### Classification report

PPCR: 0.526

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.995| 0.709| 0.975| 0.814| 1781| 2501| 0.712 |
| `⏎` | 0.981| 0.973| 0.472| 0.977| 0.637| 366| 755| 0.485 |
| `␣` | 0.933| 0.809| 0.228| 0.867| 0.366| 293| 1040| 0.282 |
| `'` | 1.000| 1.000| 0.490| 1.000| 0.658| 278| 567| 0.490 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 175| 0.091 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 191| 0.073 |
| `micro avg` | 0.962| 0.962| 0.505| 0.962| 0.663| 2748| 5229| 0.526 |
| `weighted avg` | 0.951| 0.962| 0.505| 0.956| 0.625| 2748| 5229| 0.526 |
| `macro avg` | 0.645| 0.629| 0.316| 0.636| 0.413| 2748| 5229| 0.526 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|720 |1772 |9 |0 |0 |0 |0 |
|747 |56 |237 |0 |0 |0 |0 |
|389 |10 |0 |356 |0 |0 |0 |
|289 |0 |0 |0 |278 |0 |0 |
|177 |6 |7 |1 |0 |0 |0 |
|159 |9 |1 |6 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| config/env.js | 18 |
| src/registerServiceWorker.js | 14 |
| config/webpack.config.prod.js | 12 |
| scripts/build.js | 11 |
| scripts/start.js | 11 |
| config/paths.js | 9 |
| config/webpackDevServer.config.js | 6 |
| config/webpack.config.dev.js | 5 |
| scripts/test.js | 4 |
| src/components/Nav.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 278}, "macro avg": {"f1-score": 0.6364098459810759, "precision": 0.6450123702636701, "recall": 0.6294163291575283, "support": 2748}, "micro avg": {"f1-score": 0.9617903930131004, "precision": 0.9617903930131004, "recall": 0.9617903930131004, "support": 2748}, "weighted avg": {"f1-score": 0.9556965878832568, "precision": 0.9510477588049824, "recall": 0.9617903930131004, "support": 2748}, "\u2205": {"f1-score": 0.9752339020363237, "precision": 0.956287101996762, "recall": 0.9949466591802358, "support": 1781}, "\u23ce": {"f1-score": 0.9766803840877916, "precision": 0.9807162534435262, "recall": 0.9726775956284153, "support": 366}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.8665447897623402, "precision": 0.9330708661417323, "recall": 0.8088737201365188, "support": 293}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6579881656804732, "precision": 1.0, "recall": 0.49029982363315694, "support": 567}, "macro avg": {"f1-score": 0.4125183141584982, "precision": 0.6450123702636701, "recall": 0.31637070186472904, "support": 5229}, "micro avg": {"f1-score": 0.6626551335088379, "precision": 0.9617903930131004, "recall": 0.5054503729202524, "support": 5229}, "weighted avg": {"f1-score": 0.625470424224676, "precision": 0.8930022019948681, "recall": 0.5054503729202524, "support": 5229}, "\u2205": {"f1-score": 0.8139641708773541, "precision": 0.956287101996762, "recall": 0.7085165933626549, "support": 2501}, "\u23ce": {"f1-score": 0.6368515205724508, "precision": 0.9807162534435262, "recall": 0.47152317880794703, "support": 755}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 191}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u2423": {"f1-score": 0.366306027820711, "precision": 0.9330708661417323, "recall": 0.22788461538461538, "support": 1040}},
  "ppcr": 0.525530694205393
}
```
</details>
