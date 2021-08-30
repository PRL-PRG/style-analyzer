# Train report for javascript / file:///tmp/top-repos-quality-repos-56o0yir_/workflow.git HEAD 59e818e3c3db3b359272d8dad05d09bd6e355266

### Classification report

PPCR: 0.726

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.989| 0.802| 0.983| 0.881| 13884| 17117| 0.811 |
| `␣` | 0.972| 0.977| 0.733| 0.974| 0.836| 7612| 10145| 0.750 |
| `'` | 1.000| 1.000| 0.978| 1.000| 0.989| 3456| 3533| 0.978 |
| `⏎␣⁺␣⁺` | 0.949| 0.896| 0.836| 0.922| 0.889| 1360| 1458| 0.933 |
| `⏎` | 1.000| 0.787| 0.078| 0.881| 0.145| 235| 2368| 0.099 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 76| 1308| 0.058 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 732| 0.000 |
| `macro avg` | 0.700| 0.664| 0.490| 0.680| 0.534| 26623| 36661| 0.726 |
| `weighted avg` | 0.975| 0.977| 0.710| 0.976| 0.783| 26623| 36661| 0.726 |
| `micro avg` | 0.977| 0.977| 0.710| 0.977| 0.822| 26623| 36661| 0.726 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|3233 |13726 |155 |0 |0 |3 |0 |0 |
|2533 |115 |7436 |0 |0 |61 |0 |0 |
|77 |0 |0 |3456 |0 |0 |0 |0 |
|2133 |4 |45 |0 |185 |1 |0 |0 |
|98 |124 |17 |0 |0 |1219 |0 |0 |
|1232 |75 |1 |0 |0 |0 |0 |0 |
|732 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/webpack-box/lib/createMockMiddleware.js | 36 |
| packages/stylelint/lint.js | 26 |
| packages/webpack-box/util/ProjectPackageManager.js | 21 |
| packages/cli-box/index.js | 20 |
| packages/cli/commands/commit.js | 18 |
| packages/cli/util/ProjectPackageManager.js | 17 |
| packages/node-box/bin/index.js | 17 |
| packages/vite-box/bin/index.js | 17 |
| packages/webpack-box/config/style.js | 15 |
| packages/shared-utils/lib/ipc.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3456}, "macro avg": {"f1-score": 0.6800267589309035, "precision": 0.6997502836945303, "recall": 0.664150882702813, "support": 26623}, "micro avg": {"f1-score": 0.9774255343124366, "precision": 0.9774255343124366, "recall": 0.9774255343124366, "support": 26623}, "weighted avg": {"f1-score": 0.9758467730442678, "precision": 0.974607361480505, "recall": 0.9774255343124366, "support": 26623}, "\u2205": {"f1-score": 0.9829561730163277, "precision": 0.9773568783822273, "recall": 0.9886199942379718, "support": 13884}, "\u23ce": {"f1-score": 0.880952380952381, "precision": 1.0, "recall": 0.7872340425531915, "support": 235}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9220877458396369, "precision": 0.9493769470404985, "recall": 0.8963235294117647, "support": 1360}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u2423": {"f1-score": 0.9741910127079785, "precision": 0.9715181604389862, "recall": 0.976878612716763, "support": 7612}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9889826870796966, "precision": 1.0, "recall": 0.9782054910840645, "support": 3533}, "macro avg": {"f1-score": 0.5342239734289829, "precision": 0.6997502836945303, "recall": 0.48961029586317995, "support": 36661}, "micro avg": {"f1-score": 0.8223879653624929, "precision": 0.9774255343124366, "recall": 0.7098006055481302, "support": 36661}, "weighted avg": {"f1-score": 0.782573155915989, "precision": 0.9238880830775796, "recall": 0.7098006055481302, "support": 36661}, "\u2205": {"f1-score": 0.8809730111357145, "precision": 0.9773568783822273, "recall": 0.8018928550563768, "support": 17117}, "\u23ce": {"f1-score": 0.14492753623188406, "precision": 1.0, "recall": 0.078125, "support": 2368}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 732}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8891320204230488, "precision": 0.9493769470404985, "recall": 0.836076817558299, "support": 1458}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1308}, "\u2423": {"f1-score": 0.8355525591325357, "precision": 0.9715181604389862, "recall": 0.732971907343519, "support": 10145}},
  "ppcr": 0.726194048171081
}
```
</details>
