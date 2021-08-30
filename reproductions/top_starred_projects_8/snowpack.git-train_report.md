# Train report for javascript / file:///tmp/top-repos-quality-repos-s4oh_2zs/snowpack.git HEAD d8cc54a7816faef9d98459a1e24f54912163868b

### Classification report

PPCR: 0.806

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.945| 0.996| 0.971| 0.970| 0.958| 28299| 29018| 0.975 |
| `␣` | 0.977| 0.919| 0.717| 0.947| 0.827| 10875| 13931| 0.781 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 2790| 5580| 0.500 |
| `⏎␣⁻␣⁻` | 0.945| 0.883| 0.793| 0.913| 0.862| 1609| 1790| 0.899 |
| `⏎␣⁺␣⁺` | 0.894| 0.780| 0.517| 0.833| 0.655| 1236| 1866| 0.662 |
| `⏎` | 0.951| 0.439| 0.106| 0.601| 0.190| 845| 3514| 0.240 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 162| 1022| 0.159 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 142| 0.000 |
| `weighted avg` | 0.951| 0.954| 0.769| 0.950| 0.817| 45816| 56863| 0.806 |
| `macro avg` | 0.714| 0.627| 0.450| 0.658| 0.520| 45816| 56863| 0.806 |
| `micro avg` | 0.954| 0.954| 0.769| 0.954| 0.852| 45816| 56863| 0.806 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|719 |28180 |88 |0 |9 |22 |0 |0 |0 |
|3056 |700 |9993 |0 |7 |92 |83 |0 |0 |
|2790 |0 |0 |2790 |0 |0 |0 |0 |0 |
|2669 |383 |91 |0 |371 |0 |0 |0 |0 |
|630 |229 |43 |0 |0 |964 |0 |0 |0 |
|181 |187 |0 |0 |2 |0 |1420 |0 |0 |
|860 |149 |12 |0 |1 |0 |0 |0 |0 |
|142 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/esinstall/import-missing/packages/tslib/tslib.es6.js | 264 |
| test/esinstall/import-missing/packages/tslib/tslib.js | 257 |
| plugins/plugin-webpack/plugin.js | 101 |
| snowpack/assets/hmr-client.js | 100 |
| plugins/plugin-sass/plugin.js | 73 |
| create-snowpack-app/cli/createSnowpackApp.js | 71 |
| plugins/plugin-svelte/plugin.js | 52 |
| test/test-utils.js | 51 |
| plugins/plugin-optimize/lib/css.js | 50 |
| plugins/plugin-optimize/plugin.js | 45 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2790}, "macro avg": {"f1-score": 0.6579138079836411, "precision": 0.7140221326914892, "recall": 0.627026965267348, "support": 45816}, "micro avg": {"f1-score": 0.9542081368954077, "precision": 0.9542081368954077, "recall": 0.9542081368954077, "support": 45816}, "weighted avg": {"f1-score": 0.9502025212843581, "precision": 0.9512162073468947, "recall": 0.9542081368954077, "support": 45816}, "\u2205": {"f1-score": 0.9696010459855146, "precision": 0.9447498994233606, "recall": 0.9957949044135835, "support": 28299}, "\u23ce": {"f1-score": 0.6008097165991902, "precision": 0.9512820512820512, "recall": 0.4390532544378698, "support": 845}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 162}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8331892826274848, "precision": 0.8942486085343229, "recall": 0.7799352750809061, "support": 1236}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9125964010282777, "precision": 0.9447771124417831, "recall": 0.8825357364822871, "support": 1609}, "\u2423": {"f1-score": 0.9471140176286609, "precision": 0.977119389850396, "recall": 0.918896551724138, "support": 10875}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 5580}, "macro avg": {"f1-score": 0.5198890797818152, "precision": 0.7140221326914892, "recall": 0.45049116376824616, "support": 56863}, "micro avg": {"f1-score": 0.8515470544122947, "precision": 0.9542081368954077, "recall": 0.7688303466225841, "support": 56863}, "weighted avg": {"f1-score": 0.8172438756555205, "precision": 0.937509924985912, "recall": 0.7688303466225841, "support": 56863}, "\u2205": {"f1-score": 0.9577541379193149, "precision": 0.9447498994233606, "recall": 0.9711213729409333, "support": 29018}, "\u23ce": {"f1-score": 0.1900614754098361, "precision": 0.9512820512820512, "recall": 0.10557768924302789, "support": 3514}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1022}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6548913043478259, "precision": 0.8942486085343229, "recall": 0.5166130760986066, "support": 1866}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8624354691770423, "precision": 0.9447771124417831, "recall": 0.7932960893854749, "support": 1790}, "\u2423": {"f1-score": 0.8273035847338356, "precision": 0.977119389850396, "recall": 0.7173210824779269, "support": 13931}},
  "ppcr": 0.8057260432970473
}
```
</details>
