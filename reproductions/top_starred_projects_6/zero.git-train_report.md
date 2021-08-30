# Train report for javascript / file:///tmp/top-repos-quality-repos-nh1i2qer/zero.git HEAD e6a72ee49e03008a64c86ee5c34e2120a3b52908

### Classification report

PPCR: 0.694

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.941| 0.992| 0.933| 0.966| 0.937| 13115| 13949| 0.940 |
| `␣` | 0.954| 0.880| 0.345| 0.916| 0.506| 2618| 6687| 0.392 |
| `"` | 1.000| 1.000| 0.927| 1.000| 0.962| 2487| 2684| 0.927 |
| `⏎` | 0.953| 0.653| 0.220| 0.775| 0.357| 591| 1758| 0.336 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 169| 1041| 0.162 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 137| 927| 0.148 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 508| 0.035 |
| `micro avg` | 0.951| 0.951| 0.660| 0.951| 0.779| 19135| 27554| 0.694 |
| `weighted avg` | 0.935| 0.951| 0.660| 0.941| 0.714| 19135| 27554| 0.694 |
| `macro avg` | 0.550| 0.504| 0.346| 0.522| 0.395| 19135| 27554| 0.694 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|834 |13015 |100 |0 |0 |0 |0 |0 |
|4069 |313 |2305 |0 |0 |0 |0 |0 |
|197 |0 |0 |2487 |0 |0 |0 |0 |
|1167 |197 |8 |0 |386 |0 |0 |0 |
|872 |167 |2 |0 |0 |0 |0 |0 |
|790 |136 |0 |0 |1 |0 |0 |0 |
|490 |0 |0 |0 |18 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/core/lib/build.js | 51 |
| packages/builder-react/bundler/bundle.js | 43 |
| packages/handler-react/renderer/index.js | 41 |
| packages/core/lib/manifest/installPackages.js | 39 |
| packages/handler-vue/renderer/index.js | 30 |
| packages/core/lib/router/index.js | 29 |
| packages/parcel-bundler/mods/WorkerFarm.js | 29 |
| packages/process/index.js | 28 |
| packages/core/lib/manifest/buildManifest.js | 28 |
| packages/builder-vue/bundler/bundle.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2487}, "macro avg": {"f1-score": 0.5224526782746659, "precision": 0.5498205733858363, "recall": 0.5037069309913661, "support": 19135}, "micro avg": {"f1-score": 0.95077083877711, "precision": 0.95077083877711, "recall": 0.95077083877711, "support": 19135}, "weighted avg": {"f1-score": 0.9413971373934447, "precision": 0.9350900257278042, "recall": 0.95077083877711, "support": 19135}, "\u2205": {"f1-score": 0.9661136473295476, "precision": 0.9412062481920741, "recall": 0.9923751429660694, "support": 13115}, "\u23ce": {"f1-score": 0.7751004016064257, "precision": 0.9530864197530864, "recall": 0.6531302876480541, "support": 591}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 169}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 137}, "\u2423": {"f1-score": 0.9159546989866878, "precision": 0.9544513457556936, "recall": 0.8804430863254392, "support": 2618}},
  "cl_report_full": {"\"": {"f1-score": 0.9619029201315026, "precision": 1.0, "recall": 0.926602086438152, "support": 2684}, "macro avg": {"f1-score": 0.39462899252452305, "precision": 0.5498205733858363, "recall": 0.34627289159510477, "support": 27554}, "micro avg": {"f1-score": 0.779327036346891, "precision": 0.95077083877711, "recall": 0.6602671118530885, "support": 27554}, "weighted avg": {"f1-score": 0.7137888827795432, "precision": 0.8663289551798464, "recall": 0.6602671118530885, "support": 27554}, "\u2205": {"f1-score": 0.9371062389746913, "precision": 0.9412062481920741, "recall": 0.9330417951107606, "support": 13949}, "\u23ce": {"f1-score": 0.35691169671752193, "precision": 0.9530864197530864, "recall": 0.21956769055745165, "support": 1758}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 508}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1041}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 927}, "\u2423": {"f1-score": 0.5064820918479455, "precision": 0.9544513457556936, "recall": 0.3446986690593689, "support": 6687}},
  "ppcr": 0.6944545256587066
}
```
</details>
