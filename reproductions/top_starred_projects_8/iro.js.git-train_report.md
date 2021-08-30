# Train report for javascript / file:///tmp/top-repos-quality-repos-t86o6_ao/iro.js.git HEAD 979b85d5177d621c69edd20f8da5e1ddb0b34ba6

### Classification report

PPCR: 0.713

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 0.999| 0.856| 0.978| 0.904| 5501| 6423| 0.856 |
| `␣` | 0.984| 0.895| 0.553| 0.937| 0.708| 1821| 2944| 0.619 |
| `'` | 1.000| 1.000| 0.518| 1.000| 0.683| 523| 1009| 0.518 |
| `⏎` | 0.977| 0.858| 0.392| 0.914| 0.559| 346| 758| 0.456 |
| `⏎␣⁻␣⁻` | 0.950| 0.958| 0.881| 0.954| 0.914| 239| 260| 0.919 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 271| 0.063 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 194| 0.052 |
| `weighted avg` | 0.964| 0.967| 0.689| 0.964| 0.779| 8457| 11859| 0.713 |
| `macro avg` | 0.696| 0.673| 0.457| 0.683| 0.538| 8457| 11859| 0.713 |
| `micro avg` | 0.967| 0.967| 0.689| 0.967| 0.805| 8457| 11859| 0.713 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|922 |5496 |2 |0 |0 |0 |3 |0 |
|1123 |184 |1629 |0 |0 |0 |8 |0 |
|486 |0 |0 |523 |0 |0 |0 |0 |
|412 |30 |18 |0 |297 |0 |1 |0 |
|254 |16 |1 |0 |0 |0 |0 |0 |
|21 |4 |6 |0 |0 |0 |229 |0 |
|184 |3 |0 |0 |7 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| docs/.vuepress/theme/util/index.js | 134 |
| tests/color.test.js | 38 |
| rollup.config.js | 37 |
| docs/.vuepress/config.js | 17 |
| docs/.vuepress/theme/index.js | 16 |
| demo/demo.js | 12 |
| tests/util.createWidget.test.js | 8 |
| docs/.vuepress/theme/enhanceApp.js | 7 |
| tests/colorPicker.test.js | 5 |
| tests/util.usePlugins.test.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 523}, "macro avg": {"f1-score": 0.6833551057236723, "precision": 0.6956481703565605, "recall": 0.6728849999635462, "support": 8457}, "micro avg": {"f1-score": 0.9665365969019748, "precision": 0.9665365969019747, "recall": 0.9665365969019747, "support": 8457}, "weighted avg": {"f1-score": 0.9644128125171645, "precision": 0.9640573551578571, "recall": 0.9665365969019747, "support": 8457}, "\u2205": {"f1-score": 0.978458251735802, "precision": 0.9586603872318158, "recall": 0.9990910743501181, "support": 5501}, "\u23ce": {"f1-score": 0.913846153846154, "precision": 0.9769736842105263, "recall": 0.8583815028901735, "support": 346}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9541666666666666, "precision": 0.950207468879668, "recall": 0.9581589958158996, "support": 239}, "\u2423": {"f1-score": 0.9370146678170836, "precision": 0.9836956521739131, "recall": 0.8945634266886326, "support": 1821}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6827676240208878, "precision": 1.0, "recall": 0.5183349851337958, "support": 1009}, "macro avg": {"f1-score": 0.5383952859350247, "precision": 0.6956481703565605, "recall": 0.45713264556975436, "support": 11859}, "micro avg": {"f1-score": 0.8046859618035046, "precision": 0.9665365969019747, "recall": 0.6892655367231638, "support": 11859}, "weighted avg": {"f1-score": 0.7794626635521199, "precision": 0.9317881492309845, "recall": 0.6892655367231638, "support": 11859}, "\u2205": {"f1-score": 0.9042448173741362, "precision": 0.9586603872318158, "recall": 0.8556749182624942, "support": 6423}, "\u23ce": {"f1-score": 0.559322033898305, "precision": 0.9769736842105263, "recall": 0.391820580474934, "support": 758}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 271}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9141716566866267, "precision": 0.950207468879668, "recall": 0.8807692307692307, "support": 260}, "\u2423": {"f1-score": 0.7082608695652173, "precision": 0.9836956521739131, "recall": 0.553328804347826, "support": 2944}},
  "ppcr": 0.7131292689096889
}
```
</details>
